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

/// Load the kernel from a directory of `.shard` files. The file list
/// is fixed (the kernel itself is not yet a module tree — see
/// docs/REVISIT.md, "Kernel loader is a flat path list"); this helper
/// is shared by tests and the `check` binary so the list is in one
/// place.
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
        p("check.shard"),
        p("lia.shard"),
        p("eqdec.shard"),
        p("ord.shard"),
        p("farkas.shard"),
    ])
}

/// The kernel directory that ships with this crate (compile-time
/// `CARGO_MANIFEST_DIR/kernel`). Convenience for callers that don't
/// need to point at a different tree.
pub fn default_kernel_dir() -> std::path::PathBuf {
    std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("kernel")
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
    /// file list stays in one place (shared with the `check` binary).
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
            acc = nctor("Cons", vec![ast::Expr::IntLit(x), acc]);
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
        assert_eq!(eval_str(m, "(len \"hello\")"), ast::Expr::IntLit(5));
        assert_eq!(eval_str(m, "(len \"\")"), ast::Expr::IntLit(0));
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
            .join("examples/calc/calc.shard");
        let src = std::fs::read_to_string(&path).expect("read calc.shard");
        let m = load::module_from_str_with_base(&src, Some(&kernel))
            .expect("calc module loads");
        let e = load::expr_from_str(call, &m).expect("call parses");
        eval::eval(&m, &e).expect("eval succeeds")
    }

    fn tnum(n: i64) -> ast::Expr {
        nctor("TNum", vec![ast::Expr::IntLit(n)])
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
            ast::Expr::IntLit(3)
        );
        assert_eq!(
            calc_eval("(eval (Sub (Add (Num 10) (Num 3)) (Num 2)))"),
            ast::Expr::IntLit(11) // 10+3-2
        );
    }

    #[test]
    fn calc_parse_builds_left_assoc_tree() {
        let num = |n| nctor("Num", vec![ast::Expr::IntLit(n)]);
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
        let some = |n| nctor("Some", vec![ast::Expr::IntLit(n)]);
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
        // MOk (Cons (IntLit 7) Nil)
        let intlit_7 = nctor("IntLit", vec![ast::Expr::IntLit(7)]);
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

    /// Insts naming a nonexistent ∀-var must be rejected. The cited
    /// premise here is a ground equation (no ∀-binders), so any Inst
    /// names a name not in cited_params — rejected by all_insts_named.
    /// Negative regression for the validation path.
    #[test]
    fn check_seq_rewrite_insts_unknown_name_rejected() {
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

    // ------------------------------------------------------------------
    // Two-step induction (Induct2). Soundness of this TCB addition rests
    // on: (a) all three arms (Z, SZ, SS) required; (b) a FALSE arm fails;
    // (c) the type must be EXACTLY nullary+unary-recursive (else some
    // values are uncovered). These tests pin each.
    // ------------------------------------------------------------------

    /// `∀ n : Nat. n = n` by Induct2. Z: (Z=Z); SZ: ((S Z)=(S Z)); SS:
    /// ((S (S _f))=(S (S _f))) — all close by Refl (IH unused).
    #[test]
    fn check_seq_induct2_trivial_refl() {
        let m = load_kernel();
        let mod_v = nat_module();
        let seq = sequent(
            vec![param("n", tcon("Nat", vec![]))],
            vec![], vec![],
            equation(fvar("n"), fvar("n")),
        );
        let pf = induct2("n", vec![
            case_arm("Z", refl()),
            case_arm("SZ", refl()),
            case_arm("SS", refl()),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Induct2 must reject a FALSE claim: `∀ n : Nat. n = Z` with all
    /// arms Refl. The SZ arm's goal is (S Z) = Z — Refl fails — so the
    /// whole induction is rejected. (Soundness: a bogus arm sinks it.)
    #[test]
    fn check_seq_induct2_rejects_false_claim() {
        let m = load_kernel();
        let mod_v = nat_module();
        let seq = sequent(
            vec![param("n", tcon("Nat", vec![]))],
            vec![], vec![],
            equation(fvar("n"), ctor_app("Z", vec![])),
        );
        let pf = induct2("n", vec![
            case_arm("Z", refl()),
            case_arm("SZ", refl()),
            case_arm("SS", refl()),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    /// Induct2 must reject when an arm is missing (here SS): every value
    /// class must be discharged or coverage is incomplete.
    #[test]
    fn check_seq_induct2_rejects_missing_arm() {
        let m = load_kernel();
        let mod_v = nat_module();
        let seq = sequent(
            vec![param("n", tcon("Nat", vec![]))],
            vec![], vec![],
            equation(fvar("n"), fvar("n")),
        );
        let pf = induct2("n", vec![
            case_arm("Z", refl()),
            case_arm("SZ", refl()),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    /// SOUNDNESS GUARD: Induct2 must reject a type with a THIRD ctor —
    /// Z/SZ/SS only cover succ-towers over zero, so a third constructor's
    /// values would be left unproven. Here `Tri = A | B Tri | C` is a
    /// valid type and `∀ x : Tri. x = x` is TRUE, but Induct2 must still
    /// reject it (is_two_ctors fails), forcing the honest single-step
    /// Induct instead. Without this guard, Induct2 would be UNSOUND.
    #[test]
    fn check_seq_induct2_rejects_three_ctor_type() {
        let m = load_kernel();
        let tri_td = type_def("Tri", vec![], vec![
            ctor_def("A", vec![]),
            ctor_def("B", vec![tcon("Tri", vec![])]),
            ctor_def("C", vec![]),
        ]);
        let mod_v = module(vec![tri_td], vec![], vec![]);
        let seq = sequent(
            vec![param("x", tcon("Tri", vec![]))],
            vec![], vec![],
            equation(fvar("x"), fvar("x")),
        );
        // Even with a well-formed-looking proof, the 3-ctor type is rejected.
        let pf = induct2("x", vec![
            case_arm("Z", refl()),
            case_arm("SZ", refl()),
            case_arm("SS", refl()),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, false_v());
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

    // ------------------------------------------------------------------
    // Slice 15: bigger inductive proof — 2-arg recursive fn.
    //
    // Stress-tests the slice 14 setup. Proves
    //   ∀ n : Nat. (add_nat n Z) = n
    // where add_nat is the standard recursive definition pattern-
    // matching on its first arg.
    //
    // What's new vs slice 14's id_match proof:
    //   - 2-argument fn — body's BVars index BOTH params + pattern-
    //     introduced vars. The S arm body has BVar 0 (k, pat-bound)
    //     AND BVar 1 (b, outer fn param, shifted by pat_arity=1).
    //   - The goal eq mentions the inductive var in TWO positions
    //     (in the Call and in the rhs). build_ih substitutes both,
    //     producing IH = (add_nat _f Z) = _f.
    //   - The S case's lhs after Reduce contains the IH's lhs as a
    //     subterm of (Ctor S [...]), forcing Rewrite to descend into
    //     a Ctor's args. Slice 12 covered Rewrite's descent in
    //     isolation; here it composes with everything else.
    // ------------------------------------------------------------------

    /// Headline: `∀ n : Nat. (add_nat n Z) = n` by induction.
    #[test]
    fn check_seq_induct_add_nat_zero_right() {
        let m = load_kernel();
        let nat_td = type_def("Nat", vec![], vec![
            ctor_def("Z", vec![]),
            ctor_def("S", vec![tcon("Nat", vec![])]),
        ]);
        // (fn add_nat ((a Nat) (b Nat)) Nat
        //   (match a
        //     (Z b)
        //     ((S k) (S (add_nat k b)))))
        //
        // BVar indices in the static body (innermost-first):
        //   outer-fn-body: BVar 0 = b (last param), BVar 1 = a
        //   S arm body (pat_arity=1, so outer BVars shift up by 1):
        //     BVar 0 = k (pat-bound), BVar 1 = b
        let body = nmatch(
            bvar(1),                            // scrutinee: a
            vec![
                // Z arm: body is b, which is BVar 0 at the outer level
                narm(pctor("Z", vec![]), bvar(0)),
                // S arm: body is (S (add_nat k b)).
                // k = BVar 0 (innermost, pat-bound), b = BVar 1 (shifted).
                narm(pctor("S", vec![pvar()]),
                     ctor_app("S", vec![
                        call("add_nat", vec![bvar(0), bvar(1)]),
                     ])),
            ],
        );
        let add_nat_fn = fn_def("add_nat",
            vec![tcon("Nat", vec![]), tcon("Nat", vec![])],
            tcon("Nat", vec![]),
            body,
        );
        let mod_v = module(vec![nat_td], vec![add_nat_fn], vec![]);

        let seq = sequent(
            vec![param("n", tcon("Nat", vec![]))],
            vec![], vec![],
            equation(
                call("add_nat", vec![fvar("n"), ctor_app("Z", vec![])]),
                fvar("n"),
            ),
        );

        // Z case after substitution (n := Z):
        //   (add_nat Z Z) = Z
        //   Unfold add_nat → opens body with args [Z, Z] (reversed).
        //     Match scrutinee = a's value = Z (BVar 1 → bindings[1] = Z).
        //     Z arm body = b's value = Z (BVar 0 → bindings[0] = Z).
        //     Result: (Match Z [(Z → Z); (S k) → ...]).
        //   Reduce ι → fires Z arm → Z. Refl on Z = Z.
        let z_case = steps(
            vec![
                unfold("add_nat", side_lhs()),
                reduce(side_lhs()),
            ],
            refl(),
        );

        // S case after substitution (n := S _f):
        //   (add_nat (S _f) Z) = (S _f)
        //   IH at Hyp 0: (Goal [] [] ((add_nat _f Z) = _f))
        //   Unfold add_nat → opens body with args [Z, (S _f)] (reversed):
        //     Match scrutinee = (S _f) (BVar 1 → bindings[1]).
        //     Z arm body = Z (BVar 0 → bindings[0] = Z).
        //     S arm body: BVar 0 stays BVar 0 (pat-bound k),
        //                 BVar 1 → bindings[0] = Z (b's value).
        //     Result: (Match (S _f) [(Z → Z); ((S k) → (S (add_nat k Z)))]).
        //   Reduce ι → fires S arm with k captured = _f:
        //     → (S (add_nat _f Z))
        //     Then ι tries to step the Ctor S args. The inner Call
        //     to add_nat is NOT unfolded (ι doesn't do δ). All args
        //     of that Call are value-headed (FVar / Ctor). step_iota
        //     stops here.
        //   Rewrite Hyp 0 Lr Lhs all=True:
        //     pat = (Call add_nat [_f, Z]), repl = (FVar _f).
        //     Found inside (Ctor S [...]). Replace.
        //     Lhs becomes (S _f).
        //   Refl on (S _f) = (S _f).
        let s_case = steps(
            vec![
                unfold("add_nat", side_lhs()),
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

    // ------------------------------------------------------------------
    // Slice 16: polymorphic-type Induct.
    //
    // First runtime exercise of:
    //   - type_subst_list / type_subst / assoc_sym_type with a
    //     non-empty type_env (substituting T -> Int in Cons's field
    //     types when inducting on (List Int)).
    //   - build_ihs identifying ONLY the recursive field for IH
    //     generation (Cons has fields [T, List T]; only the second
    //     gets an IH, since type_eq (List T) T is false).
    //
    // These code paths exist in check.shard since slice 4 but have
    // never run with a non-Nil type_env.
    // ------------------------------------------------------------------

    /// `∀ xs : (List Int). (id_list xs) = xs` by induction.
    ///
    /// id_list is the structural identity on lists:
    ///   (fn id_list ((xs (List Int))) (List Int)
    ///     (match xs
    ///       (Nil Nil)
    ///       ((Cons h t) (Cons h (id_list t)))))
    ///
    /// Nil case:  Unfold + Reduce + Refl.
    /// Cons case: Unfold + Reduce + Rewrite (IH on the tail) + Refl.
    ///
    /// Polymorphic-typedef plumbing exercised:
    ///   - In do_induct, type_head (TCon "List" [Int]) yields
    ///     (Pair "List" [Int]); zip_pairs ["T"] [Int] builds
    ///     [(Pair "T" Int)] as type_env.
    ///   - For the Cons ctor in check_induct_cases, type_subst_list
    ///     on field_types [TVar "T", TCon "List" [TVar "T"]] under
    ///     that env produces [TCon "Int" [], TCon "List" [TCon "Int" []]]
    ///     — i.e. fully-instantiated Int and (List Int) field types.
    ///   - build_ihs walks the resulting field_params and emits ONE
    ///     IH (for _f2 : List Int), not two — _f1 : Int isn't the
    ///     inductive type.
    #[test]
    fn check_seq_induct_polymorphic_list_id() {
        let m = load_kernel();

        // (type (List T) (Nil) (Cons T (List T)))
        let list_td = type_def("List", vec!["T"], vec![
            ctor_def("Nil",  vec![]),
            ctor_def("Cons", vec![tvar("T"), tcon("List", vec![tvar("T")])]),
        ]);

        // (fn id_list ((xs (List Int))) (List Int)
        //   (match xs (Nil Nil) ((Cons h t) (Cons h (id_list t)))))
        //
        // Static body BVar conventions:
        //   outer: BVar 0 = xs (only param)
        //   Cons arm (pat_arity=2, source-order h then t):
        //     BVar 0 = t (innermost, last source-order PVar)
        //     BVar 1 = h
        //     BVar 2+ = outer-shifted (no outer refs in our body)
        let body = nmatch(
            bvar(0),                                   // scrutinee: xs
            vec![
                // Nil arm: returns Nil
                narm(pctor("Nil", vec![]),
                     ctor_app("Nil", vec![])),
                // Cons arm: returns (Cons h (id_list t))
                narm(pctor("Cons", vec![pvar(), pvar()]),
                     ctor_app("Cons", vec![
                        bvar(1),                       // h
                        call("id_list", vec![bvar(0)]),// (id_list t)
                     ])),
            ],
        );
        let list_int = tcon("List", vec![tcon("Int", vec![])]);
        let id_list_fn = fn_def("id_list",
            vec![list_int.clone()],
            list_int.clone(),
            body,
        );
        let mod_v = module(vec![list_td], vec![id_list_fn], vec![]);

        let seq = sequent(
            vec![param("xs", list_int)],
            vec![], vec![],
            equation(call("id_list", vec![fvar("xs")]), fvar("xs")),
        );

        // Nil case after subst (xs := Nil): (id_list Nil) = Nil
        //   Unfold + Reduce → Nil. Refl.
        let nil_case = steps(
            vec![
                unfold("id_list", side_lhs()),
                reduce(side_lhs()),
            ],
            refl(),
        );

        // Cons case after subst (xs := (Cons _f1 _f2)):
        //   (id_list (Cons _f1 _f2)) = (Cons _f1 _f2)
        //   IH at Hyp 0: (id_list _f2) = _f2.
        //   Unfold + Reduce ι → (Cons _f1 (id_list _f2)) (stops at
        //   the recursive call by ι semantics).
        //   Rewrite Hyp 0 Lr Lhs replaces (id_list _f2) → _f2 inside
        //   the Cons args. Lhs becomes (Cons _f1 _f2). Refl.
        let cons_case = steps(
            vec![
                unfold("id_list", side_lhs()),
                reduce(side_lhs()),
                rewrite(er_hyp(0), dir_lr(), side_lhs(), bool_true(), vec![]),
            ],
            refl(),
        );

        let pf = induct("xs", vec![
            case_arm("Nil",  nil_case),
            case_arm("Cons", cons_case),
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

    // ------------------------------------------------------------------
    // Slice 17: lemma citation via (Lemma name) EqRef.
    //
    // First runtime exercise of:
    //   - lookup_lemma (in check.shard) — walks a TheoryCons-shaped
    //     accumulator looking for a named Proven or Axiom entry.
    //   - resolve_eq's Lemma arm — returns the cited Goal.
    //   - Building a non-empty Theory value (theory_cons + proven /
    //     axiom) and threading it through check_sequent.
    //
    // Up to slice 16 every test passed `theory_empty()`. The Theory
    // type and its citation path have existed since slice 4 but have
    // never been driven runtime.
    //
    // V2 limitation (unchanged): cited Goal must be ground
    // (Goal Nil Nil eq) for Rewrite to use it — same gate as the
    // Premise / Hyp paths. ∀-binder capture in cited lemmas is the
    // pattern-variable Rewrite slice.
    // ------------------------------------------------------------------

    /// Headline: cite a Proven lemma to rewrite the goal lhs.
    ///   Theory: [Proven "double_5_is_10" (Goal Nil Nil ((double 5) = 10))]
    ///   Goal:   (double 5) = 10
    ///   Proof:  Rewrite (Lemma "double_5_is_10") Lr Lhs True []; Refl
    /// After rewrite, lhs becomes 10; goal becomes 10 = 10; Refl closes.
    #[test]
    fn check_seq_cites_proven_lemma() {
        let m = load_kernel();
        let mod_v = double_module();
        let lemma_goal = goal(
            vec![], vec![],
            equation(call("double", vec![intlit(5)]), intlit(10)),
        );
        let th = theory_cons(
            proven("double_5_is_10", lemma_goal),
            theory_empty(),
        );
        let seq = sequent(
            vec![], vec![], vec![],
            equation(call("double", vec![intlit(5)]), intlit(10)),
        );
        let pf = steps(
            vec![rewrite(er_lemma("double_5_is_10"),
                         dir_lr(), side_lhs(), bool_true(), vec![])],
            refl(),
        );
        let r = run_check_sequent(&m, mod_v, th, seq, pf);
        assert_eq!(r, true_v());
    }

    /// Lemma citation composes with Simp. Use the lemma to substitute
    /// a sub-expression, then Simp computes the result.
    ///   Theory: [Proven "double_5_is_10" ((double 5) = 10)]
    ///   Goal:   (+ (double 5) 1) = 11
    ///   Proof:  Rewrite Lemma Lr Lhs (replaces (double 5) -> 10);
    ///           Simp Lhs (computes (+ 10 1) -> 11); Refl.
    #[test]
    fn check_seq_lemma_then_simp() {
        let m = load_kernel();
        let mod_v = double_module();
        let lemma_goal = goal(
            vec![], vec![],
            equation(call("double", vec![intlit(5)]), intlit(10)),
        );
        let th = theory_cons(
            proven("double_5_is_10", lemma_goal),
            theory_empty(),
        );
        let seq = sequent(
            vec![], vec![], vec![],
            equation(
                call("+", vec![call("double", vec![intlit(5)]), intlit(1)]),
                intlit(11),
            ),
        );
        let pf = steps(
            vec![
                rewrite(er_lemma("double_5_is_10"),
                        dir_lr(), side_lhs(), bool_true(), vec![]),
                simp(side_lhs()),
            ],
            refl(),
        );
        let r = run_check_sequent(&m, mod_v, th, seq, pf);
        assert_eq!(r, true_v());
    }

    /// Negative: cite a lemma name that's not in the theory.
    /// lookup_lemma walks the entire theory, returns None at
    /// TheoryEmpty; resolve_eq returns None; Rewrite returns None;
    /// Steps fails; result False.
    #[test]
    fn check_seq_rejects_unknown_lemma() {
        let m = load_kernel();
        let mod_v = double_module();
        let lemma_goal = goal(
            vec![], vec![],
            equation(call("double", vec![intlit(5)]), intlit(10)),
        );
        let th = theory_cons(
            proven("a_different_name", lemma_goal),
            theory_empty(),
        );
        let seq = sequent(
            vec![], vec![], vec![],
            equation(call("double", vec![intlit(5)]), intlit(10)),
        );
        let pf = steps(
            vec![rewrite(er_lemma("double_5_is_10"),    // not in theory
                         dir_lr(), side_lhs(), bool_true(), vec![])],
            refl(),
        );
        let r = run_check_sequent(&m, mod_v, th, seq, pf);
        assert_eq!(r, false_v());
    }

    // ------------------------------------------------------------------
    // Slice 19: Induct with non-empty rest_params.
    //
    // First runtime exercise of build_ih's close path on a non-Nil
    // rest_params list: when the sequent has OTHER ∀-bound vars
    // besides the one being inducted on, the IH abstracts over them
    // via close_eq + reverse_syms. Until now every Induct test had
    // exactly one ∀-bound var (rest_params = []), so close_eq was
    // always called with an empty names list and the path was a no-op.
    //
    // Theorem: ∀ a b : Nat. (add_nat (S a) b) = (S (add_nat a b))
    //   Induct on `a`, keeping `b` as a ∀-var.
    //
    // Both subgoals close via Unfold add_nat Lhs + Reduce Lhs + Refl
    // because the statement is computationally direct: one unfold of
    // add_nat on the lhs fires the (S k) arm and produces the rhs's
    // shape exactly. The IH that build_ih constructs in the S case
    // — namely (Goal [Param b Nat] [] (closed eq)) where the closed
    // eq references BVar 0 for the abstracted b — is built but not
    // consumed. This isolates the test to the build mechanism.
    //
    // Note: the IH cannot be consumed by Rewrite at v2 anyway — its
    // Goal has a non-Nil params list, which the Rewrite arm rejects
    // (gates require Goal Nil Nil). Pattern-variable Rewrite would
    // unlock that path; it's not implemented.
    // ------------------------------------------------------------------

    /// ∀ a b : Nat. (add_nat (S a) b) = (S (add_nat a b)).
    /// Induct on a; b survives as an outer ∀-var.
    #[test]
    fn check_seq_induct_with_other_universal_present() {
        let m = load_kernel();
        let nat_td = type_def("Nat", vec![], vec![
            ctor_def("Z", vec![]),
            ctor_def("S", vec![tcon("Nat", vec![])]),
        ]);
        // (fn add_nat ((a Nat) (b Nat)) Nat
        //   (match a (Z b) ((S k) (S (add_nat k b)))))
        let add_nat_body = nmatch(
            bvar(1),                                          // a
            vec![
                narm(pctor("Z", vec![]), bvar(0)),            // -> b
                narm(pctor("S", vec![pvar()]),
                     ctor_app("S", vec![
                        call("add_nat", vec![bvar(0), bvar(1)]),   // (add_nat k b)
                     ])),
            ],
        );
        let add_nat_fn = fn_def("add_nat",
            vec![tcon("Nat", vec![]), tcon("Nat", vec![])],
            tcon("Nat", vec![]),
            add_nat_body,
        );
        let mod_v = module(vec![nat_td], vec![add_nat_fn], vec![]);

        let nat = tcon("Nat", vec![]);
        let seq = sequent(
            vec![param("a", nat.clone()), param("b", nat)],
            vec![], vec![],
            equation(
                call("add_nat", vec![
                    ctor_app("S", vec![fvar("a")]),
                    fvar("b"),
                ]),
                ctor_app("S", vec![
                    call("add_nat", vec![fvar("a"), fvar("b")]),
                ]),
            ),
        );

        // Z case after subst (a := Z):
        //   (add_nat (S Z) b) = (S (add_nat Z b))
        //   Unfold + Reduce on Lhs: fires the (S k) arm with k = Z,
        //   yielding (S (add_nat Z b)). RHS is unchanged. Refl.
        // S case after subst (a := S _f):
        //   (add_nat (S (S _f)) b) = (S (add_nat (S _f) b))
        //   IH at Hyp 0: ∀ b. (add_nat (S _f) b) = (S (add_nat _f b)).
        //   The IH is BUILT via close_eq on [b] — first runtime
        //   exercise of the close path with a non-empty names list.
        //   We do NOT consume the IH; same Unfold + Reduce + Refl
        //   closes (the (S (S _f)) gets one S peeled and add_nat
        //   unfolds correspondingly).
        let branch = steps(
            vec![
                unfold("add_nat", side_lhs()),
                reduce(side_lhs()),
            ],
            refl(),
        );
        let pf = induct("a", vec![
            case_arm("Z", branch.clone()),
            case_arm("S", branch),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    // ------------------------------------------------------------------
    // Slice 18: a STRETCH proof.
    //
    //   ∀ xs : List Int. (length (id_list xs)) = (length xs)
    //
    // Composes two recursive fns. The Cons case threads
    // Unfold-Reduce-Rewrite across BOTH sides of the equation, using
    // the IH in the middle. First proof where:
    //   - Unfold has to RECURSE into a Call's args to find the target
    //     (the outer Call is `length`, the target is `id_list`).
    //     unfold_one_in_list is exercised at runtime for the first
    //     time.
    //   - The proof script has 8+ steps in one branch, interleaving
    //     side dispatches.
    //   - Primitives (`+`) appear in the equation but stay structural
    //     under ι — both sides reduce to the SAME un-evaluated
    //     (+ 1 (length _t)) shape and Refl closes via expr_eq.
    //
    // Goal of this slice: see whether the existing kernel survives a
    // proof that wasn't designed in by the prior tests. Expected
    // outcome — uncertain. Either the streak holds, or we learn
    // something specific.
    // ------------------------------------------------------------------

    /// Module with (type (List T) ...), id_list, and length.
    /// Reused inside the slice 18 test.
    fn list_id_length_module() -> ast::Expr {
        let list_td = type_def("List", vec!["T"], vec![
            ctor_def("Nil",  vec![]),
            ctor_def("Cons", vec![tvar("T"), tcon("List", vec![tvar("T")])]),
        ]);
        let list_int = tcon("List", vec![tcon("Int", vec![])]);

        // (fn id_list ((xs (List Int))) (List Int)
        //   (match xs (Nil Nil) ((Cons h t) (Cons h (id_list t)))))
        let id_list_body = nmatch(
            bvar(0),
            vec![
                narm(pctor("Nil", vec![]), ctor_app("Nil", vec![])),
                narm(pctor("Cons", vec![pvar(), pvar()]),
                     ctor_app("Cons", vec![
                        bvar(1),                            // h
                        call("id_list", vec![bvar(0)]),    // (id_list t)
                     ])),
            ],
        );
        let id_list_fn = fn_def("id_list",
            vec![list_int.clone()],
            list_int.clone(),
            id_list_body,
        );

        // (fn length ((xs (List Int))) Int
        //   (match xs (Nil 0) ((Cons h t) (+ 1 (length t)))))
        //
        // Cons arm pat_arity=2 (h then t source-order):
        //   inside arm body: BVar 0 = t (innermost), BVar 1 = h.
        // The body (+ 1 (length t)) doesn't reference h.
        let length_body = nmatch(
            bvar(0),
            vec![
                narm(pctor("Nil", vec![]), intlit(0)),
                narm(pctor("Cons", vec![pvar(), pvar()]),
                     call("+", vec![
                        intlit(1),
                        call("length", vec![bvar(0)]),     // (length t)
                     ])),
            ],
        );
        let length_fn = fn_def("length",
            vec![list_int.clone()],
            tcon("Int", vec![]),
            length_body,
        );

        module(vec![list_td], vec![id_list_fn, length_fn], vec![])
    }

    /// Headline stretch test.
    #[test]
    fn check_seq_length_of_id_list_equals_length() {
        let m = load_kernel();
        let mod_v = list_id_length_module();
        let list_int = tcon("List", vec![tcon("Int", vec![])]);

        let seq = sequent(
            vec![param("xs", list_int)],
            vec![], vec![],
            equation(
                call("length", vec![call("id_list", vec![fvar("xs")])]),
                call("length", vec![fvar("xs")]),
            ),
        );

        // Nil case after subst (xs := Nil):
        //   (length (id_list Nil)) = (length Nil)
        // Simp Both drives both to 0. Refl.
        let nil_case = steps(vec![simp(side_both())], refl());

        // Cons case after subst (xs := (Cons _h _t)):
        //   (length (id_list (Cons _h _t))) = (length (Cons _h _t))
        // IH at Hyp 0: (length (id_list _t)) = (length _t)
        //
        // Steps (Lhs side):
        //   1. Unfold id_list Lhs:
        //      Outer Call is `length`, not id_list. unfold_one_in
        //      recurses into args, finds inner Call id_list,
        //      applies. Lhs becomes
        //      (length (Match (Cons _h _t) [Nil->Nil; Cons->...])).
        //   2. Reduce Lhs (ι): fires the inner Match's Cons arm.
        //      Lhs becomes (length (Cons _h (id_list _t))).
        //   3. Unfold length Lhs:
        //      Outer Call IS length. Applies. Lhs becomes
        //      (Match (Cons _h (id_list _t)) [Nil->0; Cons->(+ 1 (length t))]).
        //   4. Reduce Lhs (ι): fires Cons arm. Lhs becomes
        //      (+ 1 (length (id_list _t))).
        //   5. Rewrite (Hyp 0) Lr Lhs all=True: pat = (length (id_list _t)),
        //      repl = (length _t). Found inside the Call + args.
        //      Lhs becomes (+ 1 (length _t)).
        //
        // Steps (Rhs side):
        //   6. Unfold length Rhs: Rhs = (length (Cons _h _t)). Becomes
        //      (Match (Cons _h _t) [Nil->0; Cons->(+ 1 (length t))]).
        //   7. Reduce Rhs (ι): fires Cons arm. Rhs becomes
        //      (+ 1 (length _t)).
        //
        //   8. Refl: both sides are (+ 1 (length _t)).
        let cons_case = steps(
            vec![
                unfold("id_list", side_lhs()),
                reduce(side_lhs()),
                unfold("length",  side_lhs()),
                reduce(side_lhs()),
                rewrite(er_hyp(0), dir_lr(), side_lhs(), bool_true(), vec![]),
                unfold("length",  side_rhs()),
                reduce(side_rhs()),
            ],
            refl(),
        );

        let pf = induct("xs", vec![
            case_arm("Nil",  nil_case),
            case_arm("Cons", cons_case),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    // ------------------------------------------------------------------
    // Slice 20: pattern-variable Rewrite — ∀-binder capture.
    //
    // The cited Goal can now have non-empty params; its ∀-bound vars
    // are opened to fresh FVars before matching, the names are passed
    // to expr_match as pat_vars, and on a successful match the
    // captured env is substituted into the replacement.
    //
    // Unlocks:
    //   - Citing ∀-quantified lemmas (slice 17 had only ground lemmas).
    //   - Consuming the inductive hypothesis built by build_ih in
    //     slice 19 (the IH has Goal [Param ... ] eq, which the old
    //     ground Rewrite rejected).
    // ------------------------------------------------------------------

    /// Simpler pat-var test: cite ∀ x. (add_nat x Z) = x as a lemma
    /// (proven externally, so admitted), use it to rewrite
    /// (add_nat (S Z) Z) to (S Z), then Refl.
    /// Captures x := (S Z).
    #[test]
    fn check_seq_cites_universally_quantified_lemma() {
        let m = load_kernel();
        let mod_v = nat_module();
        // Lemma: ∀ x : Nat. (add_nat x Z) = x.
        // Goal value: (Goal [Param x Nat] [] (Eq (add_nat BVar0 Z) BVar0))
        let lemma_goal = goal(
            vec![param("x", tcon("Nat", vec![]))],
            vec![],
            equation(
                call("add_nat", vec![bvar(0), ctor_app("Z", vec![])]),
                bvar(0),
            ),
        );
        let th = theory_cons(
            proven("add_nat_right_id", lemma_goal),
            theory_empty(),
        );
        let sz = ctor_app("S", vec![ctor_app("Z", vec![])]);
        let seq = sequent(
            vec![], vec![], vec![],
            equation(
                call("add_nat", vec![sz.clone(), ctor_app("Z", vec![])]),
                sz,
            ),
        );
        let pf = steps(
            vec![rewrite(er_lemma("add_nat_right_id"),
                         dir_lr(), side_lhs(), bool_true(), vec![])],
            refl(),
        );
        let r = run_check_sequent(&m, mod_v, th, seq, pf);
        assert_eq!(r, true_v());
    }

    /// Headline: HD-consuming inductive proof of successor pull-out.
    ///
    ///   ∀ a b : Nat. (add_nat a (S b)) = (S (add_nat a b))
    ///
    /// Induct on a, leaving b as an outer ∀-var. The IH built in the
    /// S case is
    ///   Goal [Param b Nat] [] (Eq (add_nat _f (S BVar0)) (S (add_nat _f BVar0)))
    /// — a quantified statement that the old ground Rewrite rejected.
    /// With pat-var Rewrite, the IH can be CITED in the proof.
    ///
    /// Z case: (add_nat Z (S b)) = (S (add_nat Z b))
    ///   Unfold + Reduce both sides: lhs -> (S b), rhs -> (S b). Refl.
    ///
    /// S case: (add_nat (S _f) (S b)) = (S (add_nat (S _f) b))
    ///   Unfold + Reduce Lhs -> (S (add_nat _f (S b))).
    ///   Rewrite IH Lr Lhs all=True:
    ///     Pat var fresh1 opens b. Opened_lhs = (add_nat _f (S fresh1)).
    ///     Match against (add_nat _f (S b)) -> fresh1 captures (FVar b).
    ///     Substitute into opened_rhs (S (add_nat _f fresh1)) ->
    ///       (S (add_nat _f b)).
    ///     Replace (add_nat _f (S b)) with that in the lhs.
    ///   Lhs -> (S (S (add_nat _f b))).
    ///   Unfold + Reduce Rhs -> (S (S (add_nat _f b))). Refl.
    #[test]
    fn check_seq_induct_successor_pull_out() {
        let m = load_kernel();
        let nat_td = type_def("Nat", vec![], vec![
            ctor_def("Z", vec![]),
            ctor_def("S", vec![tcon("Nat", vec![])]),
        ]);
        let add_nat_body = nmatch(
            bvar(1),                                          // a
            vec![
                narm(pctor("Z", vec![]), bvar(0)),            // -> b
                narm(pctor("S", vec![pvar()]),
                     ctor_app("S", vec![
                        call("add_nat", vec![bvar(0), bvar(1)]),
                     ])),
            ],
        );
        let add_nat_fn = fn_def("add_nat",
            vec![tcon("Nat", vec![]), tcon("Nat", vec![])],
            tcon("Nat", vec![]),
            add_nat_body,
        );
        let mod_v = module(vec![nat_td], vec![add_nat_fn], vec![]);

        let nat = tcon("Nat", vec![]);
        let seq = sequent(
            vec![param("a", nat.clone()), param("b", nat)],
            vec![], vec![],
            equation(
                // (add_nat a (S b))
                call("add_nat", vec![
                    fvar("a"),
                    ctor_app("S", vec![fvar("b")]),
                ]),
                // (S (add_nat a b))
                ctor_app("S", vec![
                    call("add_nat", vec![fvar("a"), fvar("b")]),
                ]),
            ),
        );

        let z_case = steps(
            vec![
                unfold("add_nat", side_lhs()),
                reduce(side_lhs()),
                unfold("add_nat", side_rhs()),
                reduce(side_rhs()),
            ],
            refl(),
        );
        let s_case = steps(
            vec![
                unfold("add_nat", side_lhs()),
                reduce(side_lhs()),
                rewrite(er_hyp(0), dir_lr(), side_lhs(), bool_true(), vec![]),
                unfold("add_nat", side_rhs()),
                reduce(side_rhs()),
            ],
            refl(),
        );
        let pf = induct("a", vec![
            case_arm("Z", z_case),
            case_arm("S", s_case),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    // ------------------------------------------------------------------
    // Slice 22: ByTheory + LIA decision procedure.
    //
    // First decidable-theory backend. The kernel registers one
    // theory name "lia"; the ByTheory arm dispatches there and
    // returns the result of `lia_decide` on the goal eq's sides.
    //
    // Cert payload: presence-only for v2 (LIA is poly-time; no
    // asymmetry to exploit). The slot remains for theories where
    // checking is cheaper than searching.
    //
    // LIA handles `+ - *` (the * only by integer constants),
    // IntLit, and arbitrary opaque atoms (FVar, BVar, non-arithmetic
    // Calls). Decision: lhs - rhs canonicalizes to all-zero coeffs.
    // ------------------------------------------------------------------

    /// Helper: the empty Cert for LIA.
    fn lia_cert() -> ast::Expr {
        cert("lia", symlit(""))
    }
    fn lia_pf() -> ast::Expr {
        by_theory("lia", lia_cert())
    }

    /// Commutativity over `+`: ∀ x y : Int. (+ x y) = (+ y x).
    /// Both sides normalize to [(1, x), (1, y)]; diff = 0.
    #[test]
    fn check_seq_lia_plus_commutativity() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("x", int.clone()), param("y", int)],
            vec![], vec![],
            equation(
                call("+", vec![fvar("x"), fvar("y")]),
                call("+", vec![fvar("y"), fvar("x")]),
            ),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, lia_pf());
        assert_eq!(r, true_v());
    }

    /// Constant arithmetic: (+ 1 (+ 2 3)) = 6.
    /// lhs collapses to [(6, None)], rhs to [(6, None)]; diff = 0.
    #[test]
    fn check_seq_lia_constants() {
        let m = load_kernel();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(
                call("+", vec![intlit(1),
                    call("+", vec![intlit(2), intlit(3)])]),
                intlit(6),
            ),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, lia_pf());
        assert_eq!(r, true_v());
    }

    /// Distributivity + commutativity over `*`:
    /// (* (+ x y) 2) = (+ (* 2 x) (* y 2))
    /// Both sides normalize to [(2, x), (2, y)].
    #[test]
    fn check_seq_lia_distributivity() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("x", int.clone()), param("y", int)],
            vec![], vec![],
            equation(
                call("*", vec![
                    call("+", vec![fvar("x"), fvar("y")]),
                    intlit(2),
                ]),
                call("+", vec![
                    call("*", vec![intlit(2), fvar("x")]),
                    call("*", vec![fvar("y"), intlit(2)]),
                ]),
            ),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, lia_pf());
        assert_eq!(r, true_v());
    }

    /// Subtraction: (- (+ x y) x) = y.
    /// lhs: [(1, x), (1, y), (-1, x)] -> canonical [(1, y)].
    /// rhs: [(1, y)]. Diff = 0.
    #[test]
    fn check_seq_lia_subtraction() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("x", int.clone()), param("y", int)],
            vec![], vec![],
            equation(
                call("-", vec![
                    call("+", vec![fvar("x"), fvar("y")]),
                    fvar("x"),
                ]),
                fvar("y"),
            ),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, lia_pf());
        assert_eq!(r, true_v());
    }

    /// Negative: math is wrong. (+ x y) = (+ x 1) only if y = 1, not
    /// in general. lia_decide finds diff = [(1, y), (-1, None)] ≠ 0.
    #[test]
    fn check_seq_lia_rejects_non_equal() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("x", int.clone()), param("y", int)],
            vec![], vec![],
            equation(
                call("+", vec![fvar("x"), fvar("y")]),
                call("+", vec![fvar("x"), intlit(1)]),
            ),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, lia_pf());
        assert_eq!(r, false_v());
    }

    /// Negative: unknown theory. ByTheory "magic" doesn't match any
    /// registered theory, so the ByTheory arm returns False.
    #[test]
    fn check_seq_by_theory_rejects_unknown_theory() {
        let m = load_kernel();
        let seq = sequent(
            vec![], vec![], vec![],
            equation(intlit(1), intlit(1)),    // even though this IS true
        );
        let pf = by_theory("magic", cert("magic", symlit("")));
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, pf);
        assert_eq!(r, false_v());
    }

    // ------------------------------------------------------------------
    // Slice 33: eqdec ByTheory backend (kernel/eqdec.shard).
    //
    // Second decidable-theory backend. Decides equality-reflection
    // goals `(int_eq a b) = True` (via lia_decide) and
    // `(sym_eq a b) = True` (via expr_eq). The motivating use is
    // reflexivity on a variable — `int_eq k k = True` — which the
    // reducer leaves stuck (int_eq only fires on closed IntLits).
    //
    // These Rust mirrors guard the trusted backend directly, including
    // the rejection paths the end-to-end sexp claims don't exercise:
    // distinct keys must NOT be provably equal, and only the `= True`
    // direction is decided (disequalities arrive as hypotheses).
    // ------------------------------------------------------------------

    fn eqdec_pf() -> ast::Expr {
        by_theory("eqdec", cert("eqdec", symlit("")))
    }

    /// Positive: ∀ k : Int. (int_eq k k) = True. lia_decide k k = 0.
    /// This is the `int_eq_refl` lemma's kernel core.
    #[test]
    fn check_seq_eqdec_int_eq_refl() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("k", int)],
            vec![], vec![],
            equation(call("int_eq", vec![fvar("k"), fvar("k")]), ctor_app("True", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, eqdec_pf());
        assert_eq!(r, true_v());
    }

    /// Positive: ∀ a. (sym_eq a a) = True, decided by expr_eq.
    #[test]
    fn check_seq_eqdec_sym_eq_refl() {
        let m = load_kernel();
        let sym = tcon("Symbol", vec![]);
        let seq = sequent(
            vec![param("a", sym)],
            vec![], vec![],
            equation(call("sym_eq", vec![fvar("a"), fvar("a")]), ctor_app("True", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, eqdec_pf());
        assert_eq!(r, true_v());
    }

    /// Negative (soundness): distinct keys are NOT provably equal.
    /// (int_eq j k) = True must be REJECTED — lia_decide j k = [(1,j),(-1,k)] ≠ 0.
    #[test]
    fn check_seq_eqdec_rejects_distinct_keys() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("j", int.clone()), param("k", int)],
            vec![], vec![],
            equation(call("int_eq", vec![fvar("j"), fvar("k")]), ctor_app("True", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, eqdec_pf());
        assert_eq!(r, false_v());
    }

    /// Negative (scope): eqdec only decides the `= True` direction.
    /// `(int_eq k k) = False` is FALSE and must be rejected — even
    /// though `int_eq k k` IS reflexively true, the asserted RHS is
    /// False. (Disequalities are consumed as hypotheses, never proven.)
    #[test]
    fn check_seq_eqdec_rejects_false_direction() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("k", int)],
            vec![], vec![],
            equation(call("int_eq", vec![fvar("k"), fvar("k")]), ctor_app("False", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, eqdec_pf());
        assert_eq!(r, false_v());
    }

    // ------------------------------------------------------------------
    // Slice 35: ord ByTheory backend (kernel/ord.shard).
    //
    // Decides order-reflection goals (lt a b) = True / (le a b) = True
    // by canonicalizing (b - a) via LIA and checking it's a constant of
    // the right sign. Mirrors guard the soundness boundary: strictness
    // (lt a a is rejected) and direction (le (a+1) a rejected), and the
    // "not a tautology" path (lt a b for independent vars rejected).
    // ------------------------------------------------------------------

    fn ord_pf() -> ast::Expr {
        by_theory("ord", cert("ord", symlit("")))
    }

    /// Positive: ∀ a. (lt a (a+1)) = True. diff = 1 ≥ 1.
    #[test]
    fn check_seq_ord_lt_succ() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int)],
            vec![], vec![],
            equation(
                call("lt", vec![fvar("a"), call("+", vec![fvar("a"), intlit(1)])]),
                ctor_app("True", vec![]),
            ),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, ord_pf());
        assert_eq!(r, true_v());
    }

    /// Positive: ∀ a. (le a a) = True. diff = 0 ≥ 0.
    #[test]
    fn check_seq_ord_le_refl() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int)],
            vec![], vec![],
            equation(call("le", vec![fvar("a"), fvar("a")]), ctor_app("True", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, ord_pf());
        assert_eq!(r, true_v());
    }

    /// Negative (not a tautology): (lt a b) for independent a, b. diff =
    /// b - a has variables → rejected.
    #[test]
    fn check_seq_ord_rejects_lt_distinct() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int.clone()), param("b", int)],
            vec![], vec![],
            equation(call("lt", vec![fvar("a"), fvar("b")]), ctor_app("True", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, ord_pf());
        assert_eq!(r, false_v());
    }

    /// Negative (strictness): (lt a a) = True must be REJECTED. diff = 0,
    /// which is ≥ 0 but NOT ≥ 1. Guards the strict/non-strict boundary.
    #[test]
    fn check_seq_ord_rejects_lt_irreflexive() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int)],
            vec![], vec![],
            equation(call("lt", vec![fvar("a"), fvar("a")]), ctor_app("True", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, ord_pf());
        assert_eq!(r, false_v());
    }

    /// Negative (direction): (le (a+1) a) = True must be REJECTED.
    /// diff = a - (a+1) = -1 < 0.
    #[test]
    fn check_seq_ord_rejects_le_backwards() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int)],
            vec![], vec![],
            equation(
                call("le", vec![call("+", vec![fvar("a"), intlit(1)]), fvar("a")]),
                ctor_app("True", vec![]),
            ),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, ord_pf());
        assert_eq!(r, false_v());
    }

    // ------------------------------------------------------------------
    // Slice 37: farkas ByTheory backend (kernel/farkas.shard).
    //
    // Linear-integer ENTAILMENT: premises ⊢ (lt|le a b) = True, via a
    // cert-supplied Farkas combination. The mirrors guard the two
    // soundness-critical properties:
    //   GUARD 1 — an inequality premise may only take a NONNEGATIVE
    //     multiplier (equalities take any sign). check_seq_farkas_
    //     rejects_neg_mult_on_ineq vs _allows_neg_mult_on_eq is the
    //     crux pair: the SAME negative multiplier is rejected on an
    //     inequality but accepted on an equality.
    //   GUARD 2 — a wrong witness (variables don't cancel) is rejected.
    // ------------------------------------------------------------------

    /// Build a farkas proof citing multipliers [G, M0, M1, ...]. The
    /// multipliers are RAW narrow Ints (Expr::IntLit), as a bare `1` in
    /// a cert `(list 1 1)` parses to — NOT the Expr-ADT `IntLit` ctor
    /// that nval's `intlit` builds.
    fn farkas_pf(mults: Vec<i64>) -> ast::Expr {
        let ms: Vec<ast::Expr> = mults.into_iter().map(ast::Expr::IntLit).collect();
        by_theory("farkas", cert("farkas", list(ms)))
    }

    /// Positive: ∀ p i. (lt p i)=True ⊢ (lt p (i+1))=True. cert [1,1].
    /// THE M3 loop-invariant obligation.
    #[test]
    fn check_seq_farkas_lt_succ_from_lt() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("p", int.clone()), param("i", int)],
            vec![],
            vec![equation(call("lt", vec![fvar("p"), fvar("i")]), ctor_app("True", vec![]))],
            equation(
                call("lt", vec![fvar("p"), call("+", vec![fvar("i"), intlit(1)])]),
                ctor_app("True", vec![]),
            ),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, farkas_pf(vec![1, 1]));
        assert_eq!(r, true_v());
    }

    /// Positive: transitivity. (le a b),(le b c) ⊢ (le a c). cert [1,1,1].
    #[test]
    fn check_seq_farkas_le_trans() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int.clone()), param("b", int.clone()), param("c", int)],
            vec![],
            vec![
                equation(call("le", vec![fvar("a"), fvar("b")]), ctor_app("True", vec![])),
                equation(call("le", vec![fvar("b"), fvar("c")]), ctor_app("True", vec![])),
            ],
            equation(call("le", vec![fvar("a"), fvar("c")]), ctor_app("True", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, farkas_pf(vec![1, 1, 1]));
        assert_eq!(r, true_v());
    }

    /// Negative (GUARD 1): (le a b) ⊬ (le b a). The only multipliers
    /// that cancel the variables put -1 on the INEQUALITY premise, which
    /// is rejected. cert [1,-1] must be REJECTED — without the
    /// nonneg guard this would falsely "prove" le is symmetric.
    #[test]
    fn check_seq_farkas_rejects_neg_mult_on_ineq() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int.clone()), param("b", int)],
            vec![],
            vec![equation(call("le", vec![fvar("a"), fvar("b")]), ctor_app("True", vec![]))],
            equation(call("le", vec![fvar("b"), fvar("a")]), ctor_app("True", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, farkas_pf(vec![1, -1]));
        assert_eq!(r, false_v());
    }

    /// Positive (complement to GUARD 1): a negative multiplier on an
    /// EQUALITY premise is allowed. (int_eq a b)=True ⊢ (le a b)=True
    /// with cert [1,-1] — the SAME negative multiplier the previous test
    /// rejects, here accepted because the premise is an equality.
    #[test]
    fn check_seq_farkas_allows_neg_mult_on_eq() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int.clone()), param("b", int)],
            vec![],
            vec![equation(call("int_eq", vec![fvar("a"), fvar("b")]), ctor_app("True", vec![]))],
            equation(call("le", vec![fvar("a"), fvar("b")]), ctor_app("True", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, farkas_pf(vec![1, -1]));
        assert_eq!(r, true_v());
    }

    /// Negative (GUARD 2): a wrong witness is rejected. The goal IS
    /// entailed (lt p i ⊢ lt p (i+1)) but cert [1,0] leaves the premise
    /// unused, so variables don't cancel → rejected.
    #[test]
    fn check_seq_farkas_rejects_wrong_witness() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("p", int.clone()), param("i", int)],
            vec![],
            vec![equation(call("lt", vec![fvar("p"), fvar("i")]), ctor_app("True", vec![]))],
            equation(
                call("lt", vec![fvar("p"), call("+", vec![fvar("i"), intlit(1)])]),
                ctor_app("True", vec![]),
            ),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, farkas_pf(vec![1, 0]));
        assert_eq!(r, false_v());
    }

    /// Positive (slice 38 — DISEQUALITY conclusion): (lt a b)=True ⊢
    /// (int_eq a b)=False. The goal negates to the equality a=b, so the
    /// goal multiplier may be any sign; here [1,1] suffices. This is the
    /// M3 enabler turning a strict bound into a ≠ fact for read_swap.
    #[test]
    fn check_seq_farkas_lt_implies_neq() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int.clone()), param("b", int)],
            vec![],
            vec![equation(call("lt", vec![fvar("a"), fvar("b")]), ctor_app("True", vec![]))],
            equation(call("int_eq", vec![fvar("a"), fvar("b")]), ctor_app("False", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, farkas_pf(vec![1, 1]));
        assert_eq!(r, true_v());
    }

    /// Negative (slice 38): a disequality is NOT entailed by an equality.
    /// (int_eq a b)=True ⊬ (int_eq a b)=False — assuming a=b for the
    /// negation just restates the premise; no contradiction. Rejected.
    #[test]
    fn check_seq_farkas_rejects_neq_from_eq() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int.clone()), param("b", int)],
            vec![],
            vec![equation(call("int_eq", vec![fvar("a"), fvar("b")]), ctor_app("True", vec![]))],
            equation(call("int_eq", vec![fvar("a"), fvar("b")]), ctor_app("False", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, farkas_pf(vec![1, 1]));
        assert_eq!(r, false_v());
    }

    /// Build a two-sided farkas proof for an equality goal:
    /// payload = (list le_mults ge_mults), each a raw-Int list.
    fn farkas_pf_eq(le: Vec<i64>, ge: Vec<i64>) -> ast::Expr {
        let mk = |v: Vec<i64>| list(v.into_iter().map(ast::Expr::IntLit).collect());
        by_theory("farkas", cert("farkas", list(vec![mk(le), mk(ge)])))
    }

    /// Positive (slice 41 — EQUALITY conclusion): antisymmetry.
    /// (le a b)=True, (le b a)=True ⊢ (int_eq a b)=True, two-sided.
    #[test]
    fn check_seq_farkas_eq_from_le_both() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int.clone()), param("b", int)],
            vec![],
            vec![
                equation(call("le", vec![fvar("a"), fvar("b")]), ctor_app("True", vec![])),
                equation(call("le", vec![fvar("b"), fvar("a")]), ctor_app("True", vec![])),
            ],
            equation(call("int_eq", vec![fvar("a"), fvar("b")]), ctor_app("True", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, farkas_pf_eq(vec![1, 1, 0], vec![1, 0, 1]));
        assert_eq!(r, true_v());
    }

    /// Negative (slice 41, soundness): a single bound cannot prove
    /// equality. (le a b)=True ⊬ (int_eq a b)=True — the b<=a direction
    /// is not entailed, so its refutation can't cancel the variables.
    #[test]
    fn check_seq_farkas_rejects_eq_one_sided() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int.clone()), param("b", int)],
            vec![],
            vec![equation(call("le", vec![fvar("a"), fvar("b")]), ctor_app("True", vec![]))],
            equation(call("int_eq", vec![fvar("a"), fvar("b")]), ctor_app("True", vec![])),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, farkas_pf_eq(vec![1, 1], vec![1, 1]));
        assert_eq!(r, false_v());
    }

    /// Positive (slice 42 — PLAIN term-equality goal): ⊢ (a+b) = (b+a).
    /// A raw Equation goal (not an int_eq reflection) routes to the
    /// two-sided equality path; here it's a tautology (no premises), so
    /// each direction refutes with goal multiplier 1 alone.
    #[test]
    fn check_seq_farkas_plain_eq_commute() {
        let m = load_kernel();
        let int = tcon("Int", vec![]);
        let seq = sequent(
            vec![param("a", int.clone()), param("b", int)],
            vec![], vec![],
            equation(
                call("+", vec![fvar("a"), fvar("b")]),
                call("+", vec![fvar("b"), fvar("a")]),
            ),
        );
        let r = run_check_sequent(&m,
            module(vec![], vec![], vec![]),
            theory_empty(), seq, farkas_pf_eq(vec![1], vec![1]));
        assert_eq!(r, true_v());
    }

    // ------------------------------------------------------------------
    // Slice 21: commutativity of add_nat by lemma composition.
    //
    //   ∀ a b : Nat. (add_nat a b) = (add_nat b a)
    //
    // Cites two pre-proven ∀-quantified lemmas:
    //   add_nat_right_id        : ∀ x. (add_nat x Z) = x
    //   add_nat_succ_pull_out   : ∀ a b. (add_nat a (S b)) = (S (add_nat a b))
    // Both were proven in slices 19/20 (we admit them as Proven here).
    //
    // Stresses pat-var Rewrite three different ways in a single proof:
    //   - Z case Rewrite right_id captures one var (x := b).
    //   - S case Rewrite IH captures one var (b := _f).
    //   - S case Rewrite succ_pull_out captures TWO vars (a, b).
    //
    // No new kernel code; first proof that genuinely composes lemmas
    // across the inductive structure.
    // ------------------------------------------------------------------

    /// Headline.
    #[test]
    fn check_seq_commutativity_via_lemma_composition() {
        let m = load_kernel();
        let nat = tcon("Nat", vec![]);
        let nat_td = type_def("Nat", vec![], vec![
            ctor_def("Z", vec![]),
            ctor_def("S", vec![nat.clone()]),
        ]);
        let add_nat_body = nmatch(
            bvar(1),
            vec![
                narm(pctor("Z", vec![]), bvar(0)),
                narm(pctor("S", vec![pvar()]),
                     ctor_app("S", vec![
                        call("add_nat", vec![bvar(0), bvar(1)]),
                     ])),
            ],
        );
        let add_nat_fn = fn_def("add_nat",
            vec![nat.clone(), nat.clone()],
            nat.clone(),
            add_nat_body,
        );
        let mod_v = module(vec![nat_td], vec![add_nat_fn], vec![]);

        // Lemmas as ground Goal VALUES — Proven, admitted by the
        // theory accumulator.
        //
        // right_id: Goal [Param x Nat] []
        //                ((add_nat BVar0 Z) = BVar0)
        let right_id = goal(
            vec![param("x", nat.clone())],
            vec![],
            equation(
                call("add_nat", vec![bvar(0), ctor_app("Z", vec![])]),
                bvar(0),
            ),
        );

        // succ_pull_out: Goal [Param a Nat; Param b Nat] []
        //                     ((add_nat BVar1 (S BVar0))
        //                          = (S (add_nat BVar1 BVar0)))
        //
        // BVar indices innermost-first: BVar 0 = b (last param),
        // BVar 1 = a.
        let succ_pull_out = goal(
            vec![param("a", nat.clone()), param("b", nat.clone())],
            vec![],
            equation(
                call("add_nat", vec![
                    bvar(1),                                   // a
                    ctor_app("S", vec![bvar(0)]),              // (S b)
                ]),
                ctor_app("S", vec![
                    call("add_nat", vec![bvar(1), bvar(0)]),   // (add_nat a b)
                ]),
            ),
        );

        let th = theory_cons(
            proven("add_nat_right_id", right_id),
            theory_cons(
                proven("add_nat_succ_pull_out", succ_pull_out),
                theory_empty(),
            ),
        );

        // Goal: ∀ a b : Nat. (add_nat a b) = (add_nat b a)
        let seq = sequent(
            vec![param("a", nat.clone()), param("b", nat)],
            vec![], vec![],
            equation(
                call("add_nat", vec![fvar("a"), fvar("b")]),
                call("add_nat", vec![fvar("b"), fvar("a")]),
            ),
        );

        // Z case: (add_nat Z b) = (add_nat b Z)
        //   1. Unfold add_nat Lhs (opens body with [b, Z]).
        //   2. Reduce Lhs (fires Z arm) -> (FVar b).
        //   3. Rewrite right_id Lr Rhs: matches (add_nat b Z),
        //      captures x := b, substitutes to b. Rhs -> (FVar b).
        //   4. Refl on (FVar b) = (FVar b).
        let z_case = steps(
            vec![
                unfold("add_nat", side_lhs()),
                reduce(side_lhs()),
                rewrite(er_lemma("add_nat_right_id"),
                        dir_lr(), side_rhs(), bool_true(), vec![]),
            ],
            refl(),
        );

        // S case: (add_nat (S _f) b) = (add_nat b (S _f))
        //   IH at Hyp 0: (add_nat _f BVar0) = (add_nat BVar0 _f).
        //
        //   1. Unfold add_nat Lhs.
        //   2. Reduce Lhs -> (S (add_nat _f b)).
        //   3. Rewrite IH (Hyp 0) Lr Lhs: opens BVar 0 to fresh
        //      pat_var, matches (add_nat _f b) inside (S ...),
        //      captures pat_var := b, substitutes IH's rhs
        //      (add_nat BVar0 _f) -> (add_nat b _f).
        //      Lhs -> (S (add_nat b _f)).
        //   4. Rewrite succ_pull_out Lr Rhs: matches the whole rhs
        //      (add_nat b (S _f)) against (add_nat <a> (S <b>)),
        //      captures a := b, b := _f, substitutes to
        //      (S (add_nat b _f)). Rhs -> (S (add_nat b _f)).
        //   5. Refl.
        let s_case = steps(
            vec![
                unfold("add_nat", side_lhs()),
                reduce(side_lhs()),
                rewrite(er_hyp(0), dir_lr(), side_lhs(), bool_true(), vec![]),
                rewrite(er_lemma("add_nat_succ_pull_out"),
                        dir_lr(), side_rhs(), bool_true(), vec![]),
            ],
            refl(),
        );

        let pf = induct("a", vec![
            case_arm("Z", z_case),
            case_arm("S", s_case),
        ]);
        let r = run_check_sequent(&m, mod_v, th, seq, pf);
        assert_eq!(r, true_v());
    }

    // ------------------------------------------------------------------
    // Slice 27: RewriteWith — conditional lemma citation. The cited
    // equation carries premises; the rewriter matches its conclusion
    // against the goal side, instantiates each premise with the match
    // binding env, and dispatches sub-proofs to discharge them. After
    // all premises are checked, the rewrite is applied and the rest
    // of the proof continues on the new sequent.
    // ------------------------------------------------------------------

    /// Cite a conditional axiom:
    ///   ∀ a : Nat. (a = Z) ⇒ ((add_nat a Z) = Z)
    /// to close (add_nat Z Z) = Z. Match binds a := Z; instantiated
    /// premise (Z = Z) is discharged with Refl; rewritten goal Z = Z
    /// closed with Refl.
    ///
    /// Exercises:
    ///   - apply_rewrite_with_env returning (Pair env new_eq).
    ///   - check_premise_proofs walking the cited premises in lockstep
    ///     with the supplied sub-proofs.
    ///   - resolve_eq + open_eq_with on a multi-component Goal.
    #[test]
    fn check_seq_rewrite_with_conditional_axiom() {
        let m = load_kernel();
        let mod_v = nat_module();
        // Conditional axiom: ∀ a : Nat. (a = Z) ⇒ ((add_nat a Z) = Z).
        // BVar 0 stands in for `a` throughout (innermost-first).
        let cond_goal = goal(
            vec![param("a", tcon("Nat", vec![]))],
            vec![equation(bvar(0), ctor_app("Z", vec![]))],
            equation(
                call("add_nat", vec![bvar(0), ctor_app("Z", vec![])]),
                ctor_app("Z", vec![]),
            ),
        );
        let th = theory_cons(
            axiom("triv_lemma", cond_goal),
            theory_empty(),
        );
        let seq = sequent(
            vec![], vec![], vec![],
            equation(
                call("add_nat",
                     vec![ctor_app("Z", vec![]), ctor_app("Z", vec![])]),
                ctor_app("Z", vec![]),
            ),
        );
        let pf = rewrite_with(
            er_lemma("triv_lemma"), dir_lr(), side_lhs(),
            vec![],         // no insts
            vec![refl()],   // one sub-proof for the one premise
            refl(),         // continuation after rewrite
        );
        let r = run_check_sequent(&m, mod_v, th, seq, pf);
        assert_eq!(r, true_v());
    }

    /// Arity-mismatch rejection: if the cited equation has N premises
    /// but the proof supplies M ≠ N sub-proofs, check_premise_proofs
    /// returns False.
    #[test]
    fn check_seq_rewrite_with_arity_mismatch_rejects() {
        let m = load_kernel();
        let mod_v = nat_module();
        let cond_goal = goal(
            vec![param("a", tcon("Nat", vec![]))],
            vec![equation(bvar(0), ctor_app("Z", vec![]))],
            equation(
                call("add_nat", vec![bvar(0), ctor_app("Z", vec![])]),
                ctor_app("Z", vec![]),
            ),
        );
        let th = theory_cons(
            axiom("triv_lemma", cond_goal),
            theory_empty(),
        );
        let seq = sequent(
            vec![], vec![], vec![],
            equation(
                call("add_nat",
                     vec![ctor_app("Z", vec![]), ctor_app("Z", vec![])]),
                ctor_app("Z", vec![]),
            ),
        );
        // Zero premise proofs supplied, but the lemma has one premise.
        let pf = rewrite_with(
            er_lemma("triv_lemma"), dir_lr(), side_lhs(),
            vec![], vec![], refl(),
        );
        let r = run_check_sequent(&m, mod_v, th, seq, pf);
        assert_eq!(r, false_v());
    }

    /// Non-Nil Insts is rejected in v2 (mirrors the unconditional
    /// Rewrite Step's restriction).
    #[test]
    /// RewriteWith with an Inst that pre-instantiates the cited ∀-var
    /// to a concrete value. Goal `(add_nat (S Z) Z) = (S Z)`; cite
    /// the polymorphic axiom `∀ a. (add_nat a Z) = a` with the Inst
    /// pinning a := (S Z). After Inst, the conclusion is the literal
    /// goal LHS, so the Rewrite Lr Lhs fires trivially.
    ///
    /// This is the canonical Insts pattern: skip the pattern-match
    /// inference and pin the ∀-binder directly.
    #[test]
    fn check_seq_rewrite_with_insts_pin_pivot() {
        let m = load_kernel();
        let mod_v = nat_module();
        let cond_goal = goal(
            vec![param("a", tcon("Nat", vec![]))],
            vec![],
            equation(
                call("add_nat", vec![bvar(0), ctor_app("Z", vec![])]),
                bvar(0),
            ),
        );
        let th = theory_cons(
            axiom("uncond_lemma", cond_goal),
            theory_empty(),
        );
        let sz = ctor_app("S", vec![ctor_app("Z", vec![])]);
        let seq = sequent(
            vec![], vec![], vec![],
            equation(
                call("add_nat", vec![sz.clone(), ctor_app("Z", vec![])]),
                sz.clone(),
            ),
        );
        let pf = rewrite_with(
            er_lemma("uncond_lemma"), dir_lr(), side_lhs(),
            vec![inst("a", sz)],
            vec![],  // no premises on the cited axiom
            refl(),
        );
        let r = run_check_sequent(&m, mod_v, th, seq, pf);
        assert_eq!(r, true_v());
    }

    /// Slice 29 isolation test — Rust mirror of examples/list_lemmas.shard's
    /// append_nil_right. Built with nval helpers, runs the SAME proof
    /// the binary submits. Lets us tell whether a binary FAIL is a
    /// sexp-loading bug or a real proof problem.
    #[test]
    fn check_seq_append_nil_right_rust_mirror() {
        let m = load_kernel();
        // (type (List T) (Nil) (Cons T (List T)))
        let list_td = type_def("List", vec!["T"], vec![
            ctor_def("Nil",  vec![]),
            ctor_def("Cons", vec![tvar("T"), tcon("List", vec![tvar("T")])]),
        ]);
        // (fn append ((xs (List Int)) (ys (List Int))) (List Int)
        //   (match xs (Nil ys) ((Cons h t) (Cons h (append t ys)))))
        let list_int = tcon("List", vec![tcon("Int", vec![])]);
        let body = nmatch(
            bvar(1),                                   // scrutinee: xs
            vec![
                narm(pctor("Nil", vec![]), bvar(0)),   // -> ys
                narm(pctor("Cons", vec![pvar(), pvar()]),
                     ctor_app("Cons", vec![
                        bvar(1),                       // h
                        call("append", vec![bvar(0), bvar(2)]),
                     ])),
            ],
        );
        let append_fn = fn_def("append",
            vec![list_int.clone(), list_int.clone()],
            list_int.clone(), body);
        let mod_v = module(vec![list_td], vec![append_fn], vec![]);

        let seq = sequent(
            vec![param("xs", list_int)],
            vec![], vec![],
            equation(
                call("append", vec![fvar("xs"), ctor_app("Nil", vec![])]),
                fvar("xs"),
            ),
        );
        let pf = induct("xs", vec![
            case_arm("Nil",
                steps(vec![
                    unfold("append", side_lhs()),
                    reduce(side_lhs()),
                ], refl())),
            case_arm("Cons",
                steps(vec![
                    unfold("append", side_lhs()),
                    reduce(side_lhs()),
                    rewrite(er_hyp(0), dir_lr(), side_lhs(), bool_true(), vec![]),
                ], refl())),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Axiom citation works the same as Proven. Both lookup_lemma
    /// arms match on a sym_eq of the name and return the carried
    /// Goal — the tag is just an audit marker (see BOUNDARIES.md).
    #[test]
    fn check_seq_cites_axiom() {
        let m = load_kernel();
        let mod_v = double_module();
        let ax_goal = goal(
            vec![], vec![],
            equation(call("double", vec![intlit(5)]), intlit(10)),
        );
        let th = theory_cons(
            axiom("double_5_is_10", ax_goal),
            theory_empty(),
        );
        let seq = sequent(
            vec![], vec![], vec![],
            equation(call("double", vec![intlit(5)]), intlit(10)),
        );
        let pf = steps(
            vec![rewrite(er_lemma("double_5_is_10"),
                         dir_lr(), side_lhs(), bool_true(), vec![])],
            refl(),
        );
        let r = run_check_sequent(&m, mod_v, th, seq, pf);
        assert_eq!(r, true_v());
    }

    /// Slice 30 regression: the reverse tower's append_assoc, built
    /// in Rust against the new smart simp_expr. Mirror of the sexp
    /// claim in examples/list_lemmas.shard; safety net for the kernel
    /// reducer changes (gated δ + head-only lookahead).
    ///
    /// Both Nil and Cons cases close with `(Simp Both) [...]; Refl` —
    /// no per-ctor helper lemmas needed. Before slice 30 this proof
    /// shape required Unfold + Reduce + per-arm Rewrite chains.
    #[test]
    fn check_seq_append_assoc_smart_simp() {
        let m = load_kernel();
        let list_td = type_def("List", vec!["T"], vec![
            ctor_def("Nil",  vec![]),
            ctor_def("Cons", vec![tvar("T"), tcon("List", vec![tvar("T")])]),
        ]);
        let list_int = tcon("List", vec![tcon("Int", vec![])]);
        let append_body = nmatch(
            bvar(1),
            vec![
                narm(pctor("Nil", vec![]), bvar(0)),
                narm(pctor("Cons", vec![pvar(), pvar()]),
                     ctor_app("Cons", vec![
                        bvar(1),
                        call("append", vec![bvar(0), bvar(2)]),
                     ])),
            ],
        );
        let append_fn = fn_def("append",
            vec![list_int.clone(), list_int.clone()],
            list_int.clone(), append_body);
        let mod_v = module(vec![list_td], vec![append_fn], vec![]);

        let seq = sequent(
            vec![
                param("xs", list_int.clone()),
                param("ys", list_int.clone()),
                param("zs", list_int.clone()),
            ],
            vec![], vec![],
            equation(
                call("append", vec![
                    call("append", vec![fvar("xs"), fvar("ys")]),
                    fvar("zs"),
                ]),
                call("append", vec![
                    fvar("xs"),
                    call("append", vec![fvar("ys"), fvar("zs")]),
                ]),
            ),
        );
        let pf = induct("xs", vec![
            case_arm("Nil",
                steps(vec![simp(side_both())], refl())),
            case_arm("Cons",
                steps(vec![
                    simp(side_both()),
                    rewrite(er_hyp(0), dir_lr(), side_lhs(), bool_true(), vec![]),
                ], refl())),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }

    /// Slice 31 regression: `append_nil_right` stated over POLYMORPHIC
    /// (List T) — Goal param is `(TCon List [TVar T])` rather than
    /// `(TCon List [TCon Int []])`. Hand-built with nval helpers to
    /// guard the kernel side of polymorphism — the loader's job
    /// (parsing `(fn (NAME T) …)` and `(tv T)`) is exercised by the
    /// sexp examples.
    #[test]
    fn check_seq_polymorphic_append_nil_right_rust_mirror() {
        let m = load_kernel();
        let list_td = type_def("List", vec!["T"], vec![
            ctor_def("Nil",  vec![]),
            ctor_def("Cons", vec![tvar("T"), tcon("List", vec![tvar("T")])]),
        ]);
        // append: still defined at (List Int) for the fn body (the
        // evaluator doesn't care about types; the goal's TVar will
        // get matched against the fn's Int via pattern matching).
        // Actually for fairness, let's make the fn also polymorphic
        // by using TVar in its sig — the evaluator ignores types.
        let list_t = tcon("List", vec![tvar("T")]);
        let body = nmatch(
            bvar(1),
            vec![
                narm(pctor("Nil", vec![]), bvar(0)),
                narm(pctor("Cons", vec![pvar(), pvar()]),
                     ctor_app("Cons", vec![
                        bvar(1),
                        call("append", vec![bvar(0), bvar(2)]),
                     ])),
            ],
        );
        let append_fn = fn_def("append",
            vec![list_t.clone(), list_t.clone()],
            list_t.clone(), body);
        let mod_v = module(vec![list_td], vec![append_fn], vec![]);

        // Goal: ∀ xs : (List T). (append xs Nil) = xs
        // where T is a TVar (declared somewhere — the kernel doesn't
        // care if it's lexically tied to anything).
        let seq = sequent(
            vec![param("xs", list_t)],
            vec![], vec![],
            equation(
                call("append", vec![fvar("xs"), ctor_app("Nil", vec![])]),
                fvar("xs"),
            ),
        );
        let pf = induct("xs", vec![
            case_arm("Nil",
                steps(vec![simp(side_lhs())], refl())),
            case_arm("Cons",
                steps(vec![
                    simp(side_lhs()),
                    rewrite(er_hyp(0), dir_lr(), side_lhs(), bool_true(), vec![]),
                ], refl())),
        ]);
        let r = run_check_sequent(&m, mod_v, theory_empty(), seq, pf);
        assert_eq!(r, true_v());
    }
}
