//! Methods view: one file's fns and their intra-file call edges, with the
//! dead-code / complexity triage overlay (orphan = cut candidate, warmth =
//! call degree, height = source lines). Also owns the per-fn detail panel,
//! which the Flow and Board views reuse.

use super::shared::{graph_canvas, legend_chip};
use super::{SUB_SIZE, TITLE_SIZE, ViewMode, ViewParams};
use crate::layout::{self, EndPoint, GEdge, GNode, Graph};
use crate::model::Project;
use damascene_core::prelude::*;
use std::collections::HashMap;

pub(crate) fn canvas(project: &Project, file_idx: usize, p: &ViewParams) -> El {
    let (graph, fn_of) = build_call_graph(project, file_idx);
    if graph.nodes.is_empty() {
        return column([text("This file defines no fns.").muted()]).padding(tokens::SPACE_8);
    }
    let lay = layout::layout(&graph, &layout::LayoutConfig::default());
    let node_els: Vec<El> = lay
        .nodes
        .iter()
        .enumerate()
        .map(|(i, pn)| node_box(project, fn_of[i], p.selected_fn, pn.w, pn.h))
        .collect();
    graph_canvas(&lay, node_els)
}

/// The triage-overlay key (Methods view): how node color and size encode the
/// dead-code / complexity signal.
pub(crate) fn legend() -> El {
    row([
        text("triage").mono().muted().font_size(SUB_SIZE),
        legend_chip(tokens::CARD.mix(tokens::DESTRUCTIVE, 0.5), "orphan — cut candidate"),
        legend_chip(tokens::CARD.mix(tokens::WARNING, 0.6), "hub — many callers"),
        legend_chip(tokens::CARD, "leaf"),
        legend_chip(tokens::MUTED, "sig"),
        text("· taller = more source lines").muted().font_size(SUB_SIZE),
    ])
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_2)
}

/// Build the engine `Graph` for a file's intra-file call graph, plus the fn
/// index behind each graph node (index-aligned with the layout result).
pub(crate) fn build_call_graph(project: &Project, file_idx: usize) -> (Graph, Vec<usize>) {
    let fn_of: Vec<usize> = project.files[file_idx].fns.clone();
    let local: HashMap<usize, usize> = fn_of.iter().enumerate().map(|(i, &g)| (g, i)).collect();

    let nodes: Vec<GNode> = fn_of
        .iter()
        .map(|&g| {
            let (w, h) = node_size(project, g);
            GNode::simple(w, h)
        })
        .collect();

    let mut seen = std::collections::HashSet::new();
    let mut edges = Vec::new();
    for (i, &g) in fn_of.iter().enumerate() {
        for &callee in &project.fns[g].calls {
            if let Some(&j) = local.get(&callee)
                && i != j
                && seen.insert((i, j))
            {
                edges.push(GEdge {
                    from: EndPoint { node: i, port: 0 },
                    to: EndPoint { node: j, port: 0 },
                });
            }
        }
    }
    (Graph { nodes, edges }, fn_of)
}

/// Intrinsic node size: width tracks the longer of the two label lines so the
/// engine can pack columns tightly. Height grows with the fn's source-line
/// count (a cheap complexity proxy) so large fns read as visually massive —
/// the "where's the weight" half of the triage overlay.
fn node_size(project: &Project, fn_idx: usize) -> (f32, f32) {
    let f = &project.fns[fn_idx];
    let title_len = f.name.chars().count() + if f.is_sig { 6 } else { 0 };
    let sub_len = format!("{} args → {}", f.params.len(), short_ty(&f.ret))
        .chars()
        .count();
    let chars = title_len.max(sub_len) as f32;
    let w = (chars * 7.5 + 24.0).clamp(140.0, 300.0);
    let h = (40.0 + f.src_lines() as f32 * 1.4).clamp(46.0, 130.0);
    (w, h)
}

fn node_box(project: &Project, fn_idx: usize, selected_fn: Option<usize>, w: f32, h: f32) -> El {
    let f = &project.fns[fn_idx];
    let selected = selected_fn == Some(fn_idx);
    let title = if f.is_sig {
        format!("{}  (sig)", f.name)
    } else {
        f.name.clone()
    };
    let sub = format!("{} args → {}", f.params.len(), short_ty(&f.ret));
    // Hover reveals what the ellipsized box can't: the full signature, the home
    // file, and the triage metrics (with the orphan reason spelled out).
    let b = column([
        text(title).mono().font_size(TITLE_SIZE).nowrap_text().ellipsis(),
        text(sub).muted().font_size(SUB_SIZE).nowrap_text().ellipsis(),
    ])
    .gap(2.0)
    .padding(8.0)
    .radius(8.0)
    .width(Size::Fixed(w))
    .height(Size::Fixed(h))
    // Keyed (so clicks route and pan-drag skips them) but NOT focusable: the
    // auto hover/press envelope on focusable nodes flashes the fill as the
    // cursor sweeps across the dense graph. Selection highlight (below) is the
    // only per-node visual state we want.
    .key(format!("fn:{fn_idx}"))
    .tooltip(node_tip(project, fn_idx));
    // Triage overlay (when not selected): orphans flag as cut candidates;
    // everything else warms with connectivity so hubs stand out from leaves.
    if selected {
        b.fill(tokens::ACCENT).stroke(tokens::RING)
    } else if f.is_orphan() {
        b.fill(tokens::CARD.mix(tokens::DESTRUCTIVE, 0.5))
            .stroke(tokens::DESTRUCTIVE)
    } else if f.is_sig {
        b.fill(tokens::MUTED).stroke(tokens::BORDER)
    } else {
        let warmth = (f.degree() as f32 / 14.0).min(1.0);
        b.fill(tokens::CARD.mix(tokens::WARNING, warmth * 0.6))
            .stroke(tokens::BORDER)
    }
}

/// Hover text for a fn node/card: full signature, home file, triage metrics.
/// Shared with the Board view (its cards reuse the same reveal).
pub(crate) fn node_tip(project: &Project, fn_idx: usize) -> String {
    let f = &project.fns[fn_idx];
    let ps: Vec<String> = f.params.iter().map(|(n, t)| format!("({n} {t})")).collect();
    let kw = if f.is_sig { "sig " } else { "" };
    let mut tip = format!(
        "{kw}{}({}) → {}\n{}\n{} lines · {} calls · {} callers",
        f.name,
        ps.join(" "),
        f.ret,
        project.files[f.file].rel,
        f.src_lines(),
        f.calls.len(),
        f.callers.len()
    );
    if f.is_orphan() {
        tip.push_str("\n⚠ orphan — nothing calls it (cut candidate)");
    } else if f.proof_refd && f.callers.is_empty() {
        tip.push_str("\nproof subject — reasoned about, not called");
    }
    tip
}

/// Jump-to-other-view buttons for the selected fn, omitting the current view.
/// Each reuses an existing toolbar route (the fn / file is already selected, so
/// the mode switch lands on it), so no new event plumbing is needed.
fn nav_buttons(mode: ViewMode) -> El {
    let mut bs = Vec::new();
    if mode != ViewMode::Flow {
        bs.push(button("Flow ▸").key("mode_flow").secondary().tooltip("Chart this fn's body"));
    }
    if mode != ViewMode::Board {
        bs.push(button("Board ▸").key("mode_board").ghost().tooltip("This file's call DAG, expanded"));
    }
    if mode != ViewMode::Methods {
        bs.push(button("Graph ▸").key("mode_methods").ghost().tooltip("This file's call graph"));
    }
    row(bs).gap(tokens::SPACE_2)
}

/// Fixed width of the detail panel (the source view wraps against it).
const PANEL_W: f32 = 420.0;

pub(crate) fn detail_panel(project: &Project, fn_idx: usize, mode: ViewMode) -> El {
    let f = &project.fns[fn_idx];
    let sig: Vec<String> = f.params.iter().map(|(n, t)| format!("({n} {t})")).collect();

    // Callees (within project) and callers (reverse edges, precomputed).
    let callees = &f.calls;
    let callers = &f.callers;

    // Triage metrics + a cut-candidate / proof-subject tag.
    let mut metrics = format!(
        "{} lines · {} calls · {} callers",
        f.src_lines(),
        f.calls.len(),
        f.callers.len()
    );
    if f.is_orphan() {
        metrics.push_str("  ·  ⚠ orphan — cut candidate");
    } else if f.proof_refd && f.callers.is_empty() {
        metrics.push_str("  ·  proof subject");
    }

    let mut items = vec![
        row([h3(f.name.clone()), spacer()]).gap(tokens::SPACE_2),
        text(format!("({}) → {}", sig.join(" "), f.ret))
            .mono()
            .muted()
            .font_size(tokens::TEXT_SM.size)
            .wrap_text(),
        text(format!("in {}", project.files[f.file].rel))
            .caption()
            .muted(),
        text(metrics).caption().muted(),
        nav_buttons(mode),
        separator(),
        row([text("Source").label(), spacer(), text(format!("{} lines", f.src_lines())).caption().muted()]),
        if f.src.is_empty() {
            code_block("(signature only)")
        } else {
            // Syntax-highlighted + line-numbered, manually wrapped to a column
            // budget (the source is monospace, so a character count is an exact
            // width). Budget = panel content minus the panel + code-block
            // paddings (2×SPACE_3 each), the line-number gutter, and a scrollbar
            // gutter, divided by the mono glyph width; vertical overflow scrolls.
            const MONO_CH: f32 = 7.8; // JetBrains Mono advance at TEXT_SM
            let avail = PANEL_W - 4.0 * tokens::SPACE_3 - 12.0 - 6.0 * MONO_CH;
            let max_chars = (avail / MONO_CH).floor() as usize;
            scroll([super::highlight::source_view(&f.src, max_chars)]).height(Size::Fill(1.0))
        },
    ];

    items.push(separator());
    items.push(text(format!("Calls ({})", callees.len())).label());
    items.push(fn_link_list(project, callees, f.file));
    items.push(text(format!("Called by ({})", callers.len())).label());
    items.push(fn_link_list(project, callers, f.file));

    column(items)
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .width(Size::Fixed(PANEL_W))
        .height(Size::Fill(1.0))
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .radius(10.0)
}

/// A list of clickable fn links (jump targets for navigation). Cross-file
/// targets (file `!= home`) are disambiguated with their file stem, since
/// homonyms across files are common in shard and a bare name would be
/// ambiguous; the full path is on hover.
fn fn_link_list(project: &Project, fns: &[usize], home: usize) -> El {
    if fns.is_empty() {
        return text("—").muted().font_size(tokens::TEXT_SM.size);
    }
    let chips: Vec<El> = fns
        .iter()
        .map(|&j| {
            let g = &project.fns[j];
            let rel = &project.files[g.file].rel;
            let label = if g.file == home {
                g.name.clone()
            } else {
                format!("{}  · {}", g.name, file_stem(rel))
            };
            button(label)
                .key(format!("fn:{j}"))
                .ghost()
                .tooltip(format!("in {rel}"))
        })
        .collect();
    column(chips).gap(2.0)
}

/// The bare file name (no dir, no `.shard`) — a compact cross-file tag.
fn file_stem(rel: &str) -> &str {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    file.strip_suffix(".shard").unwrap_or(file)
}

/// Trim a string so it fits a node box / signature line.
fn short_ty(ty: &str) -> String {
    const MAX: usize = 22;
    if ty.chars().count() > MAX {
        let mut s: String = ty.chars().take(MAX - 1).collect();
        s.push('…');
        s
    } else {
        ty.to_string()
    }
}
