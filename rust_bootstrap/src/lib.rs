//! The bootstrap evaluator: it reads a shard program's sources — the
//! toolchain's own (v3/kernel/) first of all — and runs them.
//!
//! The crate is in the trust base (docs/LAW.md "The trust base"). Its only
//! contract is to load and evaluate programs faithfully — no proof work
//! happens here. The checker and everything above it are programs this
//! evaluator runs.
//!
//! `plain` reads a closure as the V3 loader's plain route reads it: files as
//! modules under a root, names as identities, `use`, views linked to their
//! implementations; from the source and a table of facts about the constants
//! of Init it cites. `eval` runs what it read; `prim` is the
//! primitives' table. `v3/kernel/test/parity_test.sh` (the toolchain's
//! closures) and `plain_test.sh` (ordinary ones) compare the reader with the
//! loader.

pub mod ast;
pub mod eval;
pub mod plain;
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

    /// What a toolchain file opens of Init: the constructors of `Bool`, `List` and `Option`,
    /// by name (`Prod.mk` is written in full).
    const OPEN: &str = "(use Init.Bool false true)\n(use Init.List cons nil)\n(use Init.Option none some)\n";

    /// The table the toolchain's sources are read by, the tree's own.
    const FACTS: &str = include_str!("../../v3/kernel/init.facts");

    /// A closure on disk under a scratch root of its own: its files (the last
    /// is the root file), the facts.
    fn closure(files: &[(&str, &str)], facts: &str) -> (std::path::PathBuf, plain::Loader) {
        use std::sync::atomic::{AtomicUsize, Ordering};
        static N: AtomicUsize = AtomicUsize::new(0);
        let root = std::env::temp_dir().join(format!("shard_boot_{}_{}", std::process::id(), N.fetch_add(1, Ordering::Relaxed)));
        let _ = std::fs::remove_dir_all(&root);
        for (rel, src) in files {
            let p = root.join(rel);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(&p, src).unwrap();
        }
        let fp = root.join("facts");
        std::fs::write(&fp, facts).unwrap();
        let entry = root.join(files.last().unwrap().0);
        let l = plain::load(&root, Some(&fp), &[entry]).expect("the closure loads");
        (root, l)
    }

    /// A toolchain file `kernel/t.shard` holding `src` under those opens, read whole by the
    /// tree's table.
    fn toolchain(src: &str) -> plain::Loader {
        let file = format!("{OPEN}{src}\n");
        let (root, l) = closure(&[("kernel/t.shard", &file)], FACTS);
        let _ = std::fs::remove_dir_all(root);
        assert!(l.t.unread.is_empty(), "{:?}", l.t.unread);
        l
    }

    /// The value of `call` in a toolchain file holding `src`.
    fn eval_str(src: &str, call: &str) -> ast::Expr {
        let l = toolchain(&format!("{src}\n(fn probe () Int {call})"));
        let m = plain::out::module(&l.t);
        eval::eval(&m, &ast::Expr::Call("kernel.t.probe".into(), vec![])).expect("eval succeeds")
    }

    fn has_line(l: &plain::Loader, line: &str) {
        let dump = plain::out::dump(&l.t);
        assert!(dump.lines().any(|x| x == line), "no line `{line}` in\n{dump}");
    }

    fn int(n: i64) -> ast::Expr {
        ast::Expr::IntLit(n.into())
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

    /// A (List X) value of the runtime's cells from element values.
    fn expr_spine(items: Vec<ast::Expr>) -> ast::Expr {
        let mut acc = nctor("Nil", Vec::new());
        for it in items.into_iter().rev() {
            acc = nctor("Cons", vec![it, acc]);
        }
        acc
    }

    /// The `(List Int)` value `(Cons x0 (Cons x1 … Nil))`.
    fn int_list(xs: &[i64]) -> ast::Expr {
        expr_spine(xs.iter().map(|&x| int(x)).collect())
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
        let m = "(fn len ((xs (List Int))) Int \
                   (match xs (nil 0) ((cons _ r) (+ 1 (len r)))))";
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

    /// The table's paired division entries: `/`+`tmod` (truncating) and
    /// `ediv`+`mod` (Euclidean) each satisfy a = b*q + r across all sign
    /// combinations, with their respective remainder ranges. A source's `/`
    /// and `mod` at Int are Init's `Int.ediv` and `Int.emod`, the Euclidean pair.
    #[test]
    fn prim_division_pairs() {
        let m = "(fn id ((x Int)) Int x)";
        let int = |n: i64| ast::Expr::IntLit(n.into());
        for (a, b) in [(7i64, 10i64), (-7, 10), (7, -10), (-7, -10), (23, 4), (-23, 4)] {
            // truncating pair: q rounds toward zero, r follows the dividend.
            assert_eq!(prim2("/", a, b), Some(int(a / b)), "(/ {a} {b})");
            assert_eq!(prim2("tmod", a, b), Some(int(a % b)), "(tmod {a} {b})");
            // Euclidean pair: 0 <= r < |b|.
            let (eq, er) = (prim2("ediv", a, b).unwrap(), prim2("mod", a, b).unwrap());
            assert_eq!(er, int(a.rem_euclid(b)), "(mod {a} {b})");
            let (ast::Expr::IntLit(eqv), ast::Expr::IntLit(erv)) = (&eq, &er) else {
                panic!("non-int division result");
            };
            assert_eq!(ast::IntLit::from(b) * eqv + erv, a.into(), "ediv/mod identity at ({a}, {b})");
            // the source's pair is the Euclidean one
            assert_eq!(eval_str(m, &format!("(/ {a} {b})")), eq, "source (/ {a} {b})");
            assert_eq!(eval_str(m, &format!("(mod {a} {b})")), er, "source (mod {a} {b})");
            assert_eq!(eval_str(m, &format!("(tmod {a} {b})")), int(a % b), "source (tmod {a} {b})");
        }
    }

    /// A source literal beyond the host parser's integer range is read
    /// exactly, never as the lossy f64 lexpr falls back to (the V3 reader
    /// builds integers by arithmetic and has no such ceiling).
    #[test]
    fn oversized_literal_is_exact() {
        use std::str::FromStr;
        let big = "99999999999999999999999999999999";
        let want = ast::Expr::IntLit(ast::IntLit::from_str(big).expect("int"));
        assert_eq!(eval_str("(fn id ((x Int)) Int x)", big), want);
        assert_eq!(eval_str("(fn id ((x Int)) Int x)", &format!("(- 0 -{big})")), want);
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

    /// `if` dispatches on the boolean a comparison returns — fused with the
    /// comparison, and through a value (`neg` returns the cell, `abs2` reads it).
    #[test]
    fn if_branches() {
        let src = r#"
(fn abs ((x Int)) Int (if (< x 0) (- 0 x) x))
(fn neg ((x Int)) Bool (< x 0))
(fn abs2 ((x Int)) Int (if (neg x) (- 0 x) x))
"#;
        for (input, expected) in [("(abs -5)", 5), ("(abs 7)", 7), ("(abs 0)", 0), ("(abs2 -5)", 5), ("(abs2 7)", 7)] {
            assert_eq!(eval_str(src, input), int(expected));
        }
    }

    /// Constructors as values, no match — Ctor is just data.
    #[test]
    fn ctor_as_value() {
        let src = "(fn make_pair ((x Int) (y Int)) (Prod Int Int) (Prod.mk x y))";
        assert_eq!(eval_str(src, "(make_pair 3 7)"), nctor("Pair", vec![int(3), int(7)]));
    }

    /// Match + recursion: `len` over a List — type defs, ctor recognition,
    /// nested-list values, pattern bindings, BVar resolution under match
    /// arms, and a recursive Call.
    #[test]
    fn match_list_length() {
        let src = r#"
(fn len ((xs (List Int))) Int
  (match xs
    (nil 0)
    ((cons _ rest) (+ 1 (len rest)))))
"#;
        assert_eq!(eval_str(src, "(len (cons 10 (cons 20 (cons 30 nil))))"), int(3));
    }

    /// Nested pattern: `(Cons a (Cons b _))` binds two PVars at
    /// distinct depths, picks up arity from a sub-pattern, and
    /// the body uses both bindings.
    #[test]
    fn match_nested_pattern() {
        let src = r#"
(fn first_two_sum ((xs (List Int))) Int
  (match xs
    ((cons a (cons b _)) (+ a b))
    (_ 0)))
"#;
        assert_eq!(eval_str(src, "(first_two_sum (cons 10 (cons 20 (cons 30 nil))))"), int(30));
        assert_eq!(eval_str(src, "(first_two_sum (cons 5 nil))"), int(0));
        assert_eq!(eval_str(src, "(first_two_sum nil)"), int(0));
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
        assert_eq!(eval_str(src, "(add_squares 3 4)"), int(34));
        has_line(&toolchain(src), "fn add_squares 0 (Int Int) Int (let ((* #1 #1)) (let ((+ #0 (* #1 #1))) (+ #1 #0)))");
    }

    /// A dotted citation names an identity, in full or from the file's own
    /// module down: `Stack.mk` in a term and a pattern, `lib.push` as a call
    /// head, `lib.Stack` as a type — each printed by its last component.
    #[test]
    fn dotted_citations() {
        let src = r#"
(type Stack (mk Int))
(fn push ((n Int) (s lib.Stack)) Stack (match s ((Stack.mk k) (Stack.mk (+ k n)))))
(fn top ((s Stack)) Int (match s ((lib.Stack.mk k) k)))
(fn go () Int (top (lib.push 5 (Stack.mk 1))))
"#;
        let (root, l) = closure(&[("lib.shard", src)], "");
        let _ = std::fs::remove_dir_all(root);
        assert!(l.t.unread.is_empty(), "{:?}", l.t.unread);
        let m = plain::out::module(&l.t);
        assert_eq!(eval::eval(&m, &ast::Expr::Call("lib.go".into(), vec![])).unwrap(), int(6));
        assert_eq!(
            plain::out::dump(&l.t),
            "fn go 0 () Int (top (push 5 (Stack.mk 1)))\nfn push 0 (Int Stack) Stack (match #0 ((Stack.mk _) (Stack.mk (+ #0 #2))))\nfn top 0 (Stack) Int (match #0 ((Stack.mk _) #0))\ntype Stack 0 (Stack.mk Int)\n"
        );
    }

    /// `(T Type)` binders: a type parameter, in scope for the binders after
    /// it and the result, never a runtime parameter; the dump numbers it by
    /// first occurrence, as the V3 dump does.
    #[test]
    fn type_binders() {
        let src = r#"
(fn first ((T Type) (xs (List T)) (d T)) T
  (match xs (nil d) ((cons x _) x)))
"#;
        let l = toolchain(src);
        let m = plain::out::module(&l.t);
        let f = m.fns.iter().find(|f| f.name == "kernel.t.first").expect("the fn is declared");
        assert_eq!(f.params.len(), 2);
        assert_eq!(f.params[1], f.ret);
        assert!(matches!(f.ret, ast::Type::TVar(_)));
        assert_eq!(eval_str(src, "(first (cons 7 nil) 0)"), int(7));
        has_line(&l, "fn first 1 ((List ?0) ?0) ?0 (match #1 (List.nil #0) ((List.cons _ _) #1))");
        has_line(&l, "type List 1 (List.nil) (List.cons ?0 (List ?0))");
    }

    /// `match` on an `Option`-shaped value — bare zero-arg ctors AND
    /// ctors with bindings in the same match.
    #[test]
    fn match_option() {
        let src = r#"
(fn unwrap_or ((o (Option Int)) (default Int)) Int
  (match o
    (none default)
    ((some x) x)))
"#;
        assert_eq!(eval_str(src, "(unwrap_or (some 42) 0)"), int(42));
        assert_eq!(eval_str(src, "(unwrap_or none 99)"), int(99));
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

    // ------------------------------------------------------------------
    // Init's entries of the table (prog.shard 18–38, as ev.shard applies
    // them), the `if` rule, the reader's names, views and types.
    // ------------------------------------------------------------------

    fn prim2(name: &str, a: i64, b: i64) -> Option<ast::Expr> {
        prim::try_apply(name, &[ast::Expr::IntLit(a.into()), ast::Expr::IntLit(b.into())])
    }

    /// Int's divisions are total (a zero divisor gives 0, a remainder by
    /// zero the dividend); Nat's set is total where Lean's is, defined on
    /// naturals only, and its decisions are Init's cells.
    #[test]
    fn prim_init_entries() {
        let int = |n: i64| Some(ast::Expr::IntLit(n.into()));
        assert_eq!(prim2("Int.tdiv", -7, 2), int(-3));
        assert_eq!(prim2("Int.tdiv", 7, 0), int(0));
        assert_eq!(prim2("Int.tmod", -7, 2), int(-1));
        assert_eq!(prim2("Int.tmod", -7, 0), int(-7));
        assert_eq!(prim2("Int.ediv", -7, 2), int(-4));
        assert_eq!(prim2("Int.ediv", -7, 0), int(0));
        assert_eq!(prim2("Int.emod", -7, 2), int(1));
        assert_eq!(prim2("Int.emod", -7, 0), int(-7));
        assert_eq!(prim2("Nat.sub", 3, 5), int(0));
        assert_eq!(prim2("Nat.sub", 5, 3), int(2));
        assert_eq!(prim2("Nat.div", 7, 0), int(0));
        assert_eq!(prim2("Nat.mod", 7, 0), int(7));
        assert_eq!(prim2("Nat.mod", 7, 4), int(3));
        assert_eq!(prim2("Nat.gcd", 12, 18), int(6));
        assert_eq!(prim2("Nat.gcd", 0, 5), int(5));
        assert_eq!(prim2("Nat.pow", 2, 10), int(1024));
        assert_eq!(prim2("Nat.pow", 7, 0), int(1));
        assert_eq!(prim2("Nat.shiftLeft", 3, 4), int(48));
        assert_eq!(prim2("Nat.shiftRight", 48, 4), int(3));
        assert_eq!(prim2("Nat.shiftRight", 48, 200), int(0));
        assert_eq!(prim2("Nat.land", 12, 10), int(8));
        assert_eq!(prim2("Nat.lor", 12, 10), int(14));
        assert_eq!(prim2("Nat.xor", 12, 10), int(6));
        assert_eq!(prim2("Nat.beq", 4, 4), Some(nctor("Bool.true", vec![])));
        assert_eq!(prim2("Nat.ble", 5, 4), Some(nctor("Bool.false", vec![])));
        assert_eq!(prim2("Nat.decLt", 4, 5), Some(nctor("Decidable.isTrue", vec![])));
        assert_eq!(prim2("Nat.decEq", 4, 5), Some(nctor("Decidable.isFalse", vec![])));
        // outside the naturals, and past K's count limit: stuck, never a value
        assert_eq!(prim2("Nat.sub", -1, 1), None);
        assert_eq!(prim2("Nat.add", 1, -1), None);
        assert_eq!(prim2("Nat.pow", 2, 5_000_000_000), None);
        assert_eq!(prim2("Nat.shiftLeft", 1, 5_000_000_000), None);
        assert_eq!(prim2("Nat.shiftLeft", 0, 5_000_000_000), int(0));
    }

    /// `if` takes the then-branch iff the cell is the second constructor of
    /// a two-constructor type (prog.shard's EIf): the prelude's Bool, and
    /// any other such type the closure declares.
    #[test]
    fn if_reads_the_second_constructor() {
        let src = r#"
(type Dec (No) (Yes))
(fn pick ((d Dec)) Int (if d 1 2))
"#;
        assert_eq!(eval_str(src, "(pick Yes)"), int(1));
        assert_eq!(eval_str(src, "(pick No)"), int(2));
    }

    /// Init's `Bool`, `List`, `Option` and `Prod` are the runtime's cells, by identity:
    /// a comparison's result is a `Bool`, a symbol's bytes a `List`, read by the tree's own
    /// table of Init's facts.
    #[test]
    fn cells_are_inits_constructors() {
        let src = r#"
(use Init.Bool false true)
(use Init.List cons nil)
(use Init.Option none some)
(fn less ((a Int) (b Int)) Bool (< a b))
(fn pick ((b Bool)) Int (match b (true 1) (false 2)))
(fn count ((xs (List Int))) Int (match xs (nil 0) ((cons _ r) (+ 1 (count r)))))
(fn first ((p (Prod Int Int))) Int (match p ((Prod.mk a _) a)))
(fn get ((o (Option Int))) Int (match o (none 0) ((some v) v)))
(fn go () Int
  (+ (* 10 (pick (less 1 2)))
    (+ (pick (less 2 1))
      (+ (* 100 (count (chars_of_sym 'abc))) (* 1000 (+ (first (Prod.mk 4 5)) (get (some 3))))))))
"#;
        let (root, l) = closure(&[("kernel/main.shard", src)], FACTS);
        let _ = std::fs::remove_dir_all(root);
        assert!(l.t.unread.is_empty(), "{:?}", l.t.unread);
        let m = plain::out::module(&l.t);
        assert_eq!(eval::eval(&m, &ast::Expr::Call("kernel.main.go".into(), vec![])).unwrap(), int(7312));
    }

    /// `Scope::rank` orders names as a citation's candidates stand, without
    /// building them: every candidate has a rank, an earlier one a lower
    /// rank, and a name that is no candidate has none.
    #[test]
    fn rank_is_the_candidates_order() {
        use plain::{Open, Scope};
        let sc = Scope {
            ident: "a.b".into(),
            tag: "a.b.c".into(),
            opened: vec![
                Open::All("x.y".into()),
                Open::Some("a.b.T".into(), vec!["K".into(), "L".into()]),
                Open::All(String::new()),
                Open::All("a.b".into()),
            ],
            vis: vec![],
        };
        let bare = Scope { ident: String::new(), opened: vec![], ..sc.clone() };
        for sc in [&sc, &bare] {
            for n in ["K", "f", "T.K", "L.m", "Init.List.cons", "Init", "y.f"] {
                let cs = sc.candidates(n);
                // each candidate once, where it first stands; the empty name is none
                let firsts: Vec<&String> =
                    cs.iter().enumerate().filter(|(i, k)| !k.is_empty() && cs.iter().position(|c| c == *k) == Some(*i)).map(|(_, k)| k).collect();
                let ranks: Vec<usize> = firsts.iter().map(|k| sc.rank(k, n).unwrap_or_else(|| panic!("{k} for {n}: no rank"))).collect();
                assert!(ranks.windows(2).all(|w| w[0] < w[1]), "{n}: {firsts:?} ranked {ranks:?}");
                for other in ["q.K", "a.bK", "a.b.c.K", "xK", ""] {
                    if !cs.iter().any(|c| c == other) {
                        assert_eq!(sc.rank(other, n), None, "{other} for {n}");
                    }
                }
            }
        }
    }

    /// A directory module: the consumer reads the view; the implementation
    /// is linked after it — its `type` and `fn`s under the view's `sig`s,
    /// by a private file of the sealed directory or by the implementation
    /// file itself — and the closure runs.
    #[test]
    fn view_is_linked_to_its_implementation() {
        let view = "(sig type Stack)\n(sig fn empty () Stack)\n(sig fn push ((n Int) (s Stack)) Stack)\n(sig fn top ((s Stack)) Int)\n";
        let imp = "(import \"rep.shard\")\n(fn top ((s Stack)) Int (match s ((Stack.mk k) k)))\n";
        let rep = "(type Stack (mk Int))\n(fn empty () Stack (Stack.mk 0))\n(fn push ((n Int) (s Stack)) Stack (match s ((Stack.mk k) (Stack.mk (+ k n)))))\n";
        let app = "(import \"lib\")\n(use lib)\n(fn go () Int (top (push 5 (push 2 (empty)))))\n";
        let (root, l) =
            closure(&[("lib/mod.req.shard", view), ("lib/rep.shard", rep), ("lib/lib.shard", imp), ("app.shard", app)], "");
        let _ = std::fs::remove_dir_all(root);
        assert!(l.t.unread.is_empty(), "{:?}", l.t.unread);
        assert_eq!(
            plain::out::dump(&l.t),
            "fn empty 0 () Stack (Stack.mk 0)
fn go 0 () Int (top (push 5 (push 2 (empty))))
fn push 0 (Int Stack) Stack (match #0 ((Stack.mk _) (Stack.mk (+ #0 #2))))
fn top 0 (Stack) Int (match #0 ((Stack.mk _) #0))
type Stack 0 (Stack.mk Int)
"
        );
        let m = plain::out::module(&l.t);
        assert_eq!(eval::eval(&m, &ast::Expr::Call("app.go".into(), vec![])).unwrap(), int(7));
    }

    /// Names are identities: a module per file, a citation through `use`,
    /// a file's own types opened.
    #[test]
    fn plain_reader_names() {
        let lib = r#"
(type Stack (Mk Int))
(fn push ((n Int) (s Stack)) Stack (match s ((Mk k) (Mk (+ k n)))))
"#;
        let app = r#"
(import "lib.shard")
(use a.lib)
(use a.lib.Stack)
(fn top ((s Stack)) Int (match s ((Mk k) k)))
(fn go () Int (top (push 5 (Mk (/ 7 2)))))
"#;
        let (root, l) = closure(&[("a/lib.shard", lib), ("a/app.shard", app)], "");
        assert!(l.t.unread.is_empty(), "{:?}", l.t.unread);
        assert_eq!(
            plain::out::dump(&l.t),
            "fn go 0 () Int (top (push 5 (Stack.Mk (Int.ediv 7 2))))
fn push 0 (Int Stack) Stack (match #0 ((Stack.Mk _) (Stack.Mk (+ #0 #2))))
fn top 0 (Stack) Int (match #0 ((Stack.Mk _) #0))
type Stack 0 (Stack.Mk Int)
"
        );
        let m = plain::out::module(&l.t);
        let call = ast::Expr::Call("a.app.go".into(), vec![]);
        assert_eq!(eval::eval(&m, &call).unwrap(), ast::Expr::IntLit(8.into()));
        let _ = std::fs::remove_dir_all(root);
    }

    /// An operator's entry is its operands'
    /// type's, a comparison where Init's Bool is expected is the decision's
    /// value, an implicit type argument is not written and a proof is
    /// passed over — all from the binders' types and the facts.
    #[test]
    fn plain_reader_by_types() {
        let facts = r#"
(ind Nat - () (Nat.zero Nat.succ) int)
(ind Bool - () (Bool.false Bool.true) (e 0 (Bool.false) (Bool.true)))
(ind List - ((x k)) (List.nil List.cons) (e 1 (List.nil) (List.cons 0 (List 0))))
(ctor List.nil List ((i d)) ())
(ctor List.cons List ((i d) (x k) (x k)) (t t))
(ctor Bool.false Bool () ())
(ctor Bool.true Bool () ())
(ind Fin - ((x d)) (Fin.mk) row)
(ctor Fin.mk Fin ((i p) (x k) (x p)) (t f))
(def Fin.val - ((i k) (x k)) (Fin 0 i x))
"#;
        let app = r#"
(use Init.List cons nil)
(fn pred ((n Nat)) Nat (- n 1))
(fn half ((x Int)) Int (/ x 2))
(fn small ((n Nat)) Bool (< n 10))
(fn firsts ((xs (List Nat))) (List Nat)
  (match xs
    (nil nil)
    ((cons x t) (@List.cons Nat (pred x) (firsts t)))))
(fn index ((i (Fin 4))) Nat (Fin.val i))
(fn clamp ((n Nat) (h (< n 4))) (Fin 4) (Fin.mk n h))
(fn go () Nat (match (firsts (cons 0 (cons 5 nil))) (nil 99) ((cons a _) (+ a (if (small 3) 100 200)))))
"#;
        let (root, l) = closure(&[("app.shard", app)], facts);
        assert!(l.t.unread.is_empty(), "{:?}", l.t.unread);
        let dump = plain::out::dump(&l.t);
        for line in [
            "fn pred 0 (Nat) Nat (Nat.sub #0 1)",
            "fn half 0 (Int) Int (Int.ediv #0 2)",
            "fn small 0 (Nat) Bool (if (< #0 10) (Bool.true) (Bool.false))",
            "fn firsts 0 ((List Nat)) (List Nat) (match #0 (List.nil (List.nil)) ((List.cons _ _) (List.cons (pred #1) (firsts #0))))",
            "fn index 0 (Fin) Nat #0",
            "fn clamp 0 (Nat) Fin #0",
            "type Bool 0 (Bool.false) (Bool.true)",
            "type List 1 (List.nil) (List.cons ?0 (List ?0))",
        ] {
            assert!(dump.lines().any(|l| l == line), "no line `{line}` in\n{dump}");
        }
        let m = plain::out::module(&l.t);
        let call = ast::Expr::Call("app.go".into(), vec![]);
        // pred 0 is 0 at Nat; small 3 is Init's true
        assert_eq!(eval::eval(&m, &call).unwrap(), ast::Expr::IntLit(100.into()));
        let _ = std::fs::remove_dir_all(root);
    }

    /// What has no reading is named and left out: a template's call.
    #[test]
    fn plain_reader_names_what_it_leaves() {
        let app = r#"
(type (L T) (N) (C T (L T)))
(fn each ((f (-> Int Int)) (xs (L Int))) (L Int) (match xs (N N) ((C x t) (C (f x) (each f t)))))
(fn inc ((x Int)) Int (+ x 1))
(fn all ((xs (L Int))) (L Int) (each inc xs))
"#;
        let (root, l) = closure(&[("app.shard", app)], "");
        assert_eq!(l.t.unread.len(), 1);
        assert_eq!((l.t.unread[0].0.as_str(), l.t.unread[0].1.0), ("app.all", "template"));
        assert_eq!(plain::out::dump(&l.t), "fn inc 0 (Int) Int (+ #0 1)
type L 1 (L.N) (L.C ?0 (L ?0))
");
        let _ = std::fs::remove_dir_all(root);
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
