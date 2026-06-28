//! Cross-view primitives: the pan/zoom viewport, the laid-out-graph canvas
//! (edge overlay + absolutely-placed node elements), the edge/arrowhead vector
//! builders, and the tiny legend atoms. Shared by every visualization variant
//! so each view file holds only what's unique to it.

use super::{CANVAS_KEY, SUB_SIZE};
use crate::layout::Layout;
use damascene_core::prelude::*;

// Low enough that even the densest file (driver.shard ~ 4000×6000 content)
// fits the frame on `Fit`; FitContent never zooms below the true fit, so this
// only governs how far the user may manually zoom out.
const MIN_ZOOM: f32 = 0.04;
const MAX_ZOOM: f32 = 3.0;

/// A small colored square (an empty box used purely as a swatch).
pub(crate) fn swatch(color: Color, side: f32) -> El {
    column(Vec::<El>::new())
        .width(Size::Fixed(side))
        .height(Size::Fixed(side))
        .radius(4.0)
        .fill(color)
        .stroke(tokens::BORDER)
}

/// A small colored chip + label, for a legend.
pub(crate) fn legend_chip(color: Color, label: &str) -> El {
    row([swatch(color, 14.0), text(label).muted().font_size(SUB_SIZE)]).gap(tokens::SPACE_2)
}

/// Wrap a laid-out graph in the pan/zoom viewport: an edge overlay plus the
/// per-node elements (index-aligned with `lay.nodes`), placed at their content
/// coordinates via the `El::layout` escape hatch.
pub(crate) fn graph_canvas(lay: &Layout, node_els: Vec<El>) -> El {
    graph_canvas_edges(lay, node_els, edges_asset(lay))
}

/// Like [`graph_canvas`] but with a caller-supplied edge overlay, so views that
/// style edges by kind (e.g. flow's control vs data) can build their own.
pub(crate) fn graph_canvas_edges(lay: &Layout, node_els: Vec<El>, edges: VectorAsset) -> El {
    pan_zoom_viewport(placed_graph(lay, node_els, edges))
}

/// The absolutely-placed content layer of a laid-out graph: an edge overlay
/// plus the per-node elements positioned at their content coordinates, in a
/// `Fixed(width) × Fixed(height)` stack — **without** the pan/zoom viewport.
///
/// Sized definitely (the layout's bounds), so it composes: a block laid out
/// this way reports a real intrinsic size to its parent, letting the Map nest
/// graph layouts (fns inside a file, files inside a dir) and wrap the whole
/// thing in a single viewport at the top. The `.layout()` escape hatch reads
/// each level's own `ctx.container` origin, so nested placements compose.
pub(crate) fn placed_graph(lay: &Layout, node_els: Vec<El>, edges: VectorAsset) -> El {
    let mut children: Vec<El> = Vec::with_capacity(node_els.len() + 1);
    // Edge overlay, drawn in content coordinates. Unkeyed so it never intercepts
    // a background pan drag.
    children.push(vector(edges));
    children.extend(node_els);

    let positions: Vec<(f32, f32, f32, f32)> =
        lay.nodes.iter().map(|n| (n.x, n.y, n.w, n.h)).collect();
    let (cw, ch) = (lay.width, lay.height);

    stack(children)
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
        })
}

/// Wrap `content` in the shared pan/zoom viewport. The content sizes itself
/// (absolute-positioned graph stack, or a self-sizing nested flow tree); the
/// `viewport()` bakes the pan/zoom transform into descendant rects + hit-test.
pub(crate) fn pan_zoom_viewport(content: El) -> El {
    // No `.fill()` here: the per-node hover envelope brightens the fill of any
    // keyed node, and the viewport must be keyed (for ViewportRequest + state).
    // A fill would flash as the cursor transits between the background and node
    // children. Left unfilled, the canvas shows the window BACKGROUND instead.
    viewport([content])
        .key(CANVAS_KEY)
        .min_zoom(MIN_ZOOM)
        .max_zoom(MAX_ZOOM)
        // Center bounds: any node can be parked mid-frame (the default Contain
        // keeps the bbox glued to the edges, which fights graph navigation).
        .pan_bounds(PanBounds::Center)
        .width(Size::Fill(1.0))
        .height(Size::Fill(1.0))
}

pub(crate) fn edges_asset(lay: &Layout) -> VectorAsset {
    let mut paths = Vec::new();
    for e in &lay.edges {
        if e.points.len() < 2 {
            continue;
        }
        // Mutual-recursion return arcs get a dimmer, distinct tint so they read
        // as cycles rather than mystery lines crossing the flow.
        let color = if e.back {
            tokens::ACCENT
        } else {
            tokens::MUTED_FOREGROUND
        };
        paths.push(edge_curve(&e.points, e.back, color, 1.5));
        let n = e.points.len();
        let (fx, fy) = e.points[n - 2];
        let (tx, ty) = e.points[n - 1];
        paths.push(arrowhead(fx, fy, tx, ty, color));
    }
    VectorAsset::from_paths([0.0, 0.0, lay.width, lay.height], paths)
}

/// Build a stroked path along an edge's routed polyline as a smooth spline:
/// forward edges leave the out-port and enter the in-port horizontally with
/// Catmull-Rom interiors through the dummy bends; same-column return arcs
/// (`back`) curve out the right side and back without horizontal port tangents.
pub(crate) fn edge_curve(pts: &[(f32, f32)], back: bool, color: Color, width: f32) -> VectorPath {
    let n = pts.len();
    let mut pb = PathBuilder::new().move_to(pts[0].0, pts[0].1);
    for i in 0..n - 1 {
        let p0 = pts[i];
        let p1 = pts[i + 1];
        // Outgoing tangent at p0: horizontal off the out-port for a forward
        // edge; Catmull-Rom (neighbour-based) elsewhere.
        let m0 = if i == 0 && !back {
            ((p1.0 - p0.0).max(40.0) * 0.5, 0.0)
        } else if i == 0 {
            ((p1.0 - p0.0) / 3.0, (p1.1 - p0.1) / 3.0)
        } else {
            let pm = pts[i - 1];
            ((p1.0 - pm.0) / 6.0, (p1.1 - pm.1) / 6.0)
        };
        // Incoming tangent at p1.
        let m1 = if i + 1 == n - 1 && !back {
            (-(p1.0 - p0.0).max(40.0) * 0.5, 0.0)
        } else if i + 1 == n - 1 {
            (-(p1.0 - p0.0) / 3.0, -(p1.1 - p0.1) / 3.0)
        } else {
            let pp = pts[i + 2];
            (-(pp.0 - p0.0) / 6.0, -(pp.1 - p0.1) / 6.0)
        };
        pb = pb.cubic_to(p0.0 + m0.0, p0.1 + m0.1, p1.0 + m1.0, p1.1 + m1.1, p1.0, p1.1);
    }
    pb.stroke_solid(color, width).build()
}

/// A small filled triangle at `(tip_x, tip_y)` pointing along the direction
/// from `(from_x, from_y)` to the tip.
pub(crate) fn arrowhead(from_x: f32, from_y: f32, tip_x: f32, tip_y: f32, color: Color) -> VectorPath {
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
        .fill_solid(color)
        .build()
}
