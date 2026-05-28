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

    ((CaseOn scrut ty cases)
      (do_case_on m th seq scrut ty cases))

    ((RewriteWith er dir side insts premise_proofs rest)
      ;; TODO (rewriter, conditional): resolve er, specialize via
      ;; insts, match its lhs/rhs (per dir) against the chosen side;
      ;; instantiate each premise with the match binding and discharge
      ;; with the supplied sub-proof; continue with the rewritten seq.
      False)

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
      ;; TODO: dispatch to the per-theory checker registered under
      ;; theory_name; accept iff the cert validates the current goal.
      False)))

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
      ;; Reduce: drive simp_expr on the chosen side(s) of the
      ;; current equation. In v2 this is full δ+ι (see REVISIT —
      ;; "Reduce and Simp collapsed into one").
      (match seq
        ((Sequent params hyps premises eq)
          (Some (Sequent params hyps premises
                  (reduce_side_eq m side eq))))))
    ((Simp side)
      ;; TODO: simp — the guarded reducer (memoized; the v1 blowup
      ;; fix is non-negotiable, see REVISIT and TRANSFER notes).
      None)
    ((Rewrite er dir side all insts)
      ;; TODO (rewriter, unconditional): resolve er, specialize via
      ;; insts, require no premises, match pat against side, replace
      ;; either first occurrence or all per `all`.
      None)))

(fn reduce_side_eq ((m Module) (side Side) (eq Equation)) Equation
  (match eq
    ((Equation l r)
      (match side
        ((Lhs)  (Equation (simp_expr m l) r))
        ((Rhs)  (Equation l               (simp_expr m r)))
        ((Both) (Equation (simp_expr m l) (simp_expr m r)))))))

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

(fn find_case ((ctor_name Symbol) (cases (List Case))) (Option Proof)
  (match cases
    (Nil None)
    ((Cons c rest)
      (match c
        ((Case cn pf)
          (if (sym_eq cn ctor_name) (Some pf) (find_case ctor_name rest)))))))

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
        ((Some case_pf)
          (let ((subgoal (build_case_on_subgoal seq scrut cname field_types)))
            (if (check_sequent m th subgoal case_pf)
                (check_case_on_cases m th seq scrut rest cases)
                False)))))))

;; Build the per-ctor sub-sequent: fresh field Params, an added
;; hypothesis `scrut = (Ctor cname <fresh-fvars>)`. The hypothesis is
;; prepended to hyps, so the sub-proof cites it as (Hyp 0).
(fn build_case_on_subgoal ((seq Sequent) (scrut Expr)
                           (cname Symbol) (field_types (List Type))) Sequent
  (match seq
    ((Sequent params hyps premises eq)
      (let ((field_params (mk_fresh_params field_types)))
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
          ((Some case_pf)
            (let ((subgoal (build_induct_subgoal
                              seq var var_type cname concrete_fields)))
              (if (check_sequent m th subgoal case_pf)
                  (check_induct_cases m th seq var var_type type_env rest cases)
                  False))))))))

(fn build_induct_subgoal
    ((seq Sequent) (var Symbol) (var_type Type)
     (cname Symbol) (field_types (List Type))) Sequent
  (match seq
    ((Sequent params hyps premises eq)
      (let ((field_params (mk_fresh_params field_types))
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
