//! Render pass: geometry + screen → elements. Never feeds back into layout.
//!
//! Walks the committed levels at absolute content coordinates, culls against
//! the viewport, and prices every footprint's drawing by its size *on
//! screen*: boxes reveal contents past [`super::CONTENTS_PX`], fn slots
//! their innards past the flow threshold, labels draw at screen-constant
//! (cartographic) sizes, and the edge overlay tiers by attention cost.

use super::cards::{
    claim_card, claim_colors, claim_tip, filedoc_card, flow_card, orphan_colors, type_card,
    type_colors, type_tip,
};
use super::{
    blank, overlaps, Committed, DirNode, Member, RCtx, CONTENTS_PX, DIR_LABEL_PX, EDGE_PX,
    FILE_LABEL_PX, LEGIBLE_PX, MONO_ADV, NAME_PX,
};
use crate::layout;
use crate::view::shared::{
    composition_bar, edges_asset_filtered, edges_asset_scaled, heat_color, placed_graph,
    EdgeClass,
};
use damascene_core::prelude::*;

/// Rest-state opacity of a call/citation edge by horizontal reach: full up to
/// ~a quarter of the file's width, easing to a faint context line by ~60%.
/// Local structure stays crisp; the long-haul web recedes until a hover
/// traces it. Same-column return arcs (`back`) are short and semantically
/// loud — always full.
fn long_haul_alpha(e: &layout::RoutedEdge, lay_w: f32) -> f32 {
    if e.back || e.points.len() < 2 {
        return 1.0;
    }
    let (sx, ex) = (e.points[0].0, e.points[e.points.len() - 1].0);
    let t = (((ex - sx).abs() / lay_w - 0.25) / (0.60 - 0.25)).clamp(0.0, 1.0);
    let s = t * t * (3.0 - 2.0 * t);
    1.0 - 0.75 * s
}

/// Rest-state opacity by fan redundancy: an edge whose endpoint sits in a
/// big drawn fan fades — the hub's committed position (leftmost of its
/// dependents) already tells the popularity story, and its thirtieth line
/// adds ink, not information. Hovering the hub traces the fan at full
/// strength.
fn fan_alpha(fan: u32) -> f32 {
    let t = ((fan as f32 - 8.0) / 16.0).clamp(0.0, 1.0);
    let s = t * t * (3.0 - 2.0 * t);
    1.0 - 0.7 * s
}

/// Render one committed dir level at `origin` (the level's absolute content
/// position): each child drawn per its screen size, import edges always.
/// `shade` carries the ancestor labels' absolute rects — a screen-constant
/// label overflows its committed band at far zooms, so a descendant whose own
/// label would land under an ancestor's yields to it (the map convention:
/// the enclosing name wins until you're close enough for both).
pub(super) fn render_walk(
    ctx: &RCtx,
    com: &Committed,
    node: &DirNode,
    path: &str,
    origin: (f32, f32),
    shade: &[Rect],
) -> El {
    let geom = com.dirs.get(path).expect("committed dir level");
    let mut els: Vec<El> = Vec::with_capacity(geom.lay.nodes.len());
    let mut i = 0;
    for (name, sub) in &node.subdirs {
        let n = &geom.lay.nodes[i];
        let abs = (origin.0 + n.x, origin.1 + n.y);
        els.push(dir_el(ctx, com, sub, &format!("{path}{name}/"), name, (n.w, n.h), abs, shade));
        i += 1;
    }
    for &file in &node.files {
        let n = &geom.lay.nodes[i];
        let abs = (origin.0 + n.x, origin.1 + n.y);
        els.push(file_el(ctx, com, file, (n.w, n.h), abs, shade));
        i += 1;
    }
    placed_graph(&geom.lay, els, edges_asset_scaled(&geom.lay, ctx.hairline()))
}

/// One dir box at its committed rect: chrome + cartographic label, contents
/// only when the box earns them on screen.
#[allow(clippy::too_many_arguments)]
fn dir_el(
    ctx: &RCtx,
    com: &Committed,
    node: &DirNode,
    path: &str,
    name: &str,
    (w, h): (f32, f32),
    abs: (f32, f32),
    shade: &[Rect],
) -> El {
    if !ctx.in_view(abs.0, abs.1, w, h) {
        return blank();
    }
    let label = carto_label(&format!("▸ {name}/"), DIR_LABEL_PX, ctx.zoom, w, h, abs, shade);
    let mut layers: Vec<El> = Vec::new();
    if w.min(h) * ctx.zoom >= CONTENTS_PX {
        let geom = &com.dirs[path];
        let off = geom.off;
        // Children yield to this label (and to the ancestors' still in play).
        let mut child_shade = shade.to_vec();
        if let Some((_, r)) = &label {
            child_shade.push(*r);
        }
        let inner =
            render_walk(ctx, com, node, path, (abs.0 + off.0, abs.1 + off.1), &child_shade);
        layers.push(at(inner, off, geom.lay.width, geom.lay.height, w, h));
    }
    if let Some((lbl, _)) = label {
        layers.push(lbl);
    }
    stack(layers)
        .width(Size::Fixed(w))
        .height(Size::Fixed(h))
        .fill(tokens::BACKGROUND)
        .stroke(tokens::MUTED)
        .stroke_width(ctx.hairline())
        .radius(8.0)
}

/// One file box at its committed rect: chrome + cartographic label; past
/// [`CONTENTS_PX`] the committed fn slots (each drawn per its own screen
/// size) and — past [`EDGE_PX`] — the call-edge overlay.
fn file_el(
    ctx: &RCtx,
    com: &Committed,
    file: usize,
    (w, h): (f32, f32),
    abs: (f32, f32),
    shade: &[Rect],
) -> El {
    if !ctx.in_view(abs.0, abs.1, w, h) {
        return blank();
    }
    let f = &ctx.project.files[file];
    let base = f.rel.rsplit_once('/').map(|(_, b)| b).unwrap_or(&f.rel);
    let members = &ctx.by_file[&file];

    let mut layers: Vec<El> = Vec::new();
    if w.min(h) * ctx.zoom >= CONTENTS_PX {
        let geom = &com.files[&file];
        let inner_origin = (abs.0 + geom.off.0, abs.1 + geom.off.1);
        let slots: Vec<El> = members
            .iter()
            .enumerate()
            .map(|(i, &m)| {
                let n = &geom.lay.nodes[i];
                let slot_abs = (inner_origin.0 + n.x, inner_origin.1 + n.y);
                match m {
                    Member::Fn(g) => fn_el(ctx, g, (n.w, n.h), slot_abs),
                    Member::Claim(c) => claim_el(ctx, c, (n.w, n.h), slot_abs),
                    Member::Type(t) => type_el(ctx, t, (n.w, n.h), slot_abs),
                    Member::Doc(fi) => doc_el(ctx, fi, (n.w, n.h), slot_abs),
                }
            })
            .collect();
        // Edge LOD: below ~EDGE_PX on screen the intra-file web is unreadable
        // gray mass (placement already encodes the call structure — callers
        // left, callees right), so the overlay is dropped. Pure rendering:
        // the routes were committed with the layout and never change.
        //
        // Within the overlay, edges tier by attention cost (correlations are
        // committed maximally, shown selectively):
        // - the focus member's edges — hovered, else selected — draw at full
        //   strength whatever their class or reach: hover is the trace gesture;
        // - [`EdgeClass::Use`] (the dense shape-usage web) draws *only* then;
        // - composition and claim-subject links are sparse and load-bearing —
        //   always full;
        // - the call/citation web fades with horizontal reach (an edge
        //   crossing most of the file is context, not local structure) and
        //   with fan redundancy (a hub's thirtieth caller-line repeats what
        //   its committed leftmost position already says) — a few hundred of
        //   either at full strength are the gray spaghetti.
        let edge_overlay = if geom.lay.width.min(geom.lay.height) * ctx.zoom >= EDGE_PX {
            let focus_slot = ctx
                .focus
                .and_then(|f| members.iter().position(|&m| m == f));
            let lay_w = geom.lay.width.max(1.0);
            // Per-slot degree *within the edge's own class* (a fn with many
            // claims about it hasn't earned a faded call web): the
            // fan-redundancy signal.
            let mut flow_deg = vec![0u32; members.len()];
            let mut cite_deg = vec![0u32; members.len()];
            for (k, &(a, b)) in geom.ends.iter().enumerate() {
                match geom.classes.get(k) {
                    Some(&EdgeClass::Flow) => {
                        flow_deg[a] += 1;
                        flow_deg[b] += 1;
                    }
                    Some(&EdgeClass::Cite) => {
                        cite_deg[a] += 1;
                        cite_deg[b] += 1;
                    }
                    _ => {}
                }
            }
            edges_asset_filtered(&geom.lay, ctx.hairline(), &geom.classes, |k| {
                let focused = focus_slot.is_some_and(|s| {
                    geom.ends.get(k).is_some_and(|&(a, b)| a == s || b == s)
                });
                if focused {
                    return 1.0;
                }
                match geom.classes.get(k) {
                    Some(&EdgeClass::Use) => 0.0,
                    Some(&EdgeClass::Shape) | Some(&EdgeClass::About) => 1.0,
                    class => {
                        let deg: &[u32] = if class == Some(&EdgeClass::Cite) {
                            &cite_deg
                        } else {
                            &flow_deg
                        };
                        let fan =
                            geom.ends.get(k).map_or(0, |&(a, b)| deg[a].max(deg[b]));
                        long_haul_alpha(&geom.lay.edges[k], lay_w).min(fan_alpha(fan))
                    }
                }
            })
        } else {
            VectorAsset::from_paths([0.0, 0.0, geom.lay.width, geom.lay.height], Vec::new())
        };
        let inner = placed_graph(&geom.lay, slots, edge_overlay);
        layers.push(at(inner, geom.off, geom.lay.width, geom.lay.height, w, h));
    }
    // The composition bar — the Systems lens carried onto the map's wide
    // view: impl (cool) then proof (warm) over the comment/blank track,
    // pinned along the box's bottom edge at a screen-constant height. It
    // draws in the label-only regime; past CONTENTS_PX the cards and edges
    // themselves are the composition, and the bar would just underline them.
    if w.min(h) * ctx.zoom < CONTENTS_PX && w * ctx.zoom >= 36.0 {
        let bh = (5.0 / ctx.zoom).min(h * 0.10);
        let inset = bh;
        let bw = w - 2.0 * inset;
        layers.push(at(composition_bar(&f.counts, bw, bh), (inset, h - bh - inset), bw, bh, w, h));
    }
    let (nfns, nclaims, ntypes) =
        members.iter().fold((0, 0, 0), |(a, b, c), m| match m {
            Member::Fn(_) => (a + 1, b, c),
            Member::Claim(_) => (a, b + 1, c),
            Member::Type(_) => (a, b, c + 1),
            Member::Doc(_) => (a, b, c),
        });
    if let Some((lbl, _)) = carto_label(base, FILE_LABEL_PX, ctx.zoom, w, h, abs, shade) {
        // The label is the file's *handle*: keyed, so hovering it reveals the
        // file's account (the `;;;` header) and clicking it opens the file
        // inspector. The box itself stays unkeyed — it must never intercept
        // a background pan drag, and an unkeyed tooltip would be dead anyway.
        let mut tip = format!(
            "{} · {} fns · {} claims · {} types · {} lines",
            f.rel,
            nfns,
            nclaims,
            ntypes,
            f.counts.total()
        );
        if !f.doc.is_empty() {
            tip = format!("{tip}\n\n{}", f.doc);
        }
        layers.push(lbl.key(format!("filebox:{file}")).tooltip(tip));
    }
    // Box fill carries the proof-vs-impl heat (the other Systems lens): warm
    // boxes are proof-heavy corners of the tree, cool ones implementation.
    // Constant in zoom — at reading zooms the cards cover most of it anyway.
    let base_fill = tokens::CARD.mix(tokens::BACKGROUND, 0.4);
    let fill = match heat_color(f.counts.proof_share()) {
        Some(hc) => base_fill.mix(hc, 0.22),
        None => base_fill,
    };
    stack(layers)
        .width(Size::Fixed(w))
        .height(Size::Fixed(h))
        .fill(fill)
        .stroke(tokens::BORDER)
        .stroke_width(ctx.hairline())
        .radius(8.0)
}

/// One fn slot at its committed footprint: flow innards when in view and the
/// zoom affords them (the selected fn always), else a name slab.
fn fn_el(ctx: &RCtx, fn_idx: usize, (w, h): (f32, f32), abs: (f32, f32)) -> El {
    if !ctx.in_view(abs.0, abs.1, w, h) {
        return blank();
    }
    let selected = ctx.selected_fn == Some(fn_idx);
    if selected || ctx.zoom >= ctx.flow_z {
        // The card's intrinsic size *is* the committed slot (commit measured
        // this same construction), so it lands exactly in its footprint.
        return flow_card(ctx.project, fn_idx, selected);
    }
    // The slab: the committed footprint with the fn name at a cartographic
    // (screen-constant) size, clamped into the slot; bare when illegible.
    let f = &ctx.project.fns[fn_idx];
    let chars = f.name.chars().count().max(1) as f32;
    let font = (NAME_PX / ctx.zoom).min(h * 0.45).min(w * 0.92 / (chars * MONO_ADV));
    let body: Vec<El> = if font * ctx.zoom >= LEGIBLE_PX {
        vec![text(f.name.clone()).mono().semibold().font_size(font).nowrap_text()]
    } else {
        Vec::new()
    };
    // The orphan flag stays on at distance — red specks are exactly how a
    // triage sweep reads a wide map.
    let (fill, stroke) =
        if f.is_orphan() { orphan_colors() } else { (tokens::CARD, tokens::BORDER) };
    column(body)
        .align(Align::Center)
        .justify(Justify::Center)
        .width(Size::Fixed(w))
        .height(Size::Fixed(h))
        .radius(7.0)
        .fill(fill)
        .stroke(stroke)
        .stroke_width(ctx.hairline())
        .key(format!("fn:{fn_idx}"))
        .tooltip(crate::view::inspector::node_tip(ctx.project, fn_idx))
}

/// One claim slot at its committed footprint, mirroring [`fn_el`]: the full
/// claim card past the flow threshold, else a name slab. The kind tint stays
/// on the slab at any distance — amber specks are axioms, red ones unmet
/// requirements; that *is* the wide-view proof-layer read.
fn claim_el(ctx: &RCtx, ci: usize, (w, h): (f32, f32), abs: (f32, f32)) -> El {
    if !ctx.in_view(abs.0, abs.1, w, h) {
        return blank();
    }
    if ctx.zoom >= ctx.flow_z {
        return claim_card(ctx.project, ci);
    }
    let c = &ctx.project.claims[ci];
    let (fill, stroke) = claim_colors(c.kind, c.fulfilled);
    let chars = c.name.chars().count().max(1) as f32;
    let font = (NAME_PX / ctx.zoom).min(h * 0.45).min(w * 0.92 / (chars * MONO_ADV));
    let body: Vec<El> = if font * ctx.zoom >= LEGIBLE_PX {
        vec![text(c.name.clone()).mono().semibold().font_size(font).nowrap_text()]
    } else {
        Vec::new()
    };
    column(body)
        .align(Align::Center)
        .justify(Justify::Center)
        .width(Size::Fixed(w))
        .height(Size::Fixed(h))
        .radius(7.0)
        .fill(fill)
        .stroke(stroke)
        .stroke_width(ctx.hairline())
        .key(format!("claim:{ci}"))
        .tooltip(claim_tip(c))
}

/// One type slot at its committed footprint, mirroring [`claim_el`]: the full
/// definition card past the flow threshold, else a name slab. The blue tint
/// stays on the slab at any distance — **blue = shape**, the structural
/// counterpart of the proof layer's amber.
fn type_el(ctx: &RCtx, ti: usize, (w, h): (f32, f32), abs: (f32, f32)) -> El {
    if !ctx.in_view(abs.0, abs.1, w, h) {
        return blank();
    }
    if ctx.zoom >= ctx.flow_z {
        return type_card(ctx.project, ti, true);
    }
    let t = &ctx.project.types[ti];
    let chars = t.name.chars().count().max(1) as f32;
    let font = (NAME_PX / ctx.zoom).min(h * 0.45).min(w * 0.92 / (chars * MONO_ADV));
    let body: Vec<El> = if font * ctx.zoom >= LEGIBLE_PX {
        vec![text(t.name.clone()).mono().semibold().font_size(font).nowrap_text()]
    } else {
        Vec::new()
    };
    let (fill, stroke) = type_colors();
    column(body)
        .align(Align::Center)
        .justify(Justify::Center)
        .width(Size::Fixed(w))
        .height(Size::Fixed(h))
        .radius(7.0)
        .fill(fill)
        .stroke(stroke)
        .stroke_width(ctx.hairline())
        .key(format!("type:{ti}"))
        .tooltip(type_tip(t))
}

/// One file-doc slot at its committed footprint: the full header card at
/// reading zoom, else a quiet slab carrying the summary line while it's
/// legible. Unkeyed throughout — prose isn't a navigation target, so the
/// card stays hover-transparent (background pan works through it).
fn doc_el(ctx: &RCtx, file: usize, (w, h): (f32, f32), abs: (f32, f32)) -> El {
    if !ctx.in_view(abs.0, abs.1, w, h) {
        return blank();
    }
    if ctx.zoom >= ctx.flow_z {
        return filedoc_card(ctx.project, file);
    }
    let f = &ctx.project.files[file];
    let summary = f.doc.lines().next().unwrap_or_default().trim();
    let chars = summary.chars().count().max(1) as f32;
    let font = (NAME_PX / ctx.zoom).min(h * 0.45).min(w * 0.92 / (chars * MONO_ADV));
    let body: Vec<El> = if font * ctx.zoom >= LEGIBLE_PX {
        vec![text(summary.to_string()).muted().font_size(font).nowrap_text()]
    } else {
        Vec::new()
    };
    column(body)
        .align(Align::Center)
        .justify(Justify::Center)
        .width(Size::Fixed(w))
        .height(Size::Fixed(h))
        .radius(7.0)
        .fill(tokens::BACKGROUND.mix(tokens::CARD, 0.5))
        .stroke(tokens::BORDER.mix(tokens::BACKGROUND, 0.4))
        .stroke_width(ctx.hairline())
}

/// A label drawn *over* a box at a screen-constant font, clamped so it stays
/// inside the box, with the absolute rect it will occupy; `None` when even
/// the clamped size would be illegible on screen (the box is a distant speck
/// — its parent's label does the talking), or when it would land under an
/// ancestor's label (`shade`).
///
/// The size hierarchy keeps nested labels apart in the common case: a label
/// may take at most a quarter of its box's on-screen minor dimension, so a
/// child box's label reads smaller than an enclosing box's — country names
/// before city names. `shade` handles the rest (a top-left child under a
/// still-oversized ancestor label).
fn carto_label(
    label: &str,
    screen_px: f32,
    zoom: f32,
    w: f32,
    h: f32,
    abs: (f32, f32),
    shade: &[Rect],
) -> Option<(El, Rect)> {
    let chars = label.chars().count().max(1) as f32;
    let target = screen_px.min(w.min(h) * zoom * 0.25);
    let font = (target / zoom).min(w * 0.92 / (chars * MONO_ADV));
    if font * zoom < LEGIBLE_PX {
        return None;
    }
    let rect = Rect::new(
        abs.0 + tokens::SPACE_2,
        abs.1 + tokens::SPACE_2,
        chars * MONO_ADV * font,
        font * 1.35,
    );
    if shade.iter().any(|s| overlaps(Some(*s), rect.x, rect.y, rect.w, rect.h)) {
        return None;
    }
    let el = column([text(label.to_string()).mono().semibold().font_size(font).nowrap_text()])
        .align(Align::Start)
        .padding(tokens::SPACE_2);
    Some((el, rect))
}

/// Pin `inner` (of size `iw × ih`) at `off` inside a `w × h` box layer.
fn at(inner: El, off: (f32, f32), iw: f32, ih: f32, w: f32, h: f32) -> El {
    stack([inner])
        .width(Size::Fixed(w))
        .height(Size::Fixed(h))
        .layout(move |lc: LayoutCtx| {
            let o = lc.container;
            vec![Rect::new(o.x + off.0, o.y + off.1, iw, ih)]
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carto_label_is_screen_constant_until_the_box_clamps_it() {
        let no_shade: &[Rect] = &[];
        // Plenty of room: the font is screen_px / zoom (13px on screen).
        let l = carto_label("kernel", 13.0, 0.1, 5000.0, 3000.0, (0.0, 0.0), no_shade);
        assert!(l.is_some());
        // Tiny box on screen: even clamped, the label would be sub-legible.
        assert!(carto_label("kernel", 13.0, 0.001, 400.0, 200.0, (0.0, 0.0), no_shade).is_none());
        // Long name in a narrow box: width-clamped but still legible at 1:1.
        let long = carto_label("a_rather_long_module_name", 13.0, 1.0, 120.0, 40.0, (0.0, 0.0), no_shade);
        assert!(long.is_some());
    }

    #[test]
    fn a_label_yields_to_the_ancestor_label_over_it() {
        let (_, parent) = carto_label("kernel", 15.0, 0.1, 5000.0, 3000.0, (0.0, 0.0), &[])
            .expect("parent label draws");
        // A child box whose top-left sits inside the parent's label area:
        // its label yields. The same child clear of the shadow draws.
        let shade = [parent];
        assert!(carto_label("reader.shard", 13.0, 0.1, 2000.0, 1500.0, (10.0, 10.0), &shade).is_none());
        assert!(
            carto_label("reader.shard", 13.0, 0.1, 2000.0, 1500.0, (2500.0, 1500.0), &shade)
                .is_some()
        );
    }
}
