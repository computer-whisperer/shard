;;; The beginning of the one-step reducer.
;;;
;;; `match_pat` decides whether a value `v` matches a pattern `p`,
;;; and if so collects the captured bindings. Order convention:
;;; innermost-first — `bindings[k]` is BVar k in the arm body, so
;;; the last (rightmost) PVar encountered binds index 0.
;;;
;;; `try_match_arms` is the one-step rewriter for a stuck match: it
;;; scans arms left-to-right and returns the first arm's body opened
;;; with its bindings, or None if no arm fires.

(fn match_pat ((p Pat) (v Expr) (acc (List Expr))) (Option (List Expr))
  (match p
    ((PVar)                              ; binds next BVar — capture v as innermost
      (Some (Cons v acc)))
    ((PInt n)
      (match v
        ((IntLit m)
          (if (int_eq n m) (Some acc) None))
        (_ None)))
    ((PSym s)
      (match v
        ((SymLit t)
          (if (sym_eq s t) (Some acc) None))
        (_ None)))
    ((PCtor pc pats)
      (match v
        ((Ctor vc vargs)
          (if (sym_eq pc vc)
              (match_pats pats vargs acc)
              None))
        (_ None)))))

(fn match_pats ((ps (List Pat)) (vs (List Expr)) (acc (List Expr))) (Option (List Expr))
  (match ps
    (Nil
      (match vs
        (Nil (Some acc))
        (_   None)))
    ((Cons p prest)
      (match vs
        (Nil None)
        ((Cons v vrest)
          (match (match_pat p v acc)
            ((Some acc2) (match_pats prest vrest acc2))
            (None        None)))))))

(fn try_match_arms ((arms (List Arm)) (v Expr)) (Option Expr)
  (match arms
    (Nil None)
    ((Cons (Arm p body) rest)
      (match (match_pat p v Nil)
        ((Some bindings) (Some (open_many bindings body)))
        (None            (try_match_arms rest v))))))

;; ---------------------------------------------------------------------------
;; Function-table lookup. Names are unique within a module.
;; ---------------------------------------------------------------------------

(fn lookup_fn ((name Symbol) (fns (List FnDef))) (Option FnDef)
  (match fns
    (Nil None)
    ((Cons fd rest)
      (match fd
        ((FnDef fname _ _ _)
          (if (sym_eq name fname)
              (Some fd)
              (lookup_fn name rest)))))))

;; len_t: list-of-Type length. Yet another monomorphic helper.
(fn len_t ((ts (List Type))) Int
  (match ts
    (Nil 0)
    ((Cons _ t) (+ 1 (len_t t)))))

;; ---------------------------------------------------------------------------
;; step: one reduction step on e, or None if e is already in normal form
;; (or stuck on an unknown call / partial application).
;;
;; Strategy is leftmost-outermost: try a head reduction first; if the
;; head can't reduce, descend into subterms and step the first reducible
;; one. Primitives are NOT applied here — calls to symbols with no FnDef
;; are returned as stuck and the Rust runtime intercepts them at the
;; outer evaluator level (see REVISIT.md — Primitive call protocol).
;;
;; Reduction does NOT descend into Match arm bodies (they are under
;; binders), into FVar/BVar (irreducible), or into Let RHSs separately
;; (Let is reduced at the head by opening into the body).
;; ---------------------------------------------------------------------------

(fn step ((m Module) (e Expr)) (Option Expr)
  (match e
    ((Call f args)  (step_call m f args))
    ((Match s arms) (step_match m s arms))
    ((Let bs body)  (Some (open_many bs body)))
    ;; If: True/False at the head fires; otherwise step the condition.
    ;; Bool ctor names are hardcoded (cf. prim.rs and REVISIT.md —
    ;; "Primitive comparisons return user Bool").
    ((If c t el)
      (match c
        ((Ctor (quote True)  Nil) (Some t))
        ((Ctor (quote False) Nil) (Some el))
        (_
          (match (step m c)
            ((Some c2) (Some (If c2 t el)))
            (None      None)))))
    ((Ctor c args)
      (match (step_list m args)
        ((Some args2) (Some (Ctor c args2)))
        (None         None)))
    ((FVar _)   None)
    ((BVar _)   None)
    ((IntLit _) None)
    ((SymLit _) None)))

;; step_call: unfold user-fn application if possible; otherwise try
;; the primitive table; otherwise step an argument. Stuck calls
;; (unknown name AND not a primitive AND no reducible arg) return None.
;;
;; The primitive dispatch happens at the data level — the kernel is
;; reducing a Call VALUE, not invoking a function. See REVISIT —
;; "Primitives reachable from the kernel's reducer".
(fn step_call ((m Module) (f Symbol) (args (List Expr))) (Option Expr)
  (match m
    ((Module _ fns _)
      (match (lookup_fn f fns)
        ((Some fd)
          (match (apply_fn fd args)
            ((Some e2) (Some e2))
            (None      (step_args_in_call m f args))))
        (None
          (match (try_step_prim f args)
            ((Some e2) (Some e2))
            (None      (step_args_in_call m f args))))))))

;; try_step_prim: dispatch a Call value to the primitive table when
;; all args are already values of the expected shapes. Returns None
;; if `f` isn't a known primitive or the args don't fit (e.g. not yet
;; reduced to literals). The primitives invoked in the bodies below
;; ARE narrow calls — the Rust runtime intercepts them via the same
;; mechanism that powers ordinary in-language arithmetic.
;;
;; KEEP IN SYNC with src/prim.rs.
(fn try_step_prim ((f Symbol) (args (List Expr))) (Option Expr)
  (match args
    ;; Two-Int primitives.
    ((Cons (IntLit a) (Cons (IntLit b) Nil))
      (if (sym_eq f (quote +))      (Some (IntLit (+ a b)))
      (if (sym_eq f (quote -))      (Some (IntLit (- a b)))
      (if (sym_eq f (quote *))      (Some (IntLit (* a b)))
      (if (sym_eq f (quote /))      (Some (IntLit (/ a b)))
      (if (sym_eq f (quote mod))    (Some (IntLit (mod a b)))
      (if (sym_eq f (quote band))   (Some (IntLit (band a b)))
      (if (sym_eq f (quote bor))    (Some (IntLit (bor a b)))
      (if (sym_eq f (quote bxor))   (Some (IntLit (bxor a b)))
      (if (sym_eq f (quote bshl))   (Some (IntLit (bshl a b)))
      (if (sym_eq f (quote bshr))   (Some (IntLit (bshr a b)))
      (if (sym_eq f (quote int_eq)) (Some (bool_as_expr (int_eq a b)))
      (if (sym_eq f (quote lt))     (Some (bool_as_expr (lt a b)))
      (if (sym_eq f (quote le))     (Some (bool_as_expr (le a b)))
                                    None))))))))))))))
    ;; Two-Symbol primitives.
    ((Cons (SymLit a) (Cons (SymLit b) Nil))
      (if (sym_eq f (quote sym_eq))
          (Some (bool_as_expr (sym_eq a b)))
          None))
    (_ None)))

;; Convert a Bool VALUE to the corresponding narrow Expr VALUE
;; (Ctor "True" Nil) / (Ctor "False" Nil). Used by try_step_prim's
;; comparison cases.
(fn bool_as_expr ((b Bool)) Expr
  (if b
      (Ctor (quote True)  Nil)
      (Ctor (quote False) Nil)))

(fn step_args_in_call ((m Module) (f Symbol) (args (List Expr))) (Option Expr)
  (match (step_list m args)
    ((Some args2) (Some (Call f args2)))
    (None         None)))

;; apply_fn: open body with args if arity matches; else None.
;; Substitution happens regardless of whether args are values — this is
;; the raw unfolding step. A guarded `simp` (added later) will refuse
;; to unfold calls whose result would remain stuck.
;;
;; open_many expects bindings INNERMOST-FIRST: bindings[0] fills
;; BVar 0, which the loader assigns to the LAST parameter. Call args
;; come in source order, so reverse before opening. (The same
;; subtlety bit the Rust apply_call — first non-symmetric test
;; exposed it.)
(fn apply_fn ((fd FnDef) (args (List Expr))) (Option Expr)
  (match fd
    ((FnDef _ ptypes _ body)
      (if (int_eq (len args) (len_t ptypes))
          (Some (open_many (reverse_exprs args) body))
          None))))

;; step_match: if scrut is value-headed, try arms; otherwise step scrut.
(fn step_match ((m Module) (s Expr) (arms (List Arm))) (Option Expr)
  (match s
    ((Ctor _ _)  (try_match_arms arms s))
    ((IntLit _)  (try_match_arms arms s))
    ((SymLit _)  (try_match_arms arms s))
    (_
      (match (step m s)
        ((Some s2) (Some (Match s2 arms)))
        (None      None)))))

;; unfold_one: find the leftmost-outermost Call to `fname` in `e` and
;; replace it with `fname`'s body opened with the call's args. Returns
;; None if fname isn't a user fn in m, or if no matching call is found,
;; or if the matching call has arity mismatch.
;;
;; Limitation: does NOT descend under binders (Match arm bodies, Let
;; bodies). For unfolding inside a binder, simp_expr handles it
;; transitively, or the user can extract the term first via Reduce.
(fn unfold_one ((m Module) (fname Symbol) (e Expr)) (Option Expr)
  (match m
    ((Module _ fns _)
      (match (lookup_fn fname fns)
        (None      None)                        ; fn doesn't exist
        ((Some fd) (unfold_one_in fd fname e))))))

(fn unfold_one_in ((fd FnDef) (fname Symbol) (e Expr)) (Option Expr)
  (match e
    ((Call f args)
      (if (sym_eq f fname)
          (apply_fn fd args)                    ; hit
          (match (unfold_one_in_list fd fname args)
            ((Some args2) (Some (Call f args2)))
            (None         None))))
    ((Ctor c args)
      (match (unfold_one_in_list fd fname args)
        ((Some args2) (Some (Ctor c args2)))
        (None         None)))
    ((If c t el)
      (match (unfold_one_in fd fname c)
        ((Some c2) (Some (If c2 t el)))
        (None
          (match (unfold_one_in fd fname t)
            ((Some t2) (Some (If c t2 el)))
            (None
              (match (unfold_one_in fd fname el)
                ((Some el2) (Some (If c t el2)))
                (None       None)))))))
    (_ None)))                                  ; FVar/BVar/IntLit/SymLit/Match/Let

(fn unfold_one_in_list ((fd FnDef) (fname Symbol) (es (List Expr)))
                        (Option (List Expr))
  (match es
    (Nil None)
    ((Cons h t)
      (match (unfold_one_in fd fname h)
        ((Some h2) (Some (Cons h2 t)))
        (None
          (match (unfold_one_in_list fd fname t)
            ((Some t2) (Some (Cons h t2)))
            (None      None)))))))

;; ---------------------------------------------------------------------------
;; ι-only step. Fires ctor-headed Matches, dispatches True/False Ifs,
;; opens Lets, descends into Ctor args. Does NOT unfold Calls (neither
;; user fns nor primitives) and does NOT recurse into Match arm bodies.
;;
;; Drives `Reduce` after the v2 Reduce/Simp split — see REVISIT.
;;
;; Distinct from `step` in two ways:
;;   - Call args: descended for ι-reduction but the Call ITSELF is
;;     never unfolded. (+ 5 5) stays (+ 5 5).
;;   - Recursive calls in unfolded bodies are not chased, which is
;;     exactly what IH-consuming inductive proofs need: Unfold once
;;     to expose a Match, ι-reduce until the recursive sub-call
;;     blocks, then Rewrite with the IH.
;; ---------------------------------------------------------------------------

(fn step_iota ((m Module) (e Expr)) (Option Expr)
  (match e
    ((Match s arms) (step_match_iota m s arms))
    ((Let bs body)  (Some (open_many bs body)))         ; ζ
    ((If c t el)
      (match c
        ((Ctor (quote True)  Nil) (Some t))
        ((Ctor (quote False) Nil) (Some el))
        (_
          (match (step_iota m c)
            ((Some c2) (Some (If c2 t el)))
            (None      None)))))
    ((Ctor c args)
      (match (step_iota_list m args)
        ((Some args2) (Some (Ctor c args2)))
        (None         None)))
    ((Call f args)
      ;; Do NOT unfold the call. Try to step args ι-only.
      (match (step_iota_list m args)
        ((Some args2) (Some (Call f args2)))
        (None         None)))
    ((FVar _)   None)
    ((BVar _)   None)
    ((IntLit _) None)
    ((SymLit _) None)))

(fn step_match_iota ((m Module) (s Expr) (arms (List Arm))) (Option Expr)
  (match s
    ((Ctor _ _)  (try_match_arms arms s))
    ((IntLit _)  (try_match_arms arms s))
    ((SymLit _)  (try_match_arms arms s))
    (_
      (match (step_iota m s)
        ((Some s2) (Some (Match s2 arms)))
        (None      None)))))

(fn step_iota_list ((m Module) (es (List Expr))) (Option (List Expr))
  (match es
    (Nil None)
    ((Cons h t)
      (match (step_iota m h)
        ((Some h2) (Some (Cons h2 t)))
        (None
          (match (step_iota_list m t)
            ((Some t2) (Some (Cons h t2)))
            (None      None)))))))

;; Drive step_iota to fixed point.
(fn simp_iota_expr ((m Module) (e Expr)) Expr
  (match (step_iota m e)
    (None      e)
    ((Some e2) (simp_iota_expr m e2))))

;; simp_expr: drive `step` to fixed point. Returns the normal form of
;; e under m. Termination depends on the kernel's user-fn bodies; the
;; trusted Rust runtime will diverge if step does. See REVISIT.md —
;; "Reduce and Simp collapsed into one in v2".
(fn simp_expr ((m Module) (e Expr)) Expr
  (match (step m e)
    (None      e)
    ((Some e2) (simp_expr m e2))))

;; step_list: try to step the first reducible expression in es, left to
;; right. Returns the new list if some step succeeded, else None.
(fn step_list ((m Module) (es (List Expr))) (Option (List Expr))
  (match es
    (Nil None)
    ((Cons h t)
      (match (step m h)
        ((Some h2) (Some (Cons h2 t)))
        (None
          (match (step_list m t)
            ((Some t2) (Some (Cons h t2)))
            (None      None)))))))
