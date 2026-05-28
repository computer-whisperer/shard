//! `check` — proof-script driver for the narrow kernel.
//!
//! Loads the bundled kernel, then walks one or more user-provided
//! `.sexp` proof files. Top-level forms in those files:
//!
//!   (claim NAME GOAL PROOF)
//!     Run `check_sequent` on a fresh Sequent lifted from GOAL
//!     (hyps = Nil). If True, the claim is added to the running
//!     Theory as `(Proven NAME GOAL)`, available to subsequent
//!     claims via `(Lemma NAME)` citations.
//!
//!   (module NAME)
//!     v1: parsed but not implemented. Reserved for v2 directory-
//!     tree loading. Errors at this slice — see docs/REVISIT.md,
//!     "Proof-file module syntax".
//!
//! GOAL and PROOF are parsed as narrow expressions against the
//! kernel's ctor set (so `(Goal …)`, `(Refl)`, `(ByTheory …)` etc.
//! resolve as `Ctor`s) and then evaluated to runtime values. This
//! avoids inventing a new sexp-to-value protocol — the kernel's
//! existing ctor application IS the value-construction syntax.
//!
//! User-module slot: currently always `(Module Nil Nil Nil)` — the
//! claim file cannot yet declare its own user fns. Means v1 claims
//! are practically limited to LIA-decidable goals and goals over
//! primitive ops. Adding `(use-module …)` is a follow-up slice.
//!
//! Exit codes: 0 = all claims passed, 1 = some claim failed, 2 =
//! a load or eval error (no claim outcome could be determined).

use std::path::PathBuf;
use std::process::ExitCode;

use lexpr::Value;
use lexpr::parse::Parser;
use proving_bootstrap_v2::{ast, default_kernel_dir, eval, load, load_kernel_from};

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    if args.is_empty() {
        eprintln!("usage: check <proof_file.sexp>...");
        return ExitCode::from(2);
    }

    let kernel = match load_kernel_from(default_kernel_dir()) {
        Ok(m) => m,
        Err(e) => {
            eprintln!("error loading kernel from {}: {}",
                default_kernel_dir().display(), e);
            return ExitCode::from(2);
        }
    };

    // User module is empty for v1: no user fns, no user types beyond
    // the kernel's. Adequate for LIA claims over primitives.
    let user_module = ctor("Module", vec![nil(), nil(), nil()]);

    let mut theory = ctor("TheoryEmpty", vec![]);
    let mut passed = 0usize;
    let mut failed = 0usize;

    for path_str in &args {
        let path = PathBuf::from(path_str);
        let src = match std::fs::read_to_string(&path) {
            Ok(s) => s,
            Err(e) => {
                eprintln!("error reading {}: {}", path.display(), e);
                return ExitCode::from(2);
            }
        };

        let forms = match parse_top_level(&src) {
            Ok(fs) => fs,
            Err(e) => {
                eprintln!("error parsing {}: {}", path.display(), e);
                return ExitCode::from(2);
            }
        };

        for form in &forms {
            match process_form(form, &kernel, &user_module, &theory, &path) {
                Outcome::Pass { name, goal } => {
                    println!("PASS  {}", name);
                    // Cons onto the running theory so later claims can
                    // cite this one via (Lemma NAME).
                    let entry = ctor(
                        "Proven",
                        vec![ast::Expr::SymLit(name), goal],
                    );
                    theory = ctor("TheoryCons", vec![entry, theory]);
                    passed += 1;
                }
                Outcome::Fail { name } => {
                    println!("FAIL  {}", name);
                    failed += 1;
                }
                Outcome::Fatal(msg) => {
                    eprintln!("error: {}", msg);
                    return ExitCode::from(2);
                }
            }
        }
    }

    println!();
    println!("{} passed, {} failed", passed, failed);
    if failed > 0 { ExitCode::from(1) } else { ExitCode::SUCCESS }
}

// ----------------------------------------------------------------------
// Form processing
// ----------------------------------------------------------------------

enum Outcome {
    Pass { name: String, goal: ast::Expr },
    Fail { name: String },
    Fatal(String),
}

fn process_form(
    form: &Value,
    kernel: &ast::Module,
    user_module: &ast::Expr,
    theory: &ast::Expr,
    path: &PathBuf,
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
        "claim" => process_claim(&items, kernel, user_module, theory, path),
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
            "{}: unknown top-level form `{}` (expected `claim` or `module`)",
            path.display(), other,
        )),
    }
}

fn process_claim(
    items: &[&Value],
    kernel: &ast::Module,
    user_module: &ast::Expr,
    theory: &ast::Expr,
    path: &PathBuf,
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

    // Parse and evaluate PROOF.
    let proof_val = match build_value(items[3], kernel) {
        Ok(v) => v,
        Err(e) => return Outcome::Fatal(format!(
            "{}: claim `{}` proof: {}", path.display(), name, e,
        )),
    };

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

    // Invoke check_sequent in the kernel module.
    let call = ast::Expr::Call(
        "check_sequent".into(),
        vec![user_module.clone(), theory.clone(), sequent_val, proof_val],
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
        ast::Expr::Ctor(ref n, ref a) if n == "False" && a.is_empty() =>
            Outcome::Fail { name },
        other => Outcome::Fatal(format!(
            "{}: claim `{}`: check_sequent returned non-Bool value: {:?}",
            path.display(), name, other,
        )),
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
