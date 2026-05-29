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
//!   (axiom NAME GOAL)
//!     Admit GOAL into the Theory as `(Axiom NAME GOAL)` WITHOUT a
//!     proof — citable as `(Lemma NAME)` exactly like a proven claim.
//!     A trusted audit boundary (docs/BOUNDARIES.md): use only for
//!     facts about the runtime primitives that cannot be derived
//!     in-kernel (e.g. the Euclidean identity of `div`/`mod`).
//!     Reported as `AXIOM <name>` and tallied separately.
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
    };

    for path_str in &args {
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
// A .sexp file may mix object-level code (`type`/`fn`/`extern`),
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
            merge_module(&mut ctx.user_module, loaded);
            ctx.user_module_value = module_to_value(&ctx.user_module);
        }
        Err(e) => {
            eprintln!("error: {}: loading module: {}", path.display(), e);
            return Err(ExitCode::from(2));
        }
    }

    // Pass C: claims, in order.
    for form in &forms {
        match process_form(form, kernel, &ctx.user_module_value, &ctx.theory, path) {
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
        "axiom" => process_axiom(&items, kernel, path),
        // Handled in earlier passes: imports/aliases (A), code defs (B).
        "import" | "use-module" | "type" | "fn" | "extern" => Outcome::Skip,
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
    trace_proof(kernel, m, theory, sequent, proof, 0, &mut lines);
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
    sequent: &ast::Expr, proof: &ast::Expr, depth: usize, lines: &mut Vec<String>,
) {
    let ind = "  ".repeat(depth);
    let (head, pa) = match proof {
        ast::Expr::Ctor(n, a) => (n.as_str(), a),
        _ => { lines.push(format!("{}proof: {}", ind, render_term(proof))); return; }
    };
    match head {
        "Steps" if pa.len() == 2 => {
            let call = ast::Expr::Call("apply_steps".into(),
                vec![m.clone(), theory.clone(), sequent.clone(), pa[0].clone()]);
            match eval::eval(kernel, &call) {
                Ok(ref s) if ctor_fields(s, "Some", 1).is_some() => {
                    let seq2 = ctor_fields(s, "Some", 1).unwrap()[0].clone();
                    if let Some(eq) = sequent_eq(&seq2) {
                        lines.push(format!("{}after steps:  {}", ind, render_term(eq)));
                    }
                    trace_proof(kernel, m, theory, &seq2, &pa[1], depth, lines);
                }
                Ok(ref s) if ctor_fields(s, "None", 0).is_some() =>
                    lines.push(format!("{}a step failed to apply (Unfold/Reduce/Rewrite found no match)", ind)),
                _ => lines.push(format!("{}steps: could not replay", ind)),
            }
        }
        "RewriteWith" if pa.len() == 6 => {
            match apply_rewrite_step(kernel, theory, sequent, &pa[0], &pa[1], &pa[2], &pa[3]) {
                Some(seq2) => {
                    if let Some(eq) = sequent_eq(&seq2) {
                        lines.push(format!("{}after rewrite ({}):  {}", ind, render_eqref(&pa[0]), render_term(eq)));
                    }
                    trace_proof(kernel, m, theory, &seq2, &pa[5], depth + 1, lines);
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
        "Induct" | "Induct2" | "CaseOn" =>
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
