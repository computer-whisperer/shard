//! `check` — proof-script driver for the narrow kernel.
//!
//! DEPRECATED orchestrator. The clean architecture is a thin Rust passthrough
//! (`eval` bin) running a shard entrypoint that does its own file I/O; the
//! eval path already moved there (`kernel/eval.shard`). This binary remains
//! the working proof-checker until the check entrypoint is likewise a shard
//! app run on top of the executor — at which point its bespoke orchestration
//! (run_shard_check, the Rust module gate, run/eval subcommands) retires.
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
//! Parsing is done by the SELF-HOSTED shard reader (kernel/reader.shard):
//! the module (types/fns/externs) AND each claim's GOAL/PROOF are parsed in
//! shard, against the kernel's ctor set, by the checker driver
//! (kernel/driver.shard's `check_production_src`). The Rust loader
//! (`load.rs`) is no longer on the target path — it survives only as the
//! bootstrap floor (loading the kernel + reader toolchain into the VM, since
//! the reader cannot parse itself) and as the reference oracle the
//! parse/module/claims differential harnesses check the reader against.
//!
//! Exit codes: 0 = all claims passed, 1 = some claim failed, 2 =
//! a load or eval error (no claim outcome could be determined).

use std::path::{Path, PathBuf};
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
        eprintln!("       check run <file.shard>... [-- <args>...]   (entry: main : World -> World)");
        return ExitCode::from(2);
    }

    // `eval` subcommand: run an object-language expression rather than
    // check proofs. By default it runs through the BOOTSTRAPPED narrow
    // reducer (kernel/reduce.shard's compute_expr) — the self-hosted
    // evaluator; `--no-bootstrap` uses the native Rust eval::eval instead.
    if args[0] == "eval" {
        return run_eval(&args[1..]);
    }

    // `run` subcommand: execute a direct-style world-threading program.
    // `main : World -> World` runs in the pure reducer; the driver fills the
    // World's input field from stdin and flushes its output field. See
    // run_program and examples/io/echo_world.shard.
    if args[0] == "run" {
        return run_program(&args[1..]);
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

    // `dsl-print` subcommand: render a file's REFLECTED claims/axioms/
    // requirements as proof-DSL surface text (kernel/dsl_print.shard). A
    // migration aid — splice the output into the file, replacing the reflected
    // forms, then `check` to confirm the rendering round-trips.
    if args[0] == "dsl-print" {
        return run_dsl_print(&args[1..]);
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

    // Pass A: imports (and the use-module alias) — recurse first. An import
    // may resolve to several files when it names a directory-module.
    for form in &forms {
        if let Some(dep) = import_path(form) {
            for resolved in resolve_import(path.parent(), &dep)? {
                process_file(&resolved, ctx, kernel)?;
            }
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

/// Resolve an import directive's path (relative to the importing file's
/// directory) into the concrete list of `.shard` files to load.
///
/// A path that names a DIRECTORY is a directory-MODULE: every `*.shard` file
/// in it is loaded as one unit, with `mod.req.shard` (the module's public
/// interface) placed LAST so its public claims may cite dir-mate private
/// members. A path that names a file (legacy `.shard` import) loads exactly
/// that file. The two are disambiguated by the directive itself: `(import
/// "order")` → the module dir `order/`, `(import "order.shard")` → the file.
fn resolve_import(base_dir: Option<&Path>, dep: &str) -> Result<Vec<PathBuf>, ExitCode> {
    let joined = match base_dir {
        Some(d) => d.join(dep),
        None => PathBuf::from(dep),
    };
    if !joined.is_dir() {
        return Ok(vec![joined]); // legacy single-file import
    }
    let mut files: Vec<PathBuf> = Vec::new();
    let entries = std::fs::read_dir(&joined).map_err(|e| {
        eprintln!("error reading module dir {}: {}", joined.display(), e);
        ExitCode::from(2)
    })?;
    for ent in entries {
        let p = ent
            .map_err(|e| { eprintln!("error reading module dir entry: {}", e); ExitCode::from(2) })?
            .path();
        if p.extension().and_then(|s| s.to_str()) == Some("shard") {
            files.push(p);
        }
    }
    // Deterministic: alphabetical, then a STABLE partition pushing
    // `mod.req.shard` to the end (its public lemmas may cite private members).
    files.sort();
    files.sort_by_key(|p| {
        p.file_name().and_then(|s| s.to_str()) == Some("mod.req.shard")
    });
    Ok(files)
}

/// Expand a check/run TARGET so a directory-module always loads as a unit,
/// no matter how it is named. A target that is a module directory, or a file
/// sitting inside one (a dir containing `mod.req.shard`), loads the whole
/// module (interface last); any other file loads just itself. This makes
/// `check std/nat`, `check std/nat/mod.req.shard`, and `check std/nat.shard`
/// (the shim) all check the same complete module.
fn expand_module_target(path: &Path) -> Result<Vec<PathBuf>, ExitCode> {
    if path.is_dir() {
        return resolve_import(None, &path.to_string_lossy());
    }
    if let Some(dir) = path.parent() {
        if dir.join("mod.req.shard").is_file() {
            return resolve_import(None, &dir.to_string_lossy());
        }
    }
    Ok(vec![path.to_path_buf()])
}

// ----------------------------------------------------------------------
// The module reference gate — a LOADING well-formedness check (sibling of
// the import-cycle check), NOT proof-checking. It enforces the one rule the
// directory-module system promises: a module may reference another module's
// PUBLIC interface only — never a private member. A directory is a governed
// module iff it contains `mod.req.shard`; that file is the public surface,
// the dir's other files are private. References to non-module ("legacy")
// code, and to a module's own members, are unrestricted.
// ----------------------------------------------------------------------

/// Every symbol token anywhere in `v` (recursively). Quoted citations like
/// `(Lemma 'half_step)` parse to `(Lemma (quote half_step))`, so the cited
/// name is collected too — that is exactly the cross-module reference we gate.
fn collect_symbols(v: &Value, out: &mut std::collections::HashSet<String>) {
    if let Some(s) = v.as_symbol() {
        out.insert(s.to_string());
        return;
    }
    if let Some(iter) = v.list_iter() {
        for e in iter {
            collect_symbols(e, out);
        }
    }
}

/// The top-level names a file DECLARES: fn/extern/claim/requirement names,
/// plus type names and their constructor names.
fn declared_names(forms: &[Value]) -> std::collections::HashSet<String> {
    let mut names = std::collections::HashSet::new();
    for form in forms {
        let items: Vec<&Value> = match form.list_iter() {
            Some(it) => it.collect(),
            None => continue,
        };
        if items.len() < 2 {
            continue;
        }
        let head = match items[0].as_symbol() {
            Some(h) => h,
            None => continue,
        };
        match head {
            "fn" | "extern" | "claim" | "requirement" => {
                if let Some(n) = items[1].as_symbol() {
                    names.insert(n.to_string());
                }
            }
            "type" => {
                if let Some(n) = items[1].as_symbol() {
                    names.insert(n.to_string());
                }
                // remaining items are constructor forms `(Ctor field-types...)`
                for ctor in &items[2..] {
                    if let Some(cn) = ctor.list_iter().and_then(|mut it| it.next()).and_then(|v| v.as_symbol()) {
                        names.insert(cn.to_string());
                    }
                }
            }
            _ => {}
        }
    }
    names
}

/// Reject any file in the load closure that references a PRIVATE member of a
/// module other than its own. Run after `ordered_closure`, before checking.
fn check_module_boundaries(order: &[PathBuf]) -> Result<(), ExitCode> {
    use std::collections::{HashMap, HashSet};
    struct FileInfo {
        dir: PathBuf,
        decls: HashSet<String>,
        symbols: HashSet<String>,
        is_interface: bool,
    }
    let mut files: Vec<(PathBuf, FileInfo)> = Vec::new();
    for p in order {
        let src = match std::fs::read_to_string(p) {
            Ok(s) => s,
            Err(_) => continue,
        };
        let forms = match parse_top_level(&src) {
            Ok(f) => f,
            Err(_) => continue,
        };
        let mut symbols = HashSet::new();
        for f in &forms {
            collect_symbols(f, &mut symbols);
        }
        // Canonicalize the owning dir so module identity is path-normalized
        // (e.g. `examples/../std/nat` and `std/nat` are the same module).
        let raw_dir = p.parent().map(|d| d.to_path_buf()).unwrap_or_default();
        let dir = raw_dir.canonicalize().unwrap_or(raw_dir);
        let info = FileInfo {
            dir,
            decls: declared_names(&forms),
            symbols,
            is_interface: p.file_name().and_then(|s| s.to_str()) == Some("mod.req.shard"),
        };
        files.push((p.clone(), info));
    }

    // Governed modules: dirs with a mod.req.shard among the loaded files.
    let governed: HashSet<PathBuf> =
        files.iter().filter(|(_, fi)| fi.is_interface).map(|(_, fi)| fi.dir.clone()).collect();

    // public(D) = names from D/mod.req.shard;  all(D) = names from any D file.
    let mut public: HashMap<PathBuf, HashSet<String>> = HashMap::new();
    let mut allnames: HashMap<PathBuf, HashSet<String>> = HashMap::new();
    for (_, fi) in &files {
        if !governed.contains(&fi.dir) {
            continue;
        }
        allnames.entry(fi.dir.clone()).or_default().extend(fi.decls.iter().cloned());
        if fi.is_interface {
            public.entry(fi.dir.clone()).or_default().extend(fi.decls.iter().cloned());
        }
    }
    // private(D) = all(D) − public(D).
    let private: HashMap<PathBuf, HashSet<String>> = governed.iter().map(|d| {
        let empty = HashSet::new();
        let all = allnames.get(d).unwrap_or(&empty);
        let pu = public.get(d).unwrap_or(&empty);
        (d.clone(), all.difference(pu).cloned().collect())
    }).collect();

    // A file may not name another module's private member. Display module
    // dirs relative to the working directory when possible.
    let cwd = std::env::current_dir().ok();
    let show = |d: &PathBuf| -> String {
        match &cwd {
            Some(c) => d.strip_prefix(c).unwrap_or(d).display().to_string(),
            None => d.display().to_string(),
        }
    };
    let mut violations: Vec<String> = Vec::new();
    for (path, fi) in &files {
        for (d, priv_set) in &private {
            if d == &fi.dir {
                continue; // own module — privates are in scope
            }
            for s in fi.symbols.intersection(priv_set) {
                violations.push(format!(
                    "  {}\n      references `{}`, a private member of module `{}` \
                     (not exported by its mod.req.shard)",
                    path.display(), s, show(d)));
            }
        }
    }
    if violations.is_empty() {
        return Ok(());
    }
    violations.sort();
    eprintln!("error: module boundary violation — a module may reference only the \
               PUBLIC interface of another module:");
    for v in &violations {
        eprintln!("{}", v);
    }
    Err(ExitCode::from(1))
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
                for resolved in resolve_import(path.parent(), &dep)? {
                    visit(&resolved, order, seen)?;
                }
            }
        }
        order.push(path.clone());
        Ok(())
    }
    let mut order = Vec::new();
    let mut seen = std::collections::HashSet::new();
    for t in targets {
        for f in expand_module_target(t)? {
            visit(&f, &mut order, &mut seen)?;
        }
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
    let mut positional: Vec<&String> = Vec::new();
    for a in args {
        match a.as_str() {
            "--no-bootstrap" | "--raw" | "--native" => raw = true,
            "--both" | "--compare" => both = true,
            // `--reflected` (raw Expr-datum input) is now subsumed: parse_expr
            // parses both surface syntax and raw Expr-ctor data. Accepted as a
            // no-op for back-compat.
            "--reflected" => {}
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
    let toolchain_vm = match load_toolchain_vm(&kernel) {
        Ok(m) => m,
        Err(code) => return code,
    };
    let user_module = match build_user_module_via_shard(&toolchain_vm, &kernel, files) {
        Ok(m) => m,
        Err(code) => return code,
    };
    let ctx = Ctx {
        user_module_value: module_to_value(&user_module),
        user_module,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
    };

    // Parse EXPR with the shard reader's `parse_expr`, using the kernel + user
    // ctor set so the program's ctors resolve. It returns the program as a
    // reflected Expr datum — that IS `reflected` (fed to compute_expr on the
    // bootstrap path); `value_to_native_expr` gives the executable `native`
    // form (for the Rust engine). Surface sugar (string literals, `'X`) and
    // raw Expr-ctor data both parse here.
    let ctor_names: Vec<String> = ctx.user_module.types.iter()
        .flat_map(|td| td.ctors.iter().map(|cd| cd.name.clone()))
        .collect();
    let pe_call = ast::Expr::Call(
        "parse_expr".into(),
        vec![line_to_event_native(expr_src), sym_list_native(&ctor_names)],
    );
    let reflected: ast::Expr = match eval::eval(&toolchain_vm, &pe_call) {
        Ok(ast::Expr::Ctor(n, a)) if n == "Some" && a.len() == 1 => a[0].clone(),
        Ok(ast::Expr::Ctor(n, _)) if n == "None" => {
            eprintln!("error: could not parse expression: {}", expr_src);
            return ExitCode::from(2);
        }
        Ok(other) => {
            eprintln!("error: parse_expr returned an unexpected value: {}", show_reflected(&other));
            return ExitCode::from(2);
        }
        Err(e) => {
            eprintln!("error parsing expression: {:?}", e);
            return ExitCode::from(2);
        }
    };
    let native: Result<ast::Expr, String> = value_to_native_expr(&reflected);

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
// entire pipeline — parse the target MODULE (types/fns) AND its claims/axioms,
// desugar named hyps, un-reflect goals/proofs, run check_sequent, thread the
// theory, admit axioms, and render the `--trace`/failure diagnostics — runs in
// shard (kernel/{reader,driver,trace}.shard), via the native VM exactly like
// the kernel's own check_sequent. The reduction module M is now built BY THE
// SHARD READER (check_production_src folds parse_module over the sources); the
// Rust loader no longer parses the targets. Rust's role is reduced to:
//   - resolving the file list (imports first, then targets) and reading bytes,
//   - loading the kernel + shard toolchain into the VM (the bootstrap floor),
//   - seeding M with the kernel's own types + ctor names,
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

    // The VM that runs the driver: kernel (types + fns) + the shard toolchain.
    let toolchain_vm = match load_toolchain_vm(kernel) {
        Ok(m) => m,
        Err(code) => return code,
    };

    // Ordered file list (imports first, then targets), and their sources.
    let order = match ordered_closure(&targets) {
        Ok(o) => o,
        Err(code) => return code,
    };
    // Loading well-formedness: no module reaches into another's privates.
    if let Err(code) = check_module_boundaries(&order) {
        return code;
    }
    let mut srcs_list = nil();
    for p in order.iter().rev() {
        match std::fs::read_to_string(p) {
            Ok(s) => { srcs_list = ctor("Cons", vec![line_to_event_native(&s), srcs_list]); }
            Err(e) => { eprintln!("error reading {}: {}", p.display(), e); return ExitCode::from(2); }
        }
    }

    // M (the module proofs reduce against) is built IN SHARD by check_production_src:
    // it folds the self-hosted reader's parse_module over `srcs` (dependency order)
    // and merges onto the kernel-types seed. load.rs no longer parses the targets —
    // it only loaded the kernel + toolchain into the VM (toolchain_vm) above.
    let seed = module_to_value(&ast::Module {
        types: kernel.types.clone(),
        fns: Vec::new(),
        externs: Vec::new(),
    });
    let kernel_ctor_names: Vec<String> = kernel.types.iter()
        .flat_map(|td| td.ctors.iter().map(|cd| cd.name.clone()))
        .collect();
    let base_ctors = sym_list_native(&kernel_ctor_names);

    // (check_production_src <seed> <base_ctors> <srcs> <trace-target>) → (List ClaimOutcome).
    let call = ast::Expr::Call(
        "check_production_src".into(),
        vec![seed, base_ctors, srcs_list, ast::Expr::SymLit(trace_target)],
    );
    let result = match eval_raw(&toolchain_vm, &call).and_then(|v| value_to_native_expr(&v)) {
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
// `dsl-print` — render a file's reflected claims/axioms/requirements as
// proof-DSL surface text. Builds M from the target's import closure (so
// desugar's IH counts resolve), then calls kernel/dsl_print.shard's
// dsl_print_src, which un-reflects each decl to native and prints the DSL.
// ----------------------------------------------------------------------
fn run_dsl_print(args: &[String]) -> ExitCode {
    if args.is_empty() {
        eprintln!("usage: check dsl-print <file.shard>");
        return ExitCode::from(2);
    }
    let kernel = match load_kernel_from(default_kernel_dir()) {
        Ok(m) => m,
        Err(e) => { eprintln!("error loading kernel: {}", e); return ExitCode::from(2); }
    };
    let toolchain_vm = match load_toolchain_vm(&kernel) {
        Ok(m) => m,
        Err(code) => return code,
    };
    let target_path = PathBuf::from(&args[0]);
    let order = match ordered_closure(&[target_path.clone()]) {
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
    let target_src = match std::fs::read_to_string(&target_path) {
        Ok(s) => line_to_event_native(&s),
        Err(e) => { eprintln!("error reading {}: {}", target_path.display(), e); return ExitCode::from(2); }
    };
    let seed = module_to_value(&ast::Module {
        types: kernel.types.clone(),
        fns: Vec::new(),
        externs: Vec::new(),
    });
    let kernel_ctor_names: Vec<String> = kernel.types.iter()
        .flat_map(|td| td.ctors.iter().map(|cd| cd.name.clone()))
        .collect();
    let base_ctors = sym_list_native(&kernel_ctor_names);
    let call = ast::Expr::Call(
        "dsl_print_src".into(),
        vec![seed, base_ctors, srcs_list, target_src],
    );
    let result = match eval_raw(&toolchain_vm, &call).and_then(|v| value_to_native_expr(&v)) {
        Ok(v) => v,
        Err(e) => { eprintln!("shard dsl_print error: {}", e); return ExitCode::from(2); }
    };
    let text: String = decode_list(&result).iter().filter_map(|x| match x {
        ast::Expr::IntLit(c) => char::from_u32(*c as u32),
        _ => None,
    }).collect();
    print!("{}", text);
    ExitCode::SUCCESS
}

// ----------------------------------------------------------------------
// Loading user code through the SHARD reader.
//
// `check`, `run`, and `eval` all parse user/target `.shard` code with the
// self-hosted reader (kernel/reader.shard) rather than the Rust loader.
// load.rs survives only as (a) the bootstrap floor — it parses the kernel
// and the reader toolchain itself into the VM, since the reader cannot
// parse itself — and (b) the reference oracle the parse/module/claims
// differential harnesses validate the shard reader against.
// ----------------------------------------------------------------------

/// Build a VM module with the kernel + the self-hosted reader toolchain
/// (reader/unreflect/desugar/trace/driver) loaded, so the shard loader
/// (`build_module`, `parse_expr`, `check_production_src`, …) can be
/// evaluated on the native engine. This is the BOOTSTRAP parser load (the
/// minimal Rust parsing needed to bring the shard front-end up).
fn load_toolchain_vm(kernel: &ast::Module) -> Result<ast::Module, ExitCode> {
    let mut ctx = Ctx {
        user_module_value: module_to_value(kernel),
        user_module: ast::Module {
            types: kernel.types.clone(),
            fns: kernel.fns.clone(),
            externs: kernel.externs.clone(),
        },
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
    };
    let kdir = default_kernel_dir();
    // The kernel base is already in `user_module` (load_kernel_from). Seed the
    // `loaded` set with those files so that now-declared kernel imports
    // (reader.shard → module.shard, etc.) are recognized as already-loaded and
    // not re-parsed/re-merged. (load_kernel_from uses module_from_paths, which
    // skips imports, so it never populated `loaded`.)
    for base in &["stdlib.shard", "module.shard", "proof.shard", "term.shard",
                  "reduce.shard", "checker.shard", "lia.shard", "eqdec.shard",
                  "ord.shard", "farkas.shard"] {
        let p = kdir.join(base);
        ctx.loaded.insert(p.canonicalize().unwrap_or(p));
    }
    for tool in &["reader.shard", "unreflect.shard", "desugar.shard", "trace.shard", "driver.shard",
                  "dsl_print.shard"] {   // dsl_print: the `dsl-print` migration subcommand's renderer
        process_file(&kdir.join(tool), &mut ctx, kernel)?;
    }
    Ok(ctx.user_module)
}

/// Parse user `.shard` file(s) (with transitive imports) into a native
/// `ast::Module` using the SHARD reader's `build_module` — not the Rust
/// loader. `toolchain_vm` must have the reader loaded (load_toolchain_vm).
/// The module is seeded with the kernel's types + ctor names so user fn
/// bodies resolve kernel ctors, exactly as the Rust loader's kernel base
/// did. The reader's reflected `Module` value is un-reflected to native so
/// the program can run on the engine (where the lazy-I/O handler lives).
fn build_user_module_via_shard(
    toolchain_vm: &ast::Module,
    kernel: &ast::Module,
    files: &[&String],
) -> Result<ast::Module, ExitCode> {
    let targets: Vec<PathBuf> = files.iter().map(|f| PathBuf::from(f.as_str())).collect();
    let order = ordered_closure(&targets)?;
    // Sources in dependency order (imports first), as a (List (List Int)).
    let mut srcs_list = nil();
    for p in order.iter().rev() {
        match std::fs::read_to_string(p) {
            Ok(s) => { srcs_list = ctor("Cons", vec![line_to_event_native(&s), srcs_list]); }
            Err(e) => { eprintln!("error reading {}: {}", p.display(), e); return Err(ExitCode::from(2)); }
        }
    }
    let seed = module_to_value(&ast::Module {
        types: kernel.types.clone(),
        fns: Vec::new(),
        externs: Vec::new(),
    });
    let kernel_ctor_names: Vec<String> = kernel.types.iter()
        .flat_map(|td| td.ctors.iter().map(|cd| cd.name.clone()))
        .collect();
    let base_ctors = sym_list_native(&kernel_ctor_names);
    // (build_module <srcs> <seed> <base_ctors>) → (Option Module).
    let call = ast::Expr::Call("build_module".into(), vec![srcs_list, seed, base_ctors]);
    let result = match eval::eval(toolchain_vm, &call) {
        Ok(v) => v,
        Err(e) => { eprintln!("shard build_module error: {:?}", e); return Err(ExitCode::from(2)); }
    };
    match &result {
        ast::Expr::Ctor(n, a) if n == "Some" && a.len() == 1 =>
            unreflect_module(&a[0]).map_err(|e| {
                eprintln!("error un-reflecting module from the shard reader: {}", e);
                ExitCode::from(2)
            }),
        ast::Expr::Ctor(n, _) if n == "None" => {
            eprintln!("error: the shard reader could not parse the target module(s)");
            Err(ExitCode::from(2))
        }
        other => {
            eprintln!("error: build_module returned an unexpected value: {}", show_reflected(other));
            Err(ExitCode::from(2))
        }
    }
}

// ----------------------------------------------------------------------
// `run` subcommand — execute a direct-style world-threading program.
//
// `check run <file.shard>...` parses the program through the SHARD reader
// (build_user_module_via_shard) and reduces `(main world0)` where `main :
// World -> World`. Two modes, by whether the program declares effectful
// externs:
//   - BATCHED (no externs): World = (World clock input output) carries I/O as
//     DATA; the driver slurps stdin into `input` and drains `output`, and
//     `main` is a PURE value the checker can prove about (echo_world.shard).
//   - LAZY (externs): World is a bare clock token; a run-time effect handler
//     performs the real I/O for each stuck extern mid-reduction (cat_lazy,
//     cat_loop, filecat, calc_repl, snake_app).
// Either way `main` runs on the NATIVE reducer — which is why the program is
// un-reflected to a native module after the shard reader parses it.
// ----------------------------------------------------------------------
fn run_program(args: &[String]) -> ExitCode {
    // `--` separates the program's SOURCE files from its runtime ARGUMENTS
    // (the latter readable in-program via the `get_args` extern).
    let dash = args.iter().position(|a| a == "--");
    let (file_args, prog_args): (&[String], &[String]) = match dash {
        Some(i) => (&args[..i], &args[i + 1..]),
        None => (args, &[]),
    };
    let mut positional: Vec<&String> = Vec::new();
    for a in file_args {
        if a.starts_with("--") {
            eprintln!("error: unknown flag {} (runtime args go after `--`)", a);
            return ExitCode::from(2);
        }
        positional.push(a);
    }
    if positional.is_empty() {
        eprintln!("usage: check run <file.shard>... [-- <args>...]   (entry: main : World -> World)");
        return ExitCode::from(2);
    }

    let kernel = match load_kernel_from(default_kernel_dir()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading kernel from {}: {}", default_kernel_dir().display(), e);
            return ExitCode::from(2);
        }
    };
    // Parse the user program through the SHARD reader (build_module), seeded
    // with the kernel's types. THEN append the kernel's fns as a FALLBACK:
    // lookup_fn returns the first match, so a user fn shadows a same-named
    // kernel fn (e.g. calc's helpers vs term.shard's), while a program that
    // calls a kernel fn it doesn't define (an eval app → `compute_expr`) still
    // resolves it. Affects only `run`; proof-checking uses its own M.
    let toolchain_vm = match load_toolchain_vm(&kernel) {
        Ok(m) => m,
        Err(code) => return code,
    };
    let mut user_module = match build_user_module_via_shard(&toolchain_vm, &kernel, &positional) {
        Ok(m) => m,
        Err(code) => return code,
    };
    user_module.fns.extend(kernel.fns.iter().cloned());
    user_module.externs.extend(kernel.externs.iter().cloned());
    let ctx = Ctx {
        user_module_value: module_to_value(&user_module),
        user_module,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
    };

    // Two run modes, chosen by whether the program declares effectful externs.
    if ctx.user_module.externs.is_empty() {
        return run_batched(&ctx);
    }
    run_lazy(&ctx, prog_args)
}

// BATCHED (pure) mode: no externs, so the World carries input/output as DATA.
// Slurp stdin into the input field, reduce the pure `main`, drain the output
// field. World = (World clock input output). See examples/io/echo_world.shard.
fn run_batched(ctx: &Ctx) -> ExitCode {
    use std::io::{Read, Write as _};
    let mut stdin_src = String::new();
    if std::io::stdin().read_to_string(&mut stdin_src).is_err() {
        eprintln!("error reading stdin");
        return ExitCode::from(2);
    }
    let input_lines: Vec<ast::Expr> = stdin_src.lines().map(line_to_event_native).collect();
    let world0 = ctor("World", vec![ast::Expr::IntLit(0), list_of(input_lines), nil()]);

    let call = ast::Expr::Call("main".into(), vec![world0]);
    let result = match eval::eval(&ctx.user_module, &call) {
        Ok(r) => r,
        Err(e) => { eprintln!("error running main: {:?}", e); return ExitCode::from(2); }
    };
    let output = match &result {
        ast::Expr::Ctor(n, a) if n == "World" && a.len() == 3 => &a[2],
        other => {
            eprintln!("error: main must return (World clock input output), got: {}",
                      show_reflected(other));
            return ExitCode::from(2);
        }
    };
    let mut out = std::io::stdout();
    for line in decode_list(output) {
        let s: String = decode_list(line).iter().filter_map(|x| match x {
            ast::Expr::IntLit(c) => char::from_u32(*c as u32),
            _ => None,
        }).collect();
        if writeln!(out, "{}", s).is_err() { return ExitCode::from(2); }
    }
    ExitCode::SUCCESS
}

// LAZY (effectful) mode: the program declares externs and does I/O
// mid-reduction. We install a run-time effect handler (the TRUSTED boundary —
// it must satisfy the program's extern axioms) that performs the real read /
// write for each stuck extern call; world0 is the bare clock token (World 0).
// The handler dispatches by extern name; the World (clock) is each call's LAST
// argument and is returned bumped by 1, matching the `*_ticks` bridging axioms.
// See examples/io/cat_lazy.shard and eval::set_effect_handler.
fn run_lazy(ctx: &Ctx, prog_args: &[String]) -> ExitCode {
    use std::io::{BufRead, Read, Write as _};
    let mut reader = std::io::BufReader::new(std::io::stdin());
    let prog_args: Vec<String> = prog_args.to_vec();
    // Key-at-a-time programs (those declaring `read_key`) run with the terminal
    // in raw mode (no line buffering / echo); we restore it on exit and on
    // normal completion. Non-key programs leave the terminal alone.
    let raw = ctx.user_module.externs.iter().any(|e| e.name == "read_key");
    let saved: Option<String> = if raw {
        let s = stty_capture(&["-g"]);
        stty_apply(&["-icanon", "-echo", "-isig", "min", "1", "time", "0"]);
        s
    } else {
        None
    };
    let saved_h = saved.clone();
    // decode a (List Int) argument to a String.
    let text = |e: &ast::Expr| -> String {
        decode_list(e).iter().filter_map(|x| match x {
            ast::Expr::IntLit(c) => char::from_u32(*c as u32),
            _ => None,
        }).collect()
    };
    let handler = move |name: &str, args: &[ast::Expr]| -> Result<ast::Expr, String> {
        let clk = match args.last() {
            Some(ast::Expr::Ctor(n, a)) if n == "World" && a.len() == 1 => match &a[0] {
                ast::Expr::IntLit(k) => *k,
                other => return Err(format!("World clock is not an Int: {:?}", other)),
            },
            _ => return Err(format!("{}: expected a World as the last argument", name)),
        };
        let world1 = ctor("World", vec![ast::Expr::IntLit(clk + 1)]);
        match name {
            // --- input ---------------------------------------------------------
            "read_line" => {
                let mut buf = String::new();
                match reader.read_line(&mut buf) {
                    Ok(0) => Ok(ctor("Pair", vec![ctor("None", vec![]), world1])),
                    Ok(_) => {
                        let line = buf.trim_end_matches(|c| c == '\n' || c == '\r');
                        Ok(ctor("Pair", vec![ctor("Some", vec![line_to_event_native(line)]), world1]))
                    }
                    Err(e) => Err(format!("read_line: {}", e)),
                }
            }
            // oracle_line: input oracle returning just the (Option line), no World
            // (the program advances the clock itself via `tick`).
            "oracle_line" => {
                let mut buf = String::new();
                match reader.read_line(&mut buf) {
                    Ok(0) => Ok(ctor("None", vec![])),
                    Ok(_) => {
                        let line = buf.trim_end_matches(|c| c == '\n' || c == '\r');
                        Ok(ctor("Some", vec![line_to_event_native(line)]))
                    }
                    Err(e) => Err(format!("oracle_line: {}", e)),
                }
            }
            // get_args: the runtime arguments (after `--`) as (List (List Int)).
            "get_args" => {
                let items: Vec<ast::Expr> = prog_args.iter().map(|a| line_to_event_native(a)).collect();
                Ok(ctor("Pair", vec![list_of(items), world1]))
            }
            // read_file: file contents as (Some bytes), or None on any error.
            "read_file" => {
                match std::fs::read_to_string(text(&args[0])) {
                    Ok(contents) => Ok(ctor("Pair",
                        vec![ctor("Some", vec![line_to_event_native(&contents)]), world1])),
                    Err(_) => Ok(ctor("Pair", vec![ctor("None", vec![]), world1])),
                }
            }
            // read_key: one raw keypress as (Some byte), or None at EOF. The
            // terminal is already in raw mode (see above), so this returns per
            // key without Enter.
            "read_key" => {
                let mut b = [0u8; 1];
                match reader.read(&mut b) {
                    Ok(0) => Ok(ctor("Pair", vec![ctor("None", vec![]), world1])),
                    Ok(_) => Ok(ctor("Pair",
                        vec![ctor("Some", vec![ast::Expr::IntLit(b[0] as i64)]), world1])),
                    Err(e) => Err(format!("read_key: {}", e)),
                }
            }
            // --- output --------------------------------------------------------
            "write_line" | "emit" => { println!("{}", text(&args[0])); Ok(world1) }
            "write" => {
                print!("{}", text(&args[0]));
                let _ = std::io::stdout().flush();
                Ok(world1)
            }
            // --- termination ---------------------------------------------------
            "exit" => {
                let _ = std::io::stdout().flush();
                if raw {
                    match &saved_h {
                        Some(g) => stty_apply(&[g.as_str()]),
                        None => stty_apply(&["sane"]),
                    }
                }
                let code = match &args[0] { ast::Expr::IntLit(n) => *n, _ => 0 };
                std::process::exit(code as i32);
            }
            other => Err(format!("unknown extern `{}` (handler: read_line/oracle_line/get_args/\
                                  read_file/write_line/write/emit/exit)", other)),
        }
    };
    eval::set_effect_handler(Some(Box::new(handler)));
    let call = ast::Expr::Call("main".into(), vec![ctor("World", vec![ast::Expr::IntLit(0)])]);
    let result = eval::eval(&ctx.user_module, &call);
    eval::set_effect_handler(None);
    if raw {
        match &saved {
            Some(g) => stty_apply(&[g.as_str()]),
            None => stty_apply(&["sane"]),
        }
    }
    match result {
        Ok(_) => ExitCode::SUCCESS,
        Err(e) => { eprintln!("error running main: {:?}", e); ExitCode::from(2) }
    }
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

/// An input line → the reflected-NOTHING native `(List Int)` of its
/// codepoints (the Event). Reflected to Expr-data by the caller.
fn line_to_event_native(line: &str) -> ast::Expr {
    let mut e = nil();
    for ch in line.chars().rev() {
        e = ctor("Cons", vec![ast::Expr::IntLit(ch as u32 as i64), e]);
    }
    e
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
        ("Match", [scrut, arms]) => Ok(ast::Expr::Match(
            Box::new(value_to_native_expr(scrut)?),
            decode_list(arms).iter().map(|a| unreflect_arm(a)).collect::<Result<_, _>>()?,
        )),
        ("Let", [rhss, body]) => Ok(ast::Expr::Let(
            unreflect_list(rhss)?,
            Box::new(value_to_native_expr(body)?),
        )),
        _ => Err(format!("cannot un-reflect Expr node: {:?}", e)),
    }
}

// ----------------------------------------------------------------------
// Reflected datum → native AST un-reflectors.
//
// These are the structural inverse of the `*_to_value` reflectors below
// (module_to_value / expr_to_value / …). They turn the Module VALUE the
// self-hosted reader produces (a Ctor tree: `(Module (Cons (TypeDef …) …)
// …)`) back into a native `ast::Module` the engine can execute. This is
// the bridge that lets `run`/`eval` load user code through the shard
// reader (`build_module`) yet still run `main` on the native VM — where
// the lazy-I/O effect handler lives. Mechanical and 1:1 with
// kernel/{term,module}.shard; if those evolve, this follows (the
// `module-check` differential harness keeps the reader honest, and these
// invert the same shapes module_to_value emits).
// ----------------------------------------------------------------------

fn unreflect_arm(e: &ast::Expr) -> Result<ast::Arm, String> {
    match e {
        ast::Expr::Ctor(n, a) if n == "Arm" && a.len() == 2 =>
            Ok(ast::Arm { pat: unreflect_pat(&a[0])?, body: value_to_native_expr(&a[1])? }),
        other => Err(format!("expected an Arm datum, got: {:?}", other)),
    }
}

fn unreflect_pat(e: &ast::Expr) -> Result<ast::Pat, String> {
    let (name, args) = match e {
        ast::Expr::Ctor(n, a) => (n.as_str(), a.as_slice()),
        other => return Err(format!("expected a Pat datum, got: {:?}", other)),
    };
    match (name, args) {
        ("PVar", []) => Ok(ast::Pat::PVar),
        ("PInt", [ast::Expr::IntLit(n)]) => Ok(ast::Pat::PInt(*n)),
        ("PSym", [ast::Expr::SymLit(s)]) => Ok(ast::Pat::PSym(s.clone())),
        ("PCtor", [ast::Expr::SymLit(n), sub]) => Ok(ast::Pat::PCtor(
            n.clone(),
            decode_list(sub).iter().map(|p| unreflect_pat(p)).collect::<Result<_, _>>()?,
        )),
        _ => Err(format!("cannot un-reflect Pat node: {:?}", e)),
    }
}

fn unreflect_type(e: &ast::Expr) -> Result<ast::Type, String> {
    let (name, args) = match e {
        ast::Expr::Ctor(n, a) => (n.as_str(), a.as_slice()),
        other => return Err(format!("expected a Type datum, got: {:?}", other)),
    };
    match (name, args) {
        ("TVar", [ast::Expr::SymLit(s)]) => Ok(ast::Type::TVar(s.clone())),
        ("TCon", [ast::Expr::SymLit(n), l]) => Ok(ast::Type::TCon(
            n.clone(),
            decode_list(l).iter().map(|t| unreflect_type(t)).collect::<Result<_, _>>()?,
        )),
        _ => Err(format!("cannot un-reflect Type node: {:?}", e)),
    }
}

fn unreflect_sym(e: &ast::Expr) -> Result<ast::Symbol, String> {
    match e {
        ast::Expr::SymLit(s) => Ok(s.clone()),
        other => Err(format!("expected a Symbol, got: {:?}", other)),
    }
}

fn unreflect_ctordef(e: &ast::Expr) -> Result<ast::CtorDef, String> {
    match e {
        ast::Expr::Ctor(n, a) if n == "CtorDef" && a.len() == 2 => Ok(ast::CtorDef {
            name: unreflect_sym(&a[0])?,
            fields: decode_list(&a[1]).iter().map(|t| unreflect_type(t)).collect::<Result<_, _>>()?,
        }),
        other => Err(format!("expected a CtorDef datum, got: {:?}", other)),
    }
}

fn unreflect_typedef(e: &ast::Expr) -> Result<ast::TypeDef, String> {
    match e {
        ast::Expr::Ctor(n, a) if n == "TypeDef" && a.len() == 3 => Ok(ast::TypeDef {
            name: unreflect_sym(&a[0])?,
            params: decode_list(&a[1]).iter().map(|s| unreflect_sym(s)).collect::<Result<_, _>>()?,
            ctors: decode_list(&a[2]).iter().map(|c| unreflect_ctordef(c)).collect::<Result<_, _>>()?,
        }),
        other => Err(format!("expected a TypeDef datum, got: {:?}", other)),
    }
}

fn unreflect_fndef(e: &ast::Expr) -> Result<ast::FnDef, String> {
    match e {
        ast::Expr::Ctor(n, a) if n == "FnDef" && a.len() == 4 => Ok(ast::FnDef {
            name: unreflect_sym(&a[0])?,
            params: decode_list(&a[1]).iter().map(|t| unreflect_type(t)).collect::<Result<_, _>>()?,
            ret: unreflect_type(&a[2])?,
            body: value_to_native_expr(&a[3])?,
        }),
        other => Err(format!("expected a FnDef datum, got: {:?}", other)),
    }
}

fn unreflect_externdef(e: &ast::Expr) -> Result<ast::ExternDef, String> {
    match e {
        ast::Expr::Ctor(n, a) if n == "ExternDef" && a.len() == 3 => Ok(ast::ExternDef {
            name: unreflect_sym(&a[0])?,
            params: decode_list(&a[1]).iter().map(|t| unreflect_type(t)).collect::<Result<_, _>>()?,
            ret: unreflect_type(&a[2])?,
        }),
        other => Err(format!("expected an ExternDef datum, got: {:?}", other)),
    }
}

fn unreflect_module(e: &ast::Expr) -> Result<ast::Module, String> {
    match e {
        ast::Expr::Ctor(n, a) if n == "Module" && a.len() == 3 => Ok(ast::Module {
            types: decode_list(&a[0]).iter().map(|t| unreflect_typedef(t)).collect::<Result<_, _>>()?,
            fns: decode_list(&a[1]).iter().map(|f| unreflect_fndef(f)).collect::<Result<_, _>>()?,
            externs: decode_list(&a[2]).iter().map(|x| unreflect_externdef(x)).collect::<Result<_, _>>()?,
        }),
        other => Err(format!("expected a Module datum, got: {:?}", other)),
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
