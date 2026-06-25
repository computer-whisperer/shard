//! `shard-render` — headless verification of the viewer's graph canvas.
//!
//! Builds the view tree for one file and renders it to SVG + a lint report
//! with no GPU or window, so the layout can be *seen* and checked during
//! development. This is the cheap review loop the damascene docs describe.
//!
//!   shard-render PROJECT_ROOT FILE_SUBSTRING [OUT.svg]
//!
//! FILE_SUBSTRING selects the first file whose relative path contains it
//! (e.g. `reader` → kernel/reader.shard).

use damascene_core::prelude::*;
use shard_viewer::model::Project;
use shard_viewer::view::{self, ViewMode, ViewParams};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let root = args.next().unwrap_or_else(|| ".".to_string());
    let needle = args.next().unwrap_or_default();
    let out = args.next().unwrap_or_else(|| "graph.svg".to_string());

    let project = Project::load(std::path::Path::new(&root))?;

    // `shard-render . systems out.svg` renders the project import graph.
    let params = if needle == "systems" {
        println!("rendering systems graph ({} files)", project.files.len());
        ViewParams {
            mode: ViewMode::Systems,
            selected_file: None,
            selected_fn: None,
            zoom: 1.0,
        }
    } else {
        let file_idx = project
            .files
            .iter()
            .position(|f| f.rel.contains(&needle))
            .ok_or_else(|| format!("no file matching `{needle}`"))?;
        println!(
            "rendering {} ({} fns)",
            project.files[file_idx].rel,
            project.files[file_idx].fns.len()
        );
        // Select the file's most-called fn so the detail panel is exercised too.
        let selected_fn = project.files[file_idx]
            .fns
            .iter()
            .copied()
            .max_by_key(|&fi| project.fns.iter().filter(|g| g.calls.contains(&fi)).count());
        ViewParams {
            mode: ViewMode::Methods,
            selected_file: Some(file_idx),
            selected_fn,
            zoom: 1.0,
        }
    };
    let mut root_el = view::app_root(&project, &params);
    let (vw, vh) = (
        std::env::var("SHARD_RENDER_W").ok().and_then(|s| s.parse().ok()).unwrap_or(1600.0),
        std::env::var("SHARD_RENDER_H").ok().and_then(|s| s.parse().ok()).unwrap_or(1000.0),
    );
    let viewport = Rect::new(0.0, 0.0, vw, vh);
    let bundle = render_bundle(&mut root_el, viewport);

    std::fs::write(&out, &bundle.svg)?;
    println!("wrote {out} ({} bytes svg)", bundle.svg.len());

    // Lint findings are the headless equivalent of eyeballing the frame.
    let findings = &bundle.lint.findings;
    println!("lint: {} findings", findings.len());
    for f in findings.iter().take(20) {
        println!("  [{:?}] {}", f.kind, f.message);
    }
    Ok(())
}
