;;; The kernel: walks a Proof certificate, transforming Sequents,
;;; accepting iff the original Goal is closed.
;;;
;;; A Sequent is the proof-time state at a node of the proof tree:
;;;   - the goal's ∀-bound vars, opened to FVars carrying their types
;;;   - in-scope hypotheses (added by Induct / CaseOn)
;;;   - the goal's own premises (assumed in this sub-goal)
;;;   - the current lhs = rhs to prove
;;;
;;; Top-level entry (sketched): check_theorem opens the claim's
;;; ∀-binders to fresh FVars, builds the initial Sequent, then
;;; dispatches the Proof.
;;;
;;; Effectful primitive (see REVISIT.md — Fresh-symbol generation):
;;;   (gen_fresh) -> Symbol   ; runtime-provided, guaranteed unique
;;;
;;; This file is a SKELETON: dispatchers are concrete, Refl/Absurd/
;;; Steps/resolve_eq are concrete, and Induct / CaseOn / Rewrite /
;;; RewriteWith / Simp / Unfold / ByTheory are stubbed pending the
;;; design conversations they each merit.

;; ---------------------------------------------------------------------------
;; Sequent: the proof-time state. (Param is defined in proof.sexp; an
;; opened ∀-var carries the same shape as a Goal's bound var.)
;; ---------------------------------------------------------------------------

(type Sequent
  (Sequent
    (List Param)                          ; bound vars opened as FVars
    (List Goal)                           ; in-scope hypotheses (∀-eqs)
    (List Equation)                       ; goal's own premises
    Equation))                            ; current lhs = rhs

;; ---------------------------------------------------------------------------
;; List nth helpers, specialized. In the full language these are one
;; polymorphic `nth`; narrow eats the duplication.
;; ---------------------------------------------------------------------------

(fn nth_goal ((gs (List Goal)) (k Int)) (Option Goal)
  (match gs
    (Nil None)
    ((Cons h t)
      (if (int_eq k 0) (Some h) (nth_goal t (- k 1))))))

(fn nth_eq ((es (List Equation)) (k Int)) (Option Equation)
  (match es
    (Nil None)
    ((Cons h t)
      (if (int_eq k 0) (Some h) (nth_eq t (- k 1))))))

(fn lookup_lemma ((name Symbol) (th Theory)) (Option Goal)
  (match th
    ((TheoryEmpty) None)
    ((TheoryCons entry rest)
      (match entry
        ((Proven n g)
          (if (sym_eq name n) (Some g) (lookup_lemma name rest)))
        ((Axiom n g)
          (if (sym_eq name n) (Some g) (lookup_lemma name rest)))))))

;; ---------------------------------------------------------------------------
;; resolve_eq: turn an EqRef into a Goal (which may carry vars and
;; premises). Premises are promoted to vacuous-quantified Goals so all
;; three cases share a shape.
;; ---------------------------------------------------------------------------

(fn resolve_eq ((er EqRef) (seq Sequent) (th Theory)) (Option Goal)
  (match seq
    ((Sequent _ hyps premises _)
      (match er
        ((Hyp k)     (nth_goal hyps k))
        ((Premise k)
          (match (nth_eq premises k)
            ((Some eq) (Some (Goal Nil Nil eq)))
            (None      None)))
        ((Lemma n)   (lookup_lemma n th))))))

;; ---------------------------------------------------------------------------
;; head_clash: True iff `a` and `b` must be distinct because their
;; outermost structure can't unify under any binding. Used by Absurd
;; to detect contradictory equations after simp.
;;
;; Distinct ctors / distinct IntLits / distinct SymLits, OR a
;; cross-variant value-type mismatch (Ctor vs IntLit, etc.) — these
;; all clash. FVar / BVar / stuck Call / stuck Match / Let / If on
;; either side means "we can't decide" — return False.
;; ---------------------------------------------------------------------------

(fn head_clash ((a Expr) (b Expr)) Bool
  (match a
    ((Ctor na _)
      (match b
        ((Ctor nb _) (if (sym_eq na nb) False True))
        ((IntLit _)  True)
        ((SymLit _)  True)
        (_           False)))
    ((IntLit na)
      (match b
        ((IntLit nb) (if (int_eq na nb) False True))
        ((Ctor _ _)  True)
        ((SymLit _)  True)
        (_           False)))
    ((SymLit na)
      (match b
        ((SymLit nb) (if (sym_eq na nb) False True))
        ((Ctor _ _)  True)
        ((IntLit _)  True)
        (_           False)))
    (_ False)))

;; ---------------------------------------------------------------------------
;; check_sequent: dispatch on the Proof. Returns True iff accepted.
;; ---------------------------------------------------------------------------

(fn check_sequent ((m Module) (th Theory) (seq Sequent) (pf Proof)) Bool
  (match pf
    ((Refl)
      (match seq
        ((Sequent _ _ _ (Equation l r))
          (expr_eq l r))))

    ((Steps steps rest)
      (match (apply_steps m th seq steps)
        ((Some seq2) (check_sequent m th seq2 rest))
        (None        False)))

    ((Induct var cases)
      (do_induct m th seq var cases))

    ((Induct2 var cases)
      (do_induct2 m th seq var cases))

    ((CaseOn scrut ty cases)
      (do_case_on m th seq scrut ty cases))

    ((WfInduct measure pf)
      (do_wf_induct m th seq measure pf))

    ((RewriteWith er dir side insts premise_proofs rest)
      ;; Conditional rewrite. Cited equation may have ∀-binders AND
      ;; non-empty premises:
      ;;   1. Resolve er → cited Goal.
      ;;   2. Open its ∀-binders to fresh FVars (names = pat_vars).
      ;;   3. Match the (opened) conclusion against the chosen Side
      ;;      of the goal eq via apply_rewrite_with_env. The match
      ;;      yields a binding env over pat_vars AND the rewritten
      ;;      equation.
      ;;   4. For each cited premise (also opened), substitute the
      ;;      env and discharge with the corresponding sub-proof.
      ;;   5. If all premises discharge, continue with `rest` on the
      ;;      rewritten sequent.
      ;;
      ;; v2 limitations (documented in REVISIT):
      ;;   - single match site only (no all-occurrences variant).
      ;;   - Both selector picks lhs match first, falls back to rhs.
      ;; Insts pre-instantiate cited ∀-binders before the match, same
      ;; mechanism as Rewrite's Inst path (slice 32).
      (match (resolve_eq er seq th)
        (None False)
        ((Some g)
          (match g
            ((Goal cited_params cited_premises cited_eq)
              (if (all_insts_named insts cited_params)
                  (match (split_params_by_insts cited_params insts)
                    ((Pair openings pat_var_names)
                      (let ((opening_fvars (reverse_exprs openings)))
                        (let ((opened_eq (open_eq_with opening_fvars cited_eq))
                              (opened_premises
                                (open_eqs_with opening_fvars cited_premises)))
                          (match seq
                            ((Sequent s_params s_hyps s_premises goal_eq)
                              (match (apply_rewrite_with_env
                                       pat_var_names opened_eq dir side goal_eq)
                                (None False)
                                ((Some (Pair env new_eq))
                                  (if (check_premise_proofs
                                         m th s_params s_hyps s_premises
                                         env opened_premises premise_proofs)
                                      (check_sequent m th
                                        (Sequent s_params s_hyps s_premises new_eq)
                                        rest)
                                      False)))))))))
                  False))))))

    ((Absurd er)
      ;; Close the current goal from a contradictory in-scope equation.
      ;; Resolve er, require it to be a ground Goal (no ∀-binders, no
      ;; premises), simp both sides, accept iff they can't unify at
      ;; the head. v2 limitation: ∀-binders not yet supported — open
      ;; with fresh FVars + premise discharge belong to a later slice.
      (match (resolve_eq er seq th)
        (None False)
        ((Some g)
          (match g
            ((Goal Nil Nil (Equation l r))
              (head_clash (simp_expr m l) (simp_expr m r)))
            (_ False)))))

    ((ByTheory theory_name cert)
      ;; Per-theory dispatch. The cert's payload is theory-specific.
      ;; v2 registers two theories:
      ;;   lia   — linear integer arithmetic. Decides by normalizing
      ;;           both sides of the goal eq into polynomials and
      ;;           checking lhs - rhs canonicalizes to all zero
      ;;           coefficients. Cert payload unused (poly-time; no
      ;;           checking/searching asymmetry to exploit).
      ;;   eqdec — equality-reflection: decides `(int_eq a b) = True`
      ;;           via lia_decide and `(sym_eq a b) = True` via
      ;;           expr_eq. See eqdec.sexp. This is what discharges
      ;;           reflexivity facts like `int_eq k k = True` that the
      ;;           reducer leaves stuck on variables.
      ;;   ord   — order-reflection: decides `(lt a b) = True` and
      ;;           `(le a b) = True` when (b - a) canonicalizes to a
      ;;           constant of the right sign. See ord.sexp.
      ;;   farkas— linear-integer ENTAILMENT: premises ⊢ (lt|le a b)=True
      ;;           via a cert-supplied Farkas combination. The first
      ;;           backend that reads the sequent's PREMISES and the
      ;;           Cert payload. See farkas.sexp.
      (if (sym_eq theory_name (quote lia))
          (match seq
            ((Sequent _ _ _ (Equation lhs rhs))
              (lia_decide lhs rhs)))
          (if (sym_eq theory_name (quote eqdec))
              (match seq
                ((Sequent _ _ _ (Equation lhs rhs))
                  (eqdec_decide lhs rhs)))
              (if (sym_eq theory_name (quote ord))
                  (match seq
                    ((Sequent _ _ _ (Equation lhs rhs))
                      (ord_decide lhs rhs)))
                  (if (sym_eq theory_name (quote farkas))
                      (match seq
                        ((Sequent _ _ premises goal_eq)
                          (farkas_check premises goal_eq cert)))
                      False)))))))                    ; unknown theory

;; ---------------------------------------------------------------------------
;; apply_steps / apply_step: run non-branching steps, threading the
;; transformed Sequent. None means the step failed (kernel-rejection).
;; ---------------------------------------------------------------------------

(fn apply_steps ((m Module) (th Theory) (seq Sequent) (steps (List Step)))
                (Option Sequent)
  (match steps
    (Nil (Some seq))
    ((Cons s rest)
      (match (apply_step m th seq s)
        ((Some seq2) (apply_steps m th seq2 rest))
        (None        None)))))

(fn apply_step ((m Module) (th Theory) (seq Sequent) (s Step)) (Option Sequent)
  (match s
    ((Unfold fname side)
      ;; Replace ONE occurrence of (Call fname …) with fname's body
      ;; opened. Distinct from Reduce: preserves the surface form of
      ;; everything else (primitive calls, other user fns), so the
      ;; proof can expose structure without driving to NF.
      (match seq
        ((Sequent params hyps premises eq)
          (match (unfold_one_side_eq m fname side eq)
            ((Some eq2) (Some (Sequent params hyps premises eq2)))
            (None       None)))))
    ((Reduce side)
      ;; ι-only: fire matches, dispatch True/False ifs, open lets,
      ;; descend Ctor / Call args. Does NOT unfold Calls. This is
      ;; what IH-consuming inductive proofs need — Reduce stops at
      ;; the recursive sub-call instead of chasing it forever.
      (match seq
        ((Sequent params hyps premises eq)
          (Some (Sequent params hyps premises
                  (reduce_iota_side_eq m side eq))))))
    ((Simp side)
      ;; Full δ+ι: drives `step` (the kernel's universal reducer) to
      ;; fixed point — unfolds user fns, fires primitives, evaluates
      ;; matches, ifs, lets. The workhorse.
      (match seq
        ((Sequent params hyps premises eq)
          (Some (Sequent params hyps premises
                  (simp_side_eq m side eq))))))
    ((Compute side)
      ;; UNGATED δ+ι to normal form: like Simp but unfolds user fns
      ;; unconditionally, so a ground term whose control flow is guarded
      ;; by a user predicate evaluates to a value. Intended for closed
      ;; terms (may not terminate on symbolic input). Sound for the same
      ;; reason Simp is — every step is a definitional-equality reduction.
      (match seq
        ((Sequent params hyps premises eq)
          (Some (Sequent params hyps premises
                  (compute_side_eq m side eq))))))
    ((Rewrite er dir side all insts)
      ;; Pattern-variable Rewrite. The cited Goal's ∀-binders become
      ;; capture variables (fresh FVars listed in pat_vars) unless
      ;; pre-instantiated by an Inst — those binders are pinned to a
      ;; user-supplied Expr before the conclusion match runs.
      ;; Premises must still be Nil (unconditional). RewriteWith
      ;; handles the conditional case.
      (match (resolve_eq er seq th)
        (None None)
        ((Some g)
          (match g
            ((Goal cited_params Nil cited_eq)
              (if (all_insts_named insts cited_params)
                  (match (split_params_by_insts cited_params insts)
                    ((Pair openings pat_var_names)
                      (let ((opening_fvars (reverse_exprs openings)))
                        (let ((opened_eq (open_eq_with opening_fvars cited_eq)))
                          (match seq
                            ((Sequent s_params hyps premises goal_eq)
                              (match (apply_rewrite pat_var_names opened_eq
                                                    dir side all goal_eq)
                                ((Some new_eq)
                                  (Some (Sequent s_params hyps premises new_eq)))
                                (None None))))))))
                  None))
            (_ None)))))))                                  ;; non-Nil premises

;; Drive simp_expr (full δ+ι) on the chosen side(s) of an equation.
;; Used by apply_step's `Simp` arm.
(fn simp_side_eq ((m Module) (side Side) (eq Equation)) Equation
  (match eq
    ((Equation l r)
      (match side
        ((Lhs)  (Equation (simp_expr m l) r))
        ((Rhs)  (Equation l               (simp_expr m r)))
        ((Both) (Equation (simp_expr m l) (simp_expr m r)))))))

;; Drive compute_expr (ungated δ+ι to NF) on the chosen side(s).
;; Used by apply_step's `Compute` arm.
(fn compute_side_eq ((m Module) (side Side) (eq Equation)) Equation
  (match eq
    ((Equation l r)
      (match side
        ((Lhs)  (Equation (compute_expr m l) r))
        ((Rhs)  (Equation l                 (compute_expr m r)))
        ((Both) (Equation (compute_expr m l) (compute_expr m r)))))))

;; Drive simp_iota_expr (ι-only) on the chosen side(s) of an equation.
;; Used by apply_step's `Reduce` arm.
(fn reduce_iota_side_eq ((m Module) (side Side) (eq Equation)) Equation
  (match eq
    ((Equation l r)
      (match side
        ((Lhs)  (Equation (simp_iota_expr m l) r))
        ((Rhs)  (Equation l                    (simp_iota_expr m r)))
        ((Both) (Equation (simp_iota_expr m l) (simp_iota_expr m r)))))))

;; ---------------------------------------------------------------------------
;; Pattern-variable rewriting on Expr values. The cited equation can
;; have ∀-bound vars; they're opened to fresh FVars before matching,
;; and listed in `pat_vars` so expr_match knows to treat them as
;; capture variables rather than literal FVars.
;;
;; rewrite_first walks top-down left-to-right, replaces the FIRST hit,
;; returns Some on success. rewrite_all walks the whole tree replacing
;; every non-overlapping match (does NOT recurse into a replacement
;; so cycles like (x = (f x)) are bounded); returns Some if at least
;; one match was found, None if no match anywhere.
;;
;; Neither descends under binders (Match arm bodies, Let bodies) —
;; depth-aware rewriting still pending.
;; ---------------------------------------------------------------------------

;; sym_member: True iff x is in xs.
(fn sym_member ((x Symbol) (xs (List Symbol))) Bool
  (match xs
    (Nil False)
    ((Cons h t) (if (sym_eq x h) True (sym_member x t)))))

;; expr_match: structural match of `pat` against `cand`, treating
;; FVars whose name is in `pat_vars` as CAPTURE variables. Other
;; FVars must match literally (same name). Returns Some env on
;; successful match — env binds each captured pat_var to the
;; matched Expr. Repeated occurrences of the same pat_var must
;; match consistently (the second sighting checks expr_eq against
;; the first binding).
;;
;; v2 limitation: no descent under Match arm bodies / Let bodies.
;; A pattern that contains a Match or Let just fails to match.
(fn expr_match ((pat_vars (List Symbol)) (pat Expr) (cand Expr) (env Env))
                (Option Env)
  (match pat
    ((FVar x)
      (if (sym_member x pat_vars)
          (match (lookup x env)
            ((Some v) (if (expr_eq v cand) (Some env) None))
            (None     (Some (Bind x cand env))))
          (match cand
            ((FVar y) (if (sym_eq x y) (Some env) None))
            (_        None))))
    ((BVar k)
      (match cand
        ((BVar j) (if (int_eq k j) (Some env) None))
        (_        None)))
    ((Ctor c args)
      (match cand
        ((Ctor c2 args2)
          (if (sym_eq c c2) (expr_match_list pat_vars args args2 env) None))
        (_ None)))
    ((Call f args)
      (match cand
        ((Call f2 args2)
          (if (sym_eq f f2) (expr_match_list pat_vars args args2 env) None))
        (_ None)))
    ((If c t el)
      (match cand
        ((If c2 t2 el2)
          (match (expr_match pat_vars c c2 env)
            ((Some env1)
              (match (expr_match pat_vars t t2 env1)
                ((Some env2) (expr_match pat_vars el el2 env2))
                (None        None)))
            (None None)))
        (_ None)))
    ((IntLit n)
      (match cand
        ((IntLit m) (if (int_eq n m) (Some env) None))
        (_          None)))
    ((SymLit s)
      (match cand
        ((SymLit t) (if (sym_eq s t) (Some env) None))
        (_          None)))
    (_ None)))                                  ;; Match/Let in pat: not yet

(fn expr_match_list ((pat_vars (List Symbol)) (ps (List Expr)) (cs (List Expr)) (env Env))
                     (Option Env)
  (match ps
    (Nil
      (match cs
        (Nil (Some env))
        (_   None)))
    ((Cons p prest)
      (match cs
        (Nil None)
        ((Cons c crest)
          (match (expr_match pat_vars p c env)
            ((Some env2) (expr_match_list pat_vars prest crest env2))
            (None        None)))))))

;; ---------------------------------------------------------------------------
;; Rewriter (pattern-variable aware). When pat_vars is Nil, expr_match
;; reduces to expr_eq and rewrite_first/all behave as the old ground
;; rewriters did. The successful-match arm of each top-level case
;; substitutes the captured env into the replacement.
;; ---------------------------------------------------------------------------

(fn rewrite_first ((pat_vars (List Symbol)) (pat Expr) (repl Expr) (e Expr))
                   (Option Expr)
  (match (expr_match pat_vars pat e Empty)
    ((Some env) (Some (subst env repl)))
    (None
      (match e
        ((Ctor c args)
          (match (rewrite_first_list pat_vars pat repl args)
            ((Some args2) (Some (Ctor c args2)))
            (None         None)))
        ((Call f args)
          (match (rewrite_first_list pat_vars pat repl args)
            ((Some args2) (Some (Call f args2)))
            (None         None)))
        ((If c t el)
          (match (rewrite_first pat_vars pat repl c)
            ((Some c2) (Some (If c2 t el)))
            (None
              (match (rewrite_first pat_vars pat repl t)
                ((Some t2) (Some (If c t2 el)))
                (None
                  (match (rewrite_first pat_vars pat repl el)
                    ((Some el2) (Some (If c t el2)))
                    (None       None)))))))
        ;; Descend into a Match SCRUTINEE only — it shares the enclosing
        ;; binder scope (no shift), so rewriting it is plain congruence.
        ;; Arm bodies bind pattern vars (de Bruijn) and are NOT entered.
        ((Match scrut arms)
          (match (rewrite_first pat_vars pat repl scrut)
            ((Some scrut2) (Some (Match scrut2 arms)))
            (None          None)))
        (_ None)))))

(fn rewrite_first_list ((pat_vars (List Symbol)) (pat Expr) (repl Expr) (es (List Expr)))
                        (Option (List Expr))
  (match es
    (Nil None)
    ((Cons h t)
      (match (rewrite_first pat_vars pat repl h)
        ((Some h2) (Some (Cons h2 t)))
        (None
          (match (rewrite_first_list pat_vars pat repl t)
            ((Some t2) (Some (Cons h t2)))
            (None      None)))))))

(fn rewrite_all ((pat_vars (List Symbol)) (pat Expr) (repl Expr) (e Expr))
                 (Option Expr)
  (match (expr_match pat_vars pat e Empty)
    ((Some env) (Some (subst env repl)))      ; don't recurse into repl
    (None
      (match e
        ((Ctor c args)
          (match (rewrite_all_list pat_vars pat repl args)
            ((Some args2) (Some (Ctor c args2)))
            (None         None)))
        ((Call f args)
          (match (rewrite_all_list pat_vars pat repl args)
            ((Some args2) (Some (Call f args2)))
            (None         None)))
        ((If c t el)
          (rewrite_all_if pat_vars pat repl c t el))
        ;; Match scrutinee only (same scope, no shift); arm bodies skipped.
        ((Match scrut arms)
          (match (rewrite_all pat_vars pat repl scrut)
            ((Some scrut2) (Some (Match scrut2 arms)))
            (None          None)))
        (_ None)))))

(fn rewrite_all_list ((pat_vars (List Symbol)) (pat Expr) (repl Expr) (es (List Expr)))
                      (Option (List Expr))
  (match es
    (Nil None)
    ((Cons h t)
      (match (rewrite_all pat_vars pat repl h)
        ((Some h2)
          (match (rewrite_all_list pat_vars pat repl t)
            ((Some t2) (Some (Cons h2 t2)))
            (None      (Some (Cons h2 t)))))
        (None
          (match (rewrite_all_list pat_vars pat repl t)
            ((Some t2) (Some (Cons h t2)))
            (None      None)))))))

(fn rewrite_all_if ((pat_vars (List Symbol)) (pat Expr) (repl Expr)
                    (c Expr) (t Expr) (el Expr)) (Option Expr)
  (match (rewrite_all pat_vars pat repl c)
    ((Some c2)
      (match (rewrite_all pat_vars pat repl t)
        ((Some t2)
          (match (rewrite_all pat_vars pat repl el)
            ((Some el2) (Some (If c2 t2 el2)))
            (None       (Some (If c2 t2 el)))))
        (None
          (match (rewrite_all pat_vars pat repl el)
            ((Some el2) (Some (If c2 t el2)))
            (None       (Some (If c2 t el)))))))
    (None
      (match (rewrite_all pat_vars pat repl t)
        ((Some t2)
          (match (rewrite_all pat_vars pat repl el)
            ((Some el2) (Some (If c t2 el2)))
            (None       (Some (If c t2 el)))))
        (None
          (match (rewrite_all pat_vars pat repl el)
            ((Some el2) (Some (If c t el2)))
            (None       None)))))))

;; ---------------------------------------------------------------------------
;; rewrite_first_with_env: same walk as rewrite_first, but ALSO returns
;; the binding env produced by expr_match at the successful match site.
;;
;; RewriteWith needs this env so it can substitute it into the cited
;; equation's premises and dispatch each as a sub-sequent. Plain
;; rewrite_first discards the env after using it to subst repl.
;;
;; First-occurrence only; no `all_occ` variant. Multi-match would
;; require carrying a List Env (different match sites can bind the
;; same pat_var differently) — out of scope for v2; documented in
;; REVISIT.md, "RewriteWith — single-match only".
;; ---------------------------------------------------------------------------

(fn rewrite_first_with_env ((pat_vars (List Symbol)) (pat Expr) (repl Expr) (e Expr))
                            (Option (Pair Env Expr))
  (match (expr_match pat_vars pat e Empty)
    ((Some env) (Some (Pair env (subst env repl))))
    (None
      (match e
        ((Ctor c args)
          (match (rewrite_first_list_with_env pat_vars pat repl args)
            ((Some (Pair env args2)) (Some (Pair env (Ctor c args2))))
            (None None)))
        ((Call f args)
          (match (rewrite_first_list_with_env pat_vars pat repl args)
            ((Some (Pair env args2)) (Some (Pair env (Call f args2))))
            (None None)))
        ((If c t el)
          (match (rewrite_first_with_env pat_vars pat repl c)
            ((Some (Pair env c2)) (Some (Pair env (If c2 t el))))
            (None
              (match (rewrite_first_with_env pat_vars pat repl t)
                ((Some (Pair env t2)) (Some (Pair env (If c t2 el))))
                (None
                  (match (rewrite_first_with_env pat_vars pat repl el)
                    ((Some (Pair env el2)) (Some (Pair env (If c t el2))))
                    (None None)))))))
        ;; Match scrutinee only (same scope, no shift); arm bodies skipped.
        ((Match scrut arms)
          (match (rewrite_first_with_env pat_vars pat repl scrut)
            ((Some (Pair env scrut2)) (Some (Pair env (Match scrut2 arms))))
            (None None)))
        (_ None)))))

(fn rewrite_first_list_with_env
    ((pat_vars (List Symbol)) (pat Expr) (repl Expr) (es (List Expr)))
    (Option (Pair Env (List Expr)))
  (match es
    (Nil None)
    ((Cons h t)
      (match (rewrite_first_with_env pat_vars pat repl h)
        ((Some (Pair env h2)) (Some (Pair env (Cons h2 t))))
        (None
          (match (rewrite_first_list_with_env pat_vars pat repl t)
            ((Some (Pair env t2)) (Some (Pair env (Cons h t2))))
            (None None)))))))

;; ---------------------------------------------------------------------------
;; apply_rewrite: flip the cited equation by Dir, then rewrite on the
;; chosen Side using either rewrite_first or rewrite_all per `all`.
;; ---------------------------------------------------------------------------

(fn apply_rewrite ((pat_vars (List Symbol)) (cited Equation) (dir Dir) (side Side)
                   (all_occ Bool) (goal_eq Equation)) (Option Equation)
  (match cited
    ((Equation cl cr)
      (match dir
        ((Lr) (rewrite_side pat_vars cl cr side all_occ goal_eq))
        ((Rl) (rewrite_side pat_vars cr cl side all_occ goal_eq))))))

(fn rewrite_side ((pat_vars (List Symbol)) (pat Expr) (repl Expr) (side Side)
                  (all_occ Bool) (goal_eq Equation)) (Option Equation)
  (match goal_eq
    ((Equation gl gr)
      (match side
        ((Lhs)
          (match (rewrite_one_or_all pat_vars all_occ pat repl gl)
            ((Some gl2) (Some (Equation gl2 gr)))
            (None       None)))
        ((Rhs)
          (match (rewrite_one_or_all pat_vars all_occ pat repl gr)
            ((Some gr2) (Some (Equation gl gr2)))
            (None       None)))
        ((Both)
          (match (rewrite_one_or_all pat_vars all_occ pat repl gl)
            ((Some gl2)
              (match (rewrite_one_or_all pat_vars all_occ pat repl gr)
                ((Some gr2) (Some (Equation gl2 gr2)))
                (None       (Some (Equation gl2 gr)))))
            (None
              (match (rewrite_one_or_all pat_vars all_occ pat repl gr)
                ((Some gr2) (Some (Equation gl gr2)))
                (None       None)))))))))

(fn rewrite_one_or_all ((pat_vars (List Symbol)) (all_occ Bool)
                        (pat Expr) (repl Expr) (e Expr)) (Option Expr)
  (if all_occ
      (rewrite_all pat_vars pat repl e)
      (rewrite_first pat_vars pat repl e)))

;; ---------------------------------------------------------------------------
;; apply_rewrite_with_env: like apply_rewrite, but for RewriteWith. Uses
;; rewrite_first_with_env (always single-match — see REVISIT) and
;; returns the binding env alongside the new equation.
;;
;; Both side semantics: lhs-first single match. If lhs matches we use
;; that env; otherwise we try rhs. Multi-site Both would require
;; per-site envs in the cert shape; deferred.
;; ---------------------------------------------------------------------------

(fn apply_rewrite_with_env ((pat_vars (List Symbol)) (cited Equation)
                            (dir Dir) (side Side) (goal_eq Equation))
                            (Option (Pair Env Equation))
  (match cited
    ((Equation cl cr)
      (match dir
        ((Lr) (rewrite_side_with_env pat_vars cl cr side goal_eq))
        ((Rl) (rewrite_side_with_env pat_vars cr cl side goal_eq))))))

(fn rewrite_side_with_env ((pat_vars (List Symbol)) (pat Expr) (repl Expr)
                            (side Side) (goal_eq Equation))
                            (Option (Pair Env Equation))
  (match goal_eq
    ((Equation gl gr)
      (match side
        ((Lhs)
          (match (rewrite_first_with_env pat_vars pat repl gl)
            ((Some (Pair env gl2)) (Some (Pair env (Equation gl2 gr))))
            (None None)))
        ((Rhs)
          (match (rewrite_first_with_env pat_vars pat repl gr)
            ((Some (Pair env gr2)) (Some (Pair env (Equation gl gr2))))
            (None None)))
        ((Both)
          (match (rewrite_first_with_env pat_vars pat repl gl)
            ((Some (Pair env gl2)) (Some (Pair env (Equation gl2 gr))))
            (None
              (match (rewrite_first_with_env pat_vars pat repl gr)
                ((Some (Pair env gr2)) (Some (Pair env (Equation gl gr2))))
                (None None)))))))))

;; ---------------------------------------------------------------------------
;; check_premise_proofs: dispatch one sub-proof per cited-equation
;; premise. Each premise, after env-substitution by the rewriter's
;; match binding, becomes a concrete Equation to discharge in the
;; PARENT sequent's context (params, hyps, premises preserved).
;;
;; Arity mismatch (more or fewer proofs than premises) returns False.
;; Any sub-proof failing returns False. All pass → True.
;; ---------------------------------------------------------------------------

(fn check_premise_proofs
    ((m Module) (th Theory)
     (s_params (List Param)) (s_hyps (List Goal)) (s_premises (List Equation))
     (env Env) (cited_premises (List Equation)) (proofs (List Proof))) Bool
  (match cited_premises
    (Nil
      (match proofs
        (Nil True)                                    ; arity OK
        (_   False)))                                 ; too many proofs
    ((Cons cp cp_rest)
      (match proofs
        (Nil False)                                   ; too few proofs
        ((Cons pf pf_rest)
          (let ((sub_seq (Sequent s_params s_hyps s_premises
                                  (subst_eq env cp))))
            (if (check_sequent m th sub_seq pf)
                (check_premise_proofs m th s_params s_hyps s_premises
                                      env cp_rest pf_rest)
                False)))))))

;; Unfold one occurrence on the chosen side(s) of an equation.
;; For Both: succeed if at least one side has a matching occurrence.
;; For Lhs/Rhs: fail (None) if no occurrence on that side.
(fn unfold_one_side_eq ((m Module) (fname Symbol) (side Side) (eq Equation))
                       (Option Equation)
  (match eq
    ((Equation l r)
      (match side
        ((Lhs)
          (match (unfold_one m fname l)
            ((Some l2) (Some (Equation l2 r)))
            (None      None)))
        ((Rhs)
          (match (unfold_one m fname r)
            ((Some r2) (Some (Equation l r2)))
            (None      None)))
        ((Both)
          (match (unfold_one m fname l)
            ((Some l2)
              (match (unfold_one m fname r)
                ((Some r2) (Some (Equation l2 r2)))
                (None      (Some (Equation l2 r)))))
            (None
              (match (unfold_one m fname r)
                ((Some r2) (Some (Equation l r2)))
                (None      None)))))))))

;; ---------------------------------------------------------------------------
;; Equation / Goal substitution. FVar-based; the hyp's own bound vars
;; (BVars in its body) are untouched, so no shadowing concerns —
;; locally-nameless dissolves v1's "unless they shadow var" check.
;; ---------------------------------------------------------------------------

(fn subst_eq ((env Env) (eq Equation)) Equation
  (match eq
    ((Equation l r) (Equation (subst env l) (subst env r)))))

(fn subst_eqs ((env Env) (eqs (List Equation))) (List Equation)
  (match eqs
    (Nil Nil)
    ((Cons e rest) (Cons (subst_eq env e) (subst_eqs env rest)))))

(fn subst_goal ((env Env) (g Goal)) Goal
  (match g
    ((Goal params premises eq)
      (Goal params (subst_eqs env premises) (subst_eq env eq)))))

(fn subst_goals ((env Env) (gs (List Goal))) (List Goal)
  (match gs
    (Nil Nil)
    ((Cons g rest) (Cons (subst_goal env g) (subst_goals env rest)))))

;; ---------------------------------------------------------------------------
;; Equation closing (close_many lifted to Equation and lists thereof).
;; ---------------------------------------------------------------------------

(fn close_eq ((names (List Symbol)) (eq Equation)) Equation
  (match eq
    ((Equation l r) (Equation (close_many names l) (close_many names r)))))

(fn close_eqs ((names (List Symbol)) (eqs (List Equation))) (List Equation)
  (match eqs
    (Nil Nil)
    ((Cons e rest) (Cons (close_eq names e) (close_eqs names rest)))))

;; ---------------------------------------------------------------------------
;; close_goal_for_storage: convert a Goal that was authored / proved with
;; param-name FVars in its eq + premises into the canonical BVar-closed
;; form that the kernel uses for stored lemmas. The Sequent-form (open
;; FVars) is convenient for top-level proof steps (Induct, Rewrite by
;; name); the Theory-form (closed BVars) is what citations open back to
;; fresh FVars. This helper bridges the two.
;;
;; Innermost-first convention: BVar 0 = last param. Hence reverse_syms
;; on (param_names params) before close_many: close_many treats the
;; FIRST name in its list as BVar 0.
;; ---------------------------------------------------------------------------

(fn close_goal_for_storage ((g Goal)) Goal
  (match g
    ((Goal params premises eq)
      (let ((names (reverse_syms (param_names params))))
        (Goal params
              (close_eqs names premises)
              (close_eq  names eq))))))

;; ---------------------------------------------------------------------------
;; Equation opening (open_many lifted to Equation and lists thereof).
;; ---------------------------------------------------------------------------

(fn open_eq_with ((bindings (List Expr)) (eq Equation)) Equation
  (match eq
    ((Equation l r) (Equation (open_many bindings l) (open_many bindings r)))))

(fn open_eqs_with ((bindings (List Expr)) (eqs (List Equation))) (List Equation)
  (match eqs
    (Nil Nil)
    ((Cons e rest) (Cons (open_eq_with bindings e) (open_eqs_with bindings rest)))))

;; ---------------------------------------------------------------------------
;; Param / List helpers.
;; ---------------------------------------------------------------------------

(fn find_param ((name Symbol) (ps (List Param))) (Option Param)
  (match ps
    (Nil None)
    ((Cons p rest)
      (match p
        ((Param x _)
          (if (sym_eq x name) (Some p) (find_param name rest)))))))

(fn remove_param ((name Symbol) (ps (List Param))) (List Param)
  (match ps
    (Nil Nil)
    ((Cons p rest)
      (match p
        ((Param x _)
          (if (sym_eq x name)
              rest
              (Cons p (remove_param name rest))))))))

(fn param_names ((ps (List Param))) (List Symbol)
  (match ps
    (Nil Nil)
    ((Cons (Param x _) rest) (Cons x (param_names rest)))))

(fn params_to_fvar_exprs ((ps (List Param))) (List Expr)
  (match ps
    (Nil Nil)
    ((Cons (Param x _) rest) (Cons (FVar x) (params_to_fvar_exprs rest)))))

(fn append_params ((xs (List Param)) (ys (List Param))) (List Param)
  (match xs
    (Nil ys)
    ((Cons h t) (Cons h (append_params t ys)))))

(fn append_goals ((xs (List Goal)) (ys (List Goal))) (List Goal)
  (match xs
    (Nil ys)
    ((Cons h t) (Cons h (append_goals t ys)))))

(fn append_eqs ((xs (List Equation)) (ys (List Equation))) (List Equation)
  (match xs
    (Nil ys)
    ((Cons h t) (Cons h (append_eqs t ys)))))

(fn reverse_syms ((xs (List Symbol))) (List Symbol)
  (reverse_syms_acc xs Nil))

(fn reverse_syms_acc ((xs (List Symbol)) (acc (List Symbol))) (List Symbol)
  (match xs
    (Nil acc)
    ((Cons h t) (reverse_syms_acc t (Cons h acc)))))

(fn reverse_exprs ((xs (List Expr))) (List Expr)
  (reverse_exprs_acc xs Nil))

(fn reverse_exprs_acc ((xs (List Expr)) (acc (List Expr))) (List Expr)
  (match xs
    (Nil acc)
    ((Cons h t) (reverse_exprs_acc t (Cons h acc)))))

;; Generate fresh Params, one per type.
(fn mk_fresh_params ((types (List Type))) (List Param)
  (match types
    (Nil Nil)
    ((Cons t rest) (Cons (Param (gen_fresh) t) (mk_fresh_params rest)))))

;; Generate one fresh Symbol per Param. Used by the Rewrite arm to
;; open the cited equation's ∀-binders to fresh FVars; the names
;; double as the pat_vars list passed to expr_match.
(fn fresh_syms_for_params ((ps (List Param))) (List Symbol)
  (match ps
    (Nil Nil)
    ((Cons _ rest) (Cons (gen_fresh) (fresh_syms_for_params rest)))))

;; Convert a list of Symbols to FVar Expr values, preserving order.
(fn syms_to_fvars ((ss (List Symbol))) (List Expr)
  (match ss
    (Nil Nil)
    ((Cons s rest) (Cons (FVar s) (syms_to_fvars rest)))))

;; ---------------------------------------------------------------------------
;; Inst-processing helpers (slice 32).
;;
;; An `Inst NAME EXPR` pre-instantiates one of a cited Goal's ∀-vars
;; before the conclusion pattern match runs. Unblocks citations where
;; the LHS pattern can't infer some ∀-binder (e.g., a "pivot" var that
;; appears only on the RHS, or any binder the match would leave free).
;;
;; The Rewrite / RewriteWith arms use `split_params_by_insts` to walk
;; the cited Goal's Params in introduction order; for each Param, the
;; binding is either:
;;   - the user-supplied Inst value (if an Inst names this Param), or
;;   - a fresh FVar (otherwise — these names become pat_vars for the
;;     ordinary capture-matching path).
;;
;; Validation: `all_insts_named` rejects Insts that name a Param not
;; present in the cited Goal. Duplicates within Insts are first-match-
;; wins via `find_inst`; later duplicates are silently ignored.
;; ---------------------------------------------------------------------------

(fn find_inst ((name Symbol) (insts (List Inst))) (Option Expr)
  (match insts
    (Nil None)
    ((Cons (Inst iname ival) rest)
      (if (sym_eq iname name) (Some ival) (find_inst name rest)))))

(fn all_insts_named ((insts (List Inst)) (cited_params (List Param))) Bool
  (match insts
    (Nil True)
    ((Cons (Inst iname _) rest)
      (match (find_param iname cited_params)
        (None False)
        ((Some _) (all_insts_named rest cited_params))))))

;; Returns (openings, pat_var_names). openings is in INTRODUCTION
;; ORDER — caller reverses before passing to open_many. pat_var_names
;; contains only the fresh-FVar names; Insts-bound params contribute
;; no pat_var (their binders are already pinned, not captured).
(fn split_params_by_insts ((cited_params (List Param)) (insts (List Inst)))
                           (Pair (List Expr) (List Symbol))
  (match cited_params
    (Nil (Pair Nil Nil))
    ((Cons (Param pname _) rest)
      (match (split_params_by_insts rest insts)
        ((Pair rest_openings rest_pvs)
          (match (find_inst pname insts)
            ((Some val)
              (Pair (Cons val rest_openings) rest_pvs))
            (None
              (let ((fresh (gen_fresh)))
                (Pair (Cons (FVar fresh) rest_openings)
                      (Cons fresh rest_pvs))))))))))

;; ---------------------------------------------------------------------------
;; open_goal: enter a Goal as a Sequent. Display names are used directly
;; as the initial FVar names — a closed Goal has no prior FVars to
;; collide with. Body BVars get opened to FVars in innermost-first
;; order (BVar 0 = last Param in display order).
;; ---------------------------------------------------------------------------

(fn open_goal ((g Goal)) Sequent
  (match g
    ((Goal params premises eq)
      (let ((fvars (reverse_exprs (params_to_fvar_exprs params))))
        (Sequent
          params
          Nil
          (open_eqs_with fvars premises)
          (open_eq_with fvars eq))))))

;; ---------------------------------------------------------------------------
;; find_case: look up the Proof for a given ctor branch.
;; ---------------------------------------------------------------------------

;; Returns the matching case's (author field-names, sub-proof). For a plain
;; (Case …) the names list is Nil, meaning "generate fresh field names".
(fn find_case ((ctor_name Symbol) (cases (List Case)))
              (Option (Pair (List Symbol) Proof))
  (match cases
    (Nil None)
    ((Cons c rest)
      (match c
        ((Case cn pf)
          (if (sym_eq cn ctor_name) (Some (Pair Nil pf)) (find_case ctor_name rest)))
        ((CaseB cn names pf)
          (if (sym_eq cn ctor_name) (Some (Pair names pf)) (find_case ctor_name rest)))))))

;; ---------------------------------------------------------------------------
;; Named-field destructuring (Induct / CaseOn). When a CaseB supplies field
;; names, build the field Params from them instead of gen_fresh — so the
;; author can reference (and case-split on) a constructor field, e.g. the
;; head of an inducted list. Soundness rests on `field_names_ok`: the names
;; must match the ctor's arity, be pairwise distinct, and avoid capturing any
;; surviving sequent param.
;; ---------------------------------------------------------------------------

;; Field Params from names (one per type). Nil names ⇒ all gen_fresh.
(fn mk_field_params ((names (List Symbol)) (types (List Type))) (List Param)
  (match names
    (Nil (mk_fresh_params types))
    ((Cons _ _) (zip_field_params names types))))

(fn zip_field_params ((names (List Symbol)) (types (List Type))) (List Param)
  (match names
    (Nil Nil)
    ((Cons n nrest)
      (match types
        (Nil Nil)                                   ; length guarded by field_names_ok
        ((Cons t trest) (Cons (Param n t) (zip_field_params nrest trest)))))))

(fn same_length_st ((names (List Symbol)) (types (List Type))) Bool
  (match names
    (Nil  (match types (Nil True) (_ False)))
    ((Cons _ nr) (match types (Nil False) ((Cons _ tr) (same_length_st nr tr))))))

(fn distinct_syms ((names (List Symbol))) Bool
  (match names
    (Nil True)
    ((Cons n rest) (if (sym_member n rest) False (distinct_syms rest)))))

(fn disjoint_syms ((names (List Symbol)) (taken (List Symbol))) Bool
  (match names
    (Nil True)
    ((Cons n rest) (if (sym_member n taken) False (disjoint_syms rest taken)))))

;; Validate author-provided field names. Nil ⇒ no names supplied ⇒ OK (fresh).
;; Otherwise: arity match, pairwise distinct, and disjoint from `taken` (the
;; surviving param names) so the new field FVars cannot capture an in-scope var.
(fn field_names_ok ((names (List Symbol)) (types (List Type)) (taken (List Symbol))) Bool
  (match names
    (Nil True)
    ((Cons _ _)
      (if (same_length_st names types)
          (if (distinct_syms names)
              (disjoint_syms names taken)
              False)
          False))))

;; ---------------------------------------------------------------------------
;; do_case_on: per-ctor sub-sequents with an added hypothesis that
;; `scrut` equals (Ctor cname fresh-fields). Strictly simpler than
;; do_induct — no IH machinery, scrut is not substituted (it may not
;; even be a variable).
;;
;; v2 limitation: type-args of parametric types are NOT instantiated.
;; The ctor's field types are used as declared, which may carry TVars
;; for parametric types. Since narrow erases types at runtime, that
;; doesn't break execution; it just means the fresh field Params
;; carry abstract TVar types until a later slice supplies type-args.
;; Documented in REVISIT — "Erased polymorphism in narrow".
;; ---------------------------------------------------------------------------

(fn do_case_on ((m Module) (th Theory) (seq Sequent)
                (scrut Expr) (ty Symbol) (cases (List Case))) Bool
  (match (lookup_typedef ty m)
    (None False)                              ; unknown type
    ((Some (TypeDef _ _ ctors))
      (check_case_on_cases m th seq scrut ctors cases))))

(fn check_case_on_cases
    ((m Module) (th Theory) (seq Sequent)
     (scrut Expr) (ctors (List CtorDef)) (cases (List Case))) Bool
  (match ctors
    (Nil True)                                ; all branches checked
    ((Cons (CtorDef cname field_types) rest)
      (match (find_case cname cases)
        (None False)                          ; user didn't supply a sub-proof
        ((Some (Pair names case_pf))
          ;; CaseOn keeps all params, so the field names must avoid every one.
          (if (field_names_ok names field_types (param_names (seq_params seq)))
              (let ((subgoal (build_case_on_subgoal seq scrut cname names field_types)))
                (if (check_sequent m th subgoal case_pf)
                    (check_case_on_cases m th seq scrut rest cases)
                    False))
              False))))))                     ; bad field names — reject

(fn seq_params ((seq Sequent)) (List Param)
  (match seq ((Sequent params _ _ _) params)))

;; Build the per-ctor sub-sequent: field Params (named or fresh), an added
;; hypothesis `scrut = (Ctor cname <field-fvars>)`. The hypothesis is
;; prepended to hyps, so the sub-proof cites it as (Hyp 0).
(fn build_case_on_subgoal ((seq Sequent) (scrut Expr)
                           (cname Symbol) (names (List Symbol)) (field_types (List Type))) Sequent
  (match seq
    ((Sequent params hyps premises eq)
      (let ((field_params (mk_field_params names field_types)))
        (let ((ctor_expr (Ctor cname (params_to_fvar_exprs field_params))))
          (let ((case_hyp (Goal Nil Nil (Equation scrut ctor_expr))))
            (Sequent
              (append_params params field_params)
              (Cons case_hyp hyps)
              premises
              eq)))))))

;; ---------------------------------------------------------------------------
;; do_induct: per-ctor sub-sequents with fresh FVars and IHs.
;;
;; For each ctor C with field types T1..Tn (after instantiating the
;; inducting type's type-args via type_subst):
;;   - fresh FVars f1..fn with the concrete field types
;;   - substitute (var := (Ctor C f1..fn)) throughout seq's hyps,
;;     premises, and goal equation
;;   - one IH per recursive field (whose type = var's type): the
;;     original seq's goal with (var := the_field), with all OTHER
;;     seq vars closed as the IH's ∀-bound vars
;; ---------------------------------------------------------------------------

(fn do_induct ((m Module) (th Theory) (seq Sequent)
               (var Symbol) (cases (List Case))) Bool
  (match seq
    ((Sequent params _ _ _)
      (match (find_param var params)
        (None False)
        ((Some (Param _ var_type))
          (match (type_head var_type)
            (None False)                              ; TVar — can't induct
            ((Some (Pair tname targs))
              (match (lookup_typedef tname m)
                (None False)
                ((Some (TypeDef _ tparams ctors))
                  (check_induct_cases
                    m th seq var var_type
                    (zip_pairs tparams targs)
                    ctors cases))))))))))

(fn check_induct_cases
    ((m Module) (th Theory) (seq Sequent)
     (var Symbol) (var_type Type)
     (type_env (List (Pair Symbol Type)))
     (ctors (List CtorDef)) (cases (List Case))) Bool
  (match ctors
    (Nil True)
    ((Cons (CtorDef cname field_types) rest)
      (let ((concrete_fields (type_subst_list type_env field_types)))
        (match (find_case cname cases)
          (None False)                                ; missing case for this ctor
          ((Some (Pair names case_pf))
            ;; Induct removes `var`; field names must avoid the SURVIVING params.
            (if (field_names_ok names concrete_fields
                                 (param_names (remove_param var (seq_params seq))))
                (let ((subgoal (build_induct_subgoal
                                  seq var var_type cname names concrete_fields)))
                  (if (check_sequent m th subgoal case_pf)
                      (check_induct_cases m th seq var var_type type_env rest cases)
                      False))
                False)))))))                          ; bad field names — reject

(fn build_induct_subgoal
    ((seq Sequent) (var Symbol) (var_type Type)
     (cname Symbol) (names (List Symbol)) (field_types (List Type))) Sequent
  (match seq
    ((Sequent params hyps premises eq)
      (let ((field_params (mk_field_params names field_types))
            (rest_params  (remove_param var params)))
        (let ((ctor_expr (Ctor cname (params_to_fvar_exprs field_params))))
          (let ((env (Bind var ctor_expr Empty))
                (ihs (build_ihs var var_type rest_params premises eq field_params)))
            (Sequent
              (append_params rest_params field_params)
              (append_goals (subst_goals env hyps) ihs)
              (subst_eqs env premises)
              (subst_eq env eq))))))))

(fn build_ihs
    ((var Symbol) (var_type Type) (rest_params (List Param))
     (premises (List Equation)) (eq Equation)
     (field_params (List Param))) (List Goal)
  (match field_params
    (Nil Nil)
    ((Cons (Param fname ftype) rest)
      (if (type_eq var_type ftype)
          (Cons (build_ih var fname rest_params premises eq)
                (build_ihs var var_type rest_params premises eq rest))
          (build_ihs var var_type rest_params premises eq rest)))))

(fn build_ih ((var Symbol) (field_name Symbol)
              (rest_params (List Param))
              (premises (List Equation)) (eq Equation)) Goal
  (let ((env (Bind var (FVar field_name) Empty)))
    (let ((ih_premises (subst_eqs env premises))
          (ih_eq       (subst_eq env eq))
          (innermost_first (reverse_syms (param_names rest_params))))
      (Goal
        rest_params
        (close_eqs innermost_first ih_premises)
        (close_eq innermost_first ih_eq)))))

;; ---------------------------------------------------------------------------
;; do_wf_induct: well-founded induction on a user-supplied MEASURE (an
;; Int-valued Expr over the goal's params). Unlike Induct (structural, on a
;; datatype var) this needs no constructors — it works for Int and any type,
;; because the well-foundedness comes from the measure's image in ℤ, not the
;; term's shape.
;;
;; ONE subgoal: the original sequent with a strong induction hypothesis
;; prepended at Hyp 0. For a goal  premises(P) |- eq(P)  with measure μ(P),
;; the IH is the closed Goal
;;
;;   ∀ P'. premises(P') -> 0 <= μ(P') -> μ(P') < μ(P) -> eq(P')
;;
;; (P' a fresh copy of ALL params; μ(P) keeps the current params, free).
;;
;; SOUNDNESS (why an arbitrary Int measure is OK, not only a Nat one).
;; Suppose the subgoal holds but the conclusion fails on a nonempty set S of
;; param-tuples. Let Sp = { x in S : μ(x) >= 0 }.
;;   - Sp nonempty: its measures are >= 0, so they have a LEAST element μ(x*)
;;     (the non-negative integers are well-ordered). The IH at x* is then
;;     dischargeable — any y with 0 <= μ(y) < μ(x*) is not in S by
;;     minimality, so eq(y) holds — forcing eq(x*), contradicting x* in S.
;;   - Sp empty: every x in S has μ(x) < 0, so the IH antecedent
;;     0 <= μ(y) < μ(x) is unsatisfiable — the subgoal proves eq(x) with no
;;     IH help, contradicting x in S.
;; The `0 <= μ(P')` guard makes the negative part of μ's range harmless:
;; below 0 the IH gives nothing, so the subgoal closes those cases without
;; it. This is ordinary well-founded induction along  y ≺ x == 0<=μ(y)<μ(x).
;;
;; Citing the IH (as Hyp 0, via RewriteWith with insts for the fresh P')
;; thus carries three classes of premise to discharge: the goal's own
;; premises at P', then 0 <= μ(P'), then μ(P') < μ(P) — the latter two are
;; the termination obligations (for `/`-recursion: std/div's div_nonneg and
;; div_lt).
;; ---------------------------------------------------------------------------

;; Fresh-rename every param: the fresh Param list (same types, new names)
;; paired with an Env mapping each old name to its fresh FVar.
(fn wf_rename ((ps (List Param))) (Pair (List Param) Env)
  (match ps
    (Nil (Pair Nil Empty))
    ((Cons (Param x ty) rest)
      (match (wf_rename rest)
        ((Pair fresh_rest env_rest)
          (let ((fx (gen_fresh)))
            (Pair (Cons (Param fx ty) fresh_rest)
                  (Bind x (FVar fx) env_rest))))))))

(fn do_wf_induct ((m Module) (th Theory) (seq Sequent)
                  (measure Expr) (pf Proof)) Bool
  (match seq
    ((Sequent params hyps premises eq)
      (match (wf_rename params)
        ((Pair fresh_params env)
          (let ((measure_fresh (subst env measure))
                (close_names   (reverse_syms (param_names fresh_params))))
            (let ((wf_premises
                    (Cons (Equation
                            (Call (quote le) (list (IntLit 0) measure_fresh))
                            (Ctor (quote True) Nil))
                      (Cons (Equation
                              (Call (quote lt) (list measure_fresh measure))
                              (Ctor (quote True) Nil))
                        Nil)))
                  (ih_eq (subst_eq env eq)))
              (let ((ih (Goal
                          fresh_params
                          (close_eqs close_names
                            (append_eqs (subst_eqs env premises) wf_premises))
                          (close_eq close_names ih_eq))))
                (check_sequent m th
                  (Sequent params (Cons ih hyps) premises eq)
                  pf)))))))))

;; ---------------------------------------------------------------------------
;; do_induct2: TWO-STEP induction on a Nat-shaped var. Sound because every
;; Nat is Z, (S Z), or (S (S k)) — so proving the goal at Z and (S Z), plus
;; the step ∀k. P(k) ⟹ P(S (S k)), covers all n. This is what functions
;; recurring two-at-a-time (half_nat: half (S (S k)) = S (half k)) need:
;; single-step Induct only ever yields the IH at k, never k-1, so the
;; S(S k) arm can't reach P(k). The cases are named 'Z, 'SZ, 'SS; the SS
;; arm gets one IH = P(k) (the goal with var:=k, other vars closed as ∀,
;; exactly like single-step Induct's build_ih).
;;
;; The var's type must have a nullary ctor (the "zero") and a one-field
;; recursive ctor (the "succ", field type = var's type). Found generically
;; below, so this works for any such two-ctor type, not just the literal
;; Nat declaration. (For PARAMETRIC recursive types find_succ_ctor's
;; type_eq compares the declared field type against the instantiated
;; var_type, so it can over-reject — a false NEGATIVE, never unsound.)
;; ---------------------------------------------------------------------------

;; SOUNDNESS GUARD: two-step induction's three arms (Z, S Z, S (S k))
;; cover every value of a type IFF that type is EXACTLY a nullary ctor +
;; a unary recursive ctor (so all values are succ-towers over zero). A
;; THIRD ctor would leave values uncovered — so require exactly two.
(fn is_two_ctors ((ctors (List CtorDef))) Bool
  (match ctors
    ((Cons _ (Cons _ Nil)) True)
    (_ False)))

(fn find_zero_ctor ((ctors (List CtorDef))) (Option Symbol)
  (match ctors
    (Nil None)
    ((Cons (CtorDef cn fts) rest)
      (match fts
        (Nil (Some cn))
        (_   (find_zero_ctor rest))))))

(fn find_succ_ctor ((ctors (List CtorDef)) (var_type Type)) (Option Symbol)
  (match ctors
    (Nil None)
    ((Cons (CtorDef cn fts) rest)
      (match fts
        ((Cons ft Nil)
          (if (type_eq ft var_type) (Some cn) (find_succ_ctor rest var_type)))
        (_ (find_succ_ctor rest var_type))))))

;; Substitute var := val (a closed Expr) throughout the sequent and drop
;; var from the params. Used for the Z and (S Z) base arms (no IH).
(fn build_subst_subgoal ((seq Sequent) (var Symbol) (val Expr)) Sequent
  (match seq
    ((Sequent params hyps premises eq)
      (let ((rest_params (remove_param var params))
            (env (Bind var val Empty)))
        (Sequent rest_params
                 (subst_goals env hyps)
                 (subst_eqs env premises)
                 (subst_eq env eq))))))

;; The (S (S k)) arm: fresh k, substitute var := (succ (succ k)), and add
;; the single IH P(k) (built exactly as single-step Induct builds it).
(fn build_ss_subgoal ((seq Sequent) (var Symbol) (var_type Type)
                      (succ_c Symbol)) Sequent
  (match seq
    ((Sequent params hyps premises eq)
      (let ((kfresh (gen_fresh)))
        (let ((rest_params (remove_param var params))
              (ss_val (Ctor succ_c (list (Ctor succ_c (list (FVar kfresh)))))))
          (let ((env_ss (Bind var ss_val Empty))
                (ih (build_ih var kfresh rest_params premises eq)))
            (Sequent
              (append_params rest_params (list (Param kfresh var_type)))
              (append_goals (subst_goals env_ss hyps) (list ih))
              (subst_eqs env_ss premises)
              (subst_eq env_ss eq))))))))

;; Given the resolved zero/succ ctors, check the three arms. Z and (S Z)
;; substitute and prove directly; (S (S k)) gets the IH at k.
(fn induct2_run ((m Module) (th Theory) (seq Sequent)
                 (var Symbol) (var_type Type) (zero_c Symbol) (succ_c Symbol)
                 (cases (List Case))) Bool
  ;; Induct2's field binding (the k in S(S k)) is handled by build_ss_subgoal,
  ;; not by author names, so any CaseB names here are ignored.
  (match (find_case (quote Z) cases)
    (None False)
    ((Some (Pair _ pf_z))
      (match (find_case (quote SZ) cases)
        (None False)
        ((Some (Pair _ pf_sz))
          (match (find_case (quote SS) cases)
            (None False)
            ((Some (Pair _ pf_ss))
              (if (check_sequent m th
                    (build_subst_subgoal seq var (Ctor zero_c Nil)) pf_z)
                  (if (check_sequent m th
                        (build_subst_subgoal seq var
                          (Ctor succ_c (list (Ctor zero_c Nil)))) pf_sz)
                      (check_sequent m th
                        (build_ss_subgoal seq var var_type succ_c) pf_ss)
                      False)
                  False))))))))

(fn do_induct2 ((m Module) (th Theory) (seq Sequent)
                (var Symbol) (cases (List Case))) Bool
  (match seq
    ((Sequent params _ _ _)
      (match (find_param var params)
        (None False)
        ((Some (Param _ var_type))
          (match (type_head var_type)
            (None False)                              ; TVar — can't induct
            ((Some (Pair tname _))
              (match (lookup_typedef tname m)
                (None False)
                ((Some (TypeDef _ _ ctors))
                  (if (is_two_ctors ctors)            ; SOUNDNESS GUARD: exactly zero+succ
                      (match (find_zero_ctor ctors)
                        (None False)
                        ((Some zero_c)
                          (match (find_succ_ctor ctors var_type)
                            (None False)
                            ((Some succ_c)
                              (induct2_run m th seq var var_type zero_c succ_c cases)))))
                      False))))))))))

;; ---------------------------------------------------------------------------
;; UNTRUSTED diagnostics support (only ever called by the FAIL tracer in
;; src/bin/check.rs — never on the check path). They rebuild the subgoal a
;; branching proof would face, so the tracer can descend past WfInduct/CaseOn.
;; ---------------------------------------------------------------------------
(fn dbg_wf_subgoal ((seq Sequent) (measure Expr)) Sequent
  (match seq
    ((Sequent params hyps premises eq)
      (match (wf_rename params)
        ((Pair fresh_params env)
          (let ((measure_fresh (subst env measure))
                (close_names   (reverse_syms (param_names fresh_params))))
            (let ((wf_premises
                    (Cons (Equation (Call (quote le) (list (IntLit 0) measure_fresh)) (Ctor (quote True) Nil))
                      (Cons (Equation (Call (quote lt) (list measure_fresh measure)) (Ctor (quote True) Nil))
                        Nil)))
                  (ih_eq (subst_eq env eq)))
              (let ((ih (Goal fresh_params
                          (close_eqs close_names (append_eqs (subst_eqs env premises) wf_premises))
                          (close_eq close_names ih_eq))))
                (Sequent params (Cons ih hyps) premises eq)))))))))

(fn dbg_ctor_fields ((cname Symbol) (ctors (List CtorDef))) (Option (List Type))
  (match ctors
    (Nil None)
    ((Cons (CtorDef nm fts) rest)
      (if (sym_eq nm cname) (Some fts) (dbg_ctor_fields cname rest)))))

(fn dbg_caseon_subgoal ((m Module) (seq Sequent) (scrut Expr) (ty Symbol)
                        (cname Symbol) (names (List Symbol))) (Option Sequent)
  (match (lookup_typedef ty m)
    (None None)
    ((Some (TypeDef _ _ ctors))
      (match (dbg_ctor_fields cname ctors)
        (None None)
        ((Some fts) (Some (build_case_on_subgoal seq scrut cname names fts)))))))

;; Recursive-field count for an Induct ctor: how many IHs do_induct appends
;; (one per field whose CONCRETE type equals the inducting var's type).
;; UNTRUSTED — only the loader's named-hyp desugarer calls these.
(fn dbg_count_rec ((var_type Type) (fields (List Type))) Int
  (match fields
    (Nil 0)
    ((Cons f rest)
      (if (type_eq var_type f)
          (+ 1 (dbg_count_rec var_type rest))
          (dbg_count_rec var_type rest)))))

(fn dbg_ih_count ((var_type Type) (cname Symbol) (m Module)) Int
  (match (type_head var_type)
    (None 0)
    ((Some (Pair tname targs))
      (match (lookup_typedef tname m)
        (None 0)
        ((Some (TypeDef _ tparams ctors))
          (match (dbg_ctor_fields cname ctors)
            (None 0)
            ((Some fts)
              (dbg_count_rec var_type (type_subst_list (zip_pairs tparams targs) fts)))))))))
