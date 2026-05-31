//! `check` — proof-script driver for the narrow kernel.
//!
//! Loads the bundled kernel, then walks one or more user-provided
//! `.shard` proof files. Top-level forms in those files:
//!
//!   (claim NAME GOAL PROOF)
//!     Run `check_sequent` on a fresh Sequent lifted from GOAL
//!     (hyps = Nil). If True, the claim is added to the running
//!     Theory as `(Proven NAME GOAL)`, available to subsequent
//!     claims via `(Lemma NAME)` citations.
//!
//!   (axiom NAME GOAL)
//!     Admit GOAL into the Theory as `(Axiom NAME GOAL)` WITHOUT a
//!     proof — citable as `(Lemma NAME)` exactly like a proven claim.
//!     A trusted audit boundary (docs/BOUNDARIES.md): use only for
//!     facts about the runtime primitives that cannot be derived
//!     in-kernel (e.g. the Euclidean identity of `div`/`mod`).
//!     Reported as `AXIOM <name>` and tallied separately.
//!
//!   (use-module "path/to/file.shard")
//!     Load the named .shard file as a user-defined module (types,
//!     fns, externs) and merge it into the running user module. The
//!     `m` arg to subsequent check_sequent calls is the merged
//!     value, so claims can reason about user fns (e.g., a Simp
//!     step can unfold them). Path is relative to the proof file's
//!     directory.
//!
//!   (module NAME)
//!     Parsed but not implemented. Reserved for the directory-tree
//!     loader (a later slice). Errors at this slice — see
//!     docs/REVISIT.md, "Proof-file module syntax".
//!
//! GOAL and PROOF are parsed as narrow expressions against the
//! kernel's ctor set (so `(Goal …)`, `(Refl)`, `(ByTheory …)` etc.
//! resolve as `Ctor`s) and then evaluated to runtime values. This
//! avoids inventing a new sexp-to-value protocol — the kernel's
//! existing ctor application IS the value-construction syntax.
//!
//! Exit codes: 0 = all claims passed, 1 = some claim failed, 2 =
//! a load or eval error (no claim outcome could be determined).

use std::path::PathBuf;
use std::process::ExitCode;

use lexpr::Value;
use lexpr::parse::Parser;
use proving_bootstrap_v2::{ast, default_kernel_dir, eval, load, load_kernel_from};

fn main() -> ExitCode {
    // The bootstrap host runs shard via a recursive tree-walker (eval.rs);
    // the self-hosted reader's recursion depth scales with input size, so
    // the default 8 MiB main-thread stack overflows on large files. Give
    // the work a generous stack. (A compiled shard runtime won't need this.)
    std::thread::Builder::new()
        .stack_size(1024 * 1024 * 1024)
        .spawn(run)
        .expect("spawn worker thread")
        .join()
        .expect("worker thread panicked")
}

fn run() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: check [--trace <claim>|all] <proof_file.shard>...");
        eprintln!("       check eval [--no-bootstrap|--both] <file.shard>... <expr>");
        eprintln!("       check app [--repl|--script <events>] [--no-bootstrap] <file.shard>...");
        return ExitCode::from(2);
    }

    // `eval` subcommand: run an object-language expression rather than
    // check proofs. By default it runs through the BOOTSTRAPPED narrow
    // reducer (kernel/reduce.shard's compute_expr) — the self-hosted
    // evaluator; `--no-bootstrap` uses the native Rust eval::eval instead.
    if args[0] == "eval" {
        return run_eval(&args[1..]);
    }

    // `app` subcommand: drive a stateful application (the MVU entrypoint
    // declared via `(app …)`) through the bootstrapped reducer. The pure
    // `step` fn runs in the kernel; the event loop + effect interpretation
    // live here in the untrusted driver. See run_app.
    if args[0] == "app" {
        return run_app(&args[1..]);
    }

    // `cli` subcommand: run a shard program as a command-line app via the
    // request/response effect loop. The app emits a request Action
    // (GetArgs / ReadFile / Write / Exit); the driver services it and feeds
    // the result back as the next Event. This is the externalized-
    // continuation form of effect-as-data (BOUNDARIES mechanism C) — it
    // lets a pure shard program take args, read files, and emit output.
    if args[0] == "cli" {
        return run_cli(&args[1..]);
    }

    // `parse-check` subcommand: differential validation of the shard
    // reader (kernel/reader.shard's `parse_expr`) against the Rust parser
    // (load::expr_from_str). For each expression in a corpus file, parse
    // it both ways and compare the resulting Expr ASTs structurally.
    if args[0] == "parse-check" {
        return run_parse_check(&args[1..]);
    }

    // `module-check` subcommand: differential validation of the shard
    // module parser (kernel/reader.shard's `parse_module`) against
    // load::module_from_str_with_base over a whole .shard file.
    if args[0] == "module-check" {
        return run_module_check(&args[1..]);
    }

    // `claims-check` subcommand: differential validation of the shard claim
    // collector (kernel/reader.shard's `parse_claims`) against the Rust loader.
    // For each `(claim NAME GOAL PROOF)` form in a file, compare the raw
    // (pre-desugar) goal and proof construction Exprs parsed both ways.
    if args[0] == "claims-check" {
        return run_claims_check(&args[1..]);
    }

    let kernel = match load_kernel_from(default_kernel_dir()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading kernel from {}: {}",
                default_kernel_dir().display(), e);
            return ExitCode::from(2);
        }
    };

    // Pull out `--trace <claim>` (or `--trace all`); the rest are files.
    let mut trace_target: Option<String> = None;
    let mut files: Vec<String> = Vec::new();
    let mut ai = 0;
    while ai < args.len() {
        if args[ai] == "--trace" {
            match args.get(ai + 1) {
                Some(n) => { trace_target = Some(n.clone()); ai += 2; }
                None => { eprintln!("--trace requires a claim name (or 'all')"); return ExitCode::from(2); }
            }
        } else {
            files.push(args[ai].clone());
            ai += 1;
        }
    }

    // Checking is routed through the SELF-HOSTED shard driver
    // (kernel/driver.shard's check_production; kernel/trace.shard for --trace).
    run_shard_check(&files, &kernel, trace_target.as_deref())
}

// ----------------------------------------------------------------------
// Recursive file loader with transitive imports.
//
// A .shard file may mix object-level code (`type`/`fn`/`extern`),
// dependency directives (`import` / its legacy alias `use-module`), and
// proofs (`claim`). One file = one topic. `process_file` LOADS a file's
// code into the module in two passes so dependencies are in scope before
// use:
//   A. imports — recurse into each dependency FIRST (depth-first), so its
//      code lands in the shared module.
//   B. code — load THIS file's types/fns/externs (now that imports are
//      visible as the ctor/fn base).
// It does NOT check claims — that is the self-hosted shard driver's job
// (run_shard_check → check_production), which parses the claims out of the
// raw source itself. Dedup + cycle detection are by CANONICAL path: a file
// imported by several others is loaded once; an import cycle is a hard error.
// ----------------------------------------------------------------------

/// The accumulator `process_file` loads a module into: the merged module (as
/// native AST and as a reflected value), plus the dedup/cycle bookkeeping for
/// transitive imports. Checking is NOT done here — see run_shard_check.
struct Ctx {
    user_module: ast::Module,
    user_module_value: ast::Expr,
    loaded: std::collections::HashSet<PathBuf>,
    in_progress: Vec<PathBuf>,
}

fn process_file(path: &PathBuf, ctx: &mut Ctx, kernel: &ast::Module) -> Result<(), ExitCode> {
    let canon = path.canonicalize().unwrap_or_else(|_| path.clone());
    if ctx.loaded.contains(&canon) {
        return Ok(()); // already loaded this invocation — dedup
    }
    if ctx.in_progress.iter().any(|p| p == &canon) {
        eprintln!(
            "error: import cycle — {} is already being loaded",
            canon.display(),
        );
        return Err(ExitCode::from(2));
    }
    let src = match std::fs::read_to_string(path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error reading {}: {}", path.display(), e);
            return Err(ExitCode::from(2));
        }
    };
    let forms = match parse_top_level(&src) {
        Ok(fs) => fs,
        Err(e) => {
            eprintln!("error parsing {}: {}", path.display(), e);
            return Err(ExitCode::from(2));
        }
    };

    ctx.in_progress.push(canon.clone());

    // Pass A: imports (and the use-module alias) — recurse first.
    for form in &forms {
        if let Some(dep) = import_path(form) {
            let resolved = match path.parent() {
                Some(d) => d.join(&dep),
                None    => PathBuf::from(&dep),
            };
            process_file(&resolved, ctx, kernel)?;
        }
    }

    // Pass B: load this file's code against the (now import-augmented)
    // accumulated module as the ctor/fn base, so a local fn can see
    // imported ctors. The loader skips claim/import/use-module forms.
    match load::module_from_str_with_base(&src, Some(&ctx.user_module)) {
        Ok(loaded) => {
            // Warn on type names that shadow a built-in kernel type (e.g. the
            // reserved `Dir` rewrite-direction type): CaseOn/Induct look the
            // type up by name and would resolve the kernel's definition, with
            // a confusing "could not rebuild subgoal" failure.
            for td in &loaded.types {
                if kernel.types.iter().any(|kt| kt.name == td.name) {
                    eprintln!("warning: {}: type `{}` shadows a built-in kernel type — \
                               CaseOn/Induct may resolve the wrong one; consider renaming",
                              path.display(), td.name);
                }
            }
            merge_module(&mut ctx.user_module, loaded);
            ctx.user_module_value = module_to_value(&ctx.user_module);
        }
        Err(e) => {
            eprintln!("error: {}: loading module: {}", path.display(), e);
            return Err(ExitCode::from(2));
        }
    }

    // Claims/axioms are NOT checked here: `process_file` only loads a file's
    // code + imports into the module. All checking is done by the self-hosted
    // shard driver (run_shard_check → check_production). This loader exists to
    // build the module(s) the shard checker and the differential parser oracles
    // run against.
    ctx.in_progress.pop();
    ctx.loaded.insert(canon);
    Ok(())
}

/// If `form` is `(import "PATH")` or the legacy `(use-module "PATH")`,
/// return the path string; else None.
fn import_path(form: &Value) -> Option<String> {
    let items: Vec<&Value> = form.list_iter()?.collect();
    if items.len() != 2 {
        return None;
    }
    match items[0].as_symbol()? {
        "import" | "use-module" => items[1].as_str().map(|s| s.to_string()),
        _ => None,
    }
}

/// The full ordered, deduplicated file list to check: for each target (in CLI
/// order) its transitive imports come first (deps before dependents), then the
/// target itself. A file imported (or named) more than once appears once.
/// Post-order DFS; this is the order the production shard driver threads the
/// theory through, so a citation always sees its dependency already admitted.
fn ordered_closure(targets: &[PathBuf]) -> Result<Vec<PathBuf>, ExitCode> {
    fn visit(
        path: &PathBuf,
        order: &mut Vec<PathBuf>,
        seen: &mut std::collections::HashSet<PathBuf>,
    ) -> Result<(), ExitCode> {
        let canon = path.canonicalize().unwrap_or_else(|_| path.clone());
        if !seen.insert(canon) {
            return Ok(());
        }
        let src = std::fs::read_to_string(path).map_err(|e| {
            eprintln!("error reading {}: {}", path.display(), e);
            ExitCode::from(2)
        })?;
        let forms = parse_top_level(&src).map_err(|e| {
            eprintln!("error parsing {}: {}", path.display(), e);
            ExitCode::from(2)
        })?;
        for form in &forms {
            if let Some(dep) = import_path(form) {
                let resolved = match path.parent() {
                    Some(d) => d.join(&dep),
                    None => PathBuf::from(&dep),
                };
                visit(&resolved, order, seen)?;
            }
        }
        order.push(path.clone());
        Ok(())
    }
    let mut order = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for t in targets {
        visit(t, &mut order, &mut seen)?;
    }
    Ok(order)
}

/// Walk a narrow `(Cons h t)` / `Nil` value into a Vec of elements.
fn decode_list(v: &ast::Expr) -> Vec<&ast::Expr> {
    let mut out = Vec::new();
    let mut cur = v;
    while let ast::Expr::Ctor(n, a) = cur {
        if n == "Cons" && a.len() == 2 {
            out.push(&a[0]);
            cur = &a[1];
        } else {
            break;
        }
    }
    out
}

/// Parse a sexp `Value` as a narrow expression against the kernel's
/// ctor set, then evaluate it to a runtime value. The two steps
/// together turn `(Goal (Cons (Param 'x …) …) Nil (Equation …))` into
/// the corresponding Ctor value tree.
fn build_value(v: &Value, kernel: &ast::Module) -> Result<ast::Expr, String> {
    let ast = load::expr_from_value(v, kernel)
        .map_err(|e| format!("load: {}", e))?;
    eval::eval(kernel, &ast).map_err(|e| format!("eval: {:?}", e))
}

// ----------------------------------------------------------------------
// `eval` subcommand — run an object program, not check a proof.
//
//   check eval [--no-bootstrap|--both] [--reflected] <file.shard>... <EXPR>
//
// By default EXPR is SURFACE syntax — a normal object-language expression
// against the loaded module, including string-literal sugar:
//   (run "12-2")
// With `--reflected`, EXPR is instead the raw Expr datum as it appears in
// a claim goal (an escape hatch for hand-built terms):
//   (Call 'run (list (Ctor 'Cons (list (IntLit 49) … (Ctor 'Nil (list))))))
//
// The default evaluation path is the BOOTSTRAP: the expr is reflected and
// fed to the narrow reducer compute_expr (kernel/reduce.shard), executed by
// the Rust substrate — the self-hosted evaluator running the full-language
// program. `--no-bootstrap` runs the native Rust eval::eval directly;
// `--both` runs each and reports whether they agree (a differential test
// of reduce.shard against the native engine).
// ----------------------------------------------------------------------

fn run_eval(args: &[String]) -> ExitCode {
    let mut raw = false;
    let mut both = false;
    let mut reflected_input = false;
    let mut positional: Vec<&String> = Vec::new();
    for a in args {
        match a.as_str() {
            "--no-bootstrap" | "--raw" | "--native" => raw = true,
            "--both" | "--compare" => both = true,
            "--reflected" => reflected_input = true,
            s if s.starts_with("--") => {
                eprintln!("error: unknown flag {}", s);
                return ExitCode::from(2);
            }
            _ => positional.push(a),
        }
    }
    if positional.is_empty() {
        eprintln!("usage: check eval [--no-bootstrap|--both] [--reflected] <file.shard>... <expr>");
        return ExitCode::from(2);
    }
    let expr_src = positional.last().unwrap().as_str();
    let files = &positional[..positional.len() - 1];

    let kernel = match load_kernel_from(default_kernel_dir()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading kernel from {}: {}", default_kernel_dir().display(), e);
            return ExitCode::from(2);
        }
    };
    let user_module = ast::Module {
        types: kernel.types.clone(),
        fns: Vec::new(),
        externs: Vec::new(),
    };
    let mut ctx = Ctx {
        user_module_value: module_to_value(&user_module),
        user_module,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
    };
    for f in files {
        if let Err(code) = process_file(&PathBuf::from(f.as_str()), &mut ctx, &kernel) {
            return code;
        }
    }

    // Build the program in BOTH representations: `native` (executable, for
    // the Rust engine) and `reflected` (the Expr datum, for compute_expr).
    //   - surface input  (default): parse to native via expr_from_str, then
    //                               reflect for the bootstrap path.
    //   - reflected input (--reflected): build the datum, then un-reflect for
    //                               the native path (may fail on Match/Let).
    let (reflected, native): (ast::Expr, Result<ast::Expr, String>) = if reflected_input {
        let forms = match parse_top_level(expr_src) {
            Ok(fs) => fs,
            Err(e) => {
                eprintln!("error parsing expression: {}", e);
                return ExitCode::from(2);
            }
        };
        if forms.len() != 1 {
            eprintln!("error: expected exactly one expression, got {}", forms.len());
            return ExitCode::from(2);
        }
        let datum = match build_value(&forms[0], &kernel) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("error building expression: {}", e);
                return ExitCode::from(2);
            }
        };
        let nat = value_to_native_expr(&datum);
        (datum, nat)
    } else {
        let nat = match load::expr_from_str(expr_src, &ctx.user_module) {
            Ok(e) => e,
            Err(e) => {
                eprintln!("error parsing expression: {}", e);
                return ExitCode::from(2);
            }
        };
        (expr_to_value(&nat), Ok(nat))
    };

    // Both results are returned in REFLECTED form so they can be compared
    // and printed uniformly.
    if both {
        let b = eval_bootstrap(&kernel, &ctx.user_module_value, &reflected);
        let r = match &native {
            Ok(n) => eval_raw(&ctx.user_module, n),
            Err(e) => Err(e.clone()),
        };
        match (&b, &r) {
            (Ok(bv), Ok(rv)) => {
                println!("bootstrap : {}", show_reflected(bv));
                println!("native    : {}", show_reflected(rv));
                if bv == rv {
                    println!("agree     : yes");
                    ExitCode::SUCCESS
                } else {
                    println!("agree     : NO — reduce.shard disagrees with the native engine");
                    ExitCode::from(1)
                }
            }
            _ => {
                if let Err(e) = &b { eprintln!("bootstrap error: {}", e); }
                if let Err(e) = &r { eprintln!("native error: {}", e); }
                ExitCode::from(2)
            }
        }
    } else {
        let result = if raw {
            match &native {
                Ok(n) => eval_raw(&ctx.user_module, n),
                Err(e) => Err(e.clone()),
            }
        } else {
            eval_bootstrap(&kernel, &ctx.user_module_value, &reflected)
        };
        match result {
            Ok(v) => {
                println!("{}", show_reflected(&v));
                ExitCode::SUCCESS
            }
            Err(e) => {
                eprintln!("eval error: {}", e);
                ExitCode::from(2)
            }
        }
    }
}

// ----------------------------------------------------------------------
// `parse-check` subcommand — differential validation of the shard reader.
//
// For each expression line in a corpus file, parse it two ways:
//   reference : load::expr_from_str (the Rust parser) → reflect via
//               expr_to_value to the object Expr datum.
//   shard     : eval `parse_expr <line-as-(List Int)> <ctor-set>` (the
//               reader in kernel/reader.shard) on the NATIVE engine.
// and compare the two Expr data structurally. This is the same
// differential discipline that keeps reduce.shard honest (`eval --both`).
// ----------------------------------------------------------------------

fn sym_list_native(names: &[String]) -> ast::Expr {
    let mut acc = nil();
    for n in names.iter().rev() {
        acc = ctor("Cons", vec![ast::Expr::SymLit(n.clone()), acc]);
    }
    acc
}

fn run_parse_check(args: &[String]) -> ExitCode {
    // --unreflect: validate `parse_unreflect` (parse_expr then unreflect_expr)
    // against the native expr_from_str. The shard side then produces the
    // un-reflected native Expr, so the reference is NOT expr_to_value'd.
    let mut unreflect = false;
    let mut positional: Vec<&String> = Vec::new();
    for a in args {
        match a.as_str() {
            "--unreflect" => unreflect = true,
            s if s.starts_with("--") => {
                eprintln!("error: unknown flag {}", a);
                return ExitCode::from(2);
            }
            _ => positional.push(a),
        }
    }
    if positional.len() < 2 {
        eprintln!("usage: check parse-check [--unreflect] <file.shard>... <corpus.txt>");
        return ExitCode::from(2);
    }
    let corpus_path = positional.last().unwrap().as_str();
    let files = &positional[..positional.len() - 1];

    let kernel = match load_kernel_from(default_kernel_dir()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading kernel from {}: {}", default_kernel_dir().display(), e);
            return ExitCode::from(2);
        }
    };
    let user_module = ast::Module {
        types: kernel.types.clone(),
        fns: Vec::new(),
        externs: Vec::new(),
    };
    let mut ctx = Ctx {
        user_module_value: module_to_value(&user_module),
        user_module,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
    };
    for f in files {
        if let Err(code) = process_file(&PathBuf::from(f.as_str()), &mut ctx, &kernel) {
            return code;
        }
    }

    // The reader must classify Ctor-vs-Call using the SAME ctor set the
    // Rust parser sees — so it's built from the loaded module's types.
    let ctor_names: Vec<String> = ctx.user_module.types.iter()
        .flat_map(|td| td.ctors.iter().map(|cd| cd.name.clone()))
        .collect();
    let ctorset = sym_list_native(&ctor_names);

    let corpus = match std::fs::read_to_string(corpus_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error reading corpus {}: {}", corpus_path, e);
            return ExitCode::from(2);
        }
    };

    let mut passed = 0usize;
    let mut failed = 0usize;
    for (i, raw) in corpus.lines().enumerate() {
        let line = raw.trim();
        if line.is_empty() || line.starts_with(';') {
            continue;
        }
        // Reference: the Rust parser. In --unreflect mode the shard side
        // produces the native Expr, so compare against the native ast directly;
        // otherwise compare reflected datums (shard returns an object Expr).
        let reference = match load::expr_from_str(line, &ctx.user_module) {
            Ok(e) => if unreflect { e } else { expr_to_value(&e) },
            Err(e) => {
                println!("L{}: rust-parse error ({}) — skipped: {}", i + 1, e, line);
                continue;
            }
        };
        // Shard: parse_expr (or parse_unreflect) on the native engine.
        let call = ast::Expr::Call(
            if unreflect { "parse_unreflect" } else { "parse_expr" }.into(),
            vec![line_to_event_native(line), ctorset.clone()],
        );
        // eval_raw re-reflects its result (expr_to_value), so un-reflect
        // one layer to expose the (Some Expr) / (None) wrapper; the Expr
        // inside is then in the same representation as `reference`.
        let shard = match eval_raw(&ctx.user_module, &call)
            .and_then(|v| value_to_native_expr(&v))
        {
            Ok(v) => v,
            Err(e) => {
                failed += 1;
                println!("L{}: shard eval error ({}): {}", i + 1, e, line);
                continue;
            }
        };
        match &shard {
            ast::Expr::Ctor(n, a) if n == "Some" && a.len() == 1 => {
                if a[0] == reference {
                    passed += 1;
                } else {
                    failed += 1;
                    println!("L{}: DIFFER  {}", i + 1, line);
                    println!("    rust : {}", show_reflected(&reference));
                    println!("    shard: {}", show_reflected(&a[0]));
                }
            }
            ast::Expr::Ctor(n, _) if n == "None" => {
                failed += 1;
                println!("L{}: shard REJECTED (None), rust accepted: {}", i + 1, line);
            }
            other => {
                failed += 1;
                println!("L{}: shard returned non-Option: {}", i + 1, show_reflected(other));
            }
        }
    }
    println!();
    println!("{} passed, {} failed", passed, failed);
    if failed == 0 { ExitCode::SUCCESS } else { ExitCode::from(1) }
}

// ----------------------------------------------------------------------
// `module-check` subcommand — differential validation of the shard module
// parser. Parses a whole file two ways (load::module_from_str_with_base
// vs. the shard `parse_module`, both with the kernel's ctors as the
// ambient base) and compares the resulting Module data structurally.
// ----------------------------------------------------------------------

/// Localize a Module mismatch to the first differing type/fn/extern.
fn report_module_diff(shard: &ast::Expr, reference: &ast::Expr) {
    let unwrap = |e: &ast::Expr| -> Option<[ast::Expr; 3]> {
        if let ast::Expr::Ctor(n, a) = e {
            if n == "Module" && a.len() == 3 {
                return Some([a[0].clone(), a[1].clone(), a[2].clone()]);
            }
        }
        None
    };
    let (s, r) = match (unwrap(shard), unwrap(reference)) {
        (Some(s), Some(r)) => (s, r),
        _ => {
            println!("  shard module is malformed: {}", show_reflected(shard));
            return;
        }
    };
    let labels = ["type", "fn", "extern"];
    for i in 0..3 {
        let se = decode_list(&s[i]);
        let re = decode_list(&r[i]);
        if se.len() != re.len() {
            println!("  {} count: shard {} vs rust {}", labels[i], se.len(), re.len());
        }
        for (j, (a, b)) in se.iter().zip(re.iter()).enumerate() {
            if a != b {
                println!("  first differing {} (#{}):", labels[i], j);
                println!("    shard: {}", show_reflected(a));
                println!("    rust : {}", show_reflected(b));
                return;
            }
        }
    }
}

fn run_module_check(args: &[String]) -> ExitCode {
    let mut positional: Vec<&String> = Vec::new();
    for a in args {
        if a.starts_with("--") {
            eprintln!("error: unknown flag {}", a);
            return ExitCode::from(2);
        }
        positional.push(a);
    }
    if positional.len() < 2 {
        eprintln!("usage: check module-check <reader.shard>... <target.shard>");
        return ExitCode::from(2);
    }
    let target_path = positional.last().unwrap().as_str();
    let files = &positional[..positional.len() - 1];

    let kernel = match load_kernel_from(default_kernel_dir()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading kernel from {}: {}", default_kernel_dir().display(), e);
            return ExitCode::from(2);
        }
    };
    let user_module = ast::Module {
        types: kernel.types.clone(),
        fns: Vec::new(),
        externs: Vec::new(),
    };
    let mut ctx = Ctx {
        user_module_value: module_to_value(&user_module),
        user_module,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
    };
    for f in files {
        if let Err(code) = process_file(&PathBuf::from(f.as_str()), &mut ctx, &kernel) {
            return code;
        }
    }

    let target_src = match std::fs::read_to_string(target_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error reading {}: {}", target_path, e);
            return ExitCode::from(2);
        }
    };

    // Reference: the Rust loader, with the full loaded module (kernel +
    // any dependency files passed before the target) as the ambient base —
    // so a target using ctors declared in a sibling file resolves them.
    let module_rust = match load::module_from_str_with_base(&target_src, Some(&ctx.user_module)) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("rust load error: {:?}", e);
            return ExitCode::from(2);
        }
    };
    let reference = module_to_value(&module_rust);

    // Shard: parse_module on the native engine, same ambient ctor base.
    let ctor_names: Vec<String> = ctx.user_module.types.iter()
        .flat_map(|td| td.ctors.iter().map(|cd| cd.name.clone()))
        .collect();
    let base_ctors = sym_list_native(&ctor_names);
    let call = ast::Expr::Call(
        "parse_module".into(),
        vec![line_to_event_native(&target_src), base_ctors],
    );
    let shard = match eval_raw(&ctx.user_module, &call).and_then(|v| value_to_native_expr(&v)) {
        Ok(v) => v,
        Err(e) => {
            eprintln!("shard eval error: {}", e);
            return ExitCode::from(2);
        }
    };
    match &shard {
        ast::Expr::Ctor(n, a) if n == "Some" && a.len() == 1 => {
            if a[0] == reference {
                println!("MATCH  {}  ({} types, {} fns, {} externs)",
                    target_path, module_rust.types.len(),
                    module_rust.fns.len(), module_rust.externs.len());
                ExitCode::SUCCESS
            } else {
                println!("DIFFER  {}", target_path);
                report_module_diff(&a[0], &reference);
                ExitCode::from(1)
            }
        }
        ast::Expr::Ctor(n, _) if n == "None" => {
            println!("shard REJECTED {} (None), rust accepted", target_path);
            ExitCode::from(1)
        }
        other => {
            println!("shard returned non-Option: {}", show_reflected(other));
            ExitCode::from(1)
        }
    }
}

// ----------------------------------------------------------------------
// `claims-check` subcommand — differential validation of `parse_claims`.
//
// Reference: the Rust reader pulls each `(claim NAME GOAL PROOF)` form and
// parses GOAL/PROOF raw via `load::expr_from_value` (pre the named-hyp
// desugar / goal-eval the checker applies downstream).
// Shard: `parse_claims` collects the same claims; `claims_goals` /
// `claims_proofs` project the goal/proof Expr lists, which we un-reflect and
// compare structurally — same discipline as module-check / parse-check.
// ----------------------------------------------------------------------

fn run_claims_check(args: &[String]) -> ExitCode {
    // --unreflect: validate the shard un-reflectors. The shard side projects
    // each claim's goal through `unreflect_goal` (→ native Goal), so we compare
    // GOALS ONLY (the proof un-reflector lands in a later slice) and skip the
    // extra per-element un-reflect the raw mode needs.
    let mut unreflect = false;
    let mut positional: Vec<&String> = Vec::new();
    for a in args {
        match a.as_str() {
            "--unreflect" => unreflect = true,
            s if s.starts_with("--") => {
                eprintln!("error: unknown flag {}", a);
                return ExitCode::from(2);
            }
            _ => positional.push(a),
        }
    }
    if positional.len() < 2 {
        eprintln!("usage: check claims-check [--unreflect] <reader.shard>... <target.shard>");
        return ExitCode::from(2);
    }
    let target_path = positional.last().unwrap().as_str();
    let files = &positional[..positional.len() - 1];

    let kernel = match load_kernel_from(default_kernel_dir()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading kernel from {}: {}", default_kernel_dir().display(), e);
            return ExitCode::from(2);
        }
    };
    let user_module = ast::Module {
        types: kernel.types.clone(),
        fns: Vec::new(),
        externs: Vec::new(),
    };
    let mut ctx = Ctx {
        user_module_value: module_to_value(&user_module),
        user_module,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
    };
    for f in files {
        if let Err(code) = process_file(&PathBuf::from(f.as_str()), &mut ctx, &kernel) {
            return code;
        }
    }

    let target_src = match std::fs::read_to_string(target_path) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error reading {}: {}", target_path, e);
            return ExitCode::from(2);
        }
    };

    // Reference: pull claim forms via the Rust reader; goal/proof raw.
    let forms = match parse_top_level(&target_src) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("rust parse error: {}", e);
            return ExitCode::from(2);
        }
    };
    let mut ref_names: Vec<String> = Vec::new();
    let mut ref_goals: Vec<ast::Expr> = Vec::new();
    let mut ref_proofs: Vec<ast::Expr> = Vec::new();
    for form in &forms {
        let items: Vec<&Value> = match form.list_iter() {
            Some(it) => it.collect(),
            None => continue,
        };
        if items.len() != 4 || items[0].as_symbol() != Some("claim") {
            continue;
        }
        let name = match items[1].as_symbol() {
            Some(s) => s.to_string(),
            None => {
                eprintln!("claim NAME not a symbol");
                return ExitCode::from(2);
            }
        };
        let goal = match load::expr_from_value(items[2], &ctx.user_module) {
            Ok(e) => e,
            Err(e) => { eprintln!("rust goal parse ({}): {:?}", name, e); return ExitCode::from(2); }
        };
        let proof = match load::expr_from_value(items[3], &ctx.user_module) {
            Ok(e) => e,
            Err(e) => { eprintln!("rust proof parse ({}): {:?}", name, e); return ExitCode::from(2); }
        };
        ref_names.push(name);
        ref_goals.push(goal);
        ref_proofs.push(proof);
    }

    // Shard: parse_claims, then project + un-reflect the goal/proof lists.
    let ctor_names: Vec<String> = ctx.user_module.types.iter()
        .flat_map(|td| td.ctors.iter().map(|cd| cd.name.clone()))
        .collect();
    let base_ctors = sym_list_native(&ctor_names);
    // `extra`: in raw mode the accessor returns a (List Expr) whose elements are
    // object Exprs (reflected) — eval_raw reflects the list once more, so we
    // un-reflect once for the spine and once more per element. In --unreflect
    // mode the accessor already un-reflected each element to a native Goal, so
    // only the spine un-reflect is needed.
    let project = |proj: &str, extra: bool| -> Result<Vec<ast::Expr>, String> {
        let call = ast::Expr::Call(
            proj.into(),
            vec![ast::Expr::Call(
                "parse_claims".into(),
                vec![line_to_event_native(&target_src), base_ctors.clone()],
            )],
        );
        let native_list = value_to_native_expr(&eval_raw(&ctx.user_module, &call)?)?;
        let elems = decode_list(&native_list);
        if extra {
            elems.into_iter().map(value_to_native_expr).collect::<Result<Vec<_>, String>>()
        } else {
            Ok(elems.into_iter().cloned().collect())
        }
    };
    let goal_proj = if unreflect { "claims_goals_native" } else { "claims_goals" };
    let shard_goals = match project(goal_proj, !unreflect) {
        Ok(v) => v,
        Err(e) => { eprintln!("shard {} error: {}", goal_proj, e); return ExitCode::from(2); }
    };

    // Compare structurally.
    let mut ok = true;
    let compare = |label: &str, s: &[ast::Expr], r: &[ast::Expr], names: &[String], ok: &mut bool| {
        if s.len() != r.len() {
            println!("DIFFER  {}  {} count: shard {} vs rust {}", target_path, label, s.len(), r.len());
            *ok = false;
            return;
        }
        for (i, (a, b)) in s.iter().zip(r.iter()).enumerate() {
            if a != b {
                let nm = names.get(i).map(|x| x.as_str()).unwrap_or("?");
                println!("DIFFER  {}  claim '{}' {}:", target_path, nm, label);
                println!("    shard: {}", show_reflected(&expr_to_value(a)));
                println!("    rust : {}", show_reflected(&expr_to_value(b)));
                *ok = false;
                return;
            }
        }
    };
    compare("goal", &shard_goals, &ref_goals, &ref_names, &mut ok);
    if ok {
        let proof_proj = if unreflect { "claims_proofs_native" } else { "claims_proofs" };
        let shard_proofs = match project(proof_proj, !unreflect) {
            Ok(v) => v,
            Err(e) => { eprintln!("shard {} error: {}", proof_proj, e); return ExitCode::from(2); }
        };
        compare("proof", &shard_proofs, &ref_proofs, &ref_names, &mut ok);
    }
    if ok {
        let what = if unreflect { "claims un-reflected (goal+proof)" } else { "claims" };
        println!("MATCH  {}  ({} {})", target_path, ref_goals.len(), what);
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    }
}

// ----------------------------------------------------------------------
// Production checking through the self-hosted shard driver.
//
// This is THE checking path for `check <file>…` (including `--trace`). The
// entire claim-checking LOOP — parse claims/axioms, desugar named hyps,
// un-reflect goals/proofs, run check_sequent, thread the theory, admit axioms,
// and render the `--trace`/failure diagnostics — runs in shard
// (kernel/driver.shard's check_production + kernel/trace.shard), via the native
// VM exactly like the kernel's own check_sequent. Rust's role is reduced to:
//   - resolving the file list (imports first, then targets) and reading bytes,
//   - building the two modules (eval = kernel + toolchain; M = kernel.types +
//     all loaded code),
//   - decoding the returned (List ClaimOutcome) into PASS/FAIL/AXIOM lines.
// ----------------------------------------------------------------------

fn run_shard_check(files: &[String], kernel: &ast::Module, trace: Option<&str>) -> ExitCode {
    if files.is_empty() {
        eprintln!("usage: check [--trace <claim>|all] <proof_file.shard>...");
        return ExitCode::from(2);
    }
    let targets: Vec<PathBuf> = files.iter().map(PathBuf::from).collect();
    // The trace target as a Symbol: the claim name, "all", or a sentinel that
    // matches nothing when no --trace was given.
    let trace_target = trace.unwrap_or("__no_trace__").to_string();

    let mk_ctx = |seed: ast::Module| Ctx {
        user_module_value: module_to_value(&seed),
        user_module: seed,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
    };

    // eval module = kernel (types + fns) + the shard toolchain that runs the
    // driver. The toolchain now lives in the kernel dir.
    let mut eval_ctx = mk_ctx(ast::Module {
        types: kernel.types.clone(),
        fns: kernel.fns.clone(),
        externs: kernel.externs.clone(),
    });
    let kdir = default_kernel_dir();
    for tool in &["reader.shard", "unreflect.shard", "desugar.shard", "trace.shard", "driver.shard"] {
        if let Err(code) = process_file(&kdir.join(tool), &mut eval_ctx, kernel) {
            return code;
        }
    }

    // M module = kernel.types + all the user code (targets + their imports).
    // kernel.fns are kept OUT so the proofs reduce against the user's fns, not
    // the kernel's internals (e.g. term.shard's `len` vs std/list's `len`).
    let mut m_ctx = mk_ctx(ast::Module {
        types: kernel.types.clone(),
        fns: Vec::new(),
        externs: Vec::new(),
    });
    for t in &targets {
        if let Err(code) = process_file(t, &mut m_ctx, kernel) {
            return code;
        }
    }

    // Ordered file list (imports first, then targets), and their sources.
    let order = match ordered_closure(&targets) {
        Ok(o) => o,
        Err(code) => return code,
    };
    let mut srcs_list = nil();
    for p in order.iter().rev() {
        match std::fs::read_to_string(p) {
            Ok(s) => { srcs_list = ctor("Cons", vec![line_to_event_native(&s), srcs_list]); }
            Err(e) => { eprintln!("error reading {}: {}", p.display(), e); return ExitCode::from(2); }
        }
    }

    let ctor_names: Vec<String> = m_ctx.user_module.types.iter()
        .flat_map(|td| td.ctors.iter().map(|cd| cd.name.clone()))
        .collect();
    let ctorset = sym_list_native(&ctor_names);

    // (check_production <M> <srcs> <ctors> <trace-target>) → (List ClaimOutcome).
    let call = ast::Expr::Call(
        "check_production".into(),
        vec![m_ctx.user_module_value.clone(), srcs_list, ctorset, ast::Expr::SymLit(trace_target)],
    );
    let result = match eval_raw(&eval_ctx.user_module, &call).and_then(|v| value_to_native_expr(&v)) {
        Ok(v) => v,
        Err(e) => { eprintln!("shard check_production error: {}", e); return ExitCode::from(2); }
    };

    let sym_at = |a: &[ast::Expr]| -> String {
        match a.first() {
            Some(ast::Expr::SymLit(s)) => s.clone(),
            other => format!("{:?}", other),
        }
    };
    // an already-native (List Int) of codepoints → String.
    let text = |e: &ast::Expr| -> String {
        decode_list(e).iter().filter_map(|x| match x {
            ast::Expr::IntLit(c) => char::from_u32(*c as u32),
            _ => None,
        }).collect()
    };
    let (mut passed, mut failed, mut axioms) = (0usize, 0usize, 0usize);
    for item in decode_list(&result) {
        match item {
            ast::Expr::Ctor(tag, a) if tag == "COPass" && a.len() == 2 => {
                let block = text(&a[1]);
                if !block.is_empty() { println!("{}", block); }   // --trace block, if any
                println!("PASS  {}", sym_at(a)); passed += 1;
            }
            ast::Expr::Ctor(tag, a) if tag == "COAxiom" && a.len() == 1 => {
                println!("AXIOM {}  (admitted without proof)", sym_at(a)); axioms += 1;
            }
            ast::Expr::Ctor(tag, a) if tag == "COFail" && a.len() == 3 => {
                let block = text(&a[2]);
                if !block.is_empty() { println!("{}", block); }   // --trace block, if any
                println!("FAIL  {}", sym_at(a));
                let detail = text(&a[1]);
                if !detail.is_empty() { println!("{}", detail); }
                failed += 1;
            }
            other => { eprintln!("malformed ClaimOutcome: {}", show_reflected(other)); return ExitCode::from(2); }
        }
    }

    println!();
    if axioms > 0 {
        println!("{} passed, {} failed, {} axiom(s) admitted without proof", passed, failed, axioms);
    } else {
        println!("{} passed, {} failed", passed, failed);
    }
    if failed > 0 { ExitCode::from(1) } else { ExitCode::SUCCESS }
}

// ----------------------------------------------------------------------
// `app` subcommand — drive a stateful (Model-View-Update) application.
//
// The file declares an entrypoint:
//     (app (state S) (init INIT-EXPR) (update STEP-FN))
// where STEP-FN : S -> Event -> (Step S Action), Event = (List Int) (the
// input line as codepoints), and Action is inert effect DATA. The pure
// STEP-FN runs in the bootstrapped reducer (compute_expr); the loop and
// the effect interpretation live HERE, in the untrusted driver. The set
// of interpretable Actions (Print / Exit / Nop) is the one new trusted
// boundary — see docs/BOUNDARIES.md.
//
// State flows as REFLECTED Expr-data the whole time: init reduces to a
// state datum, and each step embeds the current state datum + the event
// datum into a reflected `(step state event)` call, reduces it, and reads
// the resulting `(Step state' action)` datum back out.
// ----------------------------------------------------------------------

fn run_app(args: &[String]) -> ExitCode {
    use std::io::{BufRead, IsTerminal};

    let mut script: Option<String> = None;
    let mut bootstrap = true;
    let mut positional: Vec<&String> = Vec::new();
    let mut it = args.iter();
    while let Some(a) = it.next() {
        match a.as_str() {
            "--repl" => script = None,
            // Default engine is the bootstrapped reducer (faithful to the
            // proof story but a double-interpreter — slow for heavy output
            // like a rendered board). --no-bootstrap runs the native Rust
            // engine instead, which an interactive app generally wants.
            "--no-bootstrap" | "--native" => bootstrap = false,
            "--script" => match it.next() {
                Some(f) => script = Some(f.clone()),
                None => {
                    eprintln!("error: --script requires a file argument (use - for stdin)");
                    return ExitCode::from(2);
                }
            },
            s if s.starts_with("--") => {
                eprintln!("error: unknown flag {}", s);
                return ExitCode::from(2);
            }
            _ => positional.push(a),
        }
    }
    if positional.is_empty() {
        eprintln!("usage: check app [--repl|--script <events>] [--no-bootstrap] <file.shard>...");
        return ExitCode::from(2);
    }

    let kernel = match load_kernel_from(default_kernel_dir()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading kernel from {}: {}", default_kernel_dir().display(), e);
            return ExitCode::from(2);
        }
    };
    let user_module = ast::Module {
        types: kernel.types.clone(),
        fns: Vec::new(),
        externs: Vec::new(),
    };
    let mut ctx = Ctx {
        user_module_value: module_to_value(&user_module),
        user_module,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
    };
    for f in &positional {
        if let Err(code) = process_file(&PathBuf::from(f.as_str()), &mut ctx, &kernel) {
            return code;
        }
    }

    // Locate the (app …) declaration among the named files.
    let app = match find_app_decl(&positional, "app") {
        Ok(Some(a)) => a,
        Ok(None) => {
            eprintln!("error: no (app …) declaration found in {}", positional.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", "));
            return ExitCode::from(2);
        }
        Err(e) => {
            eprintln!("error: {}", e);
            return ExitCode::from(2);
        }
    };
    let update_fn = match app_subform(&app, "update").and_then(|p| p.get(1).and_then(|s| s.as_symbol()).map(|s| s.to_string())) {
        Some(n) => n,
        None => {
            eprintln!("error: (app …) is missing an (update FN) field");
            return ExitCode::from(2);
        }
    };
    let init_val = match app_subform(&app, "init").and_then(|p| p.get(1).cloned()) {
        Some(v) => v,
        None => {
            eprintln!("error: (app …) is missing an (init EXPR) field");
            return ExitCode::from(2);
        }
    };
    // Optional (view FN): FN : State -> (List Int) renders the state to bytes.
    // When present the driver shows view(state) after init and after each tick
    // (so a REPL displays an initial frame); display is then the view's job and
    // the update's Action is reserved for effects (Exit/Nop).
    let view_fn: Option<String> = app_subform(&app, "view")
        .and_then(|p| p.get(1).and_then(|s| s.as_symbol()).map(|s| s.to_string()));
    // Optional (input raw): on a live terminal, read one keypress at a time
    // (no Enter) so an interactive game feels real-time. Default is line input.
    let raw_input = app_subform(&app, "input")
        .and_then(|p| p.get(1).and_then(|s| s.as_symbol()).map(|s| s == "raw"))
        .unwrap_or(false);

    // Reduce the init expression to the initial state datum.
    let init_native = match load::expr_from_value(&init_val, &ctx.user_module) {
        Ok(e) => e,
        Err(e) => {
            eprintln!("error in (init …): {}", e);
            return ExitCode::from(2);
        }
    };
    let init_result = if bootstrap {
        eval_bootstrap(&kernel, &ctx.user_module_value, &expr_to_value(&init_native))
    } else {
        eval_raw(&ctx.user_module, &init_native)
    };
    let mut state = match init_result {
        Ok(s) => s,
        Err(e) => {
            eprintln!("error evaluating (init …): {}", e);
            return ExitCode::from(2);
        }
    };

    // Initial frame (if the app declares a view).
    if let Some(vf) = &view_fn {
        if let Err(e) = render_view(bootstrap, &kernel, &ctx.user_module_value, &ctx.user_module, vf, &state) {
            eprintln!("error rendering view: {}", e);
            return ExitCode::from(2);
        }
    }

    // Interactive raw-key mode: app asked for (input raw) and we're on a live
    // terminal with no --script — read one keypress at a time (no Enter).
    if raw_input && script.is_none() && std::io::stdin().is_terminal() {
        return run_raw_loop(
            bootstrap, &kernel, &ctx.user_module_value, &ctx.user_module,
            &update_fn, &view_fn, state,
        );
    }

    // Otherwise: one event per input LINE. --script reads lines from a file (or
    // stdin via "-"); default reads stdin line-by-line.
    let lines: Box<dyn Iterator<Item = std::io::Result<String>>> = match &script {
        Some(path) if path != "-" => match std::fs::read_to_string(path) {
            Ok(src) => Box::new(src.lines().map(|s| Ok(s.to_string())).collect::<Vec<_>>().into_iter()),
            Err(e) => {
                eprintln!("error reading events file {}: {}", path, e);
                return ExitCode::from(2);
            }
        },
        _ => Box::new(std::io::stdin().lock().lines()),
    };

    for line in lines {
        let line = match line {
            Ok(l) => l,
            Err(e) => {
                eprintln!("error reading input: {}", e);
                return ExitCode::from(2);
            }
        };
        match run_tick(bootstrap, &kernel, &ctx.user_module_value, &ctx.user_module,
                       &update_fn, &view_fn, &mut state, &line) {
            Tick::Continue => {}
            Tick::Exit(code) => return ExitCode::from(code),
            Tick::Fatal => return ExitCode::from(2),
        }
    }
    ExitCode::SUCCESS
}

// ----------------------------------------------------------------------
// `cli` subcommand — run a shard program through the request/response
// effect loop. `check cli <file.shard>... [-- <app-args>...]`.
//
// The entrypoint is `(cli (state S) (init INIT) (update FN))` where
//   FN : S -> Event -> (Step S Action)
// The app is a state machine over a closed effect protocol: it emits a
// request Action, the driver performs it and feeds the result back as
// the next Event. The cycle starts with the `(Started)` event.
//
//   Action            driver does                  next Event
//   ----------------- ---------------------------- ----------------------
//   (GetArgs)         collect argv (after `--`)    (Args (List (List Int)))
//   (ReadFile path)   read the file at `path`      (FileOk bytes) | (FileErr)
//   (Write bytes)     write bytes to stdout        (Wrote)
//   (Exit code)       terminate with `code`        —
//
// This is BOUNDARIES mechanism (C): effect-as-data with the continuation
// externalized into the loop, so no HOF is needed — it runs in narrow
// shard today. The interpretable Action/Event set is the trusted edge.
// ----------------------------------------------------------------------

fn run_cli(args: &[String]) -> ExitCode {
    use std::io::Write as _;

    // `--` separates the app's SOURCE files from its runtime ARGUMENTS.
    let dash = args.iter().position(|a| a == "--");
    let src_args: &[String] = match dash { Some(i) => &args[..i], None => args };
    let app_args: &[String] = match dash { Some(i) => &args[i + 1..], None => &[] };

    let mut positional: Vec<&String> = Vec::new();
    for a in src_args {
        if a.starts_with("--") {
            eprintln!("error: unknown flag {} (runtime args go after `--`)", a);
            return ExitCode::from(2);
        }
        positional.push(a);
    }
    if positional.is_empty() {
        eprintln!("usage: check cli <file.shard>... [-- <app-args>...]");
        return ExitCode::from(2);
    }

    let kernel = match load_kernel_from(default_kernel_dir()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading kernel from {}: {}", default_kernel_dir().display(), e);
            return ExitCode::from(2);
        }
    };
    // Seed with the FULL kernel (types AND fns/externs): a cli app may call
    // kernel functions — the eval app calls `compute_expr` — so they must be
    // resolvable in the running module, not just the kernel's type decls.
    let user_module = ast::Module {
        types: kernel.types.clone(),
        fns: kernel.fns.clone(),
        externs: kernel.externs.clone(),
    };
    let mut ctx = Ctx {
        user_module_value: module_to_value(&user_module),
        user_module,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
    };
    for f in &positional {
        if let Err(code) = process_file(&PathBuf::from(f.as_str()), &mut ctx, &kernel) {
            return code;
        }
    }

    let cli = match find_app_decl(&positional, "cli") {
        Ok(Some(c)) => c,
        Ok(None) => { eprintln!("error: no (cli …) declaration found"); return ExitCode::from(2); }
        Err(e) => { eprintln!("error: {}", e); return ExitCode::from(2); }
    };
    let update_fn = match app_subform(&cli, "update").and_then(|p| p.get(1).and_then(|s| s.as_symbol()).map(|s| s.to_string())) {
        Some(n) => n,
        None => { eprintln!("error: (cli …) is missing an (update FN) field"); return ExitCode::from(2); }
    };
    let init_val = match app_subform(&cli, "init").and_then(|p| p.get(1).cloned()) {
        Some(v) => v,
        None => { eprintln!("error: (cli …) is missing an (init EXPR) field"); return ExitCode::from(2); }
    };

    let init_native = match load::expr_from_value(&init_val, &ctx.user_module) {
        Ok(e) => e,
        Err(e) => { eprintln!("error in (init …): {}", e); return ExitCode::from(2); }
    };
    // State flows as REFLECTED Expr-data between ticks (eval_raw returns it);
    // it is un-reflected to native just before each update call.
    let mut state = match eval_raw(&ctx.user_module, &init_native) {
        Ok(s) => s,
        Err(e) => { eprintln!("error evaluating (init …): {}", e); return ExitCode::from(2); }
    };

    let mut event = ctor("Started", vec![]); // native Event value
    let mut stdout = std::io::stdout();
    let cap = 50_000_000usize; // runaway-loop backstop
    for _ in 0..cap {
        let state_native = match value_to_native_expr(&state) {
            Ok(e) => e,
            Err(e) => { eprintln!("error: bad state datum: {}", e); return ExitCode::from(2); }
        };
        let call = ast::Expr::Call(update_fn.clone(), vec![state_native, event.clone()]);
        let result = match eval_raw(&ctx.user_module, &call) {
            Ok(r) => r,
            Err(e) => { eprintln!("error in {}: {}", update_fn, e); return ExitCode::from(2); }
        };
        let (sname, fields) = match as_obj_ctor(&result) {
            Some(x) => x,
            None => {
                eprintln!("error: `{}` returned a stuck term: {}", update_fn, show_reflected(&result));
                return ExitCode::from(2);
            }
        };
        if sname != "Step" || fields.len() != 2 {
            eprintln!("error: `{}` must return (Step state action), got: {}", update_fn, show_reflected(&result));
            return ExitCode::from(2);
        }
        let next_state = fields[0].clone();
        let (aname, afields) = match as_obj_ctor(fields[1]) {
            Some(x) => x,
            None => { eprintln!("error: action is not a constructor: {}", show_reflected(fields[1])); return ExitCode::from(2); }
        };
        // Service the request → the next Event (or terminate).
        event = match (aname, afields.as_slice()) {
            ("GetArgs", []) => {
                let items: Vec<ast::Expr> = app_args.iter().map(|a| line_to_event_native(a)).collect();
                ctor("Args", vec![list_of(items)])
            }
            ("ReadFile", [path]) => {
                let p = match decode_codepoints(path) {
                    Ok(s) => s,
                    Err(e) => { eprintln!("error: ReadFile path not a (List Int): {}", e); return ExitCode::from(2); }
                };
                match std::fs::read_to_string(&p) {
                    Ok(contents) => ctor("FileOk", vec![line_to_event_native(&contents)]),
                    Err(_) => ctor("FileErr", vec![]),
                }
            }
            ("Write", [payload]) => {
                let bytes = match decode_codepoints(payload) {
                    Ok(s) => s,
                    Err(e) => { eprintln!("error: Write payload not a (List Int): {}", e); return ExitCode::from(2); }
                };
                if write!(stdout, "{}", bytes).is_err() {
                    return ExitCode::from(2);
                }
                ctor("Wrote", vec![])
            }
            ("Exit", [code]) => {
                let _ = stdout.flush();
                let n = match value_to_native_expr(code) {
                    Ok(ast::Expr::IntLit(n)) => n,
                    _ => 0,
                };
                return ExitCode::from(n as u8);
            }
            (other, _) => {
                eprintln!("error: unknown cli Action `{}` (expected GetArgs/ReadFile/Write/Exit)", other);
                return ExitCode::from(2);
            }
        };
        state = next_state;
    }
    eprintln!("error: cli step cap ({}) exceeded — update never emitted Exit", cap);
    ExitCode::from(2)
}

enum Tick { Continue, Exit(u8), Fatal }

/// Run one update tick: feed `event_text` to the update fn, interpret the
/// returned Action, advance `state`, and render the view (if any). Shared by
/// the line-input and raw-key loops.
fn run_tick(
    bootstrap: bool,
    kernel: &ast::Module,
    user_module_value: &ast::Expr,
    user_module: &ast::Module,
    update_fn: &str,
    view_fn: &Option<String>,
    state: &mut ast::Expr,
    event_text: &str,
) -> Tick {
    let step_result = if bootstrap {
        let event = expr_to_value(&line_to_event_native(event_text));
        eval_bootstrap(kernel, user_module_value, &reflected_call(update_fn, vec![state.clone(), event]))
    } else {
        match value_to_native_expr(state) {
            Ok(state_native) => {
                let call = ast::Expr::Call(update_fn.into(), vec![state_native, line_to_event_native(event_text)]);
                eval_raw(user_module, &call)
            }
            Err(e) => Err(e),
        }
    };
    let result = match step_result {
        Ok(r) => r,
        Err(e) => { eprintln!("error in step: {}", e); return Tick::Fatal; }
    };
    let (sname, fields) = match as_obj_ctor(&result) {
        Some(x) => x,
        None => {
            eprintln!("error: `{}` did not return a value (stuck term): {}", update_fn, show_reflected(&result));
            return Tick::Fatal;
        }
    };
    if sname != "Step" || fields.len() != 2 {
        eprintln!("error: `{}` must return (Step state action), got: {}", update_fn, show_reflected(&result));
        return Tick::Fatal;
    }
    let next_state = fields[0].clone();
    let action = fields[1];
    match interpret_action(action) {
        Ok(ActionOutcome::Continue) => {}
        Ok(ActionOutcome::Exit(code)) => return Tick::Exit(code),
        Err(e) => { eprintln!("error: {}", e); return Tick::Fatal; }
    }
    *state = next_state;
    if let Some(vf) = view_fn {
        if let Err(e) = render_view(bootstrap, kernel, user_module_value, user_module, vf, state) {
            eprintln!("error rendering view: {}", e);
            return Tick::Fatal;
        }
    }
    Tick::Continue
}

/// Interactive single-keypress loop. Puts the terminal in non-canonical,
/// no-echo mode via `stty` (no extra dependency), feeds each byte as a
/// one-character event, and restores the terminal on exit. q / Ctrl-C /
/// Ctrl-D / EOF quit.
fn run_raw_loop(
    bootstrap: bool,
    kernel: &ast::Module,
    user_module_value: &ast::Expr,
    user_module: &ast::Module,
    update_fn: &str,
    view_fn: &Option<String>,
    mut state: ast::Expr,
) -> ExitCode {
    use std::io::Read;
    let saved = stty_capture(&["-g"]);
    stty_apply(&["-icanon", "-echo", "-isig", "min", "1", "time", "0"]);
    let mut stdin = std::io::stdin();
    let mut buf = [0u8; 1];
    let code = loop {
        match stdin.read(&mut buf) {
            Ok(0) => break 0u8,                                   // EOF
            Ok(_) => {
                let b = buf[0];
                if b == b'q' || b == 3 || b == 4 { break 0; }     // q / Ctrl-C / Ctrl-D
                let ev = (b as char).to_string();
                match run_tick(bootstrap, kernel, user_module_value, user_module,
                               update_fn, view_fn, &mut state, &ev) {
                    Tick::Continue => {}
                    Tick::Exit(c) => break c,
                    Tick::Fatal => break 2,
                }
            }
            Err(_) => break 2,
        }
    };
    // restore the terminal (to the saved settings, else a sane default).
    match &saved {
        Some(s) => stty_apply(&[s.as_str()]),
        None => stty_apply(&["sane"]),
    }
    ExitCode::from(code)
}

fn stty_capture(args: &[&str]) -> Option<String> {
    let out = std::process::Command::new("stty").args(args)
        .stdin(std::process::Stdio::inherit()).output().ok()?;
    if out.status.success() {
        Some(String::from_utf8_lossy(&out.stdout).trim().to_string())
    } else { None }
}

fn stty_apply(args: &[&str]) {
    let _ = std::process::Command::new("stty").args(args)
        .stdin(std::process::Stdio::inherit()).status();
}

/// Render one frame: evaluate `view(state)` (bootstrap or native), decode the
/// resulting (List Int) to text, and print it followed by a single newline
/// (the driver owns frame separation; the view's bytes are exact).
fn render_view(
    bootstrap: bool,
    kernel: &ast::Module,
    user_module_value: &ast::Expr,
    user_module: &ast::Module,
    view_fn: &str,
    state: &ast::Expr,
) -> Result<(), String> {
    let result = if bootstrap {
        eval_bootstrap(kernel, user_module_value, &reflected_call(view_fn, vec![state.clone()]))?
    } else {
        let call = ast::Expr::Call(view_fn.into(), vec![value_to_native_expr(state)?]);
        eval_raw(user_module, &call)?
    };
    println!("{}", decode_codepoints(&result)?);
    Ok(())
}

/// Effect interpretation — the trusted edge of the proof. Each arm
/// performs the real-world effect named by an Action datum.
enum ActionOutcome {
    Continue,
    Exit(u8),
}

fn interpret_action(action: &ast::Expr) -> Result<ActionOutcome, String> {
    let (name, fields) = as_obj_ctor(action)
        .ok_or_else(|| format!("action is not a constructor: {}", show_reflected(action)))?;
    match (name, fields.as_slice()) {
        ("Print", [payload]) => {
            println!("{}", decode_codepoints(payload)?);
            Ok(ActionOutcome::Continue)
        }
        ("Exit", [code]) => {
            let n = match value_to_native_expr(code)? {
                ast::Expr::IntLit(n) => n,
                other => return Err(format!("Exit code is not an Int: {:?}", other)),
            };
            Ok(ActionOutcome::Exit(n as u8))
        }
        ("Nop", []) => Ok(ActionOutcome::Continue),
        _ => Err(format!("unknown Action `{}` — the driver interprets only Print/Exit/Nop", name)),
    }
}

/// Find the single `(app …)` form among the named files. Errors if more
/// than one file declares an app.
fn find_app_decl(files: &[&String], head: &str) -> Result<Option<Value>, String> {
    let mut found: Option<Value> = None;
    for f in files {
        let src = std::fs::read_to_string(f.as_str())
            .map_err(|e| format!("reading {}: {}", f, e))?;
        let forms = parse_top_level(&src).map_err(|e| format!("parsing {}: {}", f, e))?;
        for form in forms {
            if form.list_iter().and_then(|mut it| it.next().and_then(|h| h.as_symbol()).map(|s| s == head)).unwrap_or(false) {
                if found.is_some() {
                    return Err(format!("multiple ({head} …) declarations found"));
                }
                found = Some(form);
            }
        }
    }
    Ok(found)
}

/// Return the parts of the `(KEY …)` sub-form inside an `(app …)` form,
/// e.g. `app_subform(app, "update")` → `[update, step]`.
fn app_subform(app: &Value, key: &str) -> Option<Vec<Value>> {
    for item in app.list_iter()? {
        if let Some(sub) = item.list_iter() {
            let parts: Vec<Value> = sub.cloned().collect();
            if parts.first().and_then(|h| h.as_symbol()) == Some(key) {
                return Some(parts);
            }
        }
    }
    None
}

/// An input line → the reflected-NOTHING native `(List Int)` of its
/// codepoints (the Event). Reflected to Expr-data by the caller.
fn line_to_event_native(line: &str) -> ast::Expr {
    let mut e = nil();
    for ch in line.chars().rev() {
        e = ctor("Cons", vec![ast::Expr::IntLit(ch as u32 as i64), e]);
    }
    e
}

/// Build the reflected Expr-data for `(Call 'fn (list arg…))`, where each
/// `arg` is already an Expr-datum (so it embeds as an in-NF argument).
fn reflected_call(fn_name: &str, arg_data: Vec<ast::Expr>) -> ast::Expr {
    let mut spine = nil();
    for a in arg_data.into_iter().rev() {
        spine = ctor("Cons", vec![a, spine]);
    }
    ctor("Call", vec![ast::Expr::SymLit(fn_name.into()), spine])
}

/// View a reflected Expr-datum that encodes an OBJECT constructor value
/// `(C f1 … fn)` — i.e. `Ctor("Ctor", [SymLit C, <field spine>])` — as its
/// ctor name plus the field data (each field is itself Expr-data). Returns
/// None for any other (e.g. stuck/partially-reduced) shape.
fn as_obj_ctor(e: &ast::Expr) -> Option<(&str, Vec<&ast::Expr>)> {
    if let ast::Expr::Ctor(c, a) = e {
        if c == "Ctor" && a.len() == 2 {
            if let ast::Expr::SymLit(name) = &a[0] {
                return Some((name.as_str(), datum_spine_refs(&a[1])?));
            }
        }
    }
    None
}

/// Walk a NATIVE `(Cons h t)` / `Nil` spine (the representation of a
/// reflected ctor's field list), returning references to each element
/// WITHOUT un-reflecting them.
fn datum_spine_refs(e: &ast::Expr) -> Option<Vec<&ast::Expr>> {
    let mut out = Vec::new();
    let mut cur = e;
    loop {
        match cur {
            ast::Expr::Ctor(n, a) if n == "Nil" && a.is_empty() => return Some(out),
            ast::Expr::Ctor(n, a) if n == "Cons" && a.len() == 2 => {
                out.push(&a[0]);
                cur = &a[1];
            }
            _ => return None,
        }
    }
}

/// Decode a reflected `(List Int)` datum (an Action's Print payload) into a
/// String of its codepoints.
fn decode_codepoints(data: &ast::Expr) -> Result<String, String> {
    let native = value_to_native_expr(data)?;
    let mut s = String::new();
    let mut cur = &native;
    loop {
        match cur {
            ast::Expr::Ctor(n, a) if n == "Nil" && a.is_empty() => return Ok(s),
            ast::Expr::Ctor(n, a) if n == "Cons" && a.len() == 2 => {
                match &a[0] {
                    ast::Expr::IntLit(cp) => match char::from_u32(*cp as u32) {
                        Some(c) => s.push(c),
                        None => return Err(format!("Print payload has non-codepoint {}", cp)),
                    },
                    other => return Err(format!("Print payload element is not an Int: {:?}", other)),
                }
                cur = &a[1];
            }
            _ => return Err(format!("Print payload is not a (List Int): {}", fmt_native_expr(&native))),
        }
    }
}

/// Bootstrap evaluation: run the narrow `compute_expr` over the reflected
/// expr, with the user module reflected as data. Returns reflected Expr.
fn eval_bootstrap(
    kernel: &ast::Module,
    user_module_value: &ast::Expr,
    reflected: &ast::Expr,
) -> Result<ast::Expr, String> {
    let call = ast::Expr::Call(
        "compute_expr".into(),
        vec![user_module_value.clone(), reflected.clone()],
    );
    eval::eval(kernel, &call).map_err(|e| format!("{:?}", e))
}

/// Native evaluation: run the Rust engine over the user module on the
/// executable program. Re-reflects the result so it matches the bootstrap
/// representation for uniform printing/comparison.
fn eval_raw(user_module: &ast::Module, native: &ast::Expr) -> Result<ast::Expr, String> {
    let out = eval::eval(user_module, native).map_err(|e| format!("{:?}", e))?;
    Ok(expr_to_value(&out))
}

/// Inverse of `expr_to_value`: turn a reflected Expr datum (a `Ctor`
/// tree over the kernel's Expr type) back into an executable native
/// `ast::Expr`. Handles the forms that appear in eval inputs and in
/// fully-reduced result values; control forms (Match/Let/If) in INPUT
/// are uncommon and rejected with a clear error.
fn value_to_native_expr(e: &ast::Expr) -> Result<ast::Expr, String> {
    let (name, args) = match e {
        ast::Expr::Ctor(n, a) => (n.as_str(), a.as_slice()),
        other => return Err(format!("cannot un-reflect non-Ctor node: {:?}", other)),
    };
    match (name, args) {
        ("IntLit", [ast::Expr::IntLit(n)]) => Ok(ast::Expr::IntLit(*n)),
        ("SymLit", [ast::Expr::SymLit(s)]) => Ok(ast::Expr::SymLit(s.clone())),
        ("FVar", [ast::Expr::SymLit(s)]) => Ok(ast::Expr::FVar(s.clone())),
        ("BVar", [ast::Expr::IntLit(k)]) => Ok(ast::Expr::BVar(*k as u32)),
        ("Ctor", [ast::Expr::SymLit(n), l]) =>
            Ok(ast::Expr::Ctor(n.clone(), unreflect_list(l)?)),
        ("Call", [ast::Expr::SymLit(n), l]) =>
            Ok(ast::Expr::Call(n.clone(), unreflect_list(l)?)),
        ("If", [c, t, el]) => Ok(ast::Expr::If(
            Box::new(value_to_native_expr(c)?),
            Box::new(value_to_native_expr(t)?),
            Box::new(value_to_native_expr(el)?),
        )),
        ("Match", _) | ("Let", _) =>
            Err(format!("un-reflection of {} in eval input is not supported", name)),
        _ => Err(format!("cannot un-reflect Expr node: {:?}", e)),
    }
}

/// Walk a reflected `(Cons h t)` / `Nil` spine, un-reflecting each element.
fn unreflect_list(e: &ast::Expr) -> Result<Vec<ast::Expr>, String> {
    let mut out = Vec::new();
    let mut cur = e;
    loop {
        match cur {
            ast::Expr::Ctor(n, args) if n == "Nil" && args.is_empty() => return Ok(out),
            ast::Expr::Ctor(n, args) if n == "Cons" && args.len() == 2 => {
                out.push(value_to_native_expr(&args[0])?);
                cur = &args[1];
            }
            other => return Err(format!("expected a reflected list spine, got: {:?}", other)),
        }
    }
}

/// Pretty-print a reflected Expr datum as readable object-level surface
/// syntax: (Some 3), None, (Cons 49 (Cons 43 Nil)). Falls back to native
/// rendering for any node that doesn't un-reflect.
fn show_reflected(e: &ast::Expr) -> String {
    match value_to_native_expr(e) {
        Ok(native) => fmt_native_expr(&native),
        Err(_) => format!("{:?}", e),
    }
}

fn fmt_native_expr(e: &ast::Expr) -> String {
    match e {
        ast::Expr::IntLit(n) => n.to_string(),
        ast::Expr::SymLit(s) => format!("'{}", s),
        ast::Expr::FVar(s) => s.clone(),
        ast::Expr::BVar(k) => format!("${}", k),
        ast::Expr::Ctor(n, args) | ast::Expr::Call(n, args) if args.is_empty() => n.clone(),
        ast::Expr::Ctor(n, args) | ast::Expr::Call(n, args) => {
            let inner: Vec<String> = args.iter().map(fmt_native_expr).collect();
            format!("({} {})", n, inner.join(" "))
        }
        other => format!("{:?}", other),
    }
}

// ----------------------------------------------------------------------
// Helpers
// ----------------------------------------------------------------------

fn ctor(name: &str, args: Vec<ast::Expr>) -> ast::Expr {
    ast::Expr::Ctor(name.into(), args)
}

fn nil() -> ast::Expr {
    ctor("Nil", vec![])
}

fn parse_top_level(src: &str) -> Result<Vec<Value>, String> {
    let mut parser = Parser::from_str(src);
    let mut out = Vec::new();
    loop {
        match parser.next_value() {
            Ok(Some(v)) => out.push(v),
            Ok(None) => return Ok(out),
            Err(e) => return Err(e.to_string()),
        }
    }
}

/// Append all items from `incoming` into `dst`. Used to accumulate
/// multiple (use-module …) declarations into one user-module value.
/// Name collisions are not checked — first wins on any later lookup
/// since the kernel's lookup_fn / lookup_typedef walks declaration
/// order and stops at the first match. Documented in REVISIT.
fn merge_module(dst: &mut ast::Module, incoming: ast::Module) {
    dst.types.extend(incoming.types);
    dst.fns.extend(incoming.fns);
    dst.externs.extend(incoming.externs);
}

// ----------------------------------------------------------------------
// ast::Module → runtime Module value
//
// The narrow kernel's check_sequent takes the user module AS A VALUE
// (Ctor("Module", […])). That value is what kernel-side lookup_fn /
// simp_expr / step_call walk to find user fn bodies for unfolding.
// The Rust-level evaluator already runs the kernel's code with the
// kernel `ast::Module`; what we need here is to reify the user's
// `ast::Module` as data the kernel can read.
//
// The conversion is mechanical 1:1 with kernel/term.shard's Expr/Pat
// and kernel/module.shard's TypeDef/CtorDef/FnDef/ExternDef/Module
// declarations. If those evolve, this file follows.
// ----------------------------------------------------------------------

fn module_to_value(m: &ast::Module) -> ast::Expr {
    ctor("Module", vec![
        list_of(m.types.iter().map(type_def_to_value).collect()),
        list_of(m.fns.iter().map(fn_def_to_value).collect()),
        list_of(m.externs.iter().map(extern_def_to_value).collect()),
    ])
}

fn type_def_to_value(td: &ast::TypeDef) -> ast::Expr {
    ctor("TypeDef", vec![
        ast::Expr::SymLit(td.name.clone()),
        list_of(td.params.iter().cloned().map(ast::Expr::SymLit).collect()),
        list_of(td.ctors.iter().map(ctor_def_to_value).collect()),
    ])
}

fn ctor_def_to_value(cd: &ast::CtorDef) -> ast::Expr {
    ctor("CtorDef", vec![
        ast::Expr::SymLit(cd.name.clone()),
        list_of(cd.fields.iter().map(type_to_value).collect()),
    ])
}

fn fn_def_to_value(fd: &ast::FnDef) -> ast::Expr {
    ctor("FnDef", vec![
        ast::Expr::SymLit(fd.name.clone()),
        list_of(fd.params.iter().map(type_to_value).collect()),
        type_to_value(&fd.ret),
        expr_to_value(&fd.body),
    ])
}

fn extern_def_to_value(ed: &ast::ExternDef) -> ast::Expr {
    ctor("ExternDef", vec![
        ast::Expr::SymLit(ed.name.clone()),
        list_of(ed.params.iter().map(type_to_value).collect()),
        type_to_value(&ed.ret),
    ])
}

fn type_to_value(t: &ast::Type) -> ast::Expr {
    match t {
        ast::Type::TCon(n, args) => ctor("TCon", vec![
            ast::Expr::SymLit(n.clone()),
            list_of(args.iter().map(type_to_value).collect()),
        ]),
        ast::Type::TVar(n) => ctor("TVar", vec![ast::Expr::SymLit(n.clone())]),
    }
}

fn expr_to_value(e: &ast::Expr) -> ast::Expr {
    match e {
        ast::Expr::FVar(n) =>
            ctor("FVar", vec![ast::Expr::SymLit(n.clone())]),
        ast::Expr::BVar(k) =>
            ctor("BVar", vec![ast::Expr::IntLit(*k as i64)]),
        ast::Expr::IntLit(n) =>
            ctor("IntLit", vec![ast::Expr::IntLit(*n)]),
        ast::Expr::SymLit(s) =>
            ctor("SymLit", vec![ast::Expr::SymLit(s.clone())]),
        ast::Expr::Ctor(n, args) => ctor("Ctor", vec![
            ast::Expr::SymLit(n.clone()),
            list_of(args.iter().map(expr_to_value).collect()),
        ]),
        ast::Expr::Call(n, args) => ctor("Call", vec![
            ast::Expr::SymLit(n.clone()),
            list_of(args.iter().map(expr_to_value).collect()),
        ]),
        ast::Expr::If(c, t, e) => ctor("If", vec![
            expr_to_value(c), expr_to_value(t), expr_to_value(e),
        ]),
        ast::Expr::Match(scrut, arms) => ctor("Match", vec![
            expr_to_value(scrut),
            list_of(arms.iter().map(arm_to_value).collect()),
        ]),
        ast::Expr::Let(rhss, body) => ctor("Let", vec![
            list_of(rhss.iter().map(expr_to_value).collect()),
            expr_to_value(body),
        ]),
    }
}

fn arm_to_value(a: &ast::Arm) -> ast::Expr {
    ctor("Arm", vec![pat_to_value(&a.pat), expr_to_value(&a.body)])
}

fn pat_to_value(p: &ast::Pat) -> ast::Expr {
    match p {
        ast::Pat::PVar => ctor("PVar", vec![]),
        ast::Pat::PCtor(n, sub) => ctor("PCtor", vec![
            ast::Expr::SymLit(n.clone()),
            list_of(sub.iter().map(pat_to_value).collect()),
        ]),
        ast::Pat::PInt(n) => ctor("PInt", vec![ast::Expr::IntLit(*n)]),
        ast::Pat::PSym(s) => ctor("PSym", vec![ast::Expr::SymLit(s.clone())]),
    }
}

fn list_of(items: Vec<ast::Expr>) -> ast::Expr {
    let mut acc = nil();
    for it in items.into_iter().rev() {
        acc = ctor("Cons", vec![it, acc]);
    }
    acc
}
