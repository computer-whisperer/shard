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

use super::flow::render_region;
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
    let lay = layout::layout(&sized, &layout::LayoutConfig::default());
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
        text("each node = a fn in expanded flow form · arrows = calls between fns")
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

/// Approximate the rendered (w, h) of a region, mirroring the view's element
/// structure closely enough that the box holds its content. Deliberately a
/// slight over-estimate (no clipping is better than overlap).
fn est(r: &Region) -> (f32, f32) {
    match r {
        Region::Var(name) => (text_w(name, 8.0) + 14.0, 26.0),
        Region::Lit(value) => {
            let chars = if value.is_empty() { 1 } else { value.chars().count() };
            (chars as f32 * 7.5 + 12.0, 24.0)
        }
        Region::Op { head, inline, args } => {
            let card_w = text_w(head, 8.5).max(text_w(inline, 7.0)) + 16.0;
            let card_h = if inline.is_empty() { 32.0 } else { 48.0 };
            if args.is_empty() {
                return (card_w, card_h);
            }
            // Compound operands stack in a column to the left, wired in.
            let mut block_w = 0.0_f32;
            let mut block_h = 0.0_f32;
            for (i, a) in args.iter().enumerate() {
                let (aw, ah) = est(a);
                block_w = block_w.max(aw + 26.0); // wire stub + gap
                block_h += ah;
                if i > 0 {
                    block_h += 6.0;
                }
            }
            (block_w + card_w + 6.0, card_h.max(block_h))
        }
        Region::Seq(steps) => {
            // A railed column of step rungs (proof ladders; fn bodies never
            // produce Seq, so the Board only meets this via future reuse).
            let mut w = 0.0_f32;
            let mut h = 0.0_f32;
            for (i, s) in steps.iter().enumerate() {
                let (sw, sh) = est(s);
                w = w.max(sw);
                h += sh;
                if i > 0 {
                    h += 4.0;
                }
            }
            (w + 11.0, h.max(20.0)) // rail + gap
        }
        Region::List { elems, tail } => {
            // header (`list · N`) over a bracketed column of element regions.
            let mut body_w = 0.0_f32;
            let mut body_h = 0.0_f32;
            for (i, e) in elems.iter().enumerate() {
                let (ew, eh) = est(e);
                body_w = body_w.max(ew);
                body_h += eh;
                if i > 0 {
                    body_h += 5.0;
                }
            }
            if let Some(t) = tail {
                let (tw, th) = est(t);
                body_w = body_w.max(tw + 16.0); // "⋯ " lead
                body_h += th.max(20.0) + 5.0;
            }
            let w = (body_w + 9.0 + 12.0).max(54.0); // bracket bar + paddings
            let h = 16.0 + body_h + 12.0; // header + body + paddings
            (w, h)
        }
        Region::Frame { kind, detail, branches } => {
            let band_w = text_w(kind.keyword(), 7.5) + text_w(detail, 7.0) + 20.0;
            // Branches stack in a column; each is its selector chip + region.
            let mut body_w = 0.0_f32;
            let mut body_h = 0.0_f32;
            for (i, b) in branches.iter().enumerate() {
                let (rw, rh) = est(&b.region);
                let chip_w = text_w(&b.label, 7.0) + 10.0;
                body_w = body_w.max(chip_w + 8.0 + rw);
                body_h += rh.max(24.0);
                if i > 0 {
                    body_h += 7.0;
                }
            }
            let w = band_w.max(body_w + 16.0);
            let h = 26.0 + body_h + 16.0; // band + body + paddings
            (w, h)
        }
    }
}

fn text_w(s: &str, per: f32) -> f32 {
    s.chars().count() as f32 * per
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
