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
use shard_viewer::scope::Scope;
use shard_viewer::view::{self, ViewMode, ViewParams};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let root = args.next().unwrap_or_else(|| ".".to_string());
    let needle = args.next().unwrap_or_default();
    let out = args.next().unwrap_or_else(|| "graph.svg".to_string());

    let project = Project::load(std::path::Path::new(&root))?;

    // `shard-render . src:FN out.svg` opens the source lightbox over one fn (so
    // the modal layout can be checked headlessly).
    let params = if let Some(fn_name) = needle.strip_prefix("src:") {
        let fn_idx = project
            .fns
            .iter()
            .position(|f| f.name == fn_name)
            .ok_or_else(|| format!("no fn named `{fn_name}`"))?;
        let f = &project.fns[fn_idx];
        println!("opening source lightbox for {} ({})", f.name, project.files[f.file].rel);
        ViewParams {
            mode: ViewMode::Methods,
            scope: Scope::File(f.file),
            selected_fn: Some(fn_idx),
            zoom: 1.0,
            pan: (0.0, 0.0),
            at_home: true,
            filter: String::new(),
            selection: Default::default(),
            source_modal: true,
            panel_w: view::DEFAULT_PANEL_W,
        }
    // `shard-render . flow:FN out.svg` charts one fn body's dataflow diagram.
    } else if let Some(fn_name) = needle.strip_prefix("flow:") {
        let fn_idx = project
            .fns
            .iter()
            .position(|f| f.name == fn_name && !f.body.is_empty())
            .ok_or_else(|| format!("no fn with a body named `{fn_name}`"))?;
        let f = &project.fns[fn_idx];
        println!("charting flow of {} ({})", f.name, project.files[f.file].rel);
        ViewParams {
            mode: ViewMode::Flow,
            scope: Scope::File(f.file),
            selected_fn: Some(fn_idx),
            zoom: 1.0,
            pan: (0.0, 0.0),
            at_home: true,
            filter: String::new(),
            selection: Default::default(),
            source_modal: false,
            panel_w: view::DEFAULT_PANEL_W,
        }
    } else if let Some(file_sub) = needle.strip_prefix("board:") {
        // `shard-render . board:SUBSTR out.svg` charts a file's call DAG with
        // each fn rendered in expanded flow form.
        let file_idx = project
            .files
            .iter()
            .position(|f| f.rel.contains(file_sub))
            .ok_or_else(|| format!("no file matching `{file_sub}`"))?;
        println!(
            "rendering board of {} ({} fns)",
            project.files[file_idx].rel,
            project.files[file_idx].fns.len()
        );
        ViewParams {
            mode: ViewMode::Board,
            scope: Scope::File(file_idx),
            selected_fn: None,
            zoom: 1.0,
            pan: (0.0, 0.0),
            at_home: true,
            filter: String::new(),
            selection: Default::default(),
            source_modal: false,
            panel_w: view::DEFAULT_PANEL_W,
        }
    } else if let Some(spec) = needle.strip_prefix("map:") {
        // `shard-render . map:SUBSTR out.svg` scopes the Map to one file; a
        // trailing slash (`map:kernel/ out.svg`) scopes to that whole directory
        // subtree, so the dir/file nesting can be checked headlessly.
        let scope = if let Some(dir) = spec.strip_suffix('/') {
            println!("rendering map scoped to dir {dir}/");
            Scope::Dir(dir.to_string())
        } else {
            let file_idx = project
                .files
                .iter()
                .position(|f| f.rel.contains(spec))
                .ok_or_else(|| format!("no file matching `{spec}`"))?;
            println!("rendering map scoped to {}", project.files[file_idx].rel);
            Scope::File(file_idx)
        };
        ViewParams {
            mode: ViewMode::Map,
            scope,
            selected_fn: None,
            zoom: 1.0,
            pan: (0.0, 0.0),
            at_home: true,
            filter: String::new(),
            selection: Default::default(),
            source_modal: false,
            panel_w: view::DEFAULT_PANEL_W,
        }
    } else if let Some(fn_name) = needle.strip_prefix("tree:") {
        // `shard-render . tree:FN out.svg` maps a fn's call neighborhood (the
        // CallTree scope: one caller level up, two callee levels down).
        let fn_idx = project
            .fns
            .iter()
            .position(|f| f.name == fn_name)
            .ok_or_else(|| format!("no fn named `{fn_name}`"))?;
        println!("rendering map scoped to the call tree of {fn_name}");
        ViewParams {
            mode: ViewMode::Map,
            scope: Scope::CallTree { root: fn_idx, up: 1, down: 2 },
            selected_fn: Some(fn_idx),
            zoom: 1.0,
            pan: (0.0, 0.0),
            at_home: true,
            filter: String::new(),
            selection: Default::default(),
            source_modal: false,
            panel_w: view::DEFAULT_PANEL_W,
        }
    } else if needle == "project" {
        // `shard-render . project out.svg` maps every fn in the project.
        println!("rendering map scoped to the whole project ({} files)", project.files.len());
        ViewParams {
            mode: ViewMode::Map,
            scope: Scope::Project,
            selected_fn: None,
            zoom: 1.0,
            pan: (0.0, 0.0),
            at_home: true,
            filter: String::new(),
            selection: Default::default(),
            source_modal: false,
            panel_w: view::DEFAULT_PANEL_W,
        }
    } else if needle == "systems" {
        println!("rendering systems graph ({} files)", project.files.len());
        // Select the biggest file so the breakdown panel is exercised too.
        let selected_file = project
            .files
            .iter()
            .enumerate()
            .max_by_key(|(_, f)| f.counts.total())
            .map(|(i, _)| i);
        ViewParams {
            mode: ViewMode::Systems,
            scope: selected_file.map_or(Scope::None, Scope::File),
            selected_fn: None,
            zoom: 1.0,
            pan: (0.0, 0.0),
            at_home: true,
            filter: String::new(),
            selection: Default::default(),
            source_modal: false,
            panel_w: view::DEFAULT_PANEL_W,
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
            scope: Scope::File(file_idx),
            selected_fn,
            zoom: 1.0,
            pan: (0.0, 0.0),
            at_home: true,
            filter: String::new(),
            selection: Default::default(),
            source_modal: false,
            panel_w: view::DEFAULT_PANEL_W,
        }
    };
    let mut root_el = view::app_root(&project, &params, None);
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
