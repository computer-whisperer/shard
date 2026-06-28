//! Systems view: the project-wide file import dependency graph with a
//! proof-vs-impl category heat map (node tint + composition bar), plus the
//! per-file breakdown side panel.

use super::shared::{graph_canvas, legend_chip, swatch};
use super::{SUB_SIZE, TITLE_SIZE, ViewParams};
use crate::layout::{self, EndPoint, GEdge, GNode, Graph};
use crate::model::Project;
use damascene_core::prelude::*;
use std::collections::HashMap;

pub(crate) fn canvas(project: &Project, p: &ViewParams) -> El {
    let (graph, file_of) = build_systems_graph(project);
    if graph.nodes.is_empty() {
        return column([text("No in-project imports to graph.").muted()]).padding(tokens::SPACE_8);
    }
    let lay = layout::layout(&graph, &layout::LayoutConfig::default());
    let node_els: Vec<El> = lay
        .nodes
        .iter()
        .enumerate()
        .map(|(i, pn)| sys_node_box(project, file_of[i], p.scope.focus_file(project), pn.w, pn.h))
        .collect();
    graph_canvas(&lay, node_els)
}

/// The Systems-view heat key: node tint encodes proof-vs-impl share, and each
/// node carries a stacked composition bar.
pub(crate) fn legend() -> El {
    let warm = tokens::ACCENT.mix(tokens::WARNING, 1.0);
    let cool = tokens::ACCENT.mix(tokens::WARNING, 0.0);
    row([
        text("heat").mono().muted().font_size(SUB_SIZE),
        legend_chip(tokens::CARD.mix(cool, 0.4), "impl-heavy"),
        legend_chip(tokens::CARD.mix(tokens::ACCENT.mix(tokens::WARNING, 0.5), 0.4), "mixed"),
        legend_chip(tokens::CARD.mix(warm, 0.4), "proof-heavy"),
        text("·  bar").mono().muted().font_size(SUB_SIZE),
        legend_chip(tokens::ACCENT, "impl"),
        legend_chip(tokens::WARNING, "proof"),
        legend_chip(tokens::BORDER, "comment/blank"),
    ])
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_2)
}

/// Heat tint for a file node: cool (ACCENT) for implementation-heavy files,
/// warm (WARNING) for proof-heavy ones, blended over CARD so labels stay
/// readable. Files with no substantive code read neutral (plain CARD).
fn heat_fill(share: Option<f32>) -> Color {
    match share {
        None => tokens::CARD,
        Some(s) => tokens::CARD.mix(tokens::ACCENT.mix(tokens::WARNING, s), 0.4),
    }
}

/// A thin stacked bar showing a file's line composition: implementation, then
/// proof burden, over a track that shows the comment/blank remainder. `inner_w`
/// is the available width inside the node's padding.
fn composition_bar(c: &crate::model::Counts, inner_w: f32) -> El {
    let total = c.total().max(1) as f32;
    let seg = |n: u32, color: Color| -> Option<El> {
        let w = (n as f32 / total) * inner_w;
        (w >= 0.5).then(|| {
            column(Vec::<El>::new())
                .width(Size::Fixed(w))
                .height(Size::Fixed(6.0))
                .fill(color)
        })
    };
    let mut segs = Vec::new();
    segs.extend(seg(c.impl_lines(), tokens::ACCENT));
    segs.extend(seg(c.proof_lines(), tokens::WARNING));
    row(segs)
        .gap(0.0)
        .width(Size::Fixed(inner_w))
        .height(Size::Fixed(6.0))
        .radius(3.0)
        .fill(tokens::BORDER) // the uncovered track = comment/blank remainder
}

/// Build the project-wide import dependency graph (file → files it imports).
/// Only files that import or are imported participate, so isolated files don't
/// clutter the canvas. Returns the file index behind each graph node.
fn build_systems_graph(project: &Project) -> (Graph, Vec<usize>) {
    let imported: std::collections::HashSet<usize> = project
        .files
        .iter()
        .flat_map(|f| f.import_targets.iter().copied())
        .collect();
    let participating: Vec<usize> = (0..project.files.len())
        .filter(|&i| !project.files[i].import_targets.is_empty() || imported.contains(&i))
        .collect();
    let local: HashMap<usize, usize> =
        participating.iter().enumerate().map(|(li, &fi)| (fi, li)).collect();

    let nodes: Vec<GNode> = participating
        .iter()
        .map(|&fi| {
            let (w, h) = file_node_size(project, fi);
            GNode::simple(w, h)
        })
        .collect();

    let mut edges = Vec::new();
    for (&fi, &li) in &local {
        for &target in &project.files[fi].import_targets {
            if let Some(&lj) = local.get(&target)
                && li != lj
            {
                edges.push(GEdge {
                    from: EndPoint { node: li, port: 0 },
                    to: EndPoint { node: lj, port: 0 },
                });
            }
        }
    }
    (Graph { nodes, edges }, participating)
}

fn file_node_size(project: &Project, file_idx: usize) -> (f32, f32) {
    let (stem, dir) = file_label(&project.files[file_idx].rel);
    let chars = stem.chars().count().max(dir.chars().count()) as f32;
    let w = (chars * 7.0 + 24.0).clamp(130.0, 280.0);
    (w, 58.0) // extra height for the composition bar
}

/// Split a rel path into (file stem, parent dir) for a compact node label.
fn file_label(rel: &str) -> (String, String) {
    let (dir, file) = rel.rsplit_once('/').unwrap_or(("", rel));
    let stem = file.strip_suffix(".shard").unwrap_or(file);
    (stem.to_string(), dir.to_string())
}

fn sys_node_box(project: &Project, file_idx: usize, selected_file: Option<usize>, w: f32, h: f32) -> El {
    let f = &project.files[file_idx];
    let (stem, dir) = file_label(&f.rel);
    let selected = selected_file == Some(file_idx);
    let sub = if dir.is_empty() {
        format!("{} fns", f.fns.len())
    } else {
        format!("{dir}  ·  {} fns", f.fns.len())
    };
    let b = column([
        text(stem).mono().font_size(TITLE_SIZE).nowrap_text().ellipsis(),
        text(sub).muted().font_size(SUB_SIZE).nowrap_text().ellipsis(),
        composition_bar(&f.counts, w - 16.0),
    ])
    .gap(3.0)
    .padding(8.0)
    .radius(8.0)
    .width(Size::Fixed(w))
    .height(Size::Fixed(h))
    .key(format!("sysfile:{file_idx}"))
    .tooltip(format!(
        "{}\n{} lines · {} fns\nimpl {} · proof {} · comment/blank {}",
        f.rel,
        f.counts.total(),
        f.fns.len(),
        f.counts.impl_lines(),
        f.counts.proof_lines(),
        f.counts.comment + f.counts.blank,
    ));
    // Tint by proof-vs-impl share so the verification-heavy corners of the tree
    // stand out at a glance; selection still wins for the focused node.
    if selected {
        b.fill(tokens::ACCENT).stroke(tokens::RING)
    } else {
        b.fill(heat_fill(f.counts.proof_share())).stroke(tokens::BORDER)
    }
}

/// Systems-mode side panel: the selected file's line-category breakdown plus
/// its import in/out degree, with a button to drill into its call graph.
pub(crate) fn detail_panel(project: &Project, file_idx: usize) -> El {
    let f = &project.files[file_idx];
    let c = &f.counts;
    let imported_by = project
        .files
        .iter()
        .filter(|g| g.import_targets.contains(&file_idx))
        .count();

    // One labelled, swatched, right-aligned count row.
    let cat_row = |label: &str, n: u32, color: Color| -> El {
        row([
            swatch(color, 12.0),
            text(label.to_string()).font_size(SUB_SIZE),
            spacer(),
            text(n.to_string()).mono().muted().font_size(SUB_SIZE),
        ])
        .gap(tokens::SPACE_2)
    };

    let items = vec![
        row([h3(file_label(&f.rel).0), spacer()]).gap(tokens::SPACE_2),
        text(f.rel.clone()).caption().muted(),
        button("Open call graph ▸").key(format!("open:{file_idx}")).secondary(),
        separator(),
        text(format!("{} lines · {} fns", c.total(), f.fns.len()))
            .caption()
            .muted(),
        composition_bar(c, 384.0),
        separator(),
        cat_row("impl", c.impl_, tokens::ACCENT),
        cat_row("measure", c.measure, tokens::WARNING),
        cat_row("proof", c.proof, tokens::WARNING),
        cat_row("reqproof", c.reqproof, tokens::WARNING),
        cat_row("req", c.req, tokens::ACCENT),
        cat_row("sidecar", c.sidecar, tokens::WARNING),
        cat_row("comment", c.comment, tokens::BORDER),
        cat_row("blank", c.blank, tokens::BORDER),
        separator(),
        text(format!(
            "imports {} · imported by {imported_by}",
            f.import_targets.len()
        ))
        .caption()
        .muted(),
    ];

    column(items)
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .width(Size::Fixed(420.0))
        .height(Size::Fill(1.0))
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .radius(10.0)
}
