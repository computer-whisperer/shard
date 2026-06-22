//! The damascene view: pure functions from project state to an `El` tree.
//!
//! Kept separate from the `App` impl (in the `shard-viewer` bin) so the same
//! tree can be rendered headlessly — to SVG + a lint report — without a GPU or
//! a window. That headless render is the build-time review loop.

use crate::layout::{self, GraphLayout, Node};
use crate::model::Project;
use damascene_core::prelude::*;

/// The whole window: sidebar + main pane.
pub fn app_root(project: &Project, selected_file: Option<usize>, selected_fn: Option<usize>) -> El {
    page([
        row([
            sidebar(project, selected_file),
            main_pane(project, selected_file, selected_fn),
        ])
        .gap(tokens::SPACE_4),
    ])
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

fn main_pane(project: &Project, selected_file: Option<usize>, selected_fn: Option<usize>) -> El {
    let body = match selected_file {
        None => {
            column([text("Select a file to see its call graph.").muted()]).padding(tokens::SPACE_8)
        }
        Some(fi) => canvas(project, fi, selected_fn),
    };
    column([header(project, selected_file, selected_fn), body])
        .gap(tokens::SPACE_3)
        .width(Size::Fill(1.0))
        .height(Size::Fill(1.0))
}

fn header(project: &Project, selected_file: Option<usize>, selected_fn: Option<usize>) -> El {
    let mut items = vec![match selected_file {
        Some(fi) => h3(project.files[fi].rel.clone()),
        None => h3("shard-viewer"),
    }];
    items.push(spacer());
    if let Some(fni) = selected_fn {
        let f = &project.fns[fni];
        let sig: Vec<String> = f.params.iter().map(|(n, t)| format!("({n} {t})")).collect();
        items.push(
            text(format!("fn {} ({}) → {}", f.name, sig.join(" "), f.ret))
                .code()
                .muted(),
        );
    }
    row(items).gap(tokens::SPACE_3).padding(tokens::SPACE_2)
}

fn canvas(project: &Project, file_idx: usize, selected_fn: Option<usize>) -> El {
    let gl = layout::layout_file(project, file_idx);
    if gl.nodes.is_empty() {
        return column([text("This file defines no fns.").muted()]).padding(tokens::SPACE_8);
    }

    let mut children: Vec<El> = Vec::with_capacity(gl.nodes.len() + 1);
    children.push(vector(edges_asset(&gl)).key("edges"));
    for node in &gl.nodes {
        children.push(node_box(project, node, selected_fn));
    }

    // Captured by the layout closure (must be 'static): node rects in canvas
    // coordinates plus the full canvas size for the edge overlay.
    let positions: Vec<(f32, f32, f32, f32)> =
        gl.nodes.iter().map(|n| (n.x, n.y, n.w, n.h)).collect();
    let (cw, ch) = (gl.width, gl.height);

    let canvas = stack(children)
        .width(Size::Fixed(cw))
        .height(Size::Fixed(ch))
        .layout(move |ctx: LayoutCtx| {
            let mut rects = Vec::with_capacity(positions.len() + 1);
            let o = ctx.container;
            // child 0 is the edge overlay: fills the whole canvas so its
            // view_box maps 1:1 onto canvas coordinates.
            rects.push(Rect::new(o.x, o.y, cw, ch));
            for &(x, y, w, h) in &positions {
                rects.push(Rect::new(o.x + x, o.y + y, w, h));
            }
            rects
        });

    scroll([canvas]).width(Size::Fill(1.0)).height(Size::Fill(1.0))
}

fn node_box(project: &Project, node: &Node, selected_fn: Option<usize>) -> El {
    let f = &project.fns[node.fn_idx];
    let selected = selected_fn == Some(node.fn_idx);
    let title = if f.is_sig {
        format!("{}  (sig)", short_ty(&f.name))
    } else {
        short_ty(&f.name)
    };
    let sub = format!("{} args → {}", f.params.len(), short_ty(&f.ret));
    let b = column([text(title).code(), text(sub).caption().muted()])
        .gap(2.0)
        .padding(tokens::SPACE_2)
        .width(Size::Fixed(layout::NODE_W))
        .height(Size::Fixed(layout::NODE_H))
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
    let (px, py) = (-uy, ux); // perpendicular
    const SIZE: f32 = 9.0;
    const HALF: f32 = 4.0;
    let bx = tip_x - ux * SIZE;
    let by = tip_y - uy * SIZE;
    PathBuilder::new()
        .move_to(tip_x, tip_y)
        .line_to(bx + px * HALF, by + py * HALF)
        .line_to(bx - px * HALF, by - py * HALF)
        .close()
        .fill_solid(tokens::MUTED_FOREGROUND)
        .build()
}

/// Trim a return-type string so it fits a node box.
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
