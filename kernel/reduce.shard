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

;; match_pat: three-valued (see MatchResult). A constructor/literal pattern
;; meeting a value of a clashing constructor/literal is MNo; meeting a
;; non-value (Call/Match/If/Let — reducible — or FVar/BVar — symbolic) at a
;; position the pattern needs to inspect is MStuck (UNDECIDED), never MNo.
;; PVar is non-forcing: it binds whatever is there, value or not.
(fn match_pat ((p Pat) (v Expr) (acc (List Expr))) MatchResult
  (match p
    ((PVar)                              ; binds next BVar — capture v as innermost
      (MOk (Cons v acc)))
    ((PInt n)
      (match v
        ((IntLit m) (if (int_eq n m) (MOk acc) (MNo)))
        ((Ctor _ _) (MNo))
        ((SymLit _) (MNo))
        (_          (MStuck))))          ; Call/Match/If/Let/FVar/BVar — undecided
    ((PSym s)
      (match v
        ((SymLit t) (if (sym_eq s t) (MOk acc) (MNo)))
        ((Ctor _ _) (MNo))
        ((IntLit _) (MNo))
        (_          (MStuck))))
    ((PCtor pc pats)
      (match v
        ((Ctor vc vargs)
          (if (sym_eq pc vc) (match_pats pats vargs acc False) (MNo)))
        ((IntLit _) (MNo))
        ((SymLit _) (MNo))
        (_          (MStuck))))))        ; Call/Match/If/Let/FVar/BVar — undecided

;; match_pats: match positionally, left to right. The combined verdict is
;; MNo if ANY position definitely clashes (short-circuit), else MStuck if
;; any position is undecided, else MOk with the accumulated bindings. The
;; `stuck` flag rides along so a later MNo still wins over an earlier MStuck
;; (an arm whose 2nd field clashes cannot match, even if its 1st is undecided).
(fn match_pats ((ps (List Pat)) (vs (List Expr)) (acc (List Expr)) (stuck Bool)) MatchResult
  (match ps
    (Nil
      (match vs
        (Nil (if stuck (MStuck) (MOk acc)))
        (_   (MNo))))                                ; arity: extra values
    ((Cons p prest)
      (match vs
        (Nil (MNo))                                  ; arity: extra patterns
        ((Cons v vrest)
          (match (match_pat p v acc)
            ((MNo)      (MNo))                        ; definite clash — short-circuit
            ((MStuck)   (match_pats prest vrest acc True))
            ((MOk acc2) (match_pats prest vrest acc2 stuck))))))))

;; try_match_arms: scan arms in order. Fire the first that definitely
;; matches (MOk). On the first UNDECIDED arm (MStuck) stop and report
;; ArmStuck — the caller must reduce the scrutinee and retry, NOT fall
;; through to a later arm (that fall-through was the soundness bug). Skip
;; arms that definitely clash (MNo). All clash ⇒ ArmNone.
(fn try_match_arms ((arms (List Arm)) (v Expr)) ArmResult
  (match arms
    (Nil (ArmNone))
    ((Cons (Arm p body) rest)
      (match (match_pat p v Nil)
        ((MOk bindings) (ArmFired (open_many bindings body)))
        ((MStuck)       (ArmStuck))
        ((MNo)          (try_match_arms rest v))))))

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
    ;; NOTE: sym_of_chars / chars_of_sym (src/prim.rs) are deliberately
    ;; NOT dispatched here. They are parser primitives; the reader runs
    ;; under native eval (not the bootstrapped reducer), and its OUTPUT
    ;; AST never contains them — so compute_expr never needs them. If a
    ;; bootstrap-run program ever calls them, decode must additionally
    ;; unwrap the object (IntLit …) element form.
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
;; ArmStuck (a deep pattern needs a not-yet-reduced position) means reduce
;; the scrutinee further and retry — NOT fall through to a later arm.
(fn step_match ((m Module) (s Expr) (arms (List Arm))) (Option Expr)
  (match s
    ((Ctor _ _)  (resolve_arms m s arms))
    ((IntLit _)  (resolve_arms m s arms))
    ((SymLit _)  (resolve_arms m s arms))
    (_
      (match (step m s)
        ((Some s2) (Some (Match s2 arms)))
        (None      None)))))

;; Resolve a value-headed match for the full reducer `step`.
(fn resolve_arms ((m Module) (s Expr) (arms (List Arm))) (Option Expr)
  (match (try_match_arms arms s)
    ((ArmFired e2) (Some e2))
    ((ArmNone)     None)                 ; all arms clash — stuck (non-exhaustive)
    ((ArmStuck)
      (match (step m s)                  ; reduce a sub-position deeper, then retry
        ((Some s2) (Some (Match s2 arms)))
        (None      None)))))             ; symbolic — leave the match unreduced

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
    ((Ctor _ _)  (resolve_arms_iota m s arms))
    ((IntLit _)  (resolve_arms_iota m s arms))
    ((SymLit _)  (resolve_arms_iota m s arms))
    (_
      (match (step_iota m s)
        ((Some s2) (Some (Match s2 arms)))
        (None      None)))))

;; Resolve a value-headed match for the ι-only reducer. On ArmStuck it
;; tries ι-reduction of the scrutinee (which does NOT unfold user calls),
;; so a deep pattern blocked on a user-fn sub-call simply stays put — the
;; documented ι-only limitation — but a wrong arm is never fired.
(fn resolve_arms_iota ((m Module) (s Expr) (arms (List Arm))) (Option Expr)
  (match (try_match_arms arms s)
    ((ArmFired e2) (Some e2))
    ((ArmNone)     None)
    ((ArmStuck)
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

;; ---------------------------------------------------------------------------
;; step_head: HEAD-only one-step reducer. Fires when the expression's
;; head can reduce in a single move without descending into subterms:
;;   - Match with value-headed (Ctor/IntLit/SymLit) scrutinee → fires
;;   - If with True/False condition → fires
;;   - Let → opens
;;   - Call that's a primitive with all-value args → fires
;;   - everything else → None (including user-fn Calls — head_step
;;     does NOT unfold those)
;;
;; This is the gate for step_smart's δ-step. Cheap, non-recursive,
;; mechanically obvious. Distinguished from `step` (which recurses
;; into scrutinees / args) and `step_iota` (which descends into Ctor
;; / Call args but doesn't fire primitives).
;; ---------------------------------------------------------------------------

(fn step_head ((e Expr)) (Option Expr)
  (match e
    ((Match s arms) (head_match s arms))
    ((Let bs body)  (Some (open_many bs body)))
    ((If c t el)
      (match c
        ((Ctor (quote True)  Nil) (Some t))
        ((Ctor (quote False) Nil) (Some el))
        (_                        None)))
    ((Call f args) (try_step_prim f args))
    (_ None)))

;; head_match: the gate's one-step lookahead. The match fires AT THE HEAD
;; only if an arm definitely matches now (ArmFired). ArmStuck (a deep
;; pattern still needs a sub-position reduced) does NOT fire at the head —
;; the body would need further reduction first — so the gate sees None and
;; declines to unfold, exactly as for a non-value scrutinee.
(fn head_match ((s Expr) (arms (List Arm))) (Option Expr)
  (match s
    ((Ctor _ _)  (arm_fired_or_none (try_match_arms arms s)))
    ((IntLit _)  (arm_fired_or_none (try_match_arms arms s)))
    ((SymLit _)  (arm_fired_or_none (try_match_arms arms s)))
    (_           None)))

(fn arm_fired_or_none ((r ArmResult)) (Option Expr)
  (match r
    ((ArmFired e2) (Some e2))
    ((ArmStuck)    None)
    ((ArmNone)     None)))

;; ---------------------------------------------------------------------------
;; step_smart: ι plus *gated* δ. Wired into `simp_expr` so the kernel's
;; full reducer is no longer naively unfolding-on-demand.
;;
;; The gate, for a user-fn Call: compute the would-be unfolded body
;; (via apply_fn), then check whether `step_head` can take a one-step
;; reduction on it AT THE HEAD. If yes — commit the unfolding; if no
;; — leave the Call stuck and try to step its args.
;;
;; What this buys: a Call whose unfolded body would immediately stick
;; on a non-value Match scrutinee (e.g. `(append xs Nil)` where `xs`
;; is FVar) is NOT unfolded. The IH-shaped subterm stays exposed for
;; Rewrite. The per-ctor-arm helper-lemma tax v1 paid (and the v2
;; pre-slice-30 author paid via append_nil_step etc.) collapses to
;; one Simp step. Primitives at the head still always reduce.
;;
;; Why step_head (not full step): full step recurses through Match
;; scrutinees into Calls, so `(append (append (Cons …) ys) zs)`
;; would gate-pass on its outer body (Match scrut is a Call that
;; itself steps), unfolding too eagerly. step_head's "head-only"
;; semantics keeps unfolding precise — only commit when the body
;; will reduce DIRECTLY.
;;
;; See REVISIT — "Simp guarding (gated δ + list-memo)".
;; ---------------------------------------------------------------------------

(fn step_smart ((m Module) (e Expr)) (Option Expr)
  (match e
    ((Call f args)  (step_smart_call m f args))
    ((Match s arms) (step_smart_match m s arms))
    ((Let bs body)  (Some (open_many bs body)))
    ((If c t el)
      (match c
        ((Ctor (quote True)  Nil) (Some t))
        ((Ctor (quote False) Nil) (Some el))
        (_
          (match (step_smart m c)
            ((Some c2) (Some (If c2 t el)))
            (None      None)))))
    ((Ctor c args)
      (match (step_smart_list m args)
        ((Some args2) (Some (Ctor c args2)))
        (None         None)))
    ((FVar _)   None)
    ((BVar _)   None)
    ((IntLit _) None)
    ((SymLit _) None)))

(fn step_smart_call ((m Module) (f Symbol) (args (List Expr))) (Option Expr)
  (match m
    ((Module _ fns _)
      (match (lookup_fn f fns)
        ((Some fd)
          ;; User fn — apply with gate.
          (match (apply_fn fd args)
            (None
              (step_smart_args_in_call m f args))   ; arity mismatch
            ((Some body)
              ;; Gate: head-only one-step lookahead.
              (match (step_head body)
                ((Some _) (Some body))              ; gate passes — commit
                (None     (step_smart_args_in_call m f args))))))
        (None
          ;; Not a user fn — try primitive table; primitives have no
          ;; gate (they always reduce when arg shapes match).
          (match (try_step_prim f args)
            ((Some e2) (Some e2))
            (None      (step_smart_args_in_call m f args))))))))

(fn step_smart_args_in_call ((m Module) (f Symbol) (args (List Expr)))
                             (Option Expr)
  (match (step_smart_list m args)
    ((Some args2) (Some (Call f args2)))
    (None         None)))

(fn step_smart_match ((m Module) (s Expr) (arms (List Arm))) (Option Expr)
  (match s
    ((Ctor _ _)  (resolve_arms_smart m s arms))
    ((IntLit _)  (resolve_arms_smart m s arms))
    ((SymLit _)  (resolve_arms_smart m s arms))
    (_
      (match (step_smart m s)
        ((Some s2) (Some (Match s2 arms)))
        (None      None)))))

;; Resolve a value-headed match for the gated reducer. On ArmStuck it
;; reduces the scrutinee with step_smart (gated), so a symbolic sub-
;; position stays put (step_smart returns None) while a forceable one
;; reduces — and no wrong arm is ever fired.
(fn resolve_arms_smart ((m Module) (s Expr) (arms (List Arm))) (Option Expr)
  (match (try_match_arms arms s)
    ((ArmFired e2) (Some e2))
    ((ArmNone)     None)
    ((ArmStuck)
      (match (step_smart m s)
        ((Some s2) (Some (Match s2 arms)))
        (None      None)))))

(fn step_smart_list ((m Module) (es (List Expr))) (Option (List Expr))
  (match es
    (Nil None)
    ((Cons h t)
      (match (step_smart m h)
        ((Some h2) (Some (Cons h2 t)))
        (None
          (match (step_smart_list m t)
            ((Some t2) (Some (Cons h t2)))
            (None      None)))))))

;; ---------------------------------------------------------------------------
;; Memoization for simp_expr's fixed-point loop.
;;
;; The memo maps inputs to their normal forms. Each top-level
;; reduction step checks the memo first; on a hit, return the cached
;; NF without re-running step_smart. On miss, step + recurse + record.
;;
;; This is the v1 lesson — "any reducer that re-traverses substituted
;; subterms needs sharing/memoization from the start" (TRANSFER.md).
;;
;; Scope: only the OUTER simp_expr loop is memoized. step_smart's
;; internal recursion (into Ctor/Call args, Match scrutinees, etc.)
;; does NOT thread the memo — the narrow language has no monadic
;; do-notation, so threading would multiply every step_smart_* fn's
;; signature. The outer memo catches "the same Expr appears multiple
;; times as a top-level reducer target" (LHS/RHS sharing, repeated
;; subterms after substitution). Finer-grained memoization is a v3
;; concern and pairs with hash-cons or structural sharing.
;;
;; TODO[v3]: replace the list-based memo with a content-addressed
;; / hash-cons store. The current quadratic cost (linear lookup ×
;; linear insert) is acceptable at the scale of v2 proof obligations
;; but won't survive larger reductions.
;; ---------------------------------------------------------------------------

(fn memo_lookup ((memo (List (Pair Expr Expr))) (e Expr)) (Option Expr)
  (match memo
    (Nil None)
    ((Cons (Pair k v) rest)
      (if (expr_eq k e) (Some v) (memo_lookup rest e)))))

(fn simp_expr_loop ((m Module) (memo (List (Pair Expr Expr))) (e Expr))
                    (Pair Expr (List (Pair Expr Expr)))
  (match (memo_lookup memo e)
    ((Some e_nf) (Pair e_nf memo))
    (None
      (match (step_smart m e)
        (None
          ;; e is already normal — record it as its own NF.
          (Pair e (Cons (Pair e e) memo)))
        ((Some e2)
          (match (simp_expr_loop m memo e2)
            ((Pair e_nf memo2)
              (Pair e_nf (Cons (Pair e e_nf) memo2)))))))))

;; simp_expr: drive `step_smart` to fixed point with memoization.
;; The public entry point — kernel callers (apply_step's Simp arm,
;; head_clash in Absurd) hit this. Termination is bounded only by
;; the underlying user fns' termination; the gate prevents
;; immediately-stuck unfoldings but does not solve general
;; non-termination.
(fn simp_expr ((m Module) (e Expr)) Expr
  (match (simp_expr_loop m Nil e)
    ((Pair e_nf _) e_nf)))

;; ---------------------------------------------------------------------------
;; Compute: drive the UNGATED `step` to a fixed point.
;;
;; simp_expr uses the gated step_smart, which refuses to unfold a user-fn
;; call unless its body would fire at the head — that keeps SYMBOLIC
;; recursion from diverging, but it also means a ground term whose control
;; flow is guarded by a user predicate (e.g. `is_digit 49`, whose body is
;; an `if` over a not-yet-reduced primitive) never evaluates: the one-step
;; lookahead sees a non-firing head and gives up. compute_expr removes the
;; gate, so such ground terms reduce to a value.
;;
;; On a GROUND (variable-free) term over total functions this is exactly
;; evaluation and always terminates. On an open/symbolic term it may unfold
;; further than simp_expr and, in the worst case, not terminate — so the
;; `Compute` tactic is intended for closed terms (validating a spec on a
;; concrete input; normalizing a ground subterm of a goal).
;;
;; SOUNDNESS is identical to Simp's: every `step` is a definitional-equality
;; reduction (it is the operational semantics that step_smart is a gated
;; subset of), so the normal form is equal to the input. The gate was only
;; ever a termination heuristic, never a soundness condition. Memoization
;; mirrors simp_expr_loop (shared subterms / LHS-RHS sharing).
;; ---------------------------------------------------------------------------
(fn compute_expr_loop ((m Module) (memo (List (Pair Expr Expr))) (e Expr))
                       (Pair Expr (List (Pair Expr Expr)))
  (match (memo_lookup memo e)
    ((Some e_nf) (Pair e_nf memo))
    (None
      (match (step m e)
        (None
          ;; e is already normal — record it as its own NF.
          (Pair e (Cons (Pair e e) memo)))
        ((Some e2)
          (match (compute_expr_loop m memo e2)
            ((Pair e_nf memo2)
              (Pair e_nf (Cons (Pair e e_nf) memo2)))))))))

(fn compute_expr ((m Module) (e Expr)) Expr
  (match (compute_expr_loop m Nil e)
    ((Pair e_nf _) e_nf)))
