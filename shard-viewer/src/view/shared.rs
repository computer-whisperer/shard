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

/// The proof-vs-impl heat color for a file: cool (ACCENT) when
/// implementation-heavy, warm (WARNING) when proof-heavy — the project-wide
/// **amber = proof** convention. `None` (a file with no substantive code)
/// stays `None`: it reads neutral, not cold.
pub(crate) fn heat_color(proof_share: Option<f32>) -> Option<Color> {
    proof_share.map(|s| tokens::ACCENT.mix(tokens::WARNING, s))
}

/// A thin stacked bar showing a file's line composition: implementation, then
/// proof burden, over a track that shows the comment/blank remainder. Sized by
/// the caller — screen px inside a panel, content-space (zoom-scaled) on the
/// Map's boxes.
pub(crate) fn composition_bar(c: &crate::model::Counts, w: f32, h: f32) -> El {
    let total = c.total().max(1) as f32;
    let seg = |n: u32, color: Color| -> Option<El> {
        let sw = (n as f32 / total) * w;
        (sw >= 0.5).then(|| {
            column(Vec::<El>::new())
                .width(Size::Fixed(sw))
                .height(Size::Fixed(h))
                .fill(color)
        })
    };
    let mut segs = Vec::new();
    segs.extend(seg(c.impl_lines(), tokens::ACCENT));
    segs.extend(seg(c.proof_lines(), tokens::WARNING));
    row(segs)
        .gap(0.0)
        .width(Size::Fixed(w))
        .height(Size::Fixed(h))
        .radius(h / 2.0)
        .fill(tokens::BORDER) // the uncovered track = comment/blank remainder
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
    pan_zoom_viewport_min(content, MIN_ZOOM)
}

/// [`pan_zoom_viewport`] with a caller-chosen zoom-out floor — the Map derives
/// it from the committed extent (a project-wide map needs to fit at ~0.005,
/// where a single small file would zoom out into nothing at that floor).
pub(crate) fn pan_zoom_viewport_min(content: El, min_zoom: f32) -> El {
    // No `.fill()` here: the per-node hover envelope brightens the fill of any
    // keyed node, and the viewport must be keyed (for ViewportRequest + state).
    // A fill would flash as the cursor transits between the background and node
    // children. Left unfilled, the canvas shows the window BACKGROUND instead.
    viewport([content])
        .key(CANVAS_KEY)
        .min_zoom(min_zoom)
        .max_zoom(MAX_ZOOM)
        // Open fitted (and stay fitted through resizes) until the user pans or
        // zooms; the toolbar's Fit re-arms it. Also what makes the headless
        // `shard-render` frame the graph instead of an unfitted 1:1 corner.
        .fit_policy(FitPolicy::Contain { padding: 24.0 })
        // Center bounds: any node can be parked mid-frame (the default Contain
        // keeps the bbox glued to the edges, which fights graph navigation).
        .pan_bounds(PanBounds::Center)
        .width(Size::Fill(1.0))
        .height(Size::Fill(1.0))
}

pub(crate) fn edges_asset(lay: &Layout) -> VectorAsset {
    edges_asset_scaled(lay, 1.0)
}

/// How an intra-level edge reads: the call/import web, a proof-layer lemma
/// citation, a claim-subject link, or the shape layer's composition/usage
/// webs. Colors keep the project-wide convention (Systems view heat):
/// **amber = proof**, and now **blue = shape** (`tokens::INFO`). All classes
/// run dependency → dependent (the cascade convention): arrows point at users.
#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum EdgeClass {
    /// callee → caller (or dir-level imported → importer) — the
    /// implementation web.
    Flow,
    /// cited claim/axiom → citing claim — the proof-dependency web.
    Cite,
    /// subject fn → claim about it — where a statement touches down on code.
    About,
    /// field type → containing type — the datastructure composition web.
    /// Always drawn (it's sparse and load-bearing).
    Shape,
    /// type → fn that constructs/matches it (or merely names it in its
    /// signature) — the shape-usage web. Dense, so it participates in layout
    /// but is only *drawn* when a hover/selection reveals it (see the Map's
    /// reveal overlay).
    Use,
}

/// [`edges_asset`] with stroke widths and arrowheads multiplied by `scale`.
/// The Map draws content at zooms far below 1:1 where a 1.5-content-px spline
/// disappears; passing `1/zoom` (clamped ≥ 1) keeps edges hairline-visible on
/// screen at any distance — cartographic line weight, not content geometry.
pub(crate) fn edges_asset_scaled(lay: &Layout, scale: f32) -> VectorAsset {
    edges_asset_classed(lay, scale, &[])
}

/// [`edges_asset_scaled`] with a per-edge class, index-aligned with the input
/// `Graph::edges` (as `Layout::edges` is). Missing entries read as [`EdgeClass::Flow`].
pub(crate) fn edges_asset_classed(lay: &Layout, scale: f32, classes: &[EdgeClass]) -> VectorAsset {
    edges_asset_filtered(lay, scale, classes, |_| 1.0)
}

/// [`edges_asset_classed`] drawing each edge at the opacity `alpha` returns
/// for its index (`<= 0` skips it entirely). The Map uses this to tier one
/// committed edge set: the local web at full strength, long-haul edges faded
/// to context, the hover-revealed [`EdgeClass::Use`] overlay, and the focused
/// member's full-strength trace — visibility is a render choice; the
/// committed layout keeps every edge.
pub(crate) fn edges_asset_filtered(
    lay: &Layout,
    scale: f32,
    classes: &[EdgeClass],
    alpha: impl Fn(usize) -> f32,
) -> VectorAsset {
    let mut paths = Vec::new();
    for (k, e) in lay.edges.iter().enumerate() {
        let a = alpha(k);
        if e.points.len() < 2 || a <= 0.0 {
            continue;
        }
        let class = classes.get(k).copied().unwrap_or(EdgeClass::Flow);
        let color = match class {
            // Mutual-recursion return arcs get a dimmer, distinct tint so they
            // read as cycles rather than mystery lines crossing the flow.
            EdgeClass::Flow if e.back => tokens::ACCENT,
            EdgeClass::Flow => tokens::MUTED_FOREGROUND,
            EdgeClass::Cite => tokens::WARNING.mix(tokens::MUTED_FOREGROUND, 0.25),
            EdgeClass::About => tokens::WARNING.mix(tokens::MUTED_FOREGROUND, 0.7),
            EdgeClass::Shape => tokens::INFO.mix(tokens::MUTED_FOREGROUND, 0.35),
            EdgeClass::Use => tokens::INFO.mix(tokens::MUTED_FOREGROUND, 0.15),
        };
        let color = if a < 1.0 { color.with_alpha(a) } else { color };
        // The head must point along the spline's ARRIVAL tangent — horizontal
        // for a forward edge (edge_curve forces the port tangent), the last
        // segment for a return arc — not the raw last polyline segment, which
        // can be diagonal. And the stroke stops at the head's base rather
        // than piercing through the triangle to the tip.
        let n = e.points.len();
        let (fx, fy) = e.points[n - 2];
        let (tx, ty) = e.points[n - 1];
        let (ux, uy) = if e.back {
            let (dx, dy) = (tx - fx, ty - fy);
            let len = (dx * dx + dy * dy).sqrt().max(0.001);
            (dx / len, dy / len)
        } else {
            (1.0, 0.0)
        };
        let head = 9.0 * scale;
        let mut pts = e.points.clone();
        pts[n - 1] = (tx - ux * (head - 1.0), ty - uy * (head - 1.0));
        paths.push(edge_curve(&pts, e.back, color, 1.5 * scale));
        paths.push(arrowhead_scaled(tx - ux, ty - uy, tx, ty, color, scale));
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
    arrowhead_scaled(from_x, from_y, tip_x, tip_y, color, 1.0)
}

/// [`arrowhead`] scaled by `scale` (see [`edges_asset_scaled`]).
pub(crate) fn arrowhead_scaled(
    from_x: f32,
    from_y: f32,
    tip_x: f32,
    tip_y: f32,
    color: Color,
    scale: f32,
) -> VectorPath {
    let (dx, dy) = (tip_x - from_x, tip_y - from_y);
    let len = (dx * dx + dy * dy).sqrt().max(0.001);
    let (ux, uy) = (dx / len, dy / len);
    let (perp_x, perp_y) = (-uy, ux);
    let size: f32 = 9.0 * scale;
    let half: f32 = 4.0 * scale;
    let bx = tip_x - ux * size;
    let by = tip_y - uy * size;
    PathBuilder::new()
        .move_to(tip_x, tip_y)
        .line_to(bx + perp_x * half, by + perp_y * half)
        .line_to(bx - perp_x * half, by - perp_y * half)
        .close()
        .fill_solid(color)
        .build()
}
