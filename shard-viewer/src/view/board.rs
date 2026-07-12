//! Board view (experimental): the "circuit board" — one file's fns laid out as
//! the call DAG (same Sugiyama engine as Methods), but each node is rendered in
//! the *expanded* flow form (the LabVIEW-style region card from [`super::flow`])
//! instead of a name box. Call edges become wires between whole fn bodies, so
//! you read both a fn's internal structure and where its calls go at once.
//!
//! ## v1 caveat — node sizing
//! The Sugiyama engine needs node sizes up front, but a flow card's true size
//! falls out of nested-element layout (which is why it never overflows). So we
//! *estimate* each card's size from its region tree ([`est`], a structural
//! walk) and size the box to that estimate — the card then holds its content
//! exactly, at the cost of large fns producing large nodes (pan/zoom copes).
//! Tightening this (measured sizes, clamped thumbnails) is the obvious next
//! iteration; this file is deliberately isolated so variants are cheap to try.

use super::flow::{est, render_region, text_w};
use super::shared::graph_canvas;
use super::{SUB_SIZE, TITLE_SIZE, ViewParams};
use crate::flow::Region;
use crate::layout::{self, GNode, Graph};
use crate::model::Project;
use damascene_core::prelude::*;

pub(crate) fn canvas(project: &Project, file_idx: usize, p: &ViewParams) -> El {
    let (graph, fn_of) = super::methods::build_call_graph(project, file_idx);
    if graph.nodes.is_empty() {
        return column([text("This file defines no fns.").muted()]).padding(tokens::SPACE_8);
    }
    // Re-size each node to its expanded flow-card estimate (the call graph from
    // Methods carries name-box sizes, which are far too small for a full body).
    let sized = Graph {
        nodes: fn_of
            .iter()
            .map(|&g| {
                let (w, h) = card_size(project, g);
                GNode::simple(w, h)
            })
            .collect(),
        edges: graph.edges,
    };
    let lay = layout::layout(&sized, &layout::LayoutConfig::for_nodes(&sized.nodes));
    let node_els: Vec<El> = lay
        .nodes
        .iter()
        .enumerate()
        .map(|(i, pn)| card(project, fn_of[i], p.selected_fn, pn.w, pn.h))
        .collect();
    graph_canvas(&lay, node_els)
}

pub(crate) fn legend() -> El {
    row([
        text("board").mono().muted().font_size(SUB_SIZE),
        text("each node = a fn in expanded flow form · arrows point at callers")
            .muted()
            .font_size(SUB_SIZE),
    ])
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_2)
}

/// One fn as a titled flow card: a name/signature header over its region tree.
fn card(project: &Project, fn_idx: usize, selected_fn: Option<usize>, w: f32, h: f32) -> El {
    let f = &project.fns[fn_idx];
    let selected = selected_fn == Some(fn_idx);

    let title = row([
        text(f.name.clone())
            .mono()
            .semibold()
            .font_size(TITLE_SIZE)
            .text_color(tokens::FOREGROUND)
            .nowrap_text()
            .ellipsis(),
        spacer(),
        text(format!("{} args → {}", f.params.len(), short_ty(&f.ret)))
            .mono()
            .muted()
            .font_size(SUB_SIZE)
            .nowrap_text(),
    ])
    .gap(tokens::SPACE_2);

    let content = match body_region(f) {
        Some(region) => render_region(&region),
        None => text("(signature only)").muted().font_size(SUB_SIZE),
    };

    let card = column([title, content])
        .gap(tokens::SPACE_2)
        .padding(8.0)
        .width(Size::Fixed(w))
        .height(Size::Fixed(h))
        .radius(8.0)
        .key(format!("fn:{fn_idx}"))
        .tooltip(super::methods::node_tip(project, fn_idx));
    // Selection tints the fill (not just the stroke) so the focused card reads
    // at a glance across a large board, the way Methods highlights its node.
    if selected {
        card.fill(tokens::CARD.mix(tokens::ACCENT, 0.18)).stroke(tokens::RING)
    } else {
        card.fill(tokens::CARD).stroke(tokens::BORDER)
    }
}

/// The region tree for a fn's body, or `None` if it's a signature (no logic).
fn body_region(f: &crate::model::FnDef) -> Option<Region> {
    if f.body.is_empty() || f.body.iter().all(|form| form.head() == Some("measure")) {
        None
    } else {
        Some(Region::build(&f.body))
    }
}

/// Estimate a card's pixel size from its region tree (header + content). This
/// feeds the layout engine, which needs sizes before the elements exist.
fn card_size(project: &Project, fn_idx: usize) -> (f32, f32) {
    let f = &project.fns[fn_idx];
    let (cw, ch) = match body_region(f) {
        Some(region) => est(&region),
        None => (120.0, 26.0),
    };
    let title_w = text_w(&f.name, 8.5)
        + text_w(&format!("{} args → {}", f.params.len(), short_ty(&f.ret)), 7.0)
        + 24.0;
    let w = title_w.max(cw) + 16.0;
    let h = ch + 44.0; // header line + gaps + padding
    (w, h)
}

/// Trim a type string for the card header (mirrors Methods).
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
