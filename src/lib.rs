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
        let manifest = env!("CARGO_MANIFEST_DIR");
        let p = |n: &str| std::path::PathBuf::from(manifest).join("kernel").join(n);
        let paths = [
            p("stdlib.sexp"),
            p("module.sexp"),
            p("proof.sexp"),
            p("term.sexp"),
            p("reduce.sexp"),
            p("check.sexp"),
        ];
        let module = load::module_from_paths(&paths).expect("kernel loads");
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
        let manifest = env!("CARGO_MANIFEST_DIR");
        let p = |n: &str| std::path::PathBuf::from(manifest).join("kernel").join(n);
        let module = load::module_from_paths(&[
            p("stdlib.sexp"),
            p("module.sexp"),
            p("proof.sexp"),
            p("term.sexp"),
            p("reduce.sexp"),
            p("check.sexp"),
        ])
        .expect("kernel loads");

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
}
