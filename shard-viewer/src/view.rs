//! The damascene view: pure functions from project state to an `El` tree.
//!
//! Kept separate from the `App` impl (in the `shard-viewer` bin) so the same
//! tree can be rendered headlessly — to SVG + a lint report — without a GPU or
//! a window. That headless render is the build-time review loop.

use crate::layout::{self, GraphLayout, Node};
use crate::model::Project;
use damascene_core::prelude::*;

/// Everything the view needs from the running app, snapshotted per frame.
pub struct ViewParams {
    pub selected_file: Option<usize>,
    pub selected_fn: Option<usize>,
    /// Current viewport zoom (read back from the runtime), for display only.
    pub zoom: f32,
}

/// Key of the pan/zoom viewport — also the target of `ViewportRequest`s.
pub const CANVAS_KEY: &str = "canvas";

const MIN_ZOOM: f32 = 0.15;
const MAX_ZOOM: f32 = 3.0;

const TITLE_SIZE: f32 = 13.0;
const SUB_SIZE: f32 = 11.0;

/// The whole window: sidebar + main pane + (when a fn is selected) detail panel.
pub fn app_root(project: &Project, p: &ViewParams) -> El {
    let mut panes = vec![
        sidebar(project, p.selected_file),
        main_pane(project, p),
    ];
    if let Some(fni) = p.selected_fn {
        panes.push(detail_panel(project, fni));
    }
    page([row(panes).gap(tokens::SPACE_4).height(Size::Fill(1.0))])
}

fn sidebar(project: &Project, selected_file: Option<usize>) -> El {
    let rows: Vec<El> = project
        .files
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let mut b = button(format!("{}  ({})", f.rel, f.fns.len()))
                .key(format!("file:{i}"))
                .ghost();
            if selected_file == Some(i) {
                b = b.selected();
            }
            b
        })
        .collect();
    column([h3("Files"), scroll(rows).height(Size::Fill(1.0))])
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .width(Size::Fixed(320.0))
        .height(Size::Fill(1.0))
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .radius(10.0)
}

fn main_pane(project: &Project, p: &ViewParams) -> El {
    let body = match p.selected_file {
        None => {
            column([text("Select a file to see its call graph.").muted()]).padding(tokens::SPACE_8)
        }
        Some(fi) => canvas(project, fi, p),
    };
    column([toolbar(project, p), body])
        .gap(tokens::SPACE_3)
        .width(Size::Fill(1.0))
        .height(Size::Fill(1.0))
}

fn toolbar(project: &Project, p: &ViewParams) -> El {
    let title = match p.selected_file {
        Some(fi) => h3(project.files[fi].rel.clone()),
        None => h3("shard-viewer"),
    };
    row([
        title,
        spacer(),
        text(format!("{:.0}%", p.zoom * 100.0))
            .mono()
            .muted()
            .center_text()
            .width(Size::Fixed(52.0)),
        button("Fit").key("fit").secondary(),
        button("Reset view").key("reset").ghost(),
    ])
    .gap(tokens::SPACE_2)
    .padding(tokens::SPACE_2)
}

fn canvas(project: &Project, file_idx: usize, p: &ViewParams) -> El {
    let gl = layout::layout_file(project, file_idx);
    if gl.nodes.is_empty() {
        return column([text("This file defines no fns.").muted()]).padding(tokens::SPACE_8);
    }

    let mut children: Vec<El> = Vec::with_capacity(gl.nodes.len() + 1);
    // Edge overlay, drawn in content coordinates; the viewport transform scales
    // it for free. Unkeyed so it never intercepts the background pan drag.
    children.push(vector(edges_asset(&gl)));
    for node in &gl.nodes {
        children.push(node_box(project, node, p.selected_fn));
    }

    let positions: Vec<(f32, f32, f32, f32)> =
        gl.nodes.iter().map(|n| (n.x, n.y, n.w, n.h)).collect();
    let (cw, ch) = (gl.width, gl.height);

    // The content layer: nodes placed at their absolute graph coordinates. No
    // pan/zoom math here — the `viewport()` wrapper bakes the transform into
    // descendant rects (hit-test included) and scales per-node chrome.
    let content = stack(children)
        .width(Size::Fixed(cw))
        .height(Size::Fixed(ch))
        .layout(move |ctx: LayoutCtx| {
            let o = ctx.container;
            let mut rects = Vec::with_capacity(positions.len() + 1);
            rects.push(Rect::new(o.x, o.y, cw, ch));
            for &(x, y, w, h) in &positions {
                rects.push(Rect::new(o.x + x, o.y + y, w, h));
            }
            rects
        });

    viewport([content])
        .key(CANVAS_KEY)
        .min_zoom(MIN_ZOOM)
        .max_zoom(MAX_ZOOM)
        .width(Size::Fill(1.0))
        .height(Size::Fill(1.0))
        .fill(tokens::BACKGROUND)
}

fn node_box(project: &Project, node: &Node, selected_fn: Option<usize>) -> El {
    let f = &project.fns[node.fn_idx];
    let selected = selected_fn == Some(node.fn_idx);
    let title = if f.is_sig {
        format!("{}  (sig)", f.name)
    } else {
        f.name.clone()
    };
    let sub = format!("{} args → {}", f.params.len(), short_ty(&f.ret));
    let b = column([
        text(title).mono().font_size(TITLE_SIZE).nowrap_text().ellipsis(),
        text(sub).muted().font_size(SUB_SIZE).nowrap_text().ellipsis(),
    ])
    .gap(2.0)
    .padding(8.0)
    .radius(8.0)
    .key(format!("fn:{}", node.fn_idx))
    .focusable();
    if selected {
        b.fill(tokens::ACCENT).stroke(tokens::RING)
    } else if f.is_sig {
        b.fill(tokens::MUTED).stroke(tokens::BORDER)
    } else {
        b.fill(tokens::CARD).stroke(tokens::BORDER)
    }
}

fn detail_panel(project: &Project, fn_idx: usize) -> El {
    let f = &project.fns[fn_idx];
    let sig: Vec<String> = f.params.iter().map(|(n, t)| format!("({n} {t})")).collect();

    // Callees (within project) and callers.
    let callees = &f.calls;
    let callers: Vec<usize> = (0..project.fns.len())
        .filter(|&j| project.fns[j].calls.contains(&fn_idx))
        .collect();

    let mut items = vec![
        row([h3(f.name.clone()), spacer()]).gap(tokens::SPACE_2),
        text(format!("({}) → {}", sig.join(" "), f.ret))
            .mono()
            .muted()
            .font_size(tokens::TEXT_SM.size),
        text(format!("in {}", project.files[f.file].rel))
            .caption()
            .muted(),
        separator(),
        text("Source").label(),
        scroll([code_block(if f.src.is_empty() {
            "(signature only)".to_string()
        } else {
            f.src.clone()
        })])
        .height(Size::Fill(1.0))
        .fill(tokens::BACKGROUND)
        .stroke(tokens::BORDER)
        .radius(8.0),
    ];

    items.push(separator());
    items.push(text(format!("Calls ({})", callees.len())).label());
    items.push(fn_link_list(project, callees));
    items.push(text(format!("Called by ({})", callers.len())).label());
    items.push(fn_link_list(project, &callers));

    column(items)
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .width(Size::Fixed(420.0))
        .height(Size::Fill(1.0))
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .radius(10.0)
}

/// A wrapped list of clickable fn links (jump targets for navigation).
fn fn_link_list(project: &Project, fns: &[usize]) -> El {
    if fns.is_empty() {
        return text("—").muted().font_size(tokens::TEXT_SM.size);
    }
    let chips: Vec<El> = fns
        .iter()
        .map(|&j| {
            let g = &project.fns[j];
            // Disambiguate cross-file targets with their module.
            let label = g.name.clone();
            button(label).key(format!("fn:{j}")).ghost()
        })
        .collect();
    column(chips).gap(2.0)
}

fn edges_asset(gl: &GraphLayout) -> VectorAsset {
    let mut paths = Vec::new();
    for &(a, b) in &gl.edges {
        let na = &gl.nodes[a];
        let nb = &gl.nodes[b];
        let x1 = na.x + na.w;
        let y1 = na.y + na.h / 2.0;
        let x2 = nb.x;
        let y2 = nb.y + nb.h / 2.0;
        let dx = (x2 - x1).abs().max(40.0);
        let (c1x, c2x) = (x1 + dx * 0.5, x2 - dx * 0.5);
        paths.push(
            PathBuilder::new()
                .move_to(x1, y1)
                .cubic_to(c1x, y1, c2x, y2, x2, y2)
                .stroke_solid(tokens::MUTED_FOREGROUND, 1.5)
                .build(),
        );
        paths.push(arrowhead(c2x, y2, x2, y2));
    }
    VectorAsset::from_paths([0.0, 0.0, gl.width, gl.height], paths)
}

/// A small filled triangle at `(tip_x, tip_y)` pointing along the direction
/// from `(from_x, from_y)` to the tip.
fn arrowhead(from_x: f32, from_y: f32, tip_x: f32, tip_y: f32) -> VectorPath {
    let (dx, dy) = (tip_x - from_x, tip_y - from_y);
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let (ux, uy) = (dx / len, dy / len);
    let (perp_x, perp_y) = (-uy, ux);
    const SIZE: f32 = 9.0;
    const HALF: f32 = 4.0;
    let bx = tip_x - ux * SIZE;
    let by = tip_y - uy * SIZE;
    PathBuilder::new()
        .move_to(tip_x, tip_y)
        .line_to(bx + perp_x * HALF, by + perp_y * HALF)
        .line_to(bx - perp_x * HALF, by - perp_y * HALF)
        .close()
        .fill_solid(tokens::MUTED_FOREGROUND)
        .build()
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
