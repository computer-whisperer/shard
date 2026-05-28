//! Bootstrap evaluator for the narrow object language.
//!
//! The Rust crate is the trusted base of the bootstrap tower. Its only
//! contract is to faithfully reduce narrow-language terms — no proof
//! work happens here (see docs/REVISIT.md, "Trusted-by-review Rust
//! component"). Everything proof-shaped is written in narrow and
//! interpreted by this evaluator.
//!
//! Slices in so far: arithmetic / bitwise / comparison primitives,
//! user-fn calls with locally-nameless body opening, `if`, `match`
//! (incl. nested patterns), `let`, constructor application, and
//! `gen_fresh`. Remaining: source loading from disk, the rest of
//! the proof-checker plumbing, and a way to drive the kernel.

pub mod ast;
pub mod eval;
pub mod load;
pub mod prim;

#[cfg(test)]
mod nval;

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_str(src: &str, call: &str) -> ast::Expr {
        let module = load::module_from_str(src).expect("module loads");
        let expr = load::expr_from_str(call, &module).expect("call parses");
        eval::eval(&module, &expr).expect("eval succeeds")
    }

    fn true_v() -> ast::Expr {
        ast::Expr::Ctor("True".into(), Vec::new())
    }
    fn false_v() -> ast::Expr {
        ast::Expr::Ctor("False".into(), Vec::new())
    }

    /// Load the on-disk narrow kernel. Order is significant for
    /// readability only — the loader handles forward refs across files.
    fn load_kernel() -> ast::Module {
        let manifest = env!("CARGO_MANIFEST_DIR");
        let p = |n: &str| std::path::PathBuf::from(manifest).join("kernel").join(n);
        load::module_from_paths(&[
            p("stdlib.sexp"),
            p("module.sexp"),
            p("proof.sexp"),
            p("term.sexp"),
            p("reduce.sexp"),
            p("check.sexp"),
        ])
        .expect("kernel loads")
    }

    /// Construct a runtime narrow-Expr ctor value: `nctor("Foo", vec![…])`
    /// builds `Ctor("Foo", […])`. Used to write expected results for
    /// tests that exercise kernel fns over Expr/Pat/Env *values*.
    fn nctor(name: &str, args: Vec<ast::Expr>) -> ast::Expr {
        ast::Expr::Ctor(name.into(), args)
    }

    // ------------------------------------------------------------------
    // Slice 1: arithmetic MVP — user fn + primitive.
    // ------------------------------------------------------------------

    #[test]
    fn mvp_add_two_three() {
        assert_eq!(
            eval_str("(fn add ((a Int) (b Int)) Int (+ a b))", "(add 2 3)"),
            ast::Expr::IntLit(5)
        );
    }

    /// Catches arg-order bugs `+` (symmetric) would mask.
    #[test]
    fn mvp_first_arg() {
        assert_eq!(
            eval_str("(fn first ((a Int) (b Int)) Int a)", "(first 7 9)"),
            ast::Expr::IntLit(7)
        );
    }

    // ------------------------------------------------------------------
    // Slice 2: primitives — exhaustive coverage of one example each.
    // ------------------------------------------------------------------

    #[test]
    fn prim_arithmetic() {
        let m = "(fn id ((x Int)) Int x)";
        assert_eq!(eval_str(m, "(- 10 3)"),     ast::Expr::IntLit(7));
        assert_eq!(eval_str(m, "(* 6 7)"),      ast::Expr::IntLit(42));
        assert_eq!(eval_str(m, "(/ 17 5)"),     ast::Expr::IntLit(3));
        assert_eq!(eval_str(m, "(mod 17 5)"),   ast::Expr::IntLit(2));
        assert_eq!(eval_str(m, "(mod -3 5)"),   ast::Expr::IntLit(2));
    }

    #[test]
    fn prim_bitwise() {
        let m = "(fn id ((x Int)) Int x)";
        assert_eq!(eval_str(m, "(band 12 10)"), ast::Expr::IntLit(8));
        assert_eq!(eval_str(m, "(bor 12 10)"),  ast::Expr::IntLit(14));
        assert_eq!(eval_str(m, "(bxor 12 10)"), ast::Expr::IntLit(6));
        assert_eq!(eval_str(m, "(bshl 1 4)"),   ast::Expr::IntLit(16));
        assert_eq!(eval_str(m, "(bshr 16 2)"),  ast::Expr::IntLit(4));
    }

    #[test]
    fn prim_bool_returning() {
        let m = "(fn id ((x Int)) Int x)";
        assert_eq!(eval_str(m, "(int_eq 3 3)"), true_v());
        assert_eq!(eval_str(m, "(int_eq 3 4)"), false_v());
        assert_eq!(eval_str(m, "(lt 1 2)"),     true_v());
        assert_eq!(eval_str(m, "(lt 2 1)"),     false_v());
        assert_eq!(eval_str(m, "(le 5 5)"),     true_v());
        assert_eq!(eval_str(m, "(le 6 5)"),     false_v());
    }

    #[test]
    fn prim_in_user_fn() {
        assert_eq!(
            eval_str("(fn square ((x Int)) Int (* x x))", "(square 9)"),
            ast::Expr::IntLit(81)
        );
    }

    #[test]
    fn prim_gen_fresh_unique() {
        let m = "(fn id ((x Int)) Int x)";
        let a = eval_str(m, "(gen_fresh)");
        let b = eval_str(m, "(gen_fresh)");
        assert!(matches!(a, ast::Expr::SymLit(_)));
        assert!(matches!(b, ast::Expr::SymLit(_)));
        assert_ne!(a, b);
    }

    // ------------------------------------------------------------------
    // Slice 3: control flow — if, match, let — and ADTs.
    // ------------------------------------------------------------------

    /// `if` dispatches on True/False ctors returned by comparison.
    #[test]
    fn if_branches() {
        let src = r#"
(type Bool (False) (True))
(fn abs ((x Int)) Int (if (lt x 0) (- 0 x) x))
"#;
        let m = load::module_from_str(src).expect("loads");
        for (input, expected) in [("(abs -5)", 5), ("(abs 7)", 7), ("(abs 0)", 0)] {
            let e = load::expr_from_str(input, &m).expect("parses");
            assert_eq!(eval::eval(&m, &e).unwrap(), ast::Expr::IntLit(expected));
        }
    }

    /// Constructors as values, no match — Ctor is just data.
    #[test]
    fn ctor_as_value() {
        let src = r#"
(type (Pair A B) (Pair A B))
(fn make_pair ((x Int) (y Int)) (Pair Int Int) (Pair x y))
"#;
        let m = load::module_from_str(src).expect("loads");
        let e = load::expr_from_str("(make_pair 3 7)", &m).expect("parses");
        let result = eval::eval(&m, &e).expect("evals");
        assert_eq!(
            result,
            ast::Expr::Ctor(
                "Pair".into(),
                vec![ast::Expr::IntLit(3), ast::Expr::IntLit(7)]
            )
        );
    }

    /// Match + recursion: `len` over a List. The keystone test for this
    /// slice — exercises type defs, ctor recognition, nested-list
    /// values, pattern bindings, BVar resolution under match arms, and
    /// recursive Call.
    #[test]
    fn match_list_length() {
        let src = r#"
(type (List T) (Nil) (Cons T (List T)))
(fn len ((xs (List Int))) Int
  (match xs
    (Nil 0)
    ((Cons _ rest) (+ 1 (len rest)))))
"#;
        let m = load::module_from_str(src).expect("loads");
        let call = "(len (Cons 10 (Cons 20 (Cons 30 Nil))))";
        let e = load::expr_from_str(call, &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), ast::Expr::IntLit(3));
    }

    /// Nested pattern: `(Cons a (Cons b _))` binds two PVars at
    /// distinct depths, picks up arity from a sub-pattern, and
    /// the body uses both bindings.
    #[test]
    fn match_nested_pattern() {
        let src = r#"
(type (List T) (Nil) (Cons T (List T)))
(fn first_two_sum ((xs (List Int))) Int
  (match xs
    ((Cons a (Cons b _)) (+ a b))
    (_ 0)))
"#;
        let m = load::module_from_str(src).expect("loads");
        let two_plus = "(first_two_sum (Cons 10 (Cons 20 (Cons 30 Nil))))";
        let too_short = "(first_two_sum (Cons 5 Nil))";
        let empty = "(first_two_sum Nil)";

        let e1 = load::expr_from_str(two_plus, &m).expect("parses");
        let e2 = load::expr_from_str(too_short, &m).expect("parses");
        let e3 = load::expr_from_str(empty, &m).expect("parses");
        assert_eq!(eval::eval(&m, &e1).unwrap(), ast::Expr::IntLit(30));
        assert_eq!(eval::eval(&m, &e2).unwrap(), ast::Expr::IntLit(0));
        assert_eq!(eval::eval(&m, &e3).unwrap(), ast::Expr::IntLit(0));
    }

    /// Parallel `let`: RHSs evaluated in outer scope; body sees both
    /// bindings. Catches mis-counting of binder depth when opening
    /// the body.
    #[test]
    fn let_parallel() {
        let src = r#"
(fn add_squares ((a Int) (b Int)) Int
  (let ((aa (* a a))
        (bb (* b b)))
    (+ aa bb)))
"#;
        let m = load::module_from_str(src).expect("loads");
        let e = load::expr_from_str("(add_squares 3 4)", &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), ast::Expr::IntLit(25));
    }

    /// `match` on an `Option`-shaped value — bare zero-arg ctors AND
    /// ctors with bindings in the same match.
    #[test]
    fn match_option() {
        let src = r#"
(type (Option T) (None) (Some T))
(fn unwrap_or ((o (Option Int)) (default Int)) Int
  (match o
    (None default)
    ((Some x) x)))
"#;
        let m = load::module_from_str(src).expect("loads");
        let e1 = load::expr_from_str("(unwrap_or (Some 42) 0)", &m).unwrap();
        let e2 = load::expr_from_str("(unwrap_or None 99)", &m).unwrap();
        assert_eq!(eval::eval(&m, &e1).unwrap(), ast::Expr::IntLit(42));
        assert_eq!(eval::eval(&m, &e2).unwrap(), ast::Expr::IntLit(99));
    }

    // ------------------------------------------------------------------
    // Slice 4: load the real narrow kernel from disk.
    // ------------------------------------------------------------------

    /// First time the actual kernel/*.sexp files meet the loader.
    /// Expected to expose any latent gaps; it's worth running just to
    /// see what falls over.
    #[test]
    fn load_real_kernel() {
        let module = load_kernel();
        // Sanity: we should have meaningfully many definitions.
        assert!(module.types.len() >= 10, "got {} types", module.types.len());
        assert!(module.fns.len() >= 20, "got {} fns", module.fns.len());

        // Spot-check a few specific definitions that should be there.
        // Catches the case where "load succeeded" really means "loader
        // silently skipped most forms."
        let type_names: Vec<&str> = module.types.iter().map(|t| t.name.as_str()).collect();
        for required in ["List", "Option", "Bool", "Expr", "Pat", "Arm",
                         "FnDef", "Module", "Goal", "Proof", "Theory", "Sequent"] {
            assert!(type_names.contains(&required),
                "missing type def: {required}\nhave: {type_names:?}");
        }

        let fn_names: Vec<&str> = module.fns.iter().map(|f| f.name.as_str()).collect();
        for required in ["lookup", "subst", "open_many", "match_pat", "step",
                         "lookup_typedef", "check_sequent", "do_induct"] {
            assert!(fn_names.contains(&required),
                "missing fn: {required}");
        }

        eprintln!(
            "loaded kernel: {} types, {} fns, {} externs",
            module.types.len(),
            module.fns.len(),
            module.externs.len()
        );
    }

    /// Don't just *load* kernel code — actually evaluate a few small
    /// functions to confirm the locally-nameless body opening + match
    /// reduction works on the real definitions, not just hand-crafted
    /// test programs.
    #[test]
    fn run_kernel_pat_arity() {
        let module = load_kernel();

        // pat_arity of various pats
        for (call, expected) in [
            ("(pat_arity (PVar))",         1),
            ("(pat_arity (PInt 42))",      0),
            ("(pat_arity (PSym (quote x)))", 0),
        ] {
            let e = load::expr_from_str(call, &module).expect("parses");
            let r = eval::eval(&module, &e).expect("evals");
            assert_eq!(r, ast::Expr::IntLit(expected), "{call}");
        }
    }

    // ------------------------------------------------------------------
    // Slice 5: exercise larger kernel functions end-to-end.
    //
    // pat_arity is one short match away from the IntLit it returns; it
    // doesn't really stress the kernel. These tests drive
    // `subst`, `open_many`, and `match_pat` over real Expr/Pat values.
    // Each one transits multiple kernel helpers (lookup, nth_opt,
    // shift, match_pats, etc.), so a regression in any of them lands
    // here.
    //
    // Inputs and expected outputs are constructed at the
    // narrow-Expr-value level: e.g. `(IntLit 42)` is source that *builds*
    // a runtime value `Ctor("IntLit", [IntLit(42)])`. This is the level
    // the kernel pattern-matches on.
    // ------------------------------------------------------------------

    /// `subst` replaces a matching FVar with the env's bound value.
    /// Exercises: `subst`'s FVar arm → `lookup` → `sym_eq` prim.
    #[test]
    fn run_kernel_subst_fvar_hit() {
        let m = load_kernel();
        let call = "(subst (Bind (quote x) (IntLit 42) (Empty)) (FVar (quote x)))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        // Expected runtime Expr value: `Ctor("IntLit", [IntLit 42])`.
        assert_eq!(r, nctor("IntLit", vec![ast::Expr::IntLit(42)]));
    }

    /// `subst` leaves an FVar untouched when the env doesn't bind it.
    /// Same code paths as above plus the `None` arm of `lookup`.
    #[test]
    fn run_kernel_subst_fvar_miss() {
        let m = load_kernel();
        let call = "(subst (Bind (quote x) (IntLit 42) (Empty)) (FVar (quote y)))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        // Unchanged — returns the FVar value.
        assert_eq!(r, nctor("FVar", vec![ast::Expr::SymLit("y".into())]));
    }

    /// `open_many` fills a BVar from the bindings list at index 0.
    /// Exercises: `open_many` → `open_many_at` → `nth_opt` → `shift`
    /// (with by=0, a no-op) → primitive `lt` / `int_eq`. The whole
    /// locally-nameless opening core in one call.
    #[test]
    fn run_kernel_open_many_bvar0() {
        let m = load_kernel();
        let call = "(open_many (Cons (IntLit 99) (Nil)) (BVar 0))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        assert_eq!(r, nctor("IntLit", vec![ast::Expr::IntLit(99)]));
    }

    /// `open_many` shifts an outer BVar down by `len bindings`.
    /// Catches off-by-one in the "outer binder" arm of `open_many_at`.
    #[test]
    fn run_kernel_open_many_outer_bvar() {
        let m = load_kernel();
        // One binding, BVar 5 — outer, becomes BVar 4.
        let call = "(open_many (Cons (IntLit 99) (Nil)) (BVar 5))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        assert_eq!(r, nctor("BVar", vec![ast::Expr::IntLit(4)]));
    }

    /// `match_pat` against a PVar: captures the value at the front of
    /// the accumulator (innermost-first).
    #[test]
    fn run_kernel_match_pat_pvar() {
        let m = load_kernel();
        let call = "(match_pat (PVar) (IntLit 7) (Nil))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        // Some (Cons (IntLit 7) Nil)
        let intlit_7 = nctor("IntLit", vec![ast::Expr::IntLit(7)]);
        let expected = nctor("Some",
            vec![nctor("Cons", vec![intlit_7, nctor("Nil", vec![])])]);
        assert_eq!(r, expected);
    }

    /// `match_pat` against an int-literal pattern: success when the
    /// values match, None when they don't. Exercises the `PInt` arm,
    /// the inner `match v` on `IntLit`, and the `int_eq` prim.
    #[test]
    fn run_kernel_match_pat_pint() {
        let m = load_kernel();
        // Hit.
        let hit = load::expr_from_str(
            "(match_pat (PInt 5) (IntLit 5) (Nil))", &m).expect("parses");
        assert_eq!(eval::eval(&m, &hit).unwrap(),
            nctor("Some", vec![nctor("Nil", vec![])]));
        // Miss.
        let miss = load::expr_from_str(
            "(match_pat (PInt 5) (IntLit 6) (Nil))", &m).expect("parses");
        assert_eq!(eval::eval(&m, &miss).unwrap(),
            nctor("None", vec![]));
    }

    /// `match_pat` over `(PCtor Cons [PVar PVar])` against a list value
    /// — exercises nested matching via `match_pats` and the
    /// innermost-first capture order. Result acc is
    /// (Cons <tail> (Cons <head> Nil)) — head first PVar → highest BVar,
    /// tail second PVar → BVar 0 → at front of acc.
    #[test]
    fn run_kernel_match_pat_pctor_nested() {
        let m = load_kernel();
        // Pattern: (PCtor Cons [PVar PVar])
        // Value:   (Ctor Cons [IntLit 1, Ctor Nil []])
        //   — note narrow Cons is a 2-arg ctor (head, tail),
        //   so its runtime layout matches the pattern's arity.
        let call = "(match_pat \
                      (PCtor (quote Cons) (Cons (PVar) (Cons (PVar) (Nil)))) \
                      (Ctor (quote Cons) (Cons (IntLit 1) (Cons (Ctor (quote Nil) (Nil)) (Nil)))) \
                      (Nil))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        let intlit_1 = nctor("IntLit", vec![ast::Expr::IntLit(1)]);
        let nil_v = nctor("Ctor", vec![
            ast::Expr::SymLit("Nil".into()),
            nctor("Nil", vec![]),
        ]);
        // After two PVar captures (head, then tail), insert-at-front
        // gives: acc = [tail, head] = (Cons nil_v (Cons intlit_1 Nil)).
        let expected = nctor("Some", vec![
            nctor("Cons", vec![
                nil_v,
                nctor("Cons", vec![intlit_1, nctor("Nil", vec![])]),
            ]),
        ]);
        assert_eq!(r, expected);
    }

    // ------------------------------------------------------------------
    // Slice 6: drive check_sequent against tiny proofs.
    //
    // First time the proof checker runs end-to-end. We exercise:
    //   - the Refl arm (the kernel's base case)
    //   - the Steps arm (recursive dispatch back into check_sequent)
    //   - the Some/None paths through apply_steps
    //
    // Everything else in check_sequent is still stubbed (returns False),
    // so the deeper Proof shapes wait for slice 7+.
    //
    // Helper note: building Sequent / Equation / Module *values* by hand
    // in sexp is verbose. If we keep this up we'll want either narrow
    // builder fns or a tiny Rust DSL. For four tests we tolerate it.
    // ------------------------------------------------------------------

    fn empty_module_v() -> &'static str {
        "(Module (Nil) (Nil) (Nil))"
    }
    fn empty_theory_v() -> &'static str {
        "(TheoryEmpty)"
    }

    /// Refl on 1 = 1 — both sides syntactically equal. Routes through
    /// the kernel's expr_eq, and through Sequent destructuring. The
    /// simplest possible "valid proof."
    #[test]
    fn check_seq_refl_hit() {
        let m = load_kernel();
        let call = format!(
            "(check_sequent {} {} \
              (Sequent (Nil) (Nil) (Nil) (Equation (IntLit 1) (IntLit 1))) \
              (Refl))",
            empty_module_v(),
            empty_theory_v()
        );
        let e = load::expr_from_str(&call, &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), true_v());
    }

    /// Refl on 1 = 2 — the same path, but expr_eq returns False.
    /// Catches the case where the kernel always returns True (e.g. a
    /// bug that ignored the equation entirely).
    #[test]
    fn check_seq_refl_miss() {
        let m = load_kernel();
        let call = format!(
            "(check_sequent {} {} \
              (Sequent (Nil) (Nil) (Nil) (Equation (IntLit 1) (IntLit 2))) \
              (Refl))",
            empty_module_v(),
            empty_theory_v()
        );
        let e = load::expr_from_str(&call, &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), false_v());
    }

    /// (Steps Nil Refl) — apply_steps on an empty step list returns
    /// (Some seq) unchanged; check_sequent then recurses into Refl.
    /// Validates that the Steps arm's recursive dispatch threads
    /// state correctly.
    #[test]
    fn check_seq_steps_empty_then_refl() {
        let m = load_kernel();
        let call = format!(
            "(check_sequent {} {} \
              (Sequent (Nil) (Nil) (Nil) (Equation (IntLit 7) (IntLit 7))) \
              (Steps (Nil) (Refl)))",
            empty_module_v(),
            empty_theory_v()
        );
        let e = load::expr_from_str(&call, &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), true_v());
    }

    // ------------------------------------------------------------------
    // Slice 7a: step's If case — small consistency fix in narrow.
    //
    // Until now step had no If arm, so any If would be stuck. With the
    // arm added: True/False head fires; otherwise step the condition
    // and rebuild. These three tests pin both branches plus the
    // stuck-c path.
    // ------------------------------------------------------------------

    /// step over (if True a b) -> Some a
    #[test]
    fn step_if_true_fires() {
        let m = load_kernel();
        let call = "(step (Module (Nil) (Nil) (Nil)) \
                          (If (Ctor (quote True) (Nil)) (IntLit 1) (IntLit 2)))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        assert_eq!(r, nctor("Some", vec![nctor("IntLit", vec![ast::Expr::IntLit(1)])]));
    }

    /// step over (if False a b) -> Some b
    #[test]
    fn step_if_false_fires() {
        let m = load_kernel();
        let call = "(step (Module (Nil) (Nil) (Nil)) \
                          (If (Ctor (quote False) (Nil)) (IntLit 1) (IntLit 2)))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        assert_eq!(r, nctor("Some", vec![nctor("IntLit", vec![ast::Expr::IntLit(2)])]));
    }

    /// step over (if <FVar> a b) -> None — condition is irreducible,
    /// so the If is stuck. Exercises the fall-through arm that
    /// recursively tries to step the condition.
    #[test]
    fn step_if_stuck_on_fvar() {
        let m = load_kernel();
        let call = "(step (Module (Nil) (Nil) (Nil)) \
                          (If (FVar (quote x)) (IntLit 1) (IntLit 2)))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        assert_eq!(r, nctor("None", vec![]));
    }

    // ------------------------------------------------------------------
    // Slice 7c: Reduce wired — first nontrivial proof end-to-end.
    //
    // With apply_step's Reduce arm calling simp_expr, the kernel can
    // now actually *reduce* a goal's side before closing with Refl.
    // The kernel checks proofs that do work — not just trivially-equal
    // statements.
    //
    // The headline test proves (if True 7 42) = 7 via the proof
    // (Steps [(Reduce Lhs)] (Refl)) — reduce normalizes the lhs, Refl
    // closes the equality. This is the smallest possible "real" proof
    // and the bootstrap's first dogfooded validation.
    // ------------------------------------------------------------------

    /// Reduce on an already-NF side is a no-op; equation unchanged; Refl
    /// closes. Sanity case: Reduce doesn't *break* trivially-closed
    /// goals.
    #[test]
    fn check_seq_reduce_on_nf_then_refl() {
        let m = load_kernel();
        let call = format!(
            "(check_sequent {} {} \
              (Sequent (Nil) (Nil) (Nil) (Equation (IntLit 1) (IntLit 1))) \
              (Steps (Cons (Reduce (Lhs)) (Nil)) (Refl)))",
            empty_module_v(),
            empty_theory_v()
        );
        let e = load::expr_from_str(&call, &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), true_v());
    }

    /// Headline: (if True 7 42) = 7. Reduce on Lhs simplifies it via
    /// the new If arm of step, Refl closes. First proof where the
    /// kernel does real reduction work.
    #[test]
    fn check_seq_proves_if_true_equals_then_branch() {
        let m = load_kernel();
        let call = format!(
            "(check_sequent {} {} \
              (Sequent (Nil) (Nil) (Nil) \
                (Equation (If (Ctor (quote True) (Nil)) (IntLit 7) (IntLit 42)) \
                          (IntLit 7))) \
              (Steps (Cons (Reduce (Lhs)) (Nil)) (Refl)))",
            empty_module_v(),
            empty_theory_v()
        );
        let e = load::expr_from_str(&call, &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), true_v());
    }

    /// Same shape, False branch. (if False 7 42) = 42.
    #[test]
    fn check_seq_proves_if_false_equals_else_branch() {
        let m = load_kernel();
        let call = format!(
            "(check_sequent {} {} \
              (Sequent (Nil) (Nil) (Nil) \
                (Equation (If (Ctor (quote False) (Nil)) (IntLit 7) (IntLit 42)) \
                          (IntLit 42))) \
              (Steps (Cons (Reduce (Lhs)) (Nil)) (Refl)))",
            empty_module_v(),
            empty_theory_v()
        );
        let e = load::expr_from_str(&call, &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), true_v());
    }

    /// Falsifying proof: claim (if True 7 42) = 42. Reduce normalizes
    /// lhs to 7; equation becomes 7 = 42; Refl fails; check_sequent
    /// returns False. The kernel REJECTS a bogus claim.
    ///
    /// This is the "doesn't admit nonsense" half of soundness — at
    /// least for this microcosm. Just as important as the positive
    /// case.
    #[test]
    fn check_seq_rejects_false_claim() {
        let m = load_kernel();
        let call = format!(
            "(check_sequent {} {} \
              (Sequent (Nil) (Nil) (Nil) \
                (Equation (If (Ctor (quote True) (Nil)) (IntLit 7) (IntLit 42)) \
                          (IntLit 42))) \
              (Steps (Cons (Reduce (Lhs)) (Nil)) (Refl)))",
            empty_module_v(),
            empty_theory_v()
        );
        let e = load::expr_from_str(&call, &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), false_v());
    }

    /// Reduce Both: simplify lhs AND rhs before Refl. Symmetric
    /// reduction in a single step.  Proves
    ///   (if True 1 2) = (if False 1 2)  ... is FALSE (1 != 2)
    /// And also
    ///   (if True 1 2) = (if False 2 1)  ... is TRUE (both -> 1).
    #[test]
    fn check_seq_reduce_both_sides() {
        let m = load_kernel();
        // Positive: both reduce to 1.
        let pos = format!(
            "(check_sequent {} {} \
              (Sequent (Nil) (Nil) (Nil) \
                (Equation (If (Ctor (quote True)  (Nil)) (IntLit 1) (IntLit 2)) \
                          (If (Ctor (quote False) (Nil)) (IntLit 2) (IntLit 1)))) \
              (Steps (Cons (Reduce (Both)) (Nil)) (Refl)))",
            empty_module_v(),
            empty_theory_v()
        );
        let e = load::expr_from_str(&pos, &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), true_v());

        // Negative: lhs->1, rhs->2.
        let neg = format!(
            "(check_sequent {} {} \
              (Sequent (Nil) (Nil) (Nil) \
                (Equation (If (Ctor (quote True)  (Nil)) (IntLit 1) (IntLit 2)) \
                          (If (Ctor (quote False) (Nil)) (IntLit 1) (IntLit 2)))) \
              (Steps (Cons (Reduce (Both)) (Nil)) (Refl)))",
            empty_module_v(),
            empty_theory_v()
        );
        let e = load::expr_from_str(&neg, &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), false_v());
    }

    // ------------------------------------------------------------------
    // Slice 8: prove equations about USER-defined functions.
    //
    // Up to slice 7, all proofs were over closed primitive expressions
    // (if/True/False, IntLits). This slice adds a Module containing a
    // user-defined fn `double`, then asks the kernel to prove
    // `(double 5) = 10` — exercising the full
    //   simp_expr → step → step_call → lookup_fn → apply_fn → open_many
    // chain on a USER fn, inside the kernel, running in our trusted
    // Rust evaluator.
    //
    // Test inputs are built with src/nval.rs (narrow-value builders)
    // rather than sexp source: constructing FnDef / Module values by
    // hand in sexp was the friction flagged at slice 6. The builder
    // approach also makes the BVar-vs-FVar / Symbol-as-arg distinctions
    // explicit at the call site.
    // ------------------------------------------------------------------

    use nval::*;

    /// Run `(check_sequent m th seq pf)` and return the result.
    fn run_check_sequent(
        m: &ast::Module,
        mod_val: ast::Expr,
        th: ast::Expr,
        seq: ast::Expr,
        pf: ast::Expr,
    ) -> ast::Expr {
        // Construct the call as a Rust ast::Expr directly. The Call's
        // four args are ALREADY runtime-value Exprs (from the
        // builders); the evaluator's CBV will see they're in normal
        // form for the constructors and pass them straight through to
        // check_sequent's body opening.
        let call_expr = ast::Expr::Call(
            "check_sequent".into(),
            vec![mod_val, th, seq, pf],
        );
        eval::eval(m, &call_expr).expect("eval succeeds")
    }

    /// Module containing one user fn: (fn double ((x Int)) Int (+ x x)).
    /// With one parameter, x = BVar 0.
    fn double_module() -> ast::Expr {
        let body = call("+", vec![bvar(0), bvar(0)]);
        let double = fn_def("double", vec![ty_int()], ty_int(), body);
        module(vec![], vec![double], vec![])
    }

    /// Headline: kernel proves (double 5) = 10 by simping the lhs.
    /// step_call finds double in the module, apply_fn opens the body
    /// with [IntLit 5] reversed → body becomes (+ 5 5), step fires the
    /// + primitive → 10. Refl on 10 = 10 closes.
    ///
    /// Uses `Simp` (full δ+ι). Previously this test used `Reduce`, but
    /// slice 14 split Reduce → ι-only and Simp → full; user-fn
    /// unfolding is δ, so it now belongs to Simp.
    #[test]
    fn check_seq_proves_double_5_equals_10() {
        let m = load_kernel();
        let mod_v = double_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(call("double", vec![intlit(5)]), intlit(10)),
        );
        let pf = steps(vec![simp(side_lhs())], refl());
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Same proof script, false claim. (double 5) = 11. Lhs simps to
    /// 10; Refl on 10 = 11 fails; check_sequent returns False.
    #[test]
    fn check_seq_rejects_double_5_equals_11() {
        let m = load_kernel();
        let mod_v = double_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(call("double", vec![intlit(5)]), intlit(11)),
        );
        let pf = steps(vec![simp(side_lhs())], refl());
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    /// Simp Both makes user-fn and primitive reductions meet in the
    /// middle: prove (double 5) = (+ 4 6). LHS: user fn double → 10.
    /// RHS: prim + → 10. Refl on 10 = 10 closes.
    #[test]
    fn check_seq_proves_user_fn_meets_primitive() {
        let m = load_kernel();
        let mod_v = double_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(
                call("double", vec![intlit(5)]),
                call("+", vec![intlit(4), intlit(6)]),
            ),
        );
        let pf = steps(vec![simp(side_both())], refl());
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Nested user-fn application: (double (double 3)) = 12.
    /// Inner call reduces to (+ 3 3) → 6, outer becomes (double 6)
    /// → (+ 6 6) → 12. simp_expr drives `step` through the whole chain.
    #[test]
    fn check_seq_proves_nested_user_fn() {
        let m = load_kernel();
        let mod_v = double_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(
                call("double", vec![call("double", vec![intlit(3)])]),
                intlit(12),
            ),
        );
        let pf = steps(vec![simp(side_lhs())], refl());
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    // ------------------------------------------------------------------
    // Slice 9: Absurd — close from a contradictory in-scope equation.
    //
    // Counterpart to Refl: where Refl closes when both sides ARE equal,
    // Absurd closes when a cited equation HAS to be false. Useful in
    // induction's vacuous cases (the "impossible" branch). v2's Absurd
    // requires the cited equation to be ground (no ∀-binders, no
    // premises); richer cases wait for later slices.
    //
    // Goals are intentionally chosen so Refl could NOT close them, to
    // make Absurd's contribution unambiguous.
    // ------------------------------------------------------------------

    /// Premise `1 = 2` is impossible (distinct IntLits). Absurd closes
    /// the arbitrary goal `0 = 1`, which Refl could not.
    #[test]
    fn check_seq_absurd_premise_int_clash() {
        let m = load_kernel();
        let seq = sequent(
            vec![], vec![],
            vec![equation(intlit(1), intlit(2))],     // false premise
            equation(intlit(0), intlit(1)),           // arbitrary goal
        );
        let pf = absurd(er_premise(0));
        let r = run_check_sequent(&m, module(vec![], vec![], vec![]), theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Hypothesis `(Cons 1 Nil) = Nil` is impossible (distinct ctors).
    /// Absurd closes any goal.
    #[test]
    fn check_seq_absurd_hyp_ctor_clash() {
        let m = load_kernel();
        let bad_hyp = goal(
            vec![], vec![],
            equation(
                ctor_app("Cons", vec![intlit(1), ctor_app("Nil", vec![])]),
                ctor_app("Nil", vec![]),
            ),
        );
        let seq = sequent(
            vec![],
            vec![bad_hyp],
            vec![],
            equation(intlit(0), intlit(1)),
        );
        let pf = absurd(er_hyp(0));
        let r = run_check_sequent(&m, module(vec![], vec![], vec![]), theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Premise `(double 5) = 10` simps to `10 = 10` — no head clash.
    /// Absurd does NOT apply, returns False. Tests that Absurd looks
    /// at the simped form, not the surface form (where lhs and rhs
    /// have different heads: Call vs IntLit).
    #[test]
    fn check_seq_absurd_rejects_actually_true_premise() {
        let m = load_kernel();
        let mod_v = double_module();
        let seq = sequent(
            vec![], vec![],
            vec![equation(call("double", vec![intlit(5)]), intlit(10))],
            equation(intlit(0), intlit(1)),
        );
        let pf = absurd(er_premise(0));
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    /// Out-of-bounds EqRef: resolve_eq returns None, Absurd returns
    /// False. Pins the safe-rejection behavior.
    #[test]
    fn check_seq_absurd_invalid_eqref() {
        let m = load_kernel();
        let seq = sequent(
            vec![], vec![], vec![],                  // no premises
            equation(intlit(0), intlit(1)),
        );
        let pf = absurd(er_premise(0));              // index 0 doesn't exist
        let r = run_check_sequent(&m, module(vec![], vec![], vec![]), theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    // ------------------------------------------------------------------
    // Slice 10: Unfold — single δ-step on a named user fn.
    //
    // Unlike Reduce, Unfold preserves intermediate structure: after
    //   (double 5) -[Unfold double]-> (+ 5 5)
    // the (+ 5 5) is NOT further simplified. This is the proof
    // language's "expose definition" knob.
    // ------------------------------------------------------------------

    /// (double 5) = (+ 5 5). They're not syntactically equal at the
    /// surface, but ONE Unfold of double turns lhs into (+ 5 5). Refl
    /// then closes. No Reduce involved.
    #[test]
    fn check_seq_proves_unfold_then_refl() {
        let m = load_kernel();
        let mod_v = double_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(
                call("double", vec![intlit(5)]),
                call("+", vec![intlit(5), intlit(5)]),
            ),
        );
        let pf = steps(vec![unfold("double", side_lhs())], refl());
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Cite a fn that doesn't exist in the module. unfold_one returns
    /// None (lookup_fn fails); apply_step returns None; Steps arm
    /// returns False.
    #[test]
    fn check_seq_rejects_unfold_nonexistent_fn() {
        let m = load_kernel();
        let mod_v = double_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(call("double", vec![intlit(5)]), intlit(10)),
        );
        let pf = steps(vec![unfold("nonexistent_fn", side_lhs())], refl());
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    /// Cite a real fn but the chosen side has no call to it. lhs is
    /// just (IntLit 5) — no double call. unfold_one returns None.
    #[test]
    fn check_seq_rejects_unfold_when_fn_absent_from_side() {
        let m = load_kernel();
        let mod_v = double_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(intlit(5), call("double", vec![intlit(2)])),  // double on RHS only
        );
        let pf = steps(vec![unfold("double", side_lhs())], refl());
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    // ------------------------------------------------------------------
    // Slice 11: CaseOn — per-ctor sub-sequents with a new hypothesis.
    //
    // The kernel splits a goal into one sub-proof per constructor of
    // the given type. Each sub-sequent gets:
    //   - fresh FVars for the ctor's fields (added to params)
    //   - a new hyp at index 0: scrut = (Ctor cname fresh-fields…)
    //
    // v2 limitation: the new hypothesis can't yet be *used* in the
    // sub-proof — that requires Rewrite. So slice 11 tests dispatch
    // structure only: every sub-proof closes via Refl or another
    // hyp-free path. Hypothesis-consuming proofs wait for slice 12.
    // ------------------------------------------------------------------

    /// Module containing just `(type Bool (False) (True))`.
    fn bool_module() -> ast::Expr {
        let bool_td = type_def("Bool", vec![], vec![
            ctor_def("False", vec![]),
            ctor_def("True",  vec![]),
        ]);
        module(vec![bool_td], vec![], vec![])
    }

    /// Both branches close trivially with Refl on `1 = 1`. Validates
    /// that CaseOn dispatches to BOTH ctor branches and accepts when
    /// ALL succeed.
    #[test]
    fn check_seq_case_on_bool_trivial() {
        let m = load_kernel();
        let mod_v = bool_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(intlit(1), intlit(1)),
        );
        let pf = case_on(fvar("x"), "Bool", vec![
            case_arm("True",  refl()),
            case_arm("False", refl()),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Only the True case provided. check_case_on_cases reaches the
    /// False ctor, find_case returns None, returns False.
    #[test]
    fn check_seq_case_on_missing_case() {
        let m = load_kernel();
        let mod_v = bool_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(intlit(1), intlit(1)),
        );
        let pf = case_on(fvar("x"), "Bool", vec![
            case_arm("True", refl()),
            // False case omitted
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    /// CaseOn on a type the module doesn't declare. lookup_typedef
    /// returns None; do_case_on returns False.
    #[test]
    fn check_seq_case_on_unknown_type() {
        let m = load_kernel();
        let mod_v = bool_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(intlit(1), intlit(1)),
        );
        let pf = case_on(fvar("x"), "Color", vec![
            case_arm("Red", refl()),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    /// CaseOn on a type with field-bearing ctors (IntList). Exercises
    /// mk_fresh_params + gen_fresh + ctor-expr construction with
    /// non-empty fresh-field lists. The sub-proofs still don't USE the
    /// new hypothesis; we just validate that the kernel builds the
    /// sub-sequents correctly when there's structure to build.
    #[test]
    fn check_seq_case_on_with_fields() {
        let m = load_kernel();
        let intlist_td = type_def("IntList", vec![], vec![
            ctor_def("INil", vec![]),
            ctor_def("ICons", vec![ty_int(), tcon("IntList", vec![])]),
        ]);
        let mod_v = module(vec![intlist_td], vec![], vec![]);
        let seq = sequent(
            vec![], vec![], vec![],
            equation(intlit(1), intlit(1)),
        );
        let pf = case_on(fvar("xs"), "IntList", vec![
            case_arm("INil",  refl()),
            case_arm("ICons", refl()),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Sub-proof fails: goal `0 = 1` can't close with Refl on either
    /// branch. check_case_on_cases short-circuits at the first failure.
    #[test]
    fn check_seq_case_on_sub_proof_fails() {
        let m = load_kernel();
        let mod_v = bool_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(intlit(0), intlit(1)),                  // false goal
        );
        let pf = case_on(fvar("x"), "Bool", vec![
            case_arm("True",  refl()),                       // Refl fails: 0 != 1
            case_arm("False", refl()),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    /// Unfold + Simp composition. Goal: (double (double 3)) = 12.
    /// Unfold the outer call -> (+ (double 3) (double 3)). Then
    /// Simp drives to 12. Refl closes. Demonstrates Unfold and Simp
    /// compose. (Previously used Reduce; now uses Simp because Reduce
    /// is ι-only after slice 14 and the post-Unfold form needs δ
    /// for the inner double-calls and `+`.)
    #[test]
    fn check_seq_proves_unfold_then_simp() {
        let m = load_kernel();
        let mod_v = double_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(
                call("double", vec![call("double", vec![intlit(3)])]),
                intlit(12),
            ),
        );
        let pf = steps(
            vec![
                unfold("double", side_lhs()),
                simp(side_lhs()),
            ],
            refl(),
        );
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    // ------------------------------------------------------------------
    // Slice 12: Rewrite — GROUND substitute via a cited equation.
    //
    // v2's first Rewrite supports only:
    //   - insts = Nil
    //   - cited Goal has Goal Nil Nil eq (no ∀-binders, no premises)
    //
    // That makes matching pure structural equality (expr_eq) — no
    // capture, no occurs check. Pattern-variable Rewrite (with ∀-bound
    // var capture from the cited equation) waits for the slice that
    // needs it (inductive proofs typically).
    // ------------------------------------------------------------------

    /// Premise `(FVar x) = (True)`. Rewrite Lr Lhs True (= all) replaces
    /// the FVar in the goal lhs with True. Then Reduce + Refl closes.
    /// First proof that a real Rewrite step transforms the goal.
    #[test]
    fn check_seq_rewrite_premise_then_reduce() {
        let m = load_kernel();
        let mod_v = bool_module();
        // Premise rhs: the EXPR VALUE representing source (True), i.e.
        // (Ctor "True" Nil). `bool_true()` would give Ctor("True", [])
        // — the *Bool* value, which step's If arm doesn't recognize
        // (it looks for the Expr-value shape, not the bare Bool ctor).
        let seq = sequent(
            vec![param("x", tcon("Bool", vec![]))],
            vec![],
            vec![equation(fvar("x"), ctor_app("True", vec![]))],
            equation(
                if_expr(fvar("x"), intlit(1), intlit(2)),
                intlit(1),
            ),
        );
        let pf = steps(
            vec![
                rewrite(er_premise(0), dir_lr(), side_lhs(), bool_true(), vec![]),
                reduce(side_lhs()),
            ],
            refl(),
        );
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Headline: CaseOn + Rewrite together. Prove `(if x 7 7) = 7` for
    /// an arbitrary Bool x. Without Rewrite, neither branch's sub-proof
    /// could close. With Rewrite, each branch rewrites x to its case
    /// ctor, then Reduce simps the if, Refl closes 7 = 7.
    ///
    /// This is the canonical case-and-rewrite proof, and the first
    /// proof where Hyp 0 (the new hypothesis introduced by CaseOn) is
    /// actually USED.
    #[test]
    fn check_seq_case_on_with_rewrite_proves_if_x_77() {
        let m = load_kernel();
        let mod_v = bool_module();
        let seq = sequent(
            vec![param("x", tcon("Bool", vec![]))],
            vec![],
            vec![],
            equation(
                if_expr(fvar("x"), intlit(7), intlit(7)),
                intlit(7),
            ),
        );
        let branch_proof = steps(
            vec![
                rewrite(er_hyp(0), dir_lr(), side_lhs(), bool_true(), vec![]),
                reduce(side_lhs()),
            ],
            refl(),
        );
        let pf = case_on(fvar("x"), "Bool", vec![
            case_arm("True",  branch_proof.clone()),
            case_arm("False", branch_proof),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Direction Rl: cited.rhs → cited.lhs. Premise `(IntLit 1) = (IntLit 2)`,
    /// goal `2 = 1`. Rewrite Rl Lhs replaces 2 with 1. Goal becomes
    /// `1 = 1`. Refl closes.
    #[test]
    fn check_seq_rewrite_dir_rl() {
        let m = load_kernel();
        let seq = sequent(
            vec![], vec![],
            vec![equation(intlit(1), intlit(2))],
            equation(intlit(2), intlit(1)),
        );
        let pf = steps(
            vec![rewrite(er_premise(0), dir_rl(), side_lhs(), bool_true(), vec![])],
            refl(),
        );
        let r = run_check_sequent(&m, module(vec![], vec![], vec![]), theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Pattern not present on the chosen side. Premise `1 = 2`, goal
    /// `5 = 6`. Rewrite Lr Lhs (pat=1) finds no 1 in the lhs. Returns
    /// None; apply_steps short-circuits; Steps arm returns False.
    #[test]
    fn check_seq_rewrite_pattern_absent() {
        let m = load_kernel();
        let seq = sequent(
            vec![], vec![],
            vec![equation(intlit(1), intlit(2))],
            equation(intlit(5), intlit(6)),
        );
        let pf = steps(
            vec![rewrite(er_premise(0), dir_lr(), side_lhs(), bool_true(), vec![])],
            refl(),
        );
        let r = run_check_sequent(&m, module(vec![], vec![], vec![]), theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    /// Non-empty insts not yet supported. Premise `(FVar x) = True`,
    /// proof tries to rewrite with `[(Inst y True)]` (which would
    /// pre-instantiate a ∀-var that doesn't exist on a ground eq
    /// anyway). v2 rejects rather than silently ignoring.
    #[test]
    fn check_seq_rewrite_insts_not_supported() {
        let m = load_kernel();
        let mod_v = bool_module();
        let seq = sequent(
            vec![param("x", tcon("Bool", vec![]))],
            vec![],
            vec![equation(fvar("x"), bool_true())],
            equation(fvar("x"), bool_true()),
        );
        let pf = steps(
            vec![rewrite(er_premise(0), dir_lr(), side_lhs(), bool_true(),
                         vec![inst("y", bool_true())])],
            refl(),
        );
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    /// `all=False` first-occurrence mode. Premise `1 = 99`, goal
    /// `(+ 1 (+ 1 1)) = (+ 99 (+ 1 1))`. Rewrite Lr Lhs all=False
    /// replaces only the FIRST 1 in lhs (leftmost-outermost), giving
    /// `(+ 99 (+ 1 1)) = (+ 99 (+ 1 1))`. Refl closes.
    /// With all=True, all three 1s would be replaced and the rhs
    /// wouldn't match — different proof.
    #[test]
    fn check_seq_rewrite_first_only() {
        let m = load_kernel();
        let seq = sequent(
            vec![], vec![],
            vec![equation(intlit(1), intlit(99))],
            equation(
                call("+", vec![intlit(1), call("+", vec![intlit(1), intlit(1)])]),
                call("+", vec![intlit(99), call("+", vec![intlit(1), intlit(1)])]),
            ),
        );
        let pf = steps(
            vec![rewrite(er_premise(0), dir_lr(), side_lhs(), bool_false(), vec![])],
            refl(),
        );
        let r = run_check_sequent(&m, module(vec![], vec![], vec![]), theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    // ------------------------------------------------------------------
    // Slice 13a: Induct — first runtime exercise of do_induct.
    //
    // do_induct + check_induct_cases + build_induct_subgoal + build_ihs
    // + build_ih have existed since slice 4 but have never executed.
    // First runs surface latent bugs.
    //
    // The smoke test proves `∀ n : Nat. n = n` by induction. Both
    // sub-sequents reduce to syntactically-equal sides (Z = Z and
    // (S _fresh0) = (S _fresh0)) so Refl closes each branch. The IH
    // in the S case is built but unused — slice 13a validates
    // construction, not IH consumption.
    //
    // Real inductive proofs that USE the IH (via Rewrite) come in a
    // follow-up slice — they need user fns with match bodies, which
    // requires more nval builders (Match, Arm, Pat).
    // ------------------------------------------------------------------

    /// Module containing just (type Nat (Z) (S Nat)).
    fn nat_module() -> ast::Expr {
        let nat_td = type_def("Nat", vec![], vec![
            ctor_def("Z", vec![]),
            ctor_def("S", vec![tcon("Nat", vec![])]),
        ]);
        module(vec![nat_td], vec![], vec![])
    }

    /// Smoke test: `∀ n : Nat. n = n` by Induction.
    /// Z case: subgoal eq becomes (Z = Z), Refl closes.
    /// S case: subgoal eq becomes ((S _fresh0) = (S _fresh0)), Refl closes.
    /// Exercises every helper in do_induct's call chain.
    #[test]
    fn check_seq_induct_trivial_refl() {
        let m = load_kernel();
        let mod_v = nat_module();
        let seq = sequent(
            vec![param("n", tcon("Nat", vec![]))],
            vec![], vec![],
            equation(fvar("n"), fvar("n")),
        );
        let pf = induct("n", vec![
            case_arm("Z", refl()),
            case_arm("S", refl()),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Inducting on a name that isn't in scope. find_param returns
    /// None; do_induct returns False.
    #[test]
    fn check_seq_induct_var_not_in_params() {
        let m = load_kernel();
        let mod_v = nat_module();
        let seq = sequent(
            vec![param("n", tcon("Nat", vec![]))],
            vec![], vec![],
            equation(fvar("n"), fvar("n")),
        );
        let pf = induct("m", vec![                    // m, not n
            case_arm("Z", refl()),
            case_arm("S", refl()),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    /// `∀ n : Nat. (id n) = n` via Induct + Unfold + Refl.
    ///
    /// id is the trivial identity (body is just BVar 0). After Unfold
    /// in each subgoal:
    ///   Z case:  (id Z) -> Z         => Z = Z, Refl closes.
    ///   S case:  (id (S _f)) -> (S _f) => (S _f) = (S _f), Refl closes.
    ///
    /// First Induct proof that goes through a step beyond Refl on the
    /// substituted equation. Validates that user fns interact correctly
    /// with the inductively-introduced fresh fvars and substitution.
    ///
    /// Note: id is non-recursive, so no IH consumption is needed (the
    /// S case's IH `(id _f) = _f` is built but unused). IH-CONSUMING
    /// proofs need a single-step ι reduction we don't have yet —
    /// `Reduce` is greedy and would unfold the recursive call too
    /// eagerly. Tracked for a follow-up slice.
    #[test]
    fn check_seq_induct_plus_unfold_proves_id() {
        let m = load_kernel();
        let nat_td = type_def("Nat", vec![], vec![
            ctor_def("Z", vec![]),
            ctor_def("S", vec![tcon("Nat", vec![])]),
        ]);
        // (fn id ((n Nat)) Nat n)  — body is BVar 0.
        let id_fn = fn_def("id",
            vec![tcon("Nat", vec![])],
            tcon("Nat", vec![]),
            bvar(0),
        );
        let mod_v = module(vec![nat_td], vec![id_fn], vec![]);
        let seq = sequent(
            vec![param("n", tcon("Nat", vec![]))],
            vec![], vec![],
            equation(call("id", vec![fvar("n")]), fvar("n")),
        );
        let branch = steps(vec![unfold("id", side_lhs())], refl());
        let pf = induct("n", vec![
            case_arm("Z", branch.clone()),
            case_arm("S", branch),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    // ------------------------------------------------------------------
    // Slice 14: Reduce/Simp split — Reduce becomes ι-only.
    //
    // Before this slice, Reduce drove simp_expr (full δ+ι). Slice 13b
    // hit the wall: IH-consuming inductive proofs need a reducer that
    // STOPS at recursive calls so Rewrite can match the exposed sub-
    // call against the IH. Slice 14 adds step_iota in the kernel and
    // routes Reduce through it, freeing Simp to take over what Reduce
    // used to do.
    //
    // Documented in REVISIT under "Reduce and Simp are now split".
    // ------------------------------------------------------------------

    /// Reduce on a user-fn-headed lhs does NOT unfold anymore.
    /// (double 5) stays (double 5); Refl fails on (double 5) = 10.
    /// Pins the semantic split.
    #[test]
    fn check_seq_reduce_no_longer_unfolds_user_fn() {
        let m = load_kernel();
        let mod_v = double_module();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(call("double", vec![intlit(5)]), intlit(10)),
        );
        let pf = steps(vec![reduce(side_lhs())], refl());
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        // Was True under the old conflated semantics; now False.
        assert_eq!(r, false_v());
    }

    /// Headline: IH-consuming inductive proof.
    ///   ∀ n : Nat. (id_match n) = n
    /// where id_match is defined with an explicit recursive Match.
    ///
    /// id_match body:
    ///   (match n (Z Z) ((S k) (S (id_match k))))
    ///
    /// Z case:  Unfold id_match Lhs -> (Match Z [Z->Z; (S k)->...])
    ///          Reduce Lhs (ι-only): fires Z arm -> Z. Refl on Z = Z.
    /// S case:  IH at Hyp 0: (id_match _f) = _f
    ///          Unfold id_match Lhs -> (Match (S _f) [...])
    ///          Reduce Lhs (ι-only): fires S arm -> (S (id_match _f)).
    ///            CRUCIALLY: Reduce stops here. Under the old full-δ+ι
    ///            Reduce, it would have unfolded (id_match _f) and
    ///            ended at (S (Match (FVar _f) [...])) — a stuck Match
    ///            that no Rewrite against the IH could repair.
    ///          Rewrite (Hyp 0) Lr Lhs all=True: replaces
    ///            (id_match _f) -> _f inside (S _). Lhs becomes (S _f).
    ///          Refl on (S _f) = (S _f).
    ///
    /// This is the first proof where the inductive hypothesis is
    /// CONSUMED — the kernel reasoning chain ties off via the IH
    /// rather than via direct reduction.
    #[test]
    fn check_seq_induct_consumes_ih_via_rewrite() {
        let m = load_kernel();
        let nat_td = type_def("Nat", vec![], vec![
            ctor_def("Z", vec![]),
            ctor_def("S", vec![tcon("Nat", vec![])]),
        ]);
        // (fn id_match ((n Nat)) Nat
        //   (match n
        //     (Z Z)
        //     ((S k) (S (id_match k)))))
        //
        // BVar indices: in the outer body, n = BVar 0. In the (S k)
        // arm body, k = BVar 0 (innermost, pattern-introduced).
        let body = nmatch(bvar(0), vec![
            narm(pctor("Z", vec![]),
                 ctor_app("Z", vec![])),
            narm(pctor("S", vec![pvar()]),
                 ctor_app("S", vec![call("id_match", vec![bvar(0)])])),
        ]);
        let id_match_fn = fn_def("id_match",
            vec![tcon("Nat", vec![])],
            tcon("Nat", vec![]),
            body,
        );
        let mod_v = module(vec![nat_td], vec![id_match_fn], vec![]);
        let seq = sequent(
            vec![param("n", tcon("Nat", vec![]))],
            vec![], vec![],
            equation(call("id_match", vec![fvar("n")]), fvar("n")),
        );

        let z_case = steps(
            vec![
                unfold("id_match", side_lhs()),
                reduce(side_lhs()),
            ],
            refl(),
        );
        let s_case = steps(
            vec![
                unfold("id_match", side_lhs()),
                reduce(side_lhs()),
                rewrite(er_hyp(0), dir_lr(), side_lhs(), bool_true(), vec![]),
            ],
            refl(),
        );
        let pf = induct("n", vec![
            case_arm("Z", z_case),
            case_arm("S", s_case),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Missing case for one ctor. check_induct_cases reaches the S
    /// ctor, find_case returns None, returns False.
    #[test]
    fn check_seq_induct_missing_case() {
        let m = load_kernel();
        let mod_v = nat_module();
        let seq = sequent(
            vec![param("n", tcon("Nat", vec![]))],
            vec![], vec![],
            equation(fvar("n"), fvar("n")),
        );
        let pf = induct("n", vec![
            case_arm("Z", refl()),
            // S case missing
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }
}
