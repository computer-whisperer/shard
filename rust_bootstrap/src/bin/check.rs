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

    // Running user module: starts with the kernel's type declarations
    // (stdlib List/Option/Bool/Pair + the kernel's own internal types
    // like Expr/Pat/Goal/Proof — which user proofs may reason about
    // meta-theoretically). Grows as (use-module …) forms are
    // processed.
    //
    // Seeding with the kernel's types is what lets do_induct's
    // `lookup_typedef` find List in (Induct 'xs …) on a user fn
    // over (List Int). Without this seed, only types declared in
    // (use-module …) files would be visible to the kernel — forcing
    // each user module to redeclare stdlib types just to induct over
    // them.
    let user_module = ast::Module {
        types: kernel.types.clone(),
        fns: Vec::new(),
        externs: Vec::new(),
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

    let mut ctx = Ctx {
        user_module_value: module_to_value(&user_module),
        user_module,
        theory: ctor("TheoryEmpty", vec![]),
        passed: 0,
        failed: 0,
        axioms: 0,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
        load_only: false,
        trace_target,
    };

    for path_str in &files {
        if let Err(code) = process_file(&PathBuf::from(path_str), &mut ctx, &kernel) {
            return code;
        }
    }

    println!();
    if ctx.axioms > 0 {
        println!("{} passed, {} failed, {} axiom(s) admitted without proof",
            ctx.passed, ctx.failed, ctx.axioms);
    } else {
        println!("{} passed, {} failed", ctx.passed, ctx.failed);
    }
    if ctx.failed > 0 { ExitCode::from(1) } else { ExitCode::SUCCESS }
}

// ----------------------------------------------------------------------
// Recursive file loader with transitive imports.
//
// A .shard file may mix object-level code (`type`/`fn`/`extern`),
// dependency directives (`import` / its legacy alias `use-module`), and
// proofs (`claim`). One file = one topic. Each file is processed in
// three passes so dependencies are in scope before use:
//   A. imports — recurse into each dependency FIRST (depth-first), so its
//      code + proven claims land in the shared module/theory.
//   B. code — load THIS file's types/fns/externs (now that imports are
//      visible as the ctor/fn base).
//   C. claims — check each, threading the growing theory.
// Dedup + cycle detection are by CANONICAL path: a file imported by
// several others is loaded once; an import cycle is a hard error.
// `import` re-checks the dependency's claims (the project's "decided not
// assumed" stance); memoization keeps that to once per invocation.
// ----------------------------------------------------------------------

struct Ctx {
    user_module: ast::Module,
    user_module_value: ast::Expr,
    theory: ast::Expr,
    passed: usize,
    failed: usize,
    axioms: usize,
    loaded: std::collections::HashSet<PathBuf>,
    in_progress: Vec<PathBuf>,
    /// When set, `process_file` loads code + imports but skips claim/axiom
    /// checking (Pass C). Used by the `eval` subcommand: it needs the
    /// module's fns in scope but not the (potentially slow) proof replay.
    load_only: bool,
    /// `--trace <name>`: print the per-step sequent trace for the claim of
    /// this name (or "all"), regardless of pass/fail. None = no tracing.
    trace_target: Option<String>,
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

    // load-only (eval subcommand): code + imports are now in scope; skip
    // claim/axiom checking entirely.
    if ctx.load_only {
        ctx.in_progress.pop();
        ctx.loaded.insert(canon);
        return Ok(());
    }

    // Pass C: claims, in order.
    for form in &forms {
        match process_form(form, kernel, &ctx.user_module_value, &ctx.theory, path, ctx.trace_target.as_deref()) {
            Outcome::Pass { name, goal } => {
                println!("PASS  {}", name);
                // Close param-name FVars in eq + premises to BVars so the
                // stored Goal matches the kernel's citation convention
                // (resolve_eq opens BVars to fresh FVars). See REVISIT,
                // "Open-vs-closed Goal forms".
                let close_call = ast::Expr::Call(
                    "close_goal_for_storage".into(),
                    vec![goal],
                );
                let closed_goal = match eval::eval(kernel, &close_call) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("error closing goal for `{}`: {:?}", name, e);
                        return Err(ExitCode::from(2));
                    }
                };
                let entry = ctor("Proven", vec![ast::Expr::SymLit(name), closed_goal]);
                ctx.theory = ctor("TheoryCons", vec![entry, ctx.theory.clone()]);
                ctx.passed += 1;
            }
            Outcome::Axiom { name, goal } => {
                // Admitted without proof — added to the theory as an
                // `(Axiom NAME GOAL)` entry, citable exactly like a proven
                // lemma. Same goal-closing as a claim so citations resolve.
                println!("AXIOM {}  (admitted without proof)", name);
                let close_call = ast::Expr::Call(
                    "close_goal_for_storage".into(),
                    vec![goal],
                );
                let closed_goal = match eval::eval(kernel, &close_call) {
                    Ok(v) => v,
                    Err(e) => {
                        eprintln!("error closing axiom goal for `{}`: {:?}", name, e);
                        return Err(ExitCode::from(2));
                    }
                };
                let entry = ctor("Axiom", vec![ast::Expr::SymLit(name), closed_goal]);
                ctx.theory = ctor("TheoryCons", vec![entry, ctx.theory.clone()]);
                ctx.axioms += 1;
            }
            Outcome::Fail { name, detail } => {
                println!("FAIL  {}", name);
                if !detail.is_empty() {
                    println!("{}", detail);
                }
                ctx.failed += 1;
            }
            Outcome::Skip => {}
            Outcome::Fatal(msg) => {
                eprintln!("error: {}", msg);
                return Err(ExitCode::from(2));
            }
        }
    }

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

// ----------------------------------------------------------------------
// Form processing
// ----------------------------------------------------------------------

enum Outcome {
    Pass { name: String, goal: ast::Expr },
    /// An `(axiom NAME GOAL)` — admitted into the theory WITHOUT a proof.
    /// This is a trusted audit boundary (see docs/BOUNDARIES.md): the
    /// GOAL becomes a citable `(Lemma NAME)` on the author's word alone.
    /// Reported loudly and counted separately so axioms are never silent.
    Axiom { name: String, goal: ast::Expr },
    Fail { name: String, detail: String },
    /// A non-claim form handled by an earlier pass (import / use-module
    /// in pass A; type / fn / extern in pass B). Ignored in pass C.
    Skip,
    Fatal(String),
}

fn process_form(
    form: &Value,
    kernel: &ast::Module,
    user_module: &ast::Expr,
    theory: &ast::Expr,
    path: &PathBuf,
    trace: Option<&str>,
) -> Outcome {
    let items: Vec<&Value> = match form.list_iter() {
        Some(it) => it.collect(),
        None => return Outcome::Fatal(format!(
            "{}: top-level form must be a list, got {}", path.display(), form,
        )),
    };
    if items.is_empty() {
        return Outcome::Fatal(format!("{}: empty top-level form", path.display()));
    }
    let head = match items[0].as_symbol() {
        Some(s) => s,
        None => return Outcome::Fatal(format!(
            "{}: top-level form head must be a symbol", path.display(),
        )),
    };
    match head {
        "claim" => process_claim(&items, kernel, user_module, theory, path, trace),
        "axiom" => process_axiom(&items, kernel, path),
        // Handled in earlier passes: imports/aliases (A), code defs (B).
        // `app` is the `check app` entrypoint declaration — inert to the
        // proof checker, like a code def.
        "import" | "use-module" | "type" | "fn" | "extern" | "app" | "cli" => Outcome::Skip,
        "module" => {
            let name = items.get(1).and_then(|v| v.as_symbol()).unwrap_or("?");
            Outcome::Fatal(format!(
                "{}: (module {}) — not yet implemented in v1. \
                 Module directives are reserved for the directory-tree \
                 loader (a later slice); see docs/REVISIT.md.",
                path.display(), name,
            ))
        }
        other => Outcome::Fatal(format!(
            "{}: unknown top-level form `{}` \
             (expected `claim`, `axiom`, `import`, `type`, `fn`, `extern`, or `use-module`)",
            path.display(), other,
        )),
    }
}

/// `(axiom NAME GOAL)` — admit GOAL into the theory WITHOUT a proof. The
/// GOAL is parsed/evaluated exactly like a claim's (so it must be a valid
/// `(Goal …)` value and resolves identically under `(Lemma NAME)`), but no
/// `check_sequent` runs. This is the project's trusted audit boundary for
/// facts that hold of the runtime primitives but can't be derived in-kernel
/// (e.g. the Euclidean identity of `div`/`mod`). Kept deliberately small and
/// loud: the only thing that distinguishes it from `(claim …)` is the absent
/// proof and the `Axiom` (vs `Proven`) theory tag.
fn process_axiom(
    items: &[&Value],
    kernel: &ast::Module,
    path: &PathBuf,
) -> Outcome {
    if items.len() != 3 {
        return Outcome::Fatal(format!(
            "{}: axiom expects (axiom NAME GOAL), got {} arg(s) after `axiom`",
            path.display(), items.len() - 1,
        ));
    }
    let name = match items[1].as_symbol() {
        Some(s) => s.to_string(),
        None => return Outcome::Fatal(format!(
            "{}: axiom NAME must be a symbol", path.display(),
        )),
    };
    let goal_val = match build_value(items[2], kernel) {
        Ok(v) => v,
        Err(e) => return Outcome::Fatal(format!(
            "{}: axiom `{}` goal: {}", path.display(), name, e,
        )),
    };
    // Sanity: it must actually evaluate to a (Goal params premises eq).
    match &goal_val {
        ast::Expr::Ctor(n, args) if n == "Goal" && args.len() == 3 => {}
        other => return Outcome::Fatal(format!(
            "{}: axiom `{}` did not evaluate to a Goal value: {:?}",
            path.display(), name, other,
        )),
    }
    Outcome::Axiom { name, goal: goal_val }
}

fn process_claim(
    items: &[&Value],
    kernel: &ast::Module,
    user_module: &ast::Expr,
    theory: &ast::Expr,
    path: &PathBuf,
    trace: Option<&str>,
) -> Outcome {
    if items.len() != 4 {
        return Outcome::Fatal(format!(
            "{}: claim expects (claim NAME GOAL PROOF), got {} arg(s) after `claim`",
            path.display(), items.len() - 1,
        ));
    }
    let name = match items[1].as_symbol() {
        Some(s) => s.to_string(),
        None => return Outcome::Fatal(format!(
            "{}: claim NAME must be a symbol", path.display(),
        )),
    };

    // Parse and evaluate GOAL.
    let goal_val = match build_value(items[2], kernel) {
        Ok(v) => v,
        Err(e) => return Outcome::Fatal(format!(
            "{}: claim `{}` goal: {}", path.display(), name, e,
        )),
    };

    // Desugar named hypotheses — (Hyp 'label) → (Hyp k) — and strip case-hyp
    // name annotations, mirroring the kernel's hyp-prepend order. Pure loader
    // sugar: the kernel only ever sees positional, name-free proofs.
    let desugared = {
        let ctx = HCtx { stack: Vec::new(), params: goal_params(items[2]) };
        match desugar_hyps(items[3], &ctx, kernel, user_module) {
            Ok(v) => v,
            Err(e) => return Outcome::Fatal(format!(
                "{}: claim `{}` — hypothesis name: {}", path.display(), name, e)),
        }
    };
    let proof_val = match build_value(&desugared, kernel) {
        Ok(v) => v,
        Err(e) => return Outcome::Fatal(format!(
            "{}: claim `{}` proof (after hyp desugar): {}", path.display(), name, e)),
    };

    // Structural validation of the proof tree BEFORE check_sequent, so a
    // malformed shape (e.g. a RewriteWith dropped into a Steps step-list)
    // is reported with a path instead of an opaque runtime failure.
    if let Err(e) = validate_proof(&proof_val, &format!("claim '{}'", name)) {
        return Outcome::Fatal(format!(
            "{}: claim `{}` — malformed proof: {}", path.display(), name, e,
        ));
    }

    // Lift Goal → Sequent (hyps = Nil).
    let sequent_val = match goal_val.clone() {
        ast::Expr::Ctor(n, args) if n == "Goal" && args.len() == 3 => {
            ctor(
                "Sequent",
                vec![
                    args[0].clone(), // params
                    nil(),           // hyps (empty for top-level claims)
                    args[1].clone(), // premises
                    args[2].clone(), // equation
                ],
            )
        }
        other => return Outcome::Fatal(format!(
            "{}: claim `{}` goal did not evaluate to a Goal value: {:?}",
            path.display(), name, other,
        )),
    };

    // --trace: print the per-step sequent evolution for this claim (pass or
    // fail), so the author can watch the goal transform instead of guessing.
    if matches!(trace, Some(t) if t == name || t == "all") {
        let mut tlines = Vec::new();
        trace_proof(kernel, user_module, theory, &sequent_val, &proof_val, 0, true, &mut tlines);
        println!("── trace: {} ──", name);
        if let Some(eq) = sequent_eq(&sequent_val) {
            println!("  goal      {}", render_term(eq));
        }
        for l in &tlines { println!("{}", l); }
        println!("──");
    }

    // Invoke check_sequent in the kernel module. (clone the sequent /
    // proof so a FAIL can replay them for diagnostics.)
    let call = ast::Expr::Call(
        "check_sequent".into(),
        vec![user_module.clone(), theory.clone(), sequent_val.clone(), proof_val.clone()],
    );
    let result = match eval::eval(kernel, &call) {
        Ok(v) => v,
        Err(e) => return Outcome::Fatal(format!(
            "{}: claim `{}`: check_sequent crashed: {:?}",
            path.display(), name, e,
        )),
    };

    match result {
        ast::Expr::Ctor(ref n, ref a) if n == "True" && a.is_empty() =>
            Outcome::Pass { name, goal: goal_val },
        ast::Expr::Ctor(ref n, ref a) if n == "False" && a.is_empty() => {
            let detail = build_failure_detail(
                kernel, user_module, theory, &sequent_val, &proof_val,
            );
            Outcome::Fail { name, detail }
        }
        other => Outcome::Fatal(format!(
            "{}: claim `{}`: check_sequent returned non-Bool value: {:?}",
            path.display(), name, other,
        )),
    }
}

// ----------------------------------------------------------------------
// Named-hypothesis desugaring (UNTRUSTED, loader sugar — runs before the
// proof ever reaches the kernel). Rewrites `(Hyp 'label)` → `(Hyp k)` and
// strips case-hyp name annotations, by simulating the kernel's hyp stack:
//   CaseOn case   : prepends 1 hyp, named via `(Case C 'h …)` / `(CaseB C (fs) 'h …)`
//   WfInduct      : prepends 1 IH, auto-named `ih`
//   Induct/Induct2: APPENDS one IH per recursive field (do_induct order),
//                   auto-named `ih`, `ih1`, …
// Positional `(Hyp k)` is untouched; names are opt-in. The kernel only ever
// sees positional, name-free proofs, so the trusted core is unchanged.
// ----------------------------------------------------------------------

#[derive(Clone)]
struct HCtx {
    /// Hyp names by position; index 0 = front = `(Hyp 0)`. None = unnamed.
    stack: Vec<Option<String>>,
    /// Induct-able vars → their `(ty …)` value, for IH counting.
    params: std::collections::HashMap<String, Value>,
}

/// A bare symbol or `(quote sym)` → the symbol text. (lexpr reads `'x` as
/// `(quote x)`.)
fn read_sym(v: &Value) -> Option<String> {
    if let Some(s) = v.as_symbol() {
        return Some(s.to_string());
    }
    let items: Vec<&Value> = v.list_iter()?.collect();
    if items.len() == 2 && items[0].as_symbol() == Some("quote") {
        return items[1].as_symbol().map(|s| s.to_string());
    }
    None
}

/// Elements of a `(list …)` form (without the leading `list`).
fn list_elems<'a>(v: &'a Value) -> Result<Vec<&'a Value>, String> {
    let items: Vec<&Value> = v.list_iter().ok_or("expected a (list …)")?.collect();
    match items.split_first() {
        Some((h, rest)) if h.as_symbol() == Some("list") => Ok(rest.to_vec()),
        _ => Err("expected a (list …)".into()),
    }
}

fn goal_params(goal: &Value) -> std::collections::HashMap<String, Value> {
    let mut m = std::collections::HashMap::new();
    let items: Vec<&Value> = match goal.list_iter() { Some(i) => i.collect(), None => return m };
    if items.len() < 2 || items[0].as_symbol() != Some("Goal") { return m; }
    if let Ok(params) = list_elems(items[1]) {
        for p in params {
            let pit: Vec<&Value> = match p.list_iter() { Some(i) => i.collect(), None => continue };
            if pit.len() == 3 && pit[0].as_symbol() == Some("Param") {
                if let Some(nm) = read_sym(pit[1]) {
                    m.insert(nm, pit[2].clone());
                }
            }
        }
    }
    m
}

fn in_scope_names(ctx: &HCtx) -> String {
    let ns: Vec<&str> = ctx.stack.iter().filter_map(|x| x.as_deref()).collect();
    if ns.is_empty() { "(none)".into() } else { ns.join(", ") }
}

/// How many IHs `do_induct` appends for ctor `cname` when inducting on `var`.
fn ih_count(ctx: &HCtx, var: &str, cname: &str,
            kernel: &ast::Module, module_val: &ast::Expr) -> usize {
    let ty_val = match ctx.params.get(var) { Some(t) => t, None => return 0 };
    let ty_expr = match build_value(ty_val, kernel) { Ok(e) => e, Err(_) => return 0 };
    let call = ast::Expr::Call("dbg_ih_count".into(),
        vec![ty_expr, ast::Expr::SymLit(cname.to_string()), module_val.clone()]);
    match eval::eval(kernel, &call) {
        Ok(ast::Expr::IntLit(n)) if n >= 0 => n as usize,
        _ => 0,
    }
}

fn ih_names(n: usize) -> Vec<Option<String>> {
    (0..n).map(|i| Some(if i == 0 { "ih".into() } else { format!("ih{}", i) })).collect()
}

fn desugar_hyps(v: &Value, ctx: &HCtx,
                kernel: &ast::Module, module_val: &ast::Expr) -> Result<Value, String> {
    let items: Vec<&Value> = match v.list_iter() {
        Some(it) => it.collect(),
        None => return Ok(v.clone()), // atom
    };
    let head = match items.first().and_then(|h| h.as_symbol()) {
        Some(h) => h,
        None => return rebuild(&items, ctx, kernel, module_val),
    };
    match head {
        "Hyp" if items.len() == 2 => {
            if let Some(name) = read_sym(items[1]) {
                match ctx.stack.iter().position(|x| x.as_deref() == Some(name.as_str())) {
                    Some(k) => Ok(Value::list(vec![Value::symbol("Hyp"), Value::from(k as i64)])),
                    None => Err(format!("unbound name '{}' in (Hyp '{}) — in scope: {}",
                        name, name, in_scope_names(ctx))),
                }
            } else {
                Ok(v.clone()) // positional (Hyp k)
            }
        }
        "WfInduct" if items.len() == 3 => {
            let measure = desugar_hyps(items[1], ctx, kernel, module_val)?;
            let mut c2 = ctx.clone();
            c2.stack.insert(0, Some("ih".into()));
            let proof = desugar_hyps(items[2], &c2, kernel, module_val)?;
            Ok(Value::list(vec![Value::symbol("WfInduct"), measure, proof]))
        }
        "CaseOn" if items.len() == 4 => {
            let scrut = desugar_hyps(items[1], ctx, kernel, module_val)?;
            let mut out = vec![Value::symbol("list")];
            for c in list_elems(items[3])? {
                out.push(desugar_case(c, ctx, None, kernel, module_val)?);
            }
            Ok(Value::list(vec![Value::symbol("CaseOn"), scrut, items[2].clone(), Value::list(out)]))
        }
        "Induct" | "Induct2" if items.len() == 3 => {
            let var = read_sym(items[1]).unwrap_or_default();
            let is2 = head == "Induct2";
            let mut out = vec![Value::symbol("list")];
            for c in list_elems(items[2])? {
                out.push(desugar_case(c, ctx, Some((&var, is2)), kernel, module_val)?);
            }
            Ok(Value::list(vec![Value::symbol(head), items[1].clone(), Value::list(out)]))
        }
        // Everything else (Steps, RewriteWith, Rewrite, Goal, Call, …) leaves
        // the hyp stack unchanged; recurse uniformly so nested (Hyp 'x) and
        // nested binders are still handled.
        _ => rebuild(&items, ctx, kernel, module_val),
    }
}

fn rebuild(items: &[&Value], ctx: &HCtx,
           kernel: &ast::Module, module_val: &ast::Expr) -> Result<Value, String> {
    let mut out = Vec::with_capacity(items.len());
    for it in items {
        out.push(desugar_hyps(it, ctx, kernel, module_val)?);
    }
    Ok(Value::list(out))
}

/// Desugar one `Case`/`CaseB`. `induct` is `Some((var, is_induct2))` when the
/// case is under Induct/Induct2 (IH-appending, no case-hyp), else `None`
/// (under CaseOn — prepends one optionally-named case hyp). Strips any name
/// annotation from the rebuilt case.
fn desugar_case(c: &Value, ctx: &HCtx, induct: Option<(&str, bool)>,
                kernel: &ast::Module, module_val: &ast::Expr) -> Result<Value, String> {
    let items: Vec<&Value> = c.list_iter().ok_or("expected a Case/CaseB")?.collect();
    let head = items.first().and_then(|h| h.as_symbol()).ok_or("malformed case")?;
    // Parse: (Case C [name] proof) | (CaseB C (fs) [name] proof). `prefix` is
    // the rebuilt case head sans name; `name` is the optional case-hyp label.
    let (prefix, name, proof): (Vec<Value>, Option<String>, &Value) = match head {
        "Case" if items.len() == 3 =>
            (vec![Value::symbol("Case"), items[1].clone()], None, items[2]),
        "Case" if items.len() == 4 && read_sym(items[2]).is_some() =>
            (vec![Value::symbol("Case"), items[1].clone()], read_sym(items[2]), items[3]),
        "CaseB" if items.len() == 4 =>
            (vec![Value::symbol("CaseB"), items[1].clone(), items[2].clone()], None, items[3]),
        "CaseB" if items.len() == 5 && read_sym(items[3]).is_some() =>
            (vec![Value::symbol("CaseB"), items[1].clone(), items[2].clone()], read_sym(items[3]), items[4]),
        _ => return Err(format!("malformed `{}` case (wrong arity / annotation)", head)),
    };
    let mut c2 = ctx.clone();
    match induct {
        None => {
            // CaseOn: one prepended case hyp (named or anonymous).
            c2.stack.insert(0, name);
        }
        Some((var, is2)) => {
            if name.is_some() {
                return Err("Induct/Induct2 cases take no hyp name (they bind `ih`, not a case hyp)".into());
            }
            let ctor = read_sym(items[1]).unwrap_or_default();
            let n = if is2 { if ctor == "SS" { 1 } else { 0 } }
                    else { ih_count(ctx, var, &ctor, kernel, module_val) };
            for nm in ih_names(n) { c2.stack.push(nm); } // appended (do_induct order)
        }
    }
    let proof2 = desugar_hyps(proof, &c2, kernel, module_val)?;
    let mut out = prefix;
    out.push(proof2);
    Ok(Value::list(out))
}

// ----------------------------------------------------------------------
// Load-time proof-structure validation (UNTRUSTED, off the check path).
// Walks the Proof / Step / Case grammar (arities mirror kernel/proof.shard)
// and returns the first structural error with a path into the tree. This
// turns class-of-bug authoring mistakes — a Proof node used where a Step
// is expected, or a wrong field count — into a clear load-time message
// rather than a deep, opaque runtime "could not replay".
// ----------------------------------------------------------------------

/// Field count for each Proof constructor, or None if `n` isn't one.
fn proof_arity(n: &str) -> Option<usize> {
    Some(match n {
        "Refl" => 0, "Steps" => 2, "Induct" => 2, "Induct2" => 2,
        "CaseOn" => 3, "WfInduct" => 2, "RewriteWith" => 6,
        "Absurd" => 1, "ByTheory" => 2,
        _ => return None,
    })
}

/// Field count for each Step constructor, or None if `n` isn't one.
fn step_arity(n: &str) -> Option<usize> {
    Some(match n {
        "Unfold" => 2, "Reduce" => 1, "Simp" => 1, "Compute" => 1, "Rewrite" => 5,
        _ => return None,
    })
}

fn validate_proof(p: &ast::Expr, path: &str) -> Result<(), String> {
    let (n, a) = match p {
        ast::Expr::Ctor(n, a) => (n.as_str(), a),
        other => return Err(format!("at {}: expected a Proof, found {}", path, render_term(other))),
    };
    let arity = match proof_arity(n) {
        Some(k) => k,
        None => {
            let hint = if step_arity(n).is_some() {
                format!(" — `{}` is a Step, not a Proof", n)
            } else { String::new() };
            return Err(format!("at {}: `{}` is not a Proof constructor{}", path, n, hint));
        }
    };
    if a.len() != arity {
        return Err(format!("at {}: `{}` takes {} field(s), got {}", path, n, arity, a.len()));
    }
    match n {
        "Steps" => {
            for (i, s) in decode_list(&a[0]).iter().enumerate() {
                validate_step(s, &format!("{} → Steps step #{}", path, i))?;
            }
            validate_proof(&a[1], &format!("{} → Steps tail", path))
        }
        "Induct" | "Induct2" => validate_cases(&a[1], &format!("{} → {}", path, n)),
        "CaseOn" => validate_cases(&a[2], &format!("{} → CaseOn", path)),
        "WfInduct" => validate_proof(&a[1], &format!("{} → WfInduct", path)),
        "RewriteWith" => {
            for (i, pp) in decode_list(&a[4]).iter().enumerate() {
                validate_proof(pp, &format!("{} → RewriteWith premise-proof #{}", path, i))?;
            }
            validate_proof(&a[5], &format!("{} → RewriteWith tail", path))
        }
        _ => Ok(()), // Refl / Absurd / ByTheory: no sub-proofs to walk
    }
}

fn validate_step(s: &ast::Expr, path: &str) -> Result<(), String> {
    let (n, a) = match s {
        ast::Expr::Ctor(n, a) => (n.as_str(), a),
        other => return Err(format!("at {}: expected a Step, found {}", path, render_term(other))),
    };
    match step_arity(n) {
        Some(k) if a.len() == k => Ok(()),
        Some(k) => Err(format!("at {}: step `{}` takes {} field(s), got {}", path, n, k, a.len())),
        None => {
            let hint = if proof_arity(n).is_some() {
                format!(" — `{}` is a Proof, not a Step; nest it as the Steps' tail (2nd field), \
                         not as an element of the step-list", n)
            } else { String::new() };
            Err(format!("at {}: `{}` is not a Step constructor{}", path, n, hint))
        }
    }
}

fn validate_cases(cases: &ast::Expr, path: &str) -> Result<(), String> {
    for (i, c) in decode_list(cases).iter().enumerate() {
        let (n, a) = match c {
            ast::Expr::Ctor(n, a) => (n.as_str(), a),
            other => return Err(format!("at {} (case #{}): expected Case/CaseB, found {}",
                path, i, render_term(other))),
        };
        let proof = match (n, a.len()) {
            ("Case", 2) => &a[1],
            ("CaseB", 3) => &a[2],
            ("Case", k) | ("CaseB", k) =>
                return Err(format!("at {} (case #{}): `{}` takes {} field(s), got {}",
                    path, i, n, if n == "Case" { 2 } else { 3 }, k)),
            _ => return Err(format!("at {} (case #{}): `{}` is not Case/CaseB", path, i, n)),
        };
        let label = sym_of(&a[0]);
        validate_proof(proof, &format!("{} (case '{})", path, label))?;
    }
    Ok(())
}

// ----------------------------------------------------------------------
// Failure diagnostics (UNTRUSTED, off the check path). On a FAIL we
// re-read the goal and — for a `Steps`-headed proof — replay the steps
// via the kernel's own `apply_steps` to show the equation as the final
// `Refl` saw it. No new trusted code: this only renders values and
// calls existing kernel fns, purely for the author's benefit.
// ----------------------------------------------------------------------

/// Render an Expr-ADT VALUE (`Ctor("Call",[SymLit "read", <list>])` …)
/// back to readable surface syntax (`(read m p)`).
fn render_term(v: &ast::Expr) -> String {
    use ast::Expr::*;
    match v {
        Ctor(name, a) => match name.as_str() {
            "FVar" if a.len() == 1 => sym_of(&a[0]),
            "BVar" if a.len() == 1 => format!("@{}", render_term(&a[0])),
            "IntLit" if a.len() == 1 => render_term(&a[0]),
            "SymLit" if a.len() == 1 => format!("'{}", sym_of(&a[0])),
            "Call" | "Ctor" if a.len() == 2 => {
                let head = sym_of(&a[0]);
                let items = decode_list(&a[1]);
                if items.is_empty() {
                    head
                } else {
                    let rs: Vec<String> = items.iter().map(|x| render_term(x)).collect();
                    format!("({} {})", head, rs.join(" "))
                }
            }
            "If" if a.len() == 3 => format!(
                "(if {} {} {})",
                render_term(&a[0]), render_term(&a[1]), render_term(&a[2]),
            ),
            "Equation" if a.len() == 2 =>
                format!("{}  =  {}", render_term(&a[0]), render_term(&a[1])),
            other => format!("<{}>", other),
        },
        SymLit(s) => s.clone(),
        IntLit(n) => n.to_string(),
        other => format!("{:?}", other),
    }
}

fn sym_of(v: &ast::Expr) -> String {
    match v {
        ast::Expr::SymLit(s) => s.clone(),
        other => render_term(other),
    }
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

/// Build the indented multi-line diagnostic for a failed claim: the goal
/// (with any premises), and — if the proof is `Steps` — the equation
/// after the steps ran (what the trailing proof / `Refl` had to close).
fn build_failure_detail(
    kernel: &ast::Module,
    m: &ast::Expr,
    theory: &ast::Expr,
    sequent: &ast::Expr,
    proof: &ast::Expr,
) -> String {
    let mut lines: Vec<String> = Vec::new();
    // Sequent = (Sequent params hyps premises eq)
    if let ast::Expr::Ctor(n, sa) = sequent {
        if n == "Sequent" && sa.len() == 4 {
            let premises = decode_list(&sa[2]);
            for p in &premises {
                lines.push(format!("given:  {}", render_term(p)));
            }
            lines.push(format!("goal:   {}", render_term(&sa[3])));
        }
    }
    // Walk the proof spine, replaying each level via the kernel's own
    // fns, to show the equation at every nesting depth and pinpoint
    // where it diverges. Branching proofs (Induct/CaseOn) stop the walk.
    trace_proof(kernel, m, theory, sequent, proof, 0, false, &mut lines);
    lines.iter().map(|l| format!("      {}", l)).collect::<Vec<_>>().join("\n")
}

// --- proof-spine tracer (UNTRUSTED diagnostics) ----------------------
// Descends the non-branching spine of a Proof — Steps and RewriteWith —
// replaying each via the kernel's own fns and rendering the equation at
// each level, so a deep nested failure is visible (not just the
// outermost finisher's entry state). Refl reports whether it closes;
// branching/decision proofs stop the walk. Adds NO trusted code: only
// orchestrates existing kernel fns and renders values.

fn ctor_fields<'a>(v: &'a ast::Expr, name: &str, n: usize) -> Option<&'a [ast::Expr]> {
    match v {
        ast::Expr::Ctor(cn, a) if cn == name && a.len() == n => Some(a),
        _ => None,
    }
}

fn sequent_eq(v: &ast::Expr) -> Option<&ast::Expr> {
    ctor_fields(v, "Sequent", 4).map(|a| &a[3])
}

fn render_eqref(v: &ast::Expr) -> String {
    match v {
        ast::Expr::Ctor(n, a) if a.len() == 1 => match n.as_str() {
            "Lemma" => format!("lemma {}", sym_of(&a[0])),
            "Premise" => format!("premise {}", render_term(&a[0])),
            "Hyp" => format!("hyp {}", render_term(&a[0])),
            _ => render_term(v),
        },
        _ => render_term(v),
    }
}

/// A short human label for one Step (for the per-step trace).
fn step_label(s: &ast::Expr) -> String {
    let side = |v: &ast::Expr| match v {
        ast::Expr::Ctor(n, _) => n.clone(),
        _ => render_term(v),
    };
    match s {
        ast::Expr::Ctor(n, a) => match (n.as_str(), a.as_slice()) {
            ("Unfold", [sym, sd]) => format!("Unfold {} {}", render_term(sym), side(sd)),
            ("Simp", [sd]) => format!("Simp {}", side(sd)),
            ("Reduce", [sd]) => format!("Reduce {}", side(sd)),
            ("Compute", [sd]) => format!("Compute {}", side(sd)),
            ("Rewrite", [er, dir, sd, ..]) =>
                format!("Rewrite {} {} {}", render_eqref(er), side(dir), side(sd)),
            _ => n.clone(),
        },
        _ => render_term(s),
    }
}

/// Replay one RewriteWith's rewrite (ignoring premise sub-proofs) to get
/// the resulting sequent, mirroring check_sequent's RewriteWith arm by
/// chaining the same kernel fns. None if any link fails (unresolved
/// lemma, no match, …).
fn apply_rewrite_step(
    kernel: &ast::Module, theory: &ast::Expr, sequent: &ast::Expr,
    eqref: &ast::Expr, dir: &ast::Expr, side: &ast::Expr, insts: &ast::Expr,
) -> Option<ast::Expr> {
    let k = |name: &str, args: Vec<ast::Expr>| {
        eval::eval(kernel, &ast::Expr::Call(name.into(), args)).ok()
    };
    let some_inner = |v: &ast::Expr| ctor_fields(v, "Some", 1).map(|a| a[0].clone());

    let g = some_inner(&k("resolve_eq", vec![eqref.clone(), sequent.clone(), theory.clone()])?)?;
    let gf = ctor_fields(&g, "Goal", 3)?;            // Goal params premises eq
    let (cited_params, cited_eq) = (gf[0].clone(), gf[2].clone());

    let pair = k("split_params_by_insts", vec![cited_params, insts.clone()])?;
    let pf = ctor_fields(&pair, "Pair", 2)?;          // Pair openings pat_var_names
    let (openings, patvars) = (pf[0].clone(), pf[1].clone());

    let ofvars = k("reverse_exprs", vec![openings])?;
    let opened_eq = k("open_eq_with", vec![ofvars, cited_eq])?;

    let sf = ctor_fields(sequent, "Sequent", 4)?;     // Sequent params hyps premises eq
    let (params, hyps, premises, goal_eq) =
        (sf[0].clone(), sf[1].clone(), sf[2].clone(), sf[3].clone());

    let r = k("apply_rewrite_with_env",
        vec![patvars, opened_eq, dir.clone(), side.clone(), goal_eq])?;
    let r = some_inner(&r)?;                           // Pair env new_eq
    let new_eq = ctor_fields(&r, "Pair", 2)?[1].clone();

    Some(ctor("Sequent", vec![params, hyps, premises, new_eq]))
}

fn trace_proof(
    kernel: &ast::Module, m: &ast::Expr, theory: &ast::Expr,
    sequent: &ast::Expr, proof: &ast::Expr, depth: usize, verbose: bool, lines: &mut Vec<String>,
) {
    let ind = "  ".repeat(depth);
    let (head, pa) = match proof {
        ast::Expr::Ctor(n, a) => (n.as_str(), a),
        _ => { lines.push(format!("{}proof: {}", ind, render_term(proof))); return; }
    };
    match head {
        "Steps" if pa.len() == 2 => {
            // Apply steps ONE at a time so the trace shows the sequent after
            // each — and a failure pinpoints exactly which step broke.
            let steps = decode_list(&pa[0]);
            let mut cur = sequent.clone();
            let mut ok = true;
            for (i, step) in steps.iter().enumerate() {
                let call = ast::Expr::Call("apply_step".into(),
                    vec![m.clone(), theory.clone(), cur.clone(), (*step).clone()]);
                match eval::eval(kernel, &call) {
                    Ok(ref s) if ctor_fields(s, "Some", 1).is_some() => {
                        cur = ctor_fields(s, "Some", 1).unwrap()[0].clone();
                        if let Some(eq) = sequent_eq(&cur) {
                            lines.push(format!("{}step {} [{}]  {}", ind, i + 1, step_label(step), render_term(eq)));
                        }
                    }
                    Ok(ref s) if ctor_fields(s, "None", 0).is_some() => {
                        lines.push(format!("{}step {} [{}]  FAILED to apply (no match)", ind, i + 1, step_label(step)));
                        ok = false;
                        break;
                    }
                    _ => {
                        lines.push(format!("{}step {} [{}]  could not replay", ind, i + 1, step_label(step)));
                        ok = false;
                        break;
                    }
                }
            }
            if ok {
                trace_proof(kernel, m, theory, &cur, &pa[1], depth, verbose, lines);
            }
        }
        "RewriteWith" if pa.len() == 6 => {
            match apply_rewrite_step(kernel, theory, sequent, &pa[0], &pa[1], &pa[2], &pa[3]) {
                Some(seq2) => {
                    if let Some(eq) = sequent_eq(&seq2) {
                        lines.push(format!("{}after rewrite ({}):  {}", ind, render_eqref(&pa[0]), render_term(eq)));
                    }
                    trace_proof(kernel, m, theory, &seq2, &pa[5], depth + 1, verbose, lines);
                }
                None => lines.push(format!(
                    "{}rewrite ({}) did NOT apply (unresolved lemma, no match, or premise mismatch)",
                    ind, render_eqref(&pa[0]))),
            }
        }
        "Refl" => match sequent_eq(sequent) {
            Some(ast::Expr::Ctor(en, ea)) if en == "Equation" && ea.len() == 2 => {
                if ea[0] == ea[1] {
                    lines.push(format!("{}Refl: closes (lhs = rhs)", ind));
                } else {
                    lines.push(format!("{}Refl: does NOT close — lhs ≠ rhs:", ind));
                    lines.push(format!("{}  lhs  {}", ind, render_term(&ea[0])));
                    lines.push(format!("{}  rhs  {}", ind, render_term(&ea[1])));
                }
            }
            _ => lines.push(format!("{}Refl", ind)),
        },
        "ByTheory" => lines.push(format!("{}ByTheory {} — decision procedure (not replayed)",
            ind, pa.get(0).map(render_term).unwrap_or_default())),
        "WfInduct" if pa.len() == 2 => {
            let call = ast::Expr::Call("dbg_wf_subgoal".into(),
                vec![sequent.clone(), pa[0].clone()]);
            match eval::eval(kernel, &call) {
                Ok(subgoal) => {
                    lines.push(format!("{}WfInduct subgoal (IH at Hyp 0):", ind));
                    trace_proof(kernel, m, theory, &subgoal, &pa[1], depth + 1, verbose, lines);
                }
                _ => lines.push(format!("{}WfInduct — could not rebuild subgoal", ind)),
            }
        }
        "CaseOn" if pa.len() == 3 => {
            for case in decode_list(&pa[2]) {
                let (cname, names, sub): (ast::Expr, ast::Expr, &ast::Expr) =
                    if let Some(a) = ctor_fields(case, "Case", 2) {
                        (a[0].clone(), nil(), &a[1])
                    } else if let Some(a) = ctor_fields(case, "CaseB", 3) {
                        (a[0].clone(), a[1].clone(), &a[2])
                    } else { continue; };
                let sg = ast::Expr::Call("dbg_caseon_subgoal".into(), vec![
                    m.clone(), sequent.clone(), pa[0].clone(), pa[1].clone(),
                    cname.clone(), names.clone()]);
                let subgoal = match eval::eval(kernel, &sg) {
                    Ok(ref s) if ctor_fields(s, "Some", 1).is_some() =>
                        ctor_fields(s, "Some", 1).unwrap()[0].clone(),
                    _ => { lines.push(format!("{}case {}: could not rebuild subgoal",
                            ind, render_term(&cname))); continue; }
                };
                let chk = ast::Expr::Call("check_sequent".into(),
                    vec![m.clone(), theory.clone(), subgoal.clone(), sub.clone()]);
                let passes = matches!(eval::eval(kernel, &chk),
                    Ok(ast::Expr::Ctor(ref n, ref a)) if n == "True" && a.is_empty());
                if passes && !verbose {
                    lines.push(format!("{}case {} = {}: ok", ind,
                        render_term(&pa[0]), render_term(&cname)));
                } else {
                    let tag = if passes { "" } else { "  FAILS  v v v" };
                    lines.push(format!("{}case {} = {}:{}", ind,
                        render_term(&pa[0]), render_term(&cname), tag));
                    trace_proof(kernel, m, theory, &subgoal, sub, depth + 1, verbose, lines);
                }
            }
        }
        "Induct" | "Induct2" | "CaseOn" | "WfInduct" =>
            lines.push(format!("{}{} — branching proof; trace stops here", ind, head)),
        other => lines.push(format!("{}{} — not replayed", ind, other)),
    }
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
        theory: ctor("TheoryEmpty", vec![]),
        passed: 0,
        failed: 0,
        axioms: 0,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
        load_only: true,
        trace_target: None,
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
        theory: ctor("TheoryEmpty", vec![]),
        passed: 0,
        failed: 0,
        axioms: 0,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
        load_only: true,
        trace_target: None,
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
        theory: ctor("TheoryEmpty", vec![]),
        passed: 0,
        failed: 0,
        axioms: 0,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
        load_only: true,
        trace_target: None,
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
        theory: ctor("TheoryEmpty", vec![]),
        passed: 0,
        failed: 0,
        axioms: 0,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
        load_only: true,
        trace_target: None,
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
        theory: ctor("TheoryEmpty", vec![]),
        passed: 0,
        failed: 0,
        axioms: 0,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
        load_only: true,
        trace_target: None,
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
        theory: ctor("TheoryEmpty", vec![]),
        passed: 0,
        failed: 0,
        axioms: 0,
        loaded: std::collections::HashSet::new(),
        in_progress: Vec::new(),
        load_only: true,
        trace_target: None,
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
