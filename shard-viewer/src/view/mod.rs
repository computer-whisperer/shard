//! The damascene view: pure functions from project state to an `El` tree.
//!
//! Kept separate from the `App` impl (in the `shard-viewer` bin) so the same
//! tree can be rendered headlessly — to SVG + a lint report — without a GPU or
//! a window. That headless render is the build-time review loop.
//!
//! This module is the **shell** around the one canvas — the sidebar, the
//! scope-breadcrumb toolbar, and the inspector dispatch:
//!
//! - [`map`] — THE view: any [`Scope`]'s members (fns, claims, types, file
//!   docs) on one committed plane, grouped by origin dir ⊃ file.
//! - [`flow`] — the region-card renderer (fn bodies / proof spines as
//!   structured LabVIEW-style diagrams) the Map's cards draw their innards
//!   with.
//! - [`inspector`] — the [`Sel`]-dispatched detail panels (fn: source +
//!   docstring + call lists; file: counts + composition + header doc).
//! - [`highlight`] — the syntax-highlighted source view inside the fn panel.
//! - [`shared`] — the pan/zoom viewport, the placed-graph canvas, the edge
//!   vector builders, and the legend atoms.

mod flow;
mod highlight;
mod inspector;
mod map;
mod shared;

use crate::model::Project;
use crate::scope::Scope;
use damascene_core::prelude::*;

/// The inspector selection: the member-shaped cursor the detail panel is
/// about. Orthogonal to [`Scope`] — a focus *within* the mapped subject. A
/// `Fn` selection also highlights its card on the canvas; a `File` selection
/// opens the file inspector (counts, composition, header doc, imports).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum Sel {
    Fn(usize),
    File(usize),
}

/// Everything the view needs from the running app, snapshotted per frame.
pub struct ViewParams {
    /// The canvas subject — what set of members the Map is about. See
    /// [`crate::scope`].
    pub scope: Scope,
    /// The inspector cursor (highlighted on the canvas, shown in the detail
    /// panel). See [`Sel`].
    pub selected: Option<Sel>,
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
    /// Zoom past which the Map's fn slots draw their flow innards. A feel
    /// knob: user-tunable live from the Map legend's −/+ controls (app state),
    /// default [`DEFAULT_FLOW_Z`].
    pub flow_z: f32,
    /// The keyed element under the pointer (damascene hover readback), if
    /// any — the Map's reveal/deck focus rides on member keys ("fn:12",
    /// "type:7"). `None` headless. Render input only, like zoom.
    pub hovered: Option<String>,
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

impl ViewParams {
    /// The selected fn when the selection is fn-shaped — what canvas
    /// highlights and fn-only affordances key off.
    pub fn selected_fn(&self) -> Option<usize> {
        match self.selected {
            Some(Sel::Fn(i)) => Some(i),
            _ => None,
        }
    }
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

pub use map::{DEFAULT_FLOW_Z, MapCache, MapTarget, region_rect};
#[doc(hidden)]
pub use map::debug_file_graph;

/// The whole window: sidebar + main pane + (when something is selected) panel.
/// `map_cache` is the Map's per-scope committed-layout cache, owned by the app
/// (the GUI passes its cell; headless render passes `None` and commits fresh —
/// a single frame has nothing to cache across).
pub fn app_root(project: &Project, p: &ViewParams, map_cache: Option<&MapCache>) -> El {
    let mut panes = vec![sidebar(project, p), main_pane(project, p, map_cache)];
    let mut fn_in_panel = None;
    match p.selected {
        Some(Sel::Fn(fni)) => {
            panes.push(inspector::detail_panel(project, fni, p.panel_w));
            fn_in_panel = Some(fni);
        }
        Some(Sel::File(fi)) => panes.push(inspector::file_panel(project, fi)),
        None => {}
    }
    let main = page([row(panes).gap(tokens::SPACE_4).height(Size::Fill(1.0))]);
    // The source lightbox: a full-size overlay layer over the workbench. It's
    // the way to read a wide/long fn body when the fixed-width detail panel
    // can't show it (e.g. driver.shard::run_decls). `page` is already an
    // overlay root (tooltips mount there), and `overlays` adds the modal as a
    // sibling layer painted on top.
    let modal = match (p.source_modal, fn_in_panel) {
        (true, Some(fni)) => Some(inspector::source_modal(project, fni)),
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
    column([
        toolbar(project, p),
        map::legend(p.flow_z),
        map::canvas(project, p, map_cache),
    ])
    .gap(tokens::SPACE_3)
    .width(Size::Fill(1.0))
    .height(Size::Fill(1.0))
}

/// The toolbar: the scope breadcrumb (the orientation device that replaced
/// the mode buttons) plus the camera controls.
fn toolbar(project: &Project, p: &ViewParams) -> El {
    let mut items = breadcrumb(project, &p.scope);
    items.extend([
        spacer(),
        text(format!("{:.0}%", p.zoom * 100.0))
            .mono()
            .muted()
            .center_text()
            .width(Size::Fixed(52.0))
            .tooltip("Canvas zoom"),
        button("Fit").key("fit").secondary().tooltip("Frame the whole map"),
        button("Reset view").key("reset").ghost().tooltip("Snap back to 1:1"),
    ]);
    row(items)
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_2)
        .align(Align::Center)
}

/// The scope breadcrumb: `◆ project ▸ dir ▸ … ▸ file ▸ fn`. Every segment
/// jumps (fly-or-rescope — the same navigation the sidebar routes through);
/// the segment the scope is anchored on renders selected, and a fn anchor is
/// plain text (it's already the subject, not a jump target). Keys are
/// `crumb*`-prefixed so they never collide with the sidebar's `dir:`/`file:`
/// buttons for the same targets.
fn breadcrumb(project: &Project, scope: &Scope) -> Vec<El> {
    let sep = || text("▸").muted().font_size(SUB_SIZE);
    let crumb = |label: String, key: String, here: bool, tip: String| {
        let b = button(label).key(key).tooltip(tip);
        if here { b.selected() } else { b.ghost() }
    };
    let mut out = vec![crumb(
        "◆ project".to_string(),
        "crumb".to_string(),
        *scope == Scope::Project,
        "Map the whole project".to_string(),
    )];

    // The cumulative dir chain (`examples/` ▸ `io/`), each link scoping to
    // its subtree; `terminal` marks the chain's end as the scope's anchor.
    let dir_chain = |out: &mut Vec<El>, dir_path: &str, terminal: bool| {
        let mut prefix = String::new();
        let segs: Vec<&str> = dir_path.split('/').filter(|s| !s.is_empty()).collect();
        for (k, seg) in segs.iter().enumerate() {
            if !prefix.is_empty() {
                prefix.push('/');
            }
            prefix.push_str(seg);
            out.push(sep());
            out.push(crumb(
                format!("{seg}/"),
                format!("crumbdir:{prefix}"),
                terminal && k + 1 == segs.len(),
                format!("Scope to everything under {prefix}/"),
            ));
        }
    };
    // A file link (its dir chain first), `terminal` when the file is the
    // scope's anchor.
    let file_link = |out: &mut Vec<El>, i: usize, terminal: bool| {
        let rel = &project.files[i].rel;
        let (dir, base) = rel.rsplit_once('/').unwrap_or(("", rel.as_str()));
        dir_chain(out, dir, false);
        out.push(sep());
        out.push(crumb(base.to_string(), format!("crumbfile:{i}"), terminal, rel.clone()));
    };
    // A fn anchor: plain text, styled like the map's card titles.
    let fn_anchor = |out: &mut Vec<El>, label: String| {
        out.push(sep());
        out.push(text(label).mono().semibold().font_size(TITLE_SIZE));
    };

    match scope {
        Scope::None | Scope::Project => {}
        Scope::Dir(d) => dir_chain(&mut out, d, true),
        Scope::File(i) => file_link(&mut out, *i, true),
        Scope::Fn(g) => {
            if let Some(f) = project.fns.get(*g) {
                file_link(&mut out, f.file, false);
                fn_anchor(&mut out, f.name.clone());
            }
        }
        Scope::CallTree { root, up, down } => {
            if let Some(f) = project.fns.get(*root) {
                file_link(&mut out, f.file, false);
                fn_anchor(&mut out, format!("{} · {up}↑ {down}↓", f.name));
            }
        }
    }
    out
}
