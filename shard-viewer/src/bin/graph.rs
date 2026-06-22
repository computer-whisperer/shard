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
    println!(
        "{} files, {} fns ({} sig), {} call edges, {} parse errors",
        project.files.len(),
        project.fns.len(),
        project.fns.iter().filter(|f| f.is_sig).count(),
        total_calls,
        parse_errors,
    );

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
    let mut indeg = vec![0usize; project.fns.len()];
    for f in &project.fns {
        for &c in &f.calls {
            indeg[c] += 1;
        }
    }
    let mut ranked: Vec<usize> = (0..project.fns.len()).collect();
    ranked.sort_by_key(|&i| std::cmp::Reverse(indeg[i]));
    println!("\n== most-called fns ==");
    for &i in ranked.iter().take(15) {
        let f = &project.fns[i];
        println!("  {:>4} callers  {}  ({})", indeg[i], f.name, project.files[f.file].rel);
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
        let callers: Vec<usize> = (0..project.fns.len())
            .filter(|&j| project.fns[j].calls.contains(&i))
            .collect();
        println!("  callers:");
        for c in callers {
            let g = &project.fns[c];
            println!("    <- {}  ({})", g.name, project.files[g.file].rel);
        }
    }
}
