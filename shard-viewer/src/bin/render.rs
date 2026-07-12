//! `shard-render` — headless verification of the viewer.
//!
//! Builds the Map's view tree for one spec and renders it to SVG + a lint
//! report with no GPU or window, so the layout can be *seen* and checked
//! during development. This is the cheap review loop the damascene docs
//! describe.
//!
//!   shard-render PROJECT_ROOT SPEC [OUT.svg]
//!
//! SPEC selects the scope (and panel) to render:
//!   SUBSTR        map scoped to the first matching file, its most-called
//!                 fn selected (detail panel exercised)
//!   map:SUBSTR    map scoped to the file, nothing selected
//!   map:DIR/      map scoped to a directory subtree
//!   project       map of the whole project
//!   fn:NAME       map scoped to one fn (Scope::Fn — "read this fn large")
//!   tree:NAME     map of the fn's call neighborhood (CallTree)
//!   inspect:SUBSTR  file scope with the file inspector open
//!   src:NAME      the source lightbox over the fn's file
//! SHARD_RENDER_HOVER=fn_name|key simulates hover; SHARD_RENDER_W/H size
//! the frame.

use damascene_core::prelude::*;
use shard_viewer::model::Project;
use shard_viewer::scope::Scope;
use shard_viewer::view::{self, ViewParams};

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
            scope: Scope::File(f.file),
            selected: Some(view::Sel::Fn(fn_idx)),
            zoom: 1.0,
            pan: (0.0, 0.0),
            canvas: canvas_estimate(),
            at_home: true,
            flow_z: view::DEFAULT_FLOW_Z,
            hovered: None,
            filter: String::new(),
            selection: Default::default(),
            source_modal: true,
            panel_w: view::DEFAULT_PANEL_W,
        }
    // `shard-render . fn:NAME out.svg` maps one fn alone (Scope::Fn): its
    // card plus the claims about it and the types it shapes — the headless
    // "read this fn large" check (successor of the old flow: spec).
    } else if let Some(fn_name) = needle.strip_prefix("fn:") {
        let fn_idx = project
            .fns
            .iter()
            .position(|f| f.name == fn_name)
            .ok_or_else(|| format!("no fn named `{fn_name}`"))?;
        let f = &project.fns[fn_idx];
        println!("rendering map scoped to fn {} ({})", f.name, project.files[f.file].rel);
        ViewParams {
            scope: Scope::Fn(fn_idx),
            selected: Some(view::Sel::Fn(fn_idx)),
            zoom: 1.0,
            pan: (0.0, 0.0),
            canvas: canvas_estimate(),
            at_home: true,
            flow_z: view::DEFAULT_FLOW_Z,
            hovered: None,
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
            scope,
            selected: None,
            zoom: 1.0,
            pan: (0.0, 0.0),
            canvas: canvas_estimate(),
            at_home: true,
            flow_z: view::DEFAULT_FLOW_Z,
            hovered: None,
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
            scope: Scope::CallTree { root: fn_idx, up: 1, down: 2 },
            selected: Some(view::Sel::Fn(fn_idx)),
            zoom: 1.0,
            pan: (0.0, 0.0),
            canvas: canvas_estimate(),
            at_home: true,
            flow_z: view::DEFAULT_FLOW_Z,
            hovered: None,
            filter: String::new(),
            selection: Default::default(),
            source_modal: false,
            panel_w: view::DEFAULT_PANEL_W,
        }
    } else if needle == "project" {
        // `shard-render . project out.svg` maps every fn in the project.
        println!("rendering map scoped to the whole project ({} files)", project.files.len());
        ViewParams {
            scope: Scope::Project,
            selected: None,
            zoom: 1.0,
            pan: (0.0, 0.0),
            canvas: canvas_estimate(),
            at_home: true,
            flow_z: view::DEFAULT_FLOW_Z,
            hovered: None,
            filter: String::new(),
            selection: Default::default(),
            source_modal: false,
            panel_w: view::DEFAULT_PANEL_W,
        }
    } else if let Some(file_sub) = needle.strip_prefix("inspect:") {
        // `shard-render . inspect:SUBSTR out.svg` opens the file inspector
        // next to a Map scoped to that file, so the Sel::File panel can be
        // checked headlessly.
        let file_idx = project
            .files
            .iter()
            .position(|f| f.rel.contains(file_sub))
            .ok_or_else(|| format!("no file matching `{file_sub}`"))?;
        println!("rendering file inspector for {}", project.files[file_idx].rel);
        ViewParams {
            scope: Scope::File(file_idx),
            selected: Some(view::Sel::File(file_idx)),
            zoom: 1.0,
            pan: (0.0, 0.0),
            canvas: canvas_estimate(),
            at_home: true,
            flow_z: view::DEFAULT_FLOW_Z,
            hovered: None,
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
            scope: Scope::File(file_idx),
            selected: selected_fn.map(view::Sel::Fn),
            zoom: 1.0,
            pan: (0.0, 0.0),
            canvas: canvas_estimate(),
            at_home: true,
            flow_z: view::DEFAULT_FLOW_Z,
            hovered: None,
            filter: String::new(),
            selection: Default::default(),
            source_modal: false,
            panel_w: view::DEFAULT_PANEL_W,
        }
    };
    // SHARD_RENDER_HOVER simulates the pointer resting on a keyed member
    // ("fn:12", or a bare fn name resolved to its key) so hover-revealed
    // rendering — the Map's Use-edge reveal and shape deck — can be checked
    // headlessly, where no real hover exists.
    let mut params = params;
    if let Ok(hov) = std::env::var("SHARD_RENDER_HOVER") {
        params.hovered = if hov.contains(':') {
            Some(hov)
        } else {
            project.fns.iter().position(|f| f.name == hov).map(|i| format!("fn:{i}"))
        };
    }
    let mut root_el = view::app_root(&project, &params, None);
    let (vw, vh) = frame_size();
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

/// The headless frame size (env-overridable: SHARD_RENDER_W/H).
fn frame_size() -> (f32, f32) {
    (
        std::env::var("SHARD_RENDER_W").ok().and_then(|s| s.parse().ok()).unwrap_or(1600.0),
        std::env::var("SHARD_RENDER_H").ok().and_then(|s| s.parse().ok()).unwrap_or(1000.0),
    )
}

/// Estimated canvas pane (the frame minus the default sidebar + chrome),
/// mirroring the GUI's estimate in viewer.rs — feeds the Map's at-home fit
/// computation so the headless LOD matches what the fitted GUI would show.
fn canvas_estimate() -> (f32, f32) {
    let (w, h) = frame_size();
    ((w - view::DEFAULT_SIDEBAR_W - 48.0).max(200.0), (h - 138.0).max(200.0))
}
