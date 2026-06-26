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
//! - [`shared`] — the pan/zoom viewport, laid-out-graph canvas, and edge/legend
//!   primitives every variant draws with.

mod board;
mod flow;
mod methods;
mod shared;
mod systems;

use crate::model::Project;
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
}

/// Everything the view needs from the running app, snapshotted per frame.
pub struct ViewParams {
    pub mode: ViewMode,
    pub selected_file: Option<usize>,
    pub selected_fn: Option<usize>,
    /// Current viewport zoom (read back from the runtime), for display only.
    pub zoom: f32,
    /// Sidebar filter text (case-insensitive substring over file paths).
    pub filter: String,
    /// Text-selection state for the filter input (app-owned, per damascene).
    pub selection: Selection,
}

/// Key of the pan/zoom viewport — also the target of `ViewportRequest`s.
pub const CANVAS_KEY: &str = "canvas";

pub(crate) const TITLE_SIZE: f32 = 13.0;
pub(crate) const SUB_SIZE: f32 = 11.0;

/// The whole window: sidebar + main pane + (when something is selected) panel.
pub fn app_root(project: &Project, p: &ViewParams) -> El {
    let mut panes = vec![sidebar(project, p), main_pane(project, p)];
    match p.mode {
        ViewMode::Methods | ViewMode::Flow | ViewMode::Board => {
            if let Some(fni) = p.selected_fn {
                panes.push(methods::detail_panel(project, fni, p.mode));
            }
        }
        ViewMode::Systems => {
            if let Some(fi) = p.selected_file {
                panes.push(systems::detail_panel(project, fi));
            }
        }
    }
    page([row(panes).gap(tokens::SPACE_4).height(Size::Fill(1.0))])
}

fn sidebar(project: &Project, p: &ViewParams) -> El {
    let needle = p.filter.to_lowercase();
    let rows: Vec<El> = project
        .files
        .iter()
        .enumerate()
        .filter(|(_, f)| needle.is_empty() || f.rel.to_lowercase().contains(&needle))
        .map(|(i, f)| {
            // Surface parse failures (otherwise invisible — the file just shows
            // 0 fns as if empty) with a marker + the error on hover.
            let label = match &f.parse_error {
                Some(_) => format!("⚠ {}  ({})", f.rel, f.fns.len()),
                None => format!("{}  ({})", f.rel, f.fns.len()),
            };
            let tip = match &f.parse_error {
                Some(e) => format!("parse error — {e}"),
                None => format!("{} · {} lines", f.module, f.counts.total()),
            };
            let mut b = button(label).key(format!("file:{i}")).ghost().tooltip(tip);
            if p.selected_file == Some(i) {
                b = b.selected();
            }
            b
        })
        .collect();

    // Header shows the filtered/total count; an X clears the filter when set.
    let shown = rows.len();
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

    column([
        row(header).gap(tokens::SPACE_2),
        text_input_with(
            "filter",
            &p.filter,
            &p.selection,
            TextInputOpts::default().placeholder("Filter files…"),
        ),
        list,
    ])
    .gap(tokens::SPACE_2)
    .padding(tokens::SPACE_3)
    .width(Size::Fixed(320.0))
    .height(Size::Fill(1.0))
    .fill(tokens::CARD)
    .stroke(tokens::BORDER)
    .radius(10.0)
}

fn main_pane(project: &Project, p: &ViewParams) -> El {
    let body = match p.mode {
        ViewMode::Systems => systems::canvas(project, p),
        ViewMode::Methods => match p.selected_file {
            None => column([text("Select a file to see its call graph.").muted()])
                .padding(tokens::SPACE_8),
            Some(fi) => methods::canvas(project, fi, p),
        },
        ViewMode::Board => match p.selected_file {
            None => column([text("Select a file to see its board.").muted()])
                .padding(tokens::SPACE_8),
            Some(fi) => board::canvas(project, fi, p),
        },
        ViewMode::Flow => match p.selected_fn {
            None => column([text("Select a fn (in Methods) to chart its body.").muted()])
                .padding(tokens::SPACE_8),
            Some(fni) => flow::canvas(project, fni),
        },
    };
    let mut head = vec![toolbar(project, p)];
    match p.mode {
        ViewMode::Methods if p.selected_file.is_some() => head.push(methods::legend()),
        ViewMode::Systems => head.push(systems::legend()),
        ViewMode::Board if p.selected_file.is_some() => head.push(board::legend()),
        ViewMode::Flow if p.selected_fn.is_some() => head.push(flow::legend()),
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
        ViewMode::Methods => match p.selected_file {
            Some(fi) => project.files[fi].rel.clone(),
            None => "shard-viewer".to_string(),
        },
        ViewMode::Board => match p.selected_file {
            Some(fi) => format!("{}  ·  board", project.files[fi].rel),
            None => "Board".to_string(),
        },
        ViewMode::Flow => match p.selected_fn {
            Some(fni) => format!("{}  ·  flow", project.fns[fni].name),
            None => "Flow".to_string(),
        },
    };
    let mode_btn = |label: &str, key: &str, active: bool, tip: &str| {
        let b = button(label).key(key.to_string()).tooltip(tip.to_string());
        if active { b.selected() } else { b.ghost() }
    };
    row([
        h3(title),
        spacer(),
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
