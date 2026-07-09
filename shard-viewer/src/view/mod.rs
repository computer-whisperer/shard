//! The damascene view: pure functions from project state to an `El` tree.
//!
//! Kept separate from the `App` impl (in the `shard-viewer` bin) so the same
//! tree can be rendered headlessly — to SVG + a lint report — without a GPU or
//! a window. That headless render is the build-time review loop.
//!
//! This module is the **shell** (sidebar / toolbar / pane dispatch); each
//! visualization *variant* lives in its own file so new experiments stay
//! isolated and cheap to try:
//!
//! - [`methods`] — one file's call graph + triage overlay (and the shared
//!   per-fn detail panel).
//! - [`systems`] — the project-wide import graph + proof/impl heat map.
//! - [`flow`] — one fn body as a structured (LabVIEW-style) region card.
//! - [`board`] — the call DAG with each node rendered in that expanded flow
//!   form (reuses `flow::render_region`).
//! - [`map`] — the unified view we're growing toward: any [`Scope`]'s fns,
//!   grouped by origin file/dir, each in expanded flow form.
//! - [`shared`] — the pan/zoom viewport, laid-out-graph canvas, and edge/legend
//!   primitives every variant draws with.

mod board;
mod flow;
mod highlight;
mod map;
mod methods;
mod shared;
mod systems;

use crate::model::Project;
use crate::scope::Scope;
use damascene_core::prelude::*;

/// Which visualization the canvas is showing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ViewMode {
    /// One file's fns and their intra-file call edges.
    Methods,
    /// The project-wide file import dependency graph.
    Systems,
    /// One fn's body as a structured (LabVIEW-style) diagram.
    Flow,
    /// One file's call DAG with each fn rendered in expanded flow form.
    Board,
    /// The unified map: any scope's fns, grouped by origin file/dir, each in
    /// expanded flow form. The view we're growing toward — still experimental.
    Map,
}

/// Everything the view needs from the running app, snapshotted per frame.
pub struct ViewParams {
    pub mode: ViewMode,
    /// The canvas subject — what set of fns the view is about. The single-file
    /// views read [`Scope::focus_file`] out of it; the Map view reads the full
    /// fn/file sets. See [`crate::scope`].
    pub scope: Scope,
    /// The focused fn cursor (highlighted in the graph, shown in the detail
    /// panel, charted by Flow). Orthogonal to `scope`: a focus *within* it.
    pub selected_fn: Option<usize>,
    /// Current viewport zoom (read back from the runtime). Shown in the
    /// toolbar, and — off home — it prices the Map's screen-space rendering
    /// decisions (what draws inside the committed footprints).
    pub zoom: f32,
    /// Current viewport pan (read back from the runtime), in the viewport's
    /// screen px. With `zoom` and `canvas` it locates the visible content
    /// window the Map culls against.
    pub pan: (f32, f32),
    /// Estimated canvas size in logical px (window minus sidebar/panel/chrome).
    /// Only feeds the Map's cull window (padded — a rough estimate is fine)
    /// and its at-home fit-zoom computation.
    pub canvas: (f32, f32),
    /// Whether the canvas viewport is still "at home" — fitted by the armed
    /// `FitPolicy` / an app `FitContent`, untouched by the user (read back via
    /// `viewport_at_home`; headless render is always at home). While at home
    /// the Map prices rendering off the exact fit zoom of the committed extent
    /// (the headless zoom readback lies) and draws without culling.
    pub at_home: bool,
    /// Sidebar filter text (case-insensitive substring over file paths).
    pub filter: String,
    /// Text-selection state for the filter input (app-owned, per damascene).
    pub selection: Selection,
    /// Whether the selected fn's source is open in the full-size lightbox.
    pub source_modal: bool,
    /// Current width of the (user-resizable) detail panel, read back from the
    /// runtime so the manually-wrapped source re-wraps to the dragged width.
    /// Defaults to [`DEFAULT_PANEL_W`] until the panel is dragged.
    pub panel_w: f32,
}

/// Key of the pan/zoom viewport — also the target of `ViewportRequest`s.
pub const CANVAS_KEY: &str = "canvas";

pub(crate) const TITLE_SIZE: f32 = 13.0;
pub(crate) const SUB_SIZE: f32 = 11.0;

/// Default (pre-drag) widths of the two resizable side panes. Both are
/// `user_resizable` — the runtime overrides these with the dragged size at
/// layout time and stores it like a scroll offset; we read it back via
/// `BuildCx::user_size` to re-wrap the panel source. Keys the panes are keyed
/// with (so the resize bands and stored sizes route correctly).
pub const DEFAULT_SIDEBAR_W: f32 = 320.0;
pub const DEFAULT_PANEL_W: f32 = 420.0;
pub const SIDEBAR_KEY: &str = "sidebar";
pub const PANEL_KEY: &str = "detail_panel";

pub use map::MapCache;

/// The whole window: sidebar + main pane + (when something is selected) panel.
/// `map_cache` is the Map's per-scope committed-layout cache, owned by the app
/// (the GUI passes its cell; headless render passes `None` and commits fresh —
/// a single frame has nothing to cache across).
pub fn app_root(project: &Project, p: &ViewParams, map_cache: Option<&MapCache>) -> El {
    let mut panes = vec![sidebar(project, p), main_pane(project, p, map_cache)];
    let mut fn_in_panel = None;
    match p.mode {
        ViewMode::Methods | ViewMode::Flow | ViewMode::Board | ViewMode::Map => {
            if let Some(fni) = p.selected_fn {
                panes.push(methods::detail_panel(project, fni, p.mode, p.panel_w));
                fn_in_panel = Some(fni);
            }
        }
        ViewMode::Systems => {
            if let Some(fi) = p.scope.focus_file(project) {
                panes.push(systems::detail_panel(project, fi));
            }
        }
    }
    let main = page([row(panes).gap(tokens::SPACE_4).height(Size::Fill(1.0))]);
    // The source lightbox: a full-size overlay layer over the workbench. It's
    // the way to read a wide/long fn body when the fixed-width detail panel
    // can't show it (e.g. driver.shard::run_decls). `page` is already an
    // overlay root (tooltips mount there), and `overlays` adds the modal as a
    // sibling layer painted on top.
    let modal = match (p.source_modal, fn_in_panel) {
        (true, Some(fni)) => Some(methods::source_modal(project, fni)),
        _ => None,
    };
    overlays(main, [modal])
}

/// The directory a file's `rel` path sits in (no trailing slash; `""` = root).
fn dir_of(rel: &str) -> &str {
    rel.rsplit_once('/').map(|(d, _)| d).unwrap_or("")
}

fn sidebar(project: &Project, p: &ViewParams) -> El {
    let needle = p.filter.to_lowercase();
    let focus_file = p.scope.focus_file(project);

    // Group the (filtered) files by their directory so the picker mirrors the
    // Map view's dir/file structure: a clickable dir header (selects the whole
    // subtree) over its files (each selects that one file).
    let mut groups: std::collections::BTreeMap<&str, Vec<usize>> = Default::default();
    let mut shown = 0;
    for (i, f) in project.files.iter().enumerate() {
        if needle.is_empty() || f.rel.to_lowercase().contains(&needle) {
            groups.entry(dir_of(&f.rel)).or_default().push(i);
            shown += 1;
        }
    }

    let mut rows: Vec<El> = Vec::new();
    for (dir, files) in &groups {
        // The dir header: selecting it scopes the canvas to the whole subtree.
        let dir_label = if dir.is_empty() { "(root)".to_string() } else { format!("{dir}/") };
        let mut dh = button(format!("▸ {dir_label}  ({})", files.len()))
            .key(format!("dir:{dir}"))
            .ghost()
            .tooltip(format!("Scope to everything under {dir_label}"));
        if p.scope == Scope::Dir(dir.to_string()) {
            dh = dh.selected();
        }
        rows.push(dh);
        for &i in files {
            let f = &project.files[i];
            // Show just the basename under the dir header; surface parse
            // failures (otherwise invisible — the file looks empty) with a
            // marker + the error on hover.
            let base = f.rel.rsplit_once('/').map(|(_, b)| b).unwrap_or(&f.rel);
            let label = match &f.parse_error {
                Some(_) => format!("  ⚠ {}  ({})", base, f.fns.len()),
                None => format!("  {}  ({})", base, f.fns.len()),
            };
            let tip = match &f.parse_error {
                Some(e) => format!("parse error — {e}"),
                None => format!("{} · {} lines", f.module, f.counts.total()),
            };
            let mut b = button(label).key(format!("file:{i}")).ghost().tooltip(tip);
            if focus_file == Some(i) {
                b = b.selected();
            }
            rows.push(b);
        }
    }

    // Header shows the filtered/total count; an X clears the filter when set.
    let total = project.files.len();
    let mut header = vec![
        h3("Files"),
        spacer(),
        text(format!("{shown}/{total}"))
            .mono()
            .muted()
            .font_size(SUB_SIZE),
    ];
    if !p.filter.is_empty() {
        header.push(button("✕").key("filter_clear").ghost().tooltip("Clear filter"));
    }

    let list = if rows.is_empty() {
        column([text("No files match.").muted().font_size(SUB_SIZE)]).padding(tokens::SPACE_3)
    } else {
        scroll(rows).height(Size::Fill(1.0))
    };

    // A top-of-list "everything" scope, above the per-dir/-file rows: the whole
    // project mapped at once. Kept out of the scroll so it stays reachable.
    let mut project_btn = button(format!("◆ Whole project  ({} files)", project.files.len()))
        .key("scope_project")
        .ghost()
        .tooltip("Map every fn in the project, grouped by dir and file");
    if p.scope == Scope::Project {
        project_btn = project_btn.selected();
    }

    column([
        row(header).gap(tokens::SPACE_2),
        text_input_with(
            "filter",
            &p.filter,
            &p.selection,
            TextInputOpts::default().placeholder("Filter files…"),
        ),
        project_btn,
        list,
    ])
    .gap(tokens::SPACE_2)
    .padding(tokens::SPACE_3)
    .width(Size::Fixed(DEFAULT_SIDEBAR_W))
    .height(Size::Fill(1.0))
    .fill(tokens::CARD)
    .stroke(tokens::BORDER)
    .radius(10.0)
    // Drag the right seam to widen the file list (long paths get cramped at
    // the default width). No app state — the runtime keeps the dragged width.
    .key(SIDEBAR_KEY)
    .user_resizable()
    .min_width(220.0)
    .max_width(620.0)
}

fn main_pane(project: &Project, p: &ViewParams, map_cache: Option<&MapCache>) -> El {
    let focus_file = p.scope.focus_file(project);
    let body = match p.mode {
        ViewMode::Systems => systems::canvas(project, p),
        ViewMode::Methods => match focus_file {
            None => column([text("Select a file to see its call graph.").muted()])
                .padding(tokens::SPACE_8),
            Some(fi) => methods::canvas(project, fi, p),
        },
        ViewMode::Board => match focus_file {
            None => column([text("Select a file to see its board.").muted()])
                .padding(tokens::SPACE_8),
            Some(fi) => board::canvas(project, fi, p),
        },
        ViewMode::Flow => match p.selected_fn {
            None => column([text("Select a fn (in Methods) to chart its body.").muted()])
                .padding(tokens::SPACE_8),
            Some(fni) => flow::canvas(project, fni),
        },
        ViewMode::Map => map::canvas(project, p, map_cache),
    };
    let mut head = vec![toolbar(project, p)];
    match p.mode {
        ViewMode::Methods if focus_file.is_some() => head.push(methods::legend()),
        ViewMode::Systems => head.push(systems::legend()),
        ViewMode::Board if focus_file.is_some() => head.push(board::legend()),
        ViewMode::Flow if p.selected_fn.is_some() => head.push(flow::legend()),
        ViewMode::Map => head.push(map::legend()),
        _ => {}
    }
    head.push(body);
    column(head)
        .gap(tokens::SPACE_3)
        .width(Size::Fill(1.0))
        .height(Size::Fill(1.0))
}

fn toolbar(project: &Project, p: &ViewParams) -> El {
    let title = match p.mode {
        ViewMode::Systems => format!("Systems · {} files", project.files.len()),
        ViewMode::Methods => match p.scope.focus_file(project) {
            Some(fi) => project.files[fi].rel.clone(),
            None => "shard-viewer".to_string(),
        },
        ViewMode::Board => match p.scope.focus_file(project) {
            Some(fi) => format!("{}  ·  board", project.files[fi].rel),
            None => "Board".to_string(),
        },
        ViewMode::Flow => match p.selected_fn {
            Some(fni) => format!("{}  ·  flow", project.fns[fni].name),
            None => "Flow".to_string(),
        },
        ViewMode::Map => format!("Map · {}", p.scope.label(project)),
    };
    let mode_btn = |label: &str, key: &str, active: bool, tip: &str| {
        let b = button(label).key(key.to_string()).tooltip(tip.to_string());
        if active { b.selected() } else { b.ghost() }
    };
    row([
        h3(title),
        spacer(),
        mode_btn("Map", "mode_map", p.mode == ViewMode::Map, "Any scope's fns, grouped by file/dir, each in flow form (experimental)"),
        mode_btn("Methods", "mode_methods", p.mode == ViewMode::Methods, "One file's call graph + triage overlay"),
        mode_btn("Systems", "mode_systems", p.mode == ViewMode::Systems, "Project-wide import graph + proof/impl heat map"),
        mode_btn("Board", "mode_board", p.mode == ViewMode::Board, "This file's call DAG, each fn in expanded flow form"),
        mode_btn("Flow", "mode_flow", p.mode == ViewMode::Flow, "The selected fn's body as a structured diagram"),
        text(format!("{:.0}%", p.zoom * 100.0))
            .mono()
            .muted()
            .center_text()
            .width(Size::Fixed(52.0))
            .tooltip("Canvas zoom"),
        button("Fit").key("fit").secondary().tooltip("Frame the whole graph"),
        button("Reset view").key("reset").ghost().tooltip("Snap back to 1:1"),
    ])
    .gap(tokens::SPACE_2)
    .padding(tokens::SPACE_2)
}
