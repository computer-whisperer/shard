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

/// Load the kernel from a directory of `.shard` files. The file list
/// is fixed (the kernel itself is not yet a module tree — see
/// docs/REVISIT.md, "Kernel loader is a flat path list"). Used by the
/// tests; the `eval` binary loads its target's import closure through
/// the self-hosted loader instead.
pub fn load_kernel_from<P: AsRef<std::path::Path>>(
    kernel_dir: P,
) -> Result<ast::Module, load::LoadError> {
    let dir = kernel_dir.as_ref();
    let p = |n: &str| dir.join(n);
    load::module_from_paths(&[
        p("stdlib.shard"),
        p("module.shard"),
        p("proof.shard"),
        p("term.shard"),
        p("reduce.shard"),
        p("checker.shard"),
        p("lia.shard"),
        p("eqdec.shard"),
        p("ord.shard"),
        p("farkas.shard"),
    ])
}

/// The kernel directory at the repo root (compile-time
/// `CARGO_MANIFEST_DIR/../kernel` — the Rust bootstrap lives in
/// `rust_bootstrap/`, the shard sources one level up). Convenience for
/// callers that don't need to point at a different tree.
pub fn default_kernel_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../kernel")
}

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
    /// Thin wrapper around the crate-public `load_kernel_from` so the
    /// file list stays in one place.
    fn load_kernel() -> ast::Module {
        super::load_kernel_from(super::default_kernel_dir())
            .expect("kernel loads")
    }

    /// Construct a runtime narrow-Expr ctor value: `nctor("Foo", vec![…])`
    /// builds `Ctor("Foo", […])`. Used to write expected results for
    /// tests that exercise kernel fns over Expr/Pat/Env *values*.
    fn nctor(name: &str, args: Vec<ast::Expr>) -> ast::Expr {
        ast::Expr::Ctor(name.into(), args)
    }

    /// Build the `(List Int)` runtime value `(Cons x0 (Cons x1 … Nil))`.
    /// Expected-result builder for string-literal / list tests.
    fn int_list(xs: &[i64]) -> ast::Expr {
        let mut acc = nctor("Nil", Vec::new());
        for &x in xs.iter().rev() {
            acc = nctor("Cons", vec![ast::Expr::IntLit(x.into()), acc]);
        }
        acc
    }

    // ------------------------------------------------------------------
    // Stage-0 slice 1: string literals — "abc" ≡ (List Int) of codepoints.
    // ------------------------------------------------------------------

    #[test]
    fn string_literal_is_codepoint_list() {
        let m = "(fn id ((x Int)) Int x)";
        // 'x'=120, '+'=43, 'y'=121.
        assert_eq!(eval_str(m, "\"x+y\""), int_list(&[120, 43, 121]));
        // Empty string is Nil.
        assert_eq!(eval_str(m, "\"\""), int_list(&[]));
        // Digits are their ASCII codepoints, not their numeric value.
        assert_eq!(eval_str(m, "\"0\""), int_list(&[48]));
    }

    /// A string is an ordinary `(List Int)`, so list-shaped fns and
    /// pattern matching work on string literals with no new machinery.
    #[test]
    fn string_reuses_list_ops() {
        let m = "(type (List T) (Nil) (Cons T (List T)))\
                 (fn len ((xs (List Int))) Int \
                   (match xs (Nil 0) ((Cons _ r) (+ 1 (len r)))))";
        assert_eq!(eval_str(m, "(len \"hello\")"), ast::Expr::IntLit(5.into()));
        assert_eq!(eval_str(m, "(len \"\")"), ast::Expr::IntLit(0.into()));
    }

    // ------------------------------------------------------------------
    // Stage-0 slice 2: the lexer — String (List Int) → (List Token).
    // ------------------------------------------------------------------

    /// Load the calc demo atop the kernel (whose ctor names — Cons/Nil/
    /// Some/None — the calc fn bodies reference) and evaluate `call`.
    /// Mirrors how the `calc` binary will run `lex` on a real (List Int):
    /// the string sugar yields raw codepoints, the evaluator runs the
    /// actual fns. Reads the on-disk file so test and shipped demo stay
    /// one source of truth.
    fn calc_eval(call: &str) -> ast::Expr {
        let kernel = load_kernel();
        let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../examples/calc/calc.shard");
        let src = std::fs::read_to_string(&path).expect("read calc.shard");
        let m = load::module_from_str_with_base(&src, Some(&kernel))
            .expect("calc module loads");
        let e = load::expr_from_str(call, &m).expect("call parses");
        eval::eval(&m, &e).expect("eval succeeds")
    }

    fn tnum(n: i64) -> ast::Expr {
        nctor("TNum", vec![ast::Expr::IntLit(n.into())])
    }

    /// `(List Token)` runtime value from a Vec of Token Exprs.
    fn tok_list(toks: Vec<ast::Expr>) -> ast::Expr {
        let mut acc = nctor("Nil", Vec::new());
        for t in toks.into_iter().rev() {
            acc = nctor("Cons", vec![t, acc]);
        }
        acc
    }

    #[test]
    fn lex_basic() {
        let plus = nctor("TPlus", Vec::new());
        let minus = nctor("TMinus", Vec::new());
        assert_eq!(calc_eval("(lex \"7\")"), tok_list(vec![tnum(7)]));
        assert_eq!(
            calc_eval("(lex \"1+2\")"),
            tok_list(vec![tnum(1), plus, tnum(2)])
        );
        assert_eq!(
            calc_eval("(lex \"9-4\")"),
            tok_list(vec![tnum(9), minus, tnum(4)])
        );
        assert_eq!(calc_eval("(lex \"\")"), tok_list(vec![])); // no tokens
    }

    #[test]
    fn lex_multidigit_and_spaces() {
        let plus = || nctor("TPlus", Vec::new());
        let minus = nctor("TMinus", Vec::new());
        // multi-digit numbers fold from their digit codepoints
        assert_eq!(
            calc_eval("(lex \"12+34\")"),
            tok_list(vec![tnum(12), plus(), tnum(34)])
        );
        // leading / interior / trailing whitespace is skipped
        assert_eq!(
            calc_eval("(lex \" 12 + 34 \")"),
            tok_list(vec![tnum(12), plus(), tnum(34)])
        );
        // a longer chain
        assert_eq!(
            calc_eval("(lex \"1+20-300\")"),
            tok_list(vec![tnum(1), plus(), tnum(20), minus, tnum(300)])
        );
    }

    // ------------------------------------------------------------------
    // Stage-0 slice 3: AST + parser + evaluator (the naive impl).
    // ------------------------------------------------------------------

    #[test]
    fn calc_eval_ast() {
        // (1 + 2) evaluates to 3; nesting works.
        assert_eq!(
            calc_eval("(eval (Add (Num 1) (Num 2)))"),
            ast::Expr::IntLit(3.into())
        );
        assert_eq!(
            calc_eval("(eval (Sub (Add (Num 10) (Num 3)) (Num 2)))"),
            ast::Expr::IntLit(11.into()) // 10+3-2
        );
    }

    #[test]
    fn calc_parse_builds_left_assoc_tree() {
        let num = |n: i64| nctor("Num", vec![ast::Expr::IntLit(n.into())]);
        let some = |e| nctor("Some", vec![e]);
        // a bare number
        assert_eq!(calc_eval("(parse (lex \"5\"))"), some(num(5)));
        // "10-3-2" parses left-associatively as (10-3)-2
        let expect = nctor(
            "Sub",
            vec![nctor("Sub", vec![num(10), num(3)]), num(2)],
        );
        assert_eq!(calc_eval("(parse (lex \"10-3-2\"))"), some(expect));
    }

    #[test]
    fn calc_run_end_to_end() {
        let some = |n: i64| nctor("Some", vec![ast::Expr::IntLit(n.into())]);
        let none = || nctor("None", Vec::new());
        // well-formed
        assert_eq!(calc_eval("(run \"1+2\")"), some(3));
        assert_eq!(calc_eval("(run \"12+34\")"), some(46));
        assert_eq!(calc_eval("(run \"9-4\")"), some(5));
        assert_eq!(calc_eval("(run \"10-3-2\")"), some(5)); // left-assoc
        assert_eq!(calc_eval("(run \"1+2-3+4\")"), some(4));
        assert_eq!(calc_eval("(run \" 7 \")"), some(7));
        assert_eq!(calc_eval("(run \"2-5\")"), some(-3)); // negatives
        // malformed → None (total: no panic, an explicit failure value)
        assert_eq!(calc_eval("(run \"1+\")"), none()); // trailing op
        assert_eq!(calc_eval("(run \"+1\")"), none()); // leading op
        assert_eq!(calc_eval("(run \"\")"), none()); // empty
        assert_eq!(calc_eval("(run \"1 2\")"), none()); // two nums, no op
    }

    // ------------------------------------------------------------------
    // Slice 1: arithmetic MVP — user fn + primitive.
    // ------------------------------------------------------------------

    #[test]
    fn mvp_add_two_three() {
        assert_eq!(
            eval_str("(fn add ((a Int) (b Int)) Int (+ a b))", "(add 2 3)"),
            ast::Expr::IntLit(5.into())
        );
    }

    /// Catches arg-order bugs `+` (symmetric) would mask.
    #[test]
    fn mvp_first_arg() {
        assert_eq!(
            eval_str("(fn first ((a Int) (b Int)) Int a)", "(first 7 9)"),
            ast::Expr::IntLit(7.into())
        );
    }

    // ------------------------------------------------------------------
    // Slice 2: primitives — exhaustive coverage of one example each.
    // ------------------------------------------------------------------

    #[test]
    fn prim_arithmetic() {
        let m = "(fn id ((x Int)) Int x)";
        assert_eq!(eval_str(m, "(- 10 3)"),     ast::Expr::IntLit(7.into()));
        assert_eq!(eval_str(m, "(* 6 7)"),      ast::Expr::IntLit(42.into()));
        assert_eq!(eval_str(m, "(/ 17 5)"),     ast::Expr::IntLit(3.into()));
        assert_eq!(eval_str(m, "(mod 17 5)"),   ast::Expr::IntLit(2.into()));
        assert_eq!(eval_str(m, "(mod -3 5)"),   ast::Expr::IntLit(2.into()));
    }

    /// Arbitrary precision: arithmetic past i64 range must be exact, not
    /// wrap. Guards the BigInt swap — under the old i64 primitives these
    /// silently wrapped in release mode (a soundness hole for the LIA /
    /// Farkas multiplier arithmetic that runs through these primitives
    /// at proof-check time).
    #[test]
    fn prim_arithmetic_is_arbitrary_precision() {
        use std::str::FromStr;
        let m = "(fn id ((x Int)) Int x)";
        let big = |s: &str| ast::Expr::IntLit(ast::IntLit::from_str(s).expect("int"));
        // i64::MAX + 1 — the classic wrap.
        assert_eq!(
            eval_str(m, "(+ 9223372036854775807 1)"),
            big("9223372036854775808")
        );
        // i64::MAX² — multiplier-product overflow, the Farkas risk shape.
        assert_eq!(
            eval_str(m, "(* 9223372036854775807 9223372036854775807)"),
            big("85070591730234615847396907784232501249")
        );
        // i64::MIN has no i64 negation; -(MIN) must come out exact.
        assert_eq!(
            eval_str(m, "(- 0 -9223372036854775808)"),
            big("9223372036854775808")
        );
        // Round-trip: the huge product divided back down is exact.
        assert_eq!(
            eval_str(
                m,
                "(/ (* 9223372036854775807 9223372036854775807) 9223372036854775807)"
            ),
            big("9223372036854775807")
        );
        // Comparisons see magnitude, not truncated bits.
        assert_eq!(
            eval_str(m, "(lt 9223372036854775807 (+ 9223372036854775807 1))"),
            true_v()
        );
    }

    /// The paired division variants: `/`+`tmod` (truncating) and
    /// `ediv`+`mod` (Euclidean) must each satisfy a = b*q + r across all
    /// sign combinations, with their respective remainder ranges. The
    /// mixed pair `/`+`mod` does NOT — that mismatch was a false axiom
    /// in std/div until guarded.
    #[test]
    fn prim_division_pairs() {
        let m = "(fn id ((x Int)) Int x)";
        let int = |n: i64| ast::Expr::IntLit(n.into());
        for (a, b) in [(7i64, 10i64), (-7, 10), (7, -10), (-7, -10), (23, 4), (-23, 4)] {
            // truncating pair: q rounds toward zero, r follows the dividend.
            let q = eval_str(m, &format!("(/ {a} {b})"));
            let r = eval_str(m, &format!("(tmod {a} {b})"));
            assert_eq!(q, int(a / b), "(/ {a} {b})");
            assert_eq!(r, int(a % b), "(tmod {a} {b})");
            // Euclidean pair: 0 <= r < |b|.
            let eq = eval_str(m, &format!("(ediv {a} {b})"));
            let er = eval_str(m, &format!("(mod {a} {b})"));
            assert_eq!(er, int(a.rem_euclid(b)), "(mod {a} {b})");
            let (ast::Expr::IntLit(eqv), ast::Expr::IntLit(erv)) = (&eq, &er) else {
                panic!("non-int division result");
            };
            assert_eq!(
                ast::IntLit::from(b) * eqv + erv,
                a.into(),
                "ediv/mod identity at ({a}, {b})"
            );
        }
        // The historical falsity: 10*(-7/10) + (-7 mod 10) = 3, not -7.
        assert_eq!(eval_str(m, "(+ (* 10 (/ -7 10)) (mod -7 10))"), int(3));
    }

    /// Source literals beyond the host parser's integer range must be a
    /// LOAD ERROR, not silently degraded — lexpr falls back to lossy f64
    /// for them. (The self-hosted reader builds ints by arithmetic and
    /// has no such ceiling.)
    #[test]
    fn oversized_literal_is_rejected_at_load() {
        let src = "(fn big (()) Int 99999999999999999999999999999999)";
        assert!(load::module_from_str(src).is_err());
    }

    #[test]
    fn prim_bitwise() {
        let m = "(fn id ((x Int)) Int x)";
        assert_eq!(eval_str(m, "(band 12 10)"), ast::Expr::IntLit(8.into()));
        assert_eq!(eval_str(m, "(bor 12 10)"),  ast::Expr::IntLit(14.into()));
        assert_eq!(eval_str(m, "(bxor 12 10)"), ast::Expr::IntLit(6.into()));
        assert_eq!(eval_str(m, "(bshl 1 4)"),   ast::Expr::IntLit(16.into()));
        assert_eq!(eval_str(m, "(bshr 16 2)"),  ast::Expr::IntLit(4.into()));
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
            ast::Expr::IntLit(81.into())
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
            assert_eq!(eval::eval(&m, &e).unwrap(), ast::Expr::IntLit(expected.into()));
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
                vec![ast::Expr::IntLit(3.into()), ast::Expr::IntLit(7.into())]
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
        assert_eq!(eval::eval(&m, &e).unwrap(), ast::Expr::IntLit(3.into()));
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
        assert_eq!(eval::eval(&m, &e1).unwrap(), ast::Expr::IntLit(30.into()));
        assert_eq!(eval::eval(&m, &e2).unwrap(), ast::Expr::IntLit(0.into()));
        assert_eq!(eval::eval(&m, &e3).unwrap(), ast::Expr::IntLit(0.into()));
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
        assert_eq!(eval::eval(&m, &e).unwrap(), ast::Expr::IntLit(25.into()));
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
        assert_eq!(eval::eval(&m, &e1).unwrap(), ast::Expr::IntLit(42.into()));
        assert_eq!(eval::eval(&m, &e2).unwrap(), ast::Expr::IntLit(99.into()));
    }

    // ------------------------------------------------------------------
    // Slice 4: load the real narrow kernel from disk.
    // ------------------------------------------------------------------

    /// First time the actual kernel/*.shard files meet the loader.
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
            assert_eq!(r, ast::Expr::IntLit(expected.into()), "{call}");
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
        assert_eq!(r, nctor("IntLit", vec![ast::Expr::IntLit(42.into())]));
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
        assert_eq!(r, nctor("IntLit", vec![ast::Expr::IntLit(99.into())]));
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
        assert_eq!(r, nctor("BVar", vec![ast::Expr::IntLit(4.into())]));
    }

    /// `match_pat` against a PVar: captures the value at the front of
    /// the accumulator (innermost-first).
    #[test]
    fn run_kernel_match_pat_pvar() {
        let m = load_kernel();
        let call = "(match_pat (PVar) (IntLit 7) (Nil))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        // MOk (Cons (IntLit 7) Nil)
        let intlit_7 = nctor("IntLit", vec![ast::Expr::IntLit(7.into())]);
        let expected = nctor("MOk",
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
            nctor("MOk", vec![nctor("Nil", vec![])]));
        // Miss.
        let miss = load::expr_from_str(
            "(match_pat (PInt 5) (IntLit 6) (Nil))", &m).expect("parses");
        assert_eq!(eval::eval(&m, &miss).unwrap(),
            nctor("MNo", vec![]));
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
                      (PCtor (QName (Cons (quote core) (Nil)) (quote Cons)) (Cons (PVar) (Cons (PVar) (Nil)))) \
                      (Ctor (QName (Cons (quote core) (Nil)) (quote Cons)) (Cons (IntLit 1) (Cons (Ctor (QName (Cons (quote core) (Nil)) (quote Nil)) (Nil)) (Nil)))) \
                      (Nil))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        let intlit_1 = nctor("IntLit", vec![ast::Expr::IntLit(1.into())]);
        let nil_v = nctor("Ctor", vec![
            obj_qname("Nil"),
            nctor("Nil", vec![]),
        ]);
        // After two PVar captures (head, then tail), insert-at-front
        // gives: acc = [tail, head] = (Cons nil_v (Cons intlit_1 Nil)).
        let expected = nctor("MOk", vec![
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
                          (If (Ctor (QName (Cons (quote core) (Nil)) (quote True)) (Nil)) (IntLit 1) (IntLit 2)))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        assert_eq!(r, nctor("Some", vec![nctor("IntLit", vec![ast::Expr::IntLit(1.into())])]));
    }

    /// step over (if False a b) -> Some b
    #[test]
    fn step_if_false_fires() {
        let m = load_kernel();
        let call = "(step (Module (Nil) (Nil) (Nil)) \
                          (If (Ctor (QName (Cons (quote core) (Nil)) (quote False)) (Nil)) (IntLit 1) (IntLit 2)))";
        let e = load::expr_from_str(call, &m).expect("parses");
        let r = eval::eval(&m, &e).expect("evals");
        assert_eq!(r, nctor("Some", vec![nctor("IntLit", vec![ast::Expr::IntLit(2.into())])]));
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
    // Primitive conformance: prim.rs::try_apply vs reduce.shard's
    // try_step_prim, swept over a value matrix.
    //
    // These are the TWO primitive tables (eval.shard's interpreter
    // reuses try_step_prim, so there is no third): the native one the
    // host applies to stuck calls, and the shard one the object reducer
    // (proof-side compute/simp) and the hosted interpreter dispatch.
    // Their "KEEP IN SYNC" contract was only a comment until now; this
    // sweep makes drift a test failure (ARCHITECTURE_REVIEW finding 5).
    //
    // Method: for each (name, args), the native answer is
    // `prim::try_apply` directly; the object answer is the kernel's
    // `try_step_prim` evaluated in the VM over the REFLECTED args (the
    // object-Expr encoding of the values). Both must agree as Options:
    // Some(reflected value) ↔ (Some <object value>), None ↔ None. An
    // eval ERROR on the object side means try_step_prim delegated to a
    // host prim outside its domain (crash where native is stuck) — the
    // exact divergence class the `/`-on-zero and shift-range bugs were.
    //
    // The name lists below are the SPEC. Adding a primitive to either
    // table without updating them is the drift this test exists to
    // catch — both tables carry a pointer here.
    // ------------------------------------------------------------------

    /// The shared two-Int primitives (in both tables, delegating arms).
    const SHARED_INT2: &[&str] = &[
        "+", "-", "*", "/", "mod", "tmod", "ediv",
        "band", "bor", "bxor", "bshl", "bshr",
        "int_eq", "lt", "le",
    ];

    /// Object-table-only names (native MUST stay ignorant — a word-prim
    /// call reaching the native table is a fail-stop UnknownCall, never
    /// a second implementation). Shapes: two-Int (uwrap/swrap), two-Word,
    /// Word-and-Int (shifts), one-Word.
    const OBJECT_ONLY_INT2: &[&str] = &["uwrap", "swrap"];
    const OBJECT_ONLY_WORD2: &[&str] = &[
        "wadd", "wsub", "wmul", "wand", "wor", "wxor",
        "udiv", "urem", "sdiv", "srem",
        "weq", "ult", "ule", "slt", "sle",
    ];
    const OBJECT_ONLY_WORD_INT: &[&str] = &["wshl", "ushr", "sshr"];
    const OBJECT_ONLY_WORD1: &[&str] = &["wneg", "wnot", "uval", "sval", "wbits", "wwidth"];

    /// The bytes primitives (same object-only discipline as words —
    /// the canonical value is (Ctor 'Bytes (L)), reduce.shard bp_*).
    /// Shapes: one-Bytes, two-Bytes, Bytes-and-Int, Bytes-Int-Int,
    /// one-(List Int).
    const OBJECT_ONLY_BYTES1: &[&str] = &["blen", "list_of_bytes"];
    const OBJECT_ONLY_BYTES2: &[&str] = &["bcat"];
    const OBJECT_ONLY_BYTES_INT: &[&str] = &["bidx"];
    const OBJECT_ONLY_BYTES_INT_INT: &[&str] = &["bslice"];
    const OBJECT_ONLY_INTLIST1: &[&str] = &["bytes_of_list"];

    /// The Int value matrix: zero, units, small values, sign mixes,
    /// shift-range edges (63/64/65, negative), i64 boundaries, and
    /// past-i64 BigInts (the Farkas-multiplier shape).
    fn int_matrix() -> Vec<ast::IntLit> {
        use std::str::FromStr;
        let mut v: Vec<ast::IntLit> = [0i64, 1, -1, 2, -7, 10, -10, 63, 64, 65, -64]
            .iter()
            .map(|&n| ast::IntLit::from(n))
            .collect();
        v.push(ast::IntLit::from(i64::MAX));
        v.push(ast::IntLit::from(i64::MIN));
        v.push(ast::IntLit::from_str("18446744073709551616").unwrap()); // 2^64
        v.push(ast::IntLit::from_str("-100000000000000000000000000000007").unwrap());
        v
    }

    /// Reflect a native runtime VALUE into the object-Expr encoding the
    /// kernel reducer matches on: IntLit n ↔ (IntLit n), SymLit s ↔
    /// (SymLit s), Ctor name args ↔ (Ctor 'name (list …)).
    fn reflect(e: &ast::Expr) -> ast::Expr {
        match e {
            ast::Expr::IntLit(n) => nctor("IntLit", vec![ast::Expr::IntLit(n.clone())]),
            ast::Expr::SymLit(s) => nctor("SymLit", vec![ast::Expr::SymLit(s.clone())]),
            // Object Ctor heads are now (QName module-id local-name), not a bare
            // Symbol; a native-origin value reflects to the `core`-qualified form
            // (see kernel/term.shard QName). The shard reducer produces the same.
            ast::Expr::Ctor(name, args) => nctor(
                "Ctor",
                vec![
                    obj_qname(name),
                    expr_spine(args.iter().map(reflect).collect()),
                ],
            ),
            other => panic!("reflect: not a value: {other:?}"),
        }
    }

    /// The object-encoded `(QName (core) 'name)` identity for a reflected head.
    /// The module-id is a `(List Symbol)` path (Stage 2); builtins live at `core`.
    fn obj_qname(name: &str) -> ast::Expr {
        nctor("QName", vec![
            expr_spine(vec![ast::Expr::SymLit("core".into())]),
            ast::Expr::SymLit(name.into()),
        ])
    }

    /// A runtime (List X) value from element values.
    fn expr_spine(items: Vec<ast::Expr>) -> ast::Expr {
        let mut acc = nctor("Nil", Vec::new());
        for it in items.into_iter().rev() {
            acc = nctor("Cons", vec![it, acc]);
        }
        acc
    }

    /// The native answer, lifted to the object encoding for comparison:
    /// None ↔ (None), Some v ↔ (Some <reflect v>).
    fn reflect_opt(o: Option<ast::Expr>) -> ast::Expr {
        match o {
            None => nctor("None", vec![]),
            Some(v) => nctor("Some", vec![reflect(&v)]),
        }
    }

    /// The object-table answer: `(try_step_prim 'name (list <reflected args>))`
    /// evaluated in the VM. Err means the reducer itself got stuck on a
    /// host prim outside its domain — a would-be crash at check time.
    fn object_apply(
        m: &ast::Module,
        name: &str,
        args: &[ast::Expr],
    ) -> Result<ast::Expr, String> {
        let call = ast::Expr::Call(
            "try_step_prim".into(),
            vec![
                ast::Expr::SymLit(name.into()),
                expr_spine(args.iter().map(reflect).collect()),
            ],
        );
        eval::eval(m, &call).map_err(|e| format!("{e:?}"))
    }

    /// Sweep one (name, args) point through both tables and demand
    /// agreement.
    fn conform(m: &ast::Module, name: &str, args: &[ast::Expr]) {
        let native = reflect_opt(prim::try_apply(name, args));
        match object_apply(m, name, args) {
            Ok(object) => assert_eq!(
                object, native,
                "table drift at ({name} {args:?}): object ≠ native"
            ),
            Err(e) => panic!(
                "object reducer CRASHED at ({name} {args:?}) where native is {native:?}: {e}"
            ),
        }
    }

    /// Every shared two-Int primitive agrees across the full pair matrix
    /// — values AND domains (b=0 divisions, out-of-range shift amounts
    /// must be None/stuck on BOTH sides, not a crash on one).
    #[test]
    fn prim_conformance_shared_int2() {
        let m = load_kernel();
        let vals = int_matrix();
        for name in SHARED_INT2 {
            for a in &vals {
                for b in &vals {
                    let args = [
                        ast::Expr::IntLit(a.clone()),
                        ast::Expr::IntLit(b.clone()),
                    ];
                    conform(&m, name, &args);
                }
            }
        }
    }

    /// The shared Symbol primitives agree (sym_eq pairs; the
    /// chars_of_sym/sym_of_chars round-trip bridge).
    #[test]
    fn prim_conformance_shared_sym() {
        let m = load_kernel();
        let syms = ["x", "y", "wadd", "_fresh0", "λ→"];
        for a in syms {
            for b in syms {
                let args = [
                    ast::Expr::SymLit(a.into()),
                    ast::Expr::SymLit(b.into()),
                ];
                conform(&m, "sym_eq", &args);
            }
            conform(&m, "chars_of_sym", &[ast::Expr::SymLit(a.into())]);
            // sym_of_chars over the symbol's own UTF-8 bytes (round-trip).
            let bs: Vec<ast::Expr> = a
                .bytes()
                .map(|b| ast::Expr::IntLit(b.into()))
                .collect();
            conform(&m, "sym_of_chars", &[expr_spine(bs)]);
        }
        conform(&m, "sym_of_chars", &[expr_spine(vec![])]);
        // out-of-domain points must be None/stuck on BOTH sides, not a
        // crash on one: a non-byte element, a bare continuation byte, an
        // encoded surrogate (ED A0 80 = U+D800), and a truncated lead.
        for bad in [vec![955], vec![0xFF], vec![0xED, 0xA0, 0x80], vec![0xC2]] {
            let es: Vec<ast::Expr> =
                bad.into_iter().map(|b: i64| ast::Expr::IntLit(b.into())).collect();
            conform(&m, "sym_of_chars", &[expr_spine(es)]);
        }
    }

    /// gen_fresh is in both tables (the hosted checker opens binders
    /// with it) but is EFFECTFUL — a process-global counter — so only
    /// Some-ness conforms, never the value.
    #[test]
    fn prim_conformance_gen_fresh_someness() {
        let m = load_kernel();
        let native = prim::try_apply("gen_fresh", &[]);
        assert!(matches!(native, Some(ast::Expr::SymLit(_))));
        let object = object_apply(&m, "gen_fresh", &[]).expect("object gen_fresh evals");
        match object {
            ast::Expr::Ctor(some, args) if some == "Some" && args.len() == 1 => {
                assert!(
                    matches!(&args[0], ast::Expr::Ctor(n, a) if n == "SymLit" && a.len() == 1),
                    "object gen_fresh: not a SymLit: {args:?}"
                );
            }
            other => panic!("object gen_fresh: {other:?}"),
        }
    }

    /// The deliberate asymmetry: word primitives exist ONLY in the
    /// object table. Native must return None for every one (fail-stop
    /// UnknownCall at run time — never a second implementation), while
    /// the object table reduces them. A name appearing in prim.rs
    /// breaks this pin on purpose.
    #[test]
    fn prim_conformance_word_prims_native_ignorant() {
        let m = load_kernel();
        let word = |w: i64, r: i64| {
            nctor(
                "Word",
                vec![ast::Expr::IntLit(w.into()), ast::Expr::IntLit(r.into())],
            )
        };
        let int = |n: i64| ast::Expr::IntLit(n.into());
        let assert_object_only = |name: &str, args: &[ast::Expr]| {
            assert_eq!(
                prim::try_apply(name, args),
                None,
                "native table learned the object-only prim {name}"
            );
            let object = object_apply(&m, name, args)
                .unwrap_or_else(|e| panic!("object {name} crashed: {e}"));
            assert!(
                matches!(&object, ast::Expr::Ctor(n, _) if n == "Some"),
                "object table dropped {name}: {object:?}"
            );
        };
        for name in OBJECT_ONLY_INT2 {
            assert_object_only(name, &[int(8), int(300)]);
        }
        for name in OBJECT_ONLY_WORD2 {
            assert_object_only(name, &[word(8, 200), word(8, 3)]);
            assert_object_only(name, &[word(8, 200), word(8, 0)]); // division-by-zero profile is TOTAL
        }
        for name in OBJECT_ONLY_WORD_INT {
            assert_object_only(name, &[word(8, 200), int(3)]);
            assert_object_only(name, &[word(8, 200), int(99)]); // saturating, still total
        }
        for name in OBJECT_ONLY_WORD1 {
            assert_object_only(name, &[word(8, 200)]);
        }
    }

    /// Bytes primitives: same deliberate asymmetry as words — object
    /// table only, native fail-stops. Also pins the TOTAL profiles
    /// (out-of-range index, clamped slice window, masked maker).
    #[test]
    fn prim_conformance_bytes_prims_native_ignorant() {
        let m = load_kernel();
        let intlist = |xs: &[i64]| {
            expr_spine(xs.iter().map(|&n| ast::Expr::IntLit(n.into())).collect())
        };
        let bytes = |xs: &[i64]| nctor("Bytes", vec![intlist(xs)]);
        let int = |n: i64| ast::Expr::IntLit(n.into());
        let assert_object_only = |name: &str, args: &[ast::Expr]| {
            assert_eq!(
                prim::try_apply(name, args),
                None,
                "native table learned the object-only prim {name}"
            );
            let object = object_apply(&m, name, args)
                .unwrap_or_else(|e| panic!("object {name} crashed: {e}"));
            assert!(
                matches!(&object, ast::Expr::Ctor(n, _) if n == "Some"),
                "object table dropped {name}: {object:?}"
            );
        };
        for name in OBJECT_ONLY_BYTES1 {
            assert_object_only(name, &[bytes(&[10, 20])]);
            assert_object_only(name, &[bytes(&[])]); // empty is canonical too
        }
        for name in OBJECT_ONLY_BYTES2 {
            assert_object_only(name, &[bytes(&[1]), bytes(&[2, 3])]);
        }
        for name in OBJECT_ONLY_BYTES_INT {
            assert_object_only(name, &[bytes(&[10, 20]), int(1)]);
            assert_object_only(name, &[bytes(&[10, 20]), int(99)]); // total: oob reads 0
            assert_object_only(name, &[bytes(&[10, 20]), int(-2)]);
        }
        for name in OBJECT_ONLY_BYTES_INT_INT {
            assert_object_only(name, &[bytes(&[7, 8, 9]), int(1), int(3)]);
            assert_object_only(name, &[bytes(&[7, 8, 9]), int(-5), int(99)]); // clamped, total
            assert_object_only(name, &[bytes(&[7, 8, 9]), int(2), int(1)]); // inverted → empty
        }
        for name in OBJECT_ONLY_INTLIST1 {
            assert_object_only(name, &[intlist(&[300, -1])]); // masked euclidean mod 256, total
        }
    }
}
