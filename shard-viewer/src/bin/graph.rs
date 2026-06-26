//! `shard-graph` — a text dump of the extracted project model.
//!
//! No GUI: this exercises the parser + call-graph extraction against a real
//! shard tree so the model can be verified before any rendering exists.
//!
//!   shard-graph [PROJECT_ROOT]          summary of files / fns / calls
//!   shard-graph [PROJECT_ROOT] FN_NAME  callers + callees of one fn

use shard_viewer::model::Project;
use std::path::Path;

fn main() -> std::io::Result<()> {
    let mut args = std::env::args().skip(1);
    let root = args.next().unwrap_or_else(|| ".".to_string());
    let focus = args.next();

    let project = Project::load(Path::new(&root))?;

    let mut parse_errors = 0;
    for f in &project.files {
        if let Some(e) = &f.parse_error {
            eprintln!("PARSE ERROR {}: {}", f.rel, e);
            parse_errors += 1;
        }
    }

    if let Some(name) = focus {
        dump_fn(&project, &name);
        return Ok(());
    }

    let total_calls: usize = project.fns.iter().map(|f| f.calls.len()).sum();
    let total_imports: usize = project.files.iter().map(|f| f.imports.len()).sum();
    let resolved_imports: usize = project.files.iter().map(|f| f.import_targets.len()).sum();
    println!(
        "{} files, {} fns ({} sig), {} call edges, {} parse errors",
        project.files.len(),
        project.fns.len(),
        project.fns.iter().filter(|f| f.is_sig).count(),
        total_calls,
        parse_errors,
    );
    println!(
        "imports: {resolved_imports} resolved / {total_imports} raw (in-project dependency edges)"
    );

    // Project-wide line tally by category — mirrors `tools/loc` (a cross-check
    // that the Rust classifier port agrees with the shard tool).
    let mut t = shard_viewer::model::Counts::default();
    for f in &project.files {
        let c = &f.counts;
        t.impl_ += c.impl_;
        t.measure += c.measure;
        t.proof += c.proof;
        t.reqproof += c.reqproof;
        t.req += c.req;
        t.comment += c.comment;
        t.blank += c.blank;
        t.sidecar += c.sidecar;
    }
    println!("\n== lines by category ==");
    println!("  impl {}  measure {}  proof {}  reqproof {}  req {}", t.impl_, t.measure, t.proof, t.reqproof, t.req);
    println!("  sidecar {}  comment {}  blank {}  TOTAL {}", t.sidecar, t.comment, t.blank, t.total());

    println!("\n== files ==");
    for f in &project.files {
        println!(
            "  {:<40} {:>3} fns  {:>2} types  {:>3} claims  {} imports",
            f.rel,
            f.fns.len(),
            f.types.len(),
            f.claims.len(),
            f.imports.len(),
        );
    }

    // Most-called fns — a cheap sanity check that resolution found real hubs.
    let mut ranked: Vec<usize> = (0..project.fns.len()).collect();
    ranked.sort_by_key(|&i| std::cmp::Reverse(project.fns[i].callers.len()));
    println!("\n== most-called fns ==");
    for &i in ranked.iter().take(15) {
        let f = &project.fns[i];
        println!("  {:>4} callers  {}  ({})", f.callers.len(), f.name, project.files[f.file].rel);
    }

    // Cut candidates: fns nothing in the project calls. Heuristic (short-name,
    // same-file-first resolution), so verify with grep before deleting — but a
    // useful first sweep for dead code. Biggest first (most code reclaimed).
    let mut orphans: Vec<usize> = (0..project.fns.len())
        .filter(|&i| project.fns[i].is_orphan())
        .collect();
    orphans.sort_by_key(|&i| std::cmp::Reverse(project.fns[i].src_lines()));
    println!(
        "\n== cut candidates: {} orphan fns (0 callers, non-sig, non-main) ==",
        orphans.len()
    );
    println!("   (heuristic — verify with grep before cutting)");
    for &i in orphans.iter().take(40) {
        let f = &project.fns[i];
        println!(
            "  {:>4} lines  {}  ({})",
            f.src_lines(),
            f.name,
            project.files[f.file].rel
        );
    }

    Ok(())
}

fn dump_fn(project: &Project, name: &str) {
    let matches: Vec<usize> = project
        .fns
        .iter()
        .enumerate()
        .filter(|(_, f)| f.name == name)
        .map(|(i, _)| i)
        .collect();
    if matches.is_empty() {
        println!("no fn named `{name}`");
        return;
    }
    for &i in &matches {
        let f = &project.fns[i];
        let sig: Vec<String> = f.params.iter().map(|(n, t)| format!("({n} {t})")).collect();
        println!(
            "fn {} ({}) {}   [{}]{}",
            f.name,
            sig.join(" "),
            f.ret,
            project.files[f.file].rel,
            if f.is_sig { " (sig)" } else { "" },
        );
        println!("  calls:");
        for &c in &f.calls {
            let g = &project.fns[c];
            println!("    -> {}  ({})", g.name, project.files[g.file].rel);
        }
        println!("  callers:");
        for &c in &f.callers {
            let g = &project.fns[c];
            println!("    <- {}  ({})", g.name, project.files[g.file].rel);
        }
    }
}
