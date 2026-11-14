//! The bootstrap evaluator: it reads the toolchain's sources (v3/kernel/)
//! and runs them.
//!
//! The crate is in the trust base (docs/LAW.md "The trust base"). Its only
//! contract is to load and evaluate programs faithfully — no proof work
//! happens here. The checker and everything above it are programs this
//! evaluator runs.

pub mod ast;
pub mod dump;
pub mod eval;
pub mod load;
pub mod prim;

// The process allocator. The evaluator's values are reference-counted
// constructor blocks and BigInts, allocated and freed at the dispatch rate;
// mimalloc's thread-local free lists make that materially cheaper than
// glibc's.
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[cfg(test)]
mod tests {
    use super::*;

    fn eval_str(src: &str, call: &str) -> ast::Expr {
        let module = load::module_from_str(src).expect("module loads");
        let expr = load::expr_from_str(call, &module).expect("call parses");
        eval::eval(&module, &expr).expect("eval succeeds")
    }

    fn fn_named<'a>(m: &'a ast::Module, name: &str) -> &'a ast::FnDef {
        m.fns.iter().find(|f| f.name == name).expect("the fn is declared")
    }

    fn true_v() -> ast::Expr {
        ast::Expr::Ctor("True".into(), Vec::new())
    }
    fn false_v() -> ast::Expr {
        ast::Expr::Ctor("False".into(), Vec::new())
    }

    /// A constructor value: `nctor("Foo", vec![…])` is `Ctor("Foo", […])`.
    fn nctor(name: &str, args: Vec<ast::Expr>) -> ast::Expr {
        ast::Expr::Ctor(name.into(), args)
    }

    /// The `(List Int)` value `(Cons x0 (Cons x1 … Nil))`.
    fn int_list(xs: &[i64]) -> ast::Expr {
        let mut acc = nctor("Nil", Vec::new());
        for &x in xs.iter().rev() {
            acc = nctor("Cons", vec![ast::Expr::IntLit(x.into()), acc]);
        }
        acc
    }

    // ------------------------------------------------------------------
    // String literals: "abc" is the (List Int) of its UTF-8 bytes.
    // ------------------------------------------------------------------

    #[test]
    fn string_literal_is_byte_list() {
        let m = "(fn id ((x Int)) Int x)";
        // 'x'=120, '+'=43, 'y'=121.
        assert_eq!(eval_str(m, "\"x+y\""), int_list(&[120, 43, 121]));
        // Empty string is Nil.
        assert_eq!(eval_str(m, "\"\""), int_list(&[]));
        // Digits are their ASCII codes, not their numeric value.
        assert_eq!(eval_str(m, "\"0\""), int_list(&[48]));
        // Bytes, not code points: λ (U+03BB) is two.
        assert_eq!(eval_str(m, "\"λ\""), int_list(&[0xCE, 0xBB]));
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
    // A user fn and a primitive.
    // ------------------------------------------------------------------

    #[test]
    fn call_add_two_three() {
        assert_eq!(
            eval_str("(fn add ((a Int) (b Int)) Int (+ a b))", "(add 2 3)"),
            ast::Expr::IntLit(5.into())
        );
    }

    /// Catches arg-order bugs `+` (symmetric) would mask.
    #[test]
    fn call_first_arg() {
        assert_eq!(
            eval_str("(fn first ((a Int) (b Int)) Int a)", "(first 7 9)"),
            ast::Expr::IntLit(7.into())
        );
    }

    // ------------------------------------------------------------------
    // Primitives: one example each.
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
    /// wrap. A machine integer wraps silently in a release build, and the
    /// checker's arithmetic runs through these primitives.
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
        // i64::MAX² — the product of two large multipliers.
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
            eval_str(m, "(< 9223372036854775807 (+ 9223372036854775807 1))"),
            true_v()
        );
    }

    /// The paired division variants: `/`+`tmod` (truncating) and
    /// `ediv`+`mod` (Euclidean) must each satisfy a = b*q + r across all
    /// sign combinations, with their respective remainder ranges. The
    /// mixed pair `/`+`mod` does NOT.
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
        // The mixed pair on a negative: 10*(-7/10) + (-7 mod 10) = 3, not -7.
        assert_eq!(eval_str(m, "(+ (* 10 (/ -7 10)) (mod -7 10))"), int(3));
    }

    /// Source literals beyond the host parser's integer range must be a
    /// LOAD ERROR, not silently degraded — lexpr falls back to lossy f64
    /// for them. (The V3 reader builds integers by arithmetic and has no
    /// such ceiling.)
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

    /// The three comparisons, in a call and as an `if` condition (the
    /// fused-compare path).
    #[test]
    fn prim_comparisons() {
        let m = "(fn pick ((a Int) (b Int)) Int (if (< a b) 1 (if (= a b) 2 (if (<= b a) 3 4))))";
        assert_eq!(eval_str(m, "(= 3 3)"),  true_v());
        assert_eq!(eval_str(m, "(= 3 4)"),  false_v());
        assert_eq!(eval_str(m, "(< 1 2)"),  true_v());
        assert_eq!(eval_str(m, "(< 2 2)"),  false_v());
        assert_eq!(eval_str(m, "(<= 5 5)"), true_v());
        assert_eq!(eval_str(m, "(<= 6 5)"), false_v());
        assert_eq!(eval_str(m, "(pick 1 2)"), ast::Expr::IntLit(1.into()));
        assert_eq!(eval_str(m, "(pick 2 2)"), ast::Expr::IntLit(2.into()));
        assert_eq!(eval_str(m, "(pick 3 2)"), ast::Expr::IntLit(3.into()));
    }

    #[test]
    fn prim_in_user_fn() {
        assert_eq!(
            eval_str("(fn square ((x Int)) Int (* x x))", "(square 9)"),
            ast::Expr::IntLit(81.into())
        );
    }

    // ------------------------------------------------------------------
    // Control flow — if, match, let — and data types.
    // ------------------------------------------------------------------

    /// `if` dispatches on True/False ctors returned by comparison.
    #[test]
    fn if_branches() {
        let src = r#"
(type Bool (False) (True))
(fn abs ((x Int)) Int (if (< x 0) (- 0 x) x))
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

    /// Match + recursion: `len` over a List — type defs, ctor recognition,
    /// nested-list values, pattern bindings, BVar resolution under match
    /// arms, and a recursive Call.
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

    /// Sequential `let`: each right-hand side sees the bindings before it,
    /// the body sees them all. Catches mis-counting of binder depth in a
    /// right-hand side and in the body.
    #[test]
    fn let_sequential() {
        let src = r#"
(fn add_squares ((a Int) (b Int)) Int
  (let ((aa (* a a))
        (bb (+ aa (* b b))))
    (+ aa bb)))
"#;
        let m = load::module_from_str(src).expect("loads");
        let e = load::expr_from_str("(add_squares 3 4)", &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), ast::Expr::IntLit(34.into()));
        let f = fn_named(&m, "add_squares");
        assert_eq!(
            dump::dump(&m),
            "fn add_squares 0 (Int Int) Int (let ((* #1 #1) (+ #0 (* #1 #1))) (+ #1 #0))\n"
        );
        assert_eq!(f.params.len(), 2);
    }

    /// A dotted citation resolves to the declared name it ends in (the
    /// flat mirror of the V3 reader's suffix table): `Stack.mk` in a term
    /// and a pattern, `lib.push` as a call head, `lib.Stack` as a type
    /// printed by its last component.
    #[test]
    fn dotted_citations() {
        let src = r#"
(type Stack (mk Int))
(fn push ((n Int) (s lib.Stack)) Stack (match s ((Stack.mk k) (Stack.mk (+ k n)))))
(fn top ((s Stack)) Int (match s ((lib.Stack.mk k) k)))
(fn go () Int (top (lib.push 5 (Stack.mk 1))))
"#;
        let m = load::module_from_str(src).expect("loads");
        let e = load::expr_from_str("(go)", &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), ast::Expr::IntLit(6.into()));
        assert_eq!(
            dump::dump(&m),
            "fn go 0 () Int (top (push 5 (mk 1)))\nfn push 0 (Int Stack) Stack (match #0 ((mk _) (mk (+ #0 #2))))\nfn top 0 (Stack) Int (match #0 ((mk _) #0))\ntype Stack 0 (mk Int)\n"
        );
    }

    /// `(T Type)` binders: a type parameter, in scope for the binders after
    /// it and the result, never a runtime parameter; the dump numbers it by
    /// first occurrence, as the V3 dump does.
    #[test]
    fn type_binders() {
        let src = r#"
(type (List T) (Nil) (Cons T (List T)))
(fn first ((T Type) (xs (List T)) (d T)) T
  (match xs (Nil d) ((Cons x _) x)))
"#;
        let m = load::module_from_str(src).expect("loads");
        let f = fn_named(&m, "first");
        assert_eq!(f.params.len(), 2);
        assert_eq!(f.params[1], ast::Type::TVar("T".into()));
        assert_eq!(f.ret, ast::Type::TVar("T".into()));
        let e = load::expr_from_str("(first (Cons 7 Nil) 0)", &m).expect("parses");
        assert_eq!(eval::eval(&m, &e).unwrap(), ast::Expr::IntLit(7.into()));
        assert_eq!(
            dump::dump(&m),
            "fn first 1 ((List ?0) ?0) ?0 (match #1 (Nil #0) ((Cons _ _) #1))\ntype List 1 (Nil) (Cons ?0 (List ?0))\n"
        );
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
    // The primitive table (prim.rs::try_apply) and the evaluator's fast arms
    // over one value matrix. The toolchain's side of the primitives is
    // v3/kernel/prog.shard's table under `ev`, compared with this evaluator
    // by v3/kernel/test/prims_test.shard.
    // ------------------------------------------------------------------

    /// The two-Int primitives of the table.
    const SHARED_INT2: &[&str] = &[
        "+", "-", "*", "/", "mod", "tmod", "ediv",
        "band", "bor", "bxor", "bshl", "bshr",
        "=", "<", "<=",
    ];

    /// The Int value matrix: zero, units, small values, sign mixes,
    /// shift-range edges (63/64/65, negative), i64 boundaries, and
    /// integers past i64.
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

    /// A (List X) value from element values.
    fn expr_spine(items: Vec<ast::Expr>) -> ast::Expr {
        let mut acc = nctor("Nil", Vec::new());
        for it in items.into_iter().rev() {
            acc = nctor("Cons", vec![it, acc]);
        }
        acc
    }

    /// The evaluator's FAST ARMS (eval.rs `apply_other`: the primitives
    /// tagged at lowering and applied directly on `Val`s) agree with the
    /// native table on the full pair matrix — values AND domains: where the
    /// table leaves a call stuck (b=0 division, a shift amount outside
    /// 0..64) the evaluator must report UnknownCall, never a value or a
    /// crash. Untagged names take the general path, so the sweep also pins
    /// that path's conversion for every shared primitive.
    #[test]
    fn prim_fast_path_matches_table() {
        let m = ast::Module::default();
        let vals = int_matrix();
        for name in SHARED_INT2 {
            for a in &vals {
                for b in &vals {
                    let args = [
                        ast::Expr::IntLit(a.clone()),
                        ast::Expr::IntLit(b.clone()),
                    ];
                    fast_path_point(&m, name, &args);
                }
            }
        }
        // The symbol bridge: names round-trip, and the out-of-domain byte
        // lists (a non-byte, a bare continuation byte, an encoded surrogate,
        // a truncated lead, a negative) stay stuck through the evaluator too.
        let syms = ["x", "y", "wadd", "_fresh0", "λ→"];
        for a in syms {
            for b in syms {
                fast_path_point(&m, "sym_eq", &[ast::Expr::SymLit(a.into()), ast::Expr::SymLit(b.into())]);
            }
            fast_path_point(&m, "chars_of_sym", &[ast::Expr::SymLit(a.into())]);
            let bs: Vec<ast::Expr> = a.bytes().map(|b| ast::Expr::IntLit(b.into())).collect();
            fast_path_point(&m, "sym_of_chars", &[expr_spine(bs)]);
        }
        fast_path_point(&m, "sym_of_chars", &[expr_spine(vec![])]);
        for bad in [vec![955], vec![0xFF], vec![0xED, 0xA0, 0x80], vec![0xC2], vec![-1]] {
            let es: Vec<ast::Expr> =
                bad.into_iter().map(|b: i64| ast::Expr::IntLit(b.into())).collect();
            fast_path_point(&m, "sym_of_chars", &[expr_spine(es)]);
        }
    }

    /// One (name, args) point through `eval::eval` against the native table:
    /// equal values, or stuck (UnknownCall) exactly where the table is None.
    fn fast_path_point(m: &ast::Module, name: &str, args: &[ast::Expr]) {
        let native = prim::try_apply(name, args);
        let call = ast::Expr::Call(name.into(), args.to_vec());
        match (eval::eval(m, &call), native) {
            (Ok(got), Some(want)) => assert_eq!(got, want, "fast path drift at ({name} {args:?})"),
            (Err(eval::EvalError::UnknownCall(n)), None) => assert_eq!(n, name),
            (got, want) => panic!(
                "fast path vs table at ({name} {args:?}): evaluator {got:?}, table {want:?}"
            ),
        }
    }

}
