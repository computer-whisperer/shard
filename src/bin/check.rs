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
//!   (use-module "path/to/file.sexp")
//!     Load the named .sexp file as a user-defined module (types,
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
    let mut user_module = ast::Module {
        types: kernel.types.clone(),
        fns: Vec::new(),
        externs: Vec::new(),
    };
    let mut user_module_value = module_to_value(&user_module);

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
            match process_form(
                form, &kernel, &user_module_value, &theory, &path,
            ) {
                Outcome::Pass { name, goal } => {
                    println!("PASS  {}", name);
                    // Close param-name FVars in eq + premises to BVars
                    // so the stored Goal matches the kernel's
                    // convention for citation (resolve_eq + the
                    // Rewrite / RewriteWith arms open BVars to fresh
                    // FVars). Authors write FVar form in claim
                    // bodies because it's friendlier; the binary
                    // does the close. See REVISIT, "Open-vs-closed
                    // Goal forms".
                    let close_call = ast::Expr::Call(
                        "close_goal_for_storage".into(),
                        vec![goal],
                    );
                    let closed_goal = match eval::eval(&kernel, &close_call) {
                        Ok(v) => v,
                        Err(e) => {
                            eprintln!(
                                "error closing goal for `{}`: {:?}",
                                name, e,
                            );
                            return ExitCode::from(2);
                        }
                    };
                    let entry = ctor(
                        "Proven",
                        vec![ast::Expr::SymLit(name), closed_goal],
                    );
                    theory = ctor("TheoryCons", vec![entry, theory]);
                    passed += 1;
                }
                Outcome::Fail { name } => {
                    println!("FAIL  {}", name);
                    failed += 1;
                }
                Outcome::UseModule(rel_path) => {
                    // Resolve relative to the proof file's dir.
                    let resolved = match path.parent() {
                        Some(d) => d.join(&rel_path),
                        None    => PathBuf::from(&rel_path),
                    };
                    // Load the user module with the kernel as a ctor
                    // base, so user fns can reference stdlib types
                    // (List / Cons / Nil, Option / Some / None, …)
                    // without re-declaring them.
                    match load::module_from_paths_with_base(
                        &[&resolved], Some(&kernel),
                    ) {
                        Ok(loaded) => {
                            merge_module(&mut user_module, loaded);
                            user_module_value = module_to_value(&user_module);
                        }
                        Err(e) => {
                            eprintln!(
                                "error: {}: use-module {}: {}",
                                path.display(), resolved.display(), e,
                            );
                            return ExitCode::from(2);
                        }
                    }
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
    /// `(use-module "rel/path.sexp")` — caller resolves and loads.
    UseModule(String),
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
        "use-module" => {
            if items.len() != 2 {
                return Outcome::Fatal(format!(
                    "{}: use-module expects (use-module \"PATH\"), got {} arg(s)",
                    path.display(), items.len() - 1,
                ));
            }
            match items[1].as_str() {
                Some(s) => Outcome::UseModule(s.to_string()),
                None => Outcome::Fatal(format!(
                    "{}: use-module PATH must be a string literal, got {}",
                    path.display(), items[1],
                )),
            }
        }
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
            "{}: unknown top-level form `{}` (expected `claim`, `use-module`, or `module`)",
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
// The conversion is mechanical 1:1 with kernel/term.sexp's Expr/Pat
// and kernel/module.sexp's TypeDef/CtorDef/FnDef/ExternDef/Module
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
