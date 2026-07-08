//! Map view (experimental): the unified canvas — any [`Scope`](crate::scope::Scope)'s
//! fns, grouped by origin **dir ⊃ file** into nested bounding boxes, each fn
//! drawn in the expanded flow form ([`super::flow::render_region`]).
//!
//! ## Layout = recursive graph placement, intrinsic sizing
//! The layout is a recursive pass mirroring the program's own containment tree,
//! and at **every level it is link-based**:
//!
//! - inside a **file** box, the fns are placed by the file's intra-file **call
//!   graph** (the Sugiyama engine in `layout.rs`);
//! - inside a **dir** box, the child file/subdir boxes are placed by the
//!   **import DAG** among them (aggregated: a subdir imports another when any of
//!   its files do).
//!
//! Each level is sized bottom-up by the *measured* intrinsic size of the level
//! below ([`intrinsic`]) — a fn card measures itself, a file box measures its
//! laid-out call graph, a dir box measures its laid-out import graph — so the
//! engine always has real sizes and there is no estimation (contrast
//! `board.rs::est`). The whole tree is placed without a viewport via
//! [`placed_graph`]; only the outermost result is wrapped in one pan/zoom
//! viewport.
//!
//! The per-level router is Sugiyama for now; because each level is one call to
//! [`layout_graph`], a better router can replace it per scale without touching
//! the box rendering.
//!
//! ## Semantic zoom (LOD)
//! A flow card is only worth its pixels when its text is near-legible, so fns
//! render at one of three tiers ([`Lod`]) chosen from an effective zoom: the
//! full **flow** card, a **name** box (name + return type — a collapsed flow
//! card), or a bare **block**. On top of that, per node, a *file* whose box
//! would land under [`CHIP_PX`] on screen collapses to a **chip** (basename +
//! fn count) — so at a wide scope the big files stay boxes while the small
//! ones fold away, and the extent stops exploding.
//!
//! Which zoom drives the tiers depends on who owns the viewport:
//! - **User-driven** (`!at_home`): the live readback zoom. No feedback — the
//!   extent doesn't move the zoom — so a pure `zoom → tier` map is stable
//!   (crossing a threshold reflows once, deterministically).
//! - **At home** (armed fit, incl. headless): the fit zoom is *derived from*
//!   the content extent, which the tiers themselves determine — deriving tiers
//!   from the live zoom would oscillate. Instead the tier comes from a
//!   **predicted** fit zoom: a cheap name-tier probe layout, refined once
//!   against the layout it produces ([`canvas`]). A pure function of the
//!   scope, so the armed fit lands on a fixed point.
//!
//! The selected fn always renders as a full flow card (and its file never
//! chips): clicking a name box expands it in place.

use super::flow::render_region;
use super::shared::{edges_asset, pan_zoom_viewport, placed_graph};
use super::SUB_SIZE;
use crate::flow::Region;
use crate::layout::{self, EndPoint, GEdge, GNode, Graph};
use crate::model::{FnDef, Project};
use crate::scope::Scope;
use crate::view::ViewParams;
use damascene_core::layout::intrinsic;
use damascene_core::prelude::*;
use std::collections::BTreeMap;

pub(crate) fn legend() -> El {
    row([
        text("map").mono().muted().font_size(SUB_SIZE),
        text("dir/file boxes placed by imports · fns by calls · zoom for detail (blocks → names → flow) · click a fn to expand it")
            .muted()
            .font_size(SUB_SIZE),
    ])
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_2)
}

/// The level of detail a fn renders at. Ordered: each tier strictly contains
/// the one below it visually (a name box is a collapsed flow card; a block is
/// a name box without the text).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Lod {
    /// A bare tinted rectangle — shape and placement only (name on hover).
    Block,
    /// Name + return type: what the Methods view draws.
    Name,
    /// The full flow card ([`fn_card`] with the region tree).
    Flow,
}

/// Flow-card text is ~11-13px; below this zoom it isn't worth drawing.
const Z_FLOW: f32 = 0.55;
/// Below this even the name box's text is illegible — draw bare blocks.
const Z_NAME: f32 = 0.22;
/// A file box smaller than this on screen (either dimension) chips.
const CHIP_PX: f32 = 48.0;
/// Below this on-screen size a file's intra-file call-edge overlay is dropped
/// (the nodes stay — placement still encodes the structure).
const EDGE_PX: f32 = 140.0;
/// Fixed size of a block-tier fn (placement still comes from the call graph).
const BLOCK_W: f32 = 56.0;
const BLOCK_H: f32 = 18.0;
/// Nominal canvas size for the at-home fit-zoom prediction. The real canvas
/// depends on the window and open panels, which the pure view can't see; a
/// tier heuristic doesn't need it exactly.
const NOMINAL_W: f32 = 1180.0;
const NOMINAL_H: f32 = 840.0;

/// The tier an effective zoom affords.
fn tier_for(zoom: f32) -> Lod {
    if zoom >= Z_FLOW {
        Lod::Flow
    } else if zoom >= Z_NAME {
        Lod::Name
    } else {
        Lod::Block
    }
}

/// The zoom an armed `FitPolicy::Contain` would land on for this extent.
fn fit_zoom(b: &Block) -> f32 {
    (NOMINAL_W / b.w).min(NOMINAL_H / b.h).min(1.0)
}

/// Quantize a user-driven zoom to half-octave bands (…, 0.25, 0.35, 0.5, 0.7,
/// 1.0, …). Every LOD decision — tier, chips, edge overlays — is priced off
/// the band, so the layout is piecewise-constant in zoom: within a band,
/// zooming is a pure visual scale (no reflow); crossing a band edge is the
/// single, re-anchored reflow moment. Unquantized zoom made the per-file chip
/// thresholds fire one after another through a zoom gesture — a continuous
/// rain of reflows.
fn quantize(zoom: f32) -> f32 {
    2f32.powf((zoom.max(0.001).log2() * 2.0).round() / 2.0)
}

/// Cross-frame Map state, owned by the app and lent to the (otherwise pure)
/// view: what the last build was keyed on, the content offset that kept it
/// anchored, and where each file's box landed. `None` between scopes and on
/// at-home frames (the armed fit frames the pristine layout; anchoring only
/// matters once the user is steering).
pub struct MapMemo {
    scope: Scope,
    selected_fn: Option<usize>,
    /// The quantized zoom the stored layout was built at.
    z_used: f32,
    /// Translation applied to the whole placed tree (accumulated re-anchors).
    offset: (f32, f32),
    /// Per-file box centers in pre-offset content coordinates.
    anchors: Vec<(usize, f32, f32)>,
}

pub type MapMemoCell = std::cell::RefCell<Option<MapMemo>>;

/// Re-anchor a reflow: pick the previous-layout file box nearest the viewport
/// center (`target`, in applied content coordinates) and return the offset
/// that pins its box center — the content point the user is steering toward
/// stays put while everything reflows around it.
fn reanchor(
    prev: &MapMemo,
    new_anchors: &[(usize, f32, f32)],
    target: (f32, f32),
) -> (f32, f32) {
    let dist2 = |x: f32, y: f32| {
        let (dx, dy) = (x - target.0, y - target.1);
        dx * dx + dy * dy
    };
    let mut best: Option<(f32, usize, f32, f32)> = None; // (d2, file, prev applied x, y)
    for &(file, x, y) in &prev.anchors {
        let (ax, ay) = (x + prev.offset.0, y + prev.offset.1);
        let d = dist2(ax, ay);
        if best.is_none_or(|(bd, ..)| d < bd) {
            best = Some((d, file, ax, ay));
        }
    }
    let Some((_, file, px, py)) = best else {
        return prev.offset;
    };
    match new_anchors.iter().find(|&&(f, ..)| f == file) {
        Some(&(_, nx, ny)) => (px - nx, py - ny),
        None => prev.offset,
    }
}

pub(crate) fn canvas(project: &Project, p: &ViewParams, memo: Option<&MapMemoCell>) -> El {
    let fns = p.scope.fns(project);
    if fns.is_empty() {
        return column([
            h3("Map"),
            text("Pick a file or directory in the sidebar to scope the map.").muted(),
        ])
        .gap(tokens::SPACE_3)
        .padding(tokens::SPACE_8);
    }

    // The in-scope fns of each file (a scope may include only some of a file's
    // fns, e.g. a call-tree), keyed by file and kept in definition order.
    let mut by_file: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &fi in &fns {
        by_file.entry(project.fns[fi].file).or_default().push(fi);
    }

    // Build the dir tree over the spanned files, then lay it out recursively.
    let mut root = DirNode::default();
    for &file in by_file.keys() {
        root.insert(&dir_segments(&project.files[file].rel), file);
    }
    let ctx = |tier: Lod, zoom_lod: f32| Ctx {
        project,
        by_file: &by_file,
        selected_fn: p.selected_fn,
        tier,
        zoom_lod,
    };
    let (placed, z_used, offset) = if p.at_home {
        // Predict the fitted zoom instead of reading it back (see module docs:
        // the armed fit derives zoom from the extent the tiers determine, so
        // the live readback would oscillate). Probe at name tier with chips
        // off, refine once against the layout the first choice produces.
        let probe = layout_children(&ctx(Lod::Name, f32::INFINITY), &root);
        let z0 = fit_zoom(&probe);
        let first = layout_children(&ctx(tier_for(z0), z0), &root);
        let z1 = fit_zoom(&first);
        if tier_for(z1) == tier_for(z0) {
            (first, z0, (0.0, 0.0))
        } else {
            let second = layout_children(&ctx(tier_for(z1), z1), &root);
            (second, z1, (0.0, 0.0))
        }
    } else {
        // User-driven: LOD from the live zoom, quantized to bands so the
        // layout only reflows at band edges — and there, re-anchored so the
        // content under the viewport center stays put.
        let zq = quantize(p.zoom);
        let placed = layout_children(&ctx(tier_for(zq), zq), &root);
        let offset = memo
            .and_then(|m| {
                let borrowed = m.borrow();
                let prev = borrowed.as_ref()?;
                // A new subject has nothing meaningful to anchor to (and the
                // scope switch refits anyway).
                if prev.scope != p.scope {
                    return None;
                }
                // Same layout key → identical layout; keep the offset.
                if prev.selected_fn == p.selected_fn && prev.z_used == zq {
                    return Some(prev.offset);
                }
                // Reflow (band edge, or the selection expanded a card): pin
                // the box nearest the viewport center. The center's content
                // position is (screen_center - pan) / zoom off the readbacks.
                let target = (
                    (NOMINAL_W * 0.5 - p.pan.0) / p.zoom,
                    (NOMINAL_H * 0.5 - p.pan.1) / p.zoom,
                );
                Some(reanchor(prev, &placed.anchors, target))
            })
            .unwrap_or((0.0, 0.0));
        (placed, zq, offset)
    };
    if let Some(m) = memo {
        *m.borrow_mut() = Some(MapMemo {
            scope: p.scope.clone(),
            selected_fn: p.selected_fn,
            z_used,
            offset,
            anchors: placed.anchors.clone(),
        });
    }

    let content = row([placed.el]).padding(tokens::SPACE_6);
    let content = if offset == (0.0, 0.0) {
        content
    } else {
        // Slide the whole placed tree by the anchor offset. Content may hang
        // outside the nominal box; the viewport pans over it regardless.
        let (w, h) = (placed.w + 48.0, placed.h + 48.0);
        let (dx, dy) = offset;
        stack([content])
            .width(Size::Fixed(w))
            .height(Size::Fixed(h))
            .layout(move |lc: LayoutCtx| {
                let o = lc.container;
                vec![Rect::new(o.x + dx, o.y + dy, w, h)]
            })
    };
    pan_zoom_viewport(content)
}

/// Shared inputs threaded through the recursion (project data + the in-scope fn
/// sets + the focus cursor), so the layout fns take one context not five args.
struct Ctx<'a> {
    project: &'a Project,
    by_file: &'a BTreeMap<usize, Vec<usize>>,
    selected_fn: Option<usize>,
    /// The LOD tier fns render at (the selected fn overrides to [`Lod::Flow`]).
    tier: Lod,
    /// The effective zoom behind `tier` — also prices the file-chip rule
    /// (`size × zoom_lod < CHIP_PX`). `INFINITY` disables chipping (probe).
    zoom_lod: f32,
}

/// A laid-out, measured sub-tree: its element plus the intrinsic size the parent
/// graph needs to place it, and the file-box centers inside it (in this
/// block's local coordinates) — the anchor candidates re-anchoring pins.
struct Block {
    el: El,
    w: f32,
    h: f32,
    anchors: Vec<(usize, f32, f32)>,
}

/// A directory in the map tree: nested subdirs plus the files directly in it.
#[derive(Default)]
struct DirNode {
    subdirs: BTreeMap<String, DirNode>,
    files: Vec<usize>,
}

impl DirNode {
    /// File at `segments` (its dir path, root-first); empty = this dir.
    fn insert(&mut self, segments: &[&str], file: usize) {
        match segments.split_first() {
            None => self.files.push(file),
            Some((head, rest)) => {
                self.subdirs.entry(head.to_string()).or_default().insert(rest, file);
            }
        }
    }

    /// Every in-scope file under this node (for import-edge aggregation).
    fn all_files(&self) -> Vec<usize> {
        let mut out = self.files.clone();
        for sub in self.subdirs.values() {
            out.extend(sub.all_files());
        }
        out
    }
}

/// The directory segments of a file's `rel` path (e.g. `examples/foo/a.shard`
/// → `["examples", "foo"]`; a root-level file → `[]`).
fn dir_segments(rel: &str) -> Vec<&str> {
    let mut segs: Vec<&str> = rel.split('/').collect();
    segs.pop(); // drop the file name
    segs
}

/// Lay out a dir node's children — subdir boxes and file boxes — by the import
/// DAG among them, returning the placed content (no enclosing band). The
/// synthetic root uses this directly as the canvas content; a named dir wraps
/// the result in [`dir_box`].
fn layout_children(ctx: &Ctx, node: &DirNode) -> Block {
    // Build each child as a measured block, tracking the in-scope files it
    // covers so import edges between children can be aggregated.
    let mut blocks: Vec<Block> = Vec::new();
    let mut files_of: Vec<Vec<usize>> = Vec::new();
    for (name, sub) in &node.subdirs {
        let inner = layout_children(ctx, sub);
        blocks.push(dir_box(name, inner));
        files_of.push(sub.all_files());
    }
    for &file in &node.files {
        blocks.push(layout_file(ctx, file));
        files_of.push(vec![file]);
    }

    // A child A → B edge when any file in A imports any in-scope file in B.
    let in_scope = |g: usize| ctx.by_file.contains_key(&g);
    let mut edges = Vec::new();
    for (i, fa) in files_of.iter().enumerate() {
        for (j, fb) in files_of.iter().enumerate() {
            if i != j && imports_between(ctx.project, fa, fb, &in_scope) {
                edges.push(GEdge {
                    from: EndPoint { node: i, port: 0 },
                    to: EndPoint { node: j, port: 0 },
                });
            }
        }
    }
    place(blocks, edges)
}

/// True when some file in `from` imports some file in `to` that is in scope.
fn imports_between(
    project: &Project,
    from: &[usize],
    to: &[usize],
    in_scope: &impl Fn(usize) -> bool,
) -> bool {
    from.iter().any(|&f| {
        project.files[f]
            .import_targets
            .iter()
            .any(|&t| in_scope(t) && to.contains(&t))
    })
}

/// Lay out one file's in-scope fns by its intra-file call graph, box the result.
fn layout_file(ctx: &Ctx, file: usize) -> Block {
    let members = &ctx.by_file[&file];
    // Each fn as an intrinsic flow card, measured for the engine.
    let cards: Vec<El> = members.iter().map(|&fi| fn_card(ctx, fi)).collect();
    let nodes: Vec<GNode> = cards
        .iter()
        .map(|c| {
            let (w, h) = intrinsic(c);
            GNode::simple(w, h)
        })
        .collect();

    // Intra-file call edges between in-scope members (dedup, no self-loops).
    let local: BTreeMap<usize, usize> =
        members.iter().enumerate().map(|(i, &g)| (g, i)).collect();
    let mut seen = std::collections::HashSet::new();
    let mut edges = Vec::new();
    for (i, &g) in members.iter().enumerate() {
        for &callee in &ctx.project.fns[g].calls {
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

    // Edge LOD: below ~EDGE_PX on screen the intra-file web is unreadable gray
    // mass (placement already encodes the call structure — callers left,
    // callees right), so the overlay is dropped. Edges still shape the layout.
    let lay = layout::layout(&Graph { nodes, edges }, &layout::LayoutConfig::default());
    let visible = |px: f32| lay.width.min(lay.height) * ctx.zoom_lod >= px;
    let edge_overlay = if visible(EDGE_PX) {
        edges_asset(&lay)
    } else {
        VectorAsset::from_paths([0.0, 0.0, lay.width, lay.height], Vec::new())
    };
    let inner =
        Block { el: placed_graph(&lay, cards, edge_overlay), w: lay.width, h: lay.height, anchors: Vec::new() };
    let f = &ctx.project.files[file];
    let base = f.rel.rsplit_once('/').map(|(_, b)| b).unwrap_or(&f.rel);
    let header = || {
        row([
            text(base.to_string()).mono().semibold().font_size(12.0).nowrap_text(),
            text(format!("{} fns", members.len()))
                .mono()
                .muted()
                .font_size(SUB_SIZE)
                .nowrap_text(),
        ])
        .gap(tokens::SPACE_2)
        .align(Align::Center)
    };

    // Per-node LOD: a file whose box would land under CHIP_PX on screen folds
    // to its header — the surrounding structure keeps reading while the extent
    // stops paying for invisible content. Never chip the selection's home (the
    // expanded selected card must stay visible).
    let holds_selection = ctx.selected_fn.is_some_and(|s| members.contains(&s));
    let (bw, bh) = (inner.w + 24.0, inner.h + 44.0); // + box padding/header
    let mut block = if !holds_selection && (bw.min(bh)) * ctx.zoom_lod < CHIP_PX {
        let chip = boxed(column([header()]), tokens::CARD.mix(tokens::BACKGROUND, 0.4), tokens::BORDER);
        Block {
            el: chip.el.tooltip(format!("{} · {} fns · {} lines", f.rel, members.len(), f.counts.total())),
            ..chip
        }
    } else {
        boxed(
            column([header(), inner.el]),
            tokens::CARD.mix(tokens::BACKGROUND, 0.4),
            tokens::BORDER,
        )
    };
    // The file's anchor is its own box center — chip or full box alike, so an
    // anchor never vanishes when a file folds or unfolds across a reflow.
    block.anchors = vec![(file, block.w * 0.5, block.h * 0.5)];
    block
}

/// Wrap a laid-out dir's content in a folder-labelled bounding box. The
/// inner block's anchors shift by the content's offset inside the box
/// (padding + label band).
fn dir_box(name: &str, inner: Block) -> Block {
    let band = text(format!("▸ {name}/"))
        .mono()
        .semibold()
        .font_size(11.0)
        .muted()
        .nowrap_text();
    let band_h = intrinsic(&band).1;
    let anchors = inner
        .anchors
        .iter()
        .map(|&(f, x, y)| (f, x + tokens::SPACE_3, y + tokens::SPACE_3 + band_h + tokens::SPACE_2))
        .collect();
    let mut block = boxed(column([band, inner.el]), tokens::BACKGROUND, tokens::MUTED);
    block.anchors = anchors;
    block
}

/// Common box chrome (padding/fill/stroke/radius) + the bottom-up measure: pin
/// the result to its intrinsic size so the parent graph places it exactly.
/// Anchors start empty; callers that carry any set them.
fn boxed(body: El, fill: Color, stroke: Color) -> Block {
    let el = body
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .fill(fill)
        .stroke(stroke)
        .radius(8.0)
        .align(Align::Start);
    let (w, h) = intrinsic(&el);
    Block { el: el.width(Size::Fixed(w)).height(Size::Fixed(h)), w, h, anchors: Vec::new() }
}

/// Place pre-measured child blocks by `edges` and return the placed content as
/// a measured block (no box) — used at the dir level. Child anchors shift to
/// their placed positions and aggregate.
fn place(blocks: Vec<Block>, edges: Vec<GEdge>) -> Block {
    let nodes: Vec<GNode> = blocks.iter().map(|b| GNode::simple(b.w, b.h)).collect();
    let graph = Graph { nodes, edges };
    let lay = layout::layout(&graph, &layout::LayoutConfig::default());
    let mut anchors = Vec::new();
    let mut els = Vec::with_capacity(blocks.len());
    for (i, b) in blocks.into_iter().enumerate() {
        let (px, py) = (lay.nodes[i].x, lay.nodes[i].y);
        anchors.extend(b.anchors.into_iter().map(|(f, x, y)| (f, x + px, y + py)));
        els.push(b.el);
    }
    let el = placed_graph(&lay, els, edges_asset(&lay));
    Block { el, w: lay.width, h: lay.height, anchors }
}

/// One fn at the context's LOD tier (the selected fn always expands to
/// [`Lod::Flow`] — click a name box and it opens in place). All tiers carry
/// the key (clicks route, pan-drag skips) and the triage tooltip.
fn fn_card(ctx: &Ctx, fn_idx: usize) -> El {
    let tier = if ctx.selected_fn == Some(fn_idx) { Lod::Flow } else { ctx.tier };
    match tier {
        Lod::Flow => flow_card(ctx, fn_idx),
        Lod::Name => name_card(ctx, fn_idx),
        // Blocks get the muted fill instead of chrome's CARD: with no text,
        // fill contrast is all that keeps them visible against the canvas.
        Lod::Block => column(Vec::<El>::new())
            .width(Size::Fixed(BLOCK_W))
            .height(Size::Fixed(BLOCK_H))
            .radius(3.0)
            .fill(tokens::MUTED)
            .stroke(tokens::BORDER)
            .key(format!("fn:{fn_idx}"))
            .tooltip(super::methods::node_tip(ctx.project, fn_idx)),
    }
}

/// The name tier: the flow card's header line and nothing else.
fn name_card(ctx: &Ctx, fn_idx: usize) -> El {
    let f = &ctx.project.fns[fn_idx];
    row([
        text(f.name.clone())
            .mono()
            .semibold()
            .font_size(super::TITLE_SIZE)
            .nowrap_text(),
        text(format!("→ {}", short_ty(&f.ret)))
            .mono()
            .muted()
            .font_size(SUB_SIZE)
            .nowrap_text(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center)
    .padding(6.0)
    .radius(7.0)
    .fill(tokens::CARD)
    .stroke(tokens::BORDER)
    .key(format!("fn:{fn_idx}"))
    .tooltip(super::methods::node_tip(ctx.project, fn_idx))
}

/// One fn as an intrinsic flow card: a name/signature header, its named
/// arguments (LabVIEW-style inputs), then its region tree (the same renderer
/// the Flow/Board views use). No fixed size — it hugs.
fn flow_card(ctx: &Ctx, fn_idx: usize) -> El {
    let f = &ctx.project.fns[fn_idx];
    let title = row([
        text(f.name.clone())
            .mono()
            .semibold()
            .font_size(super::TITLE_SIZE)
            .nowrap_text()
            .ellipsis(),
        text(format!("→ {}", short_ty(&f.ret)))
            .mono()
            .muted()
            .font_size(SUB_SIZE)
            .nowrap_text(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);

    // The fn's inputs, enumerated: `name  Type`, one per row. A LabVIEW panel
    // reads its wires by their terminals — the signature is first-class, not a
    // count. Omitted for a nullary fn (nothing to list).
    let mut parts = vec![title];
    if let Some(inputs) = params_block(f) {
        parts.push(inputs);
    }
    parts.push(match body_region(f) {
        Some(region) => render_region(&region),
        None => text("(signature only)").muted().font_size(SUB_SIZE),
    });

    let card = column(parts)
        .gap(tokens::SPACE_2)
        .padding(8.0)
        .radius(7.0)
        .key(format!("fn:{fn_idx}"))
        .tooltip(super::methods::node_tip(ctx.project, fn_idx));
    if ctx.selected_fn == Some(fn_idx) {
        card.fill(tokens::CARD.mix(tokens::ACCENT, 0.18)).stroke(tokens::RING)
    } else {
        card.fill(tokens::CARD).stroke(tokens::BORDER)
    }
}

/// The fn's parameters as a small column of `name  Type` rows, or `None` for a
/// nullary fn. Names read in the foreground, types muted — the terminals of the
/// card. Types are trimmed like the return ([`short_ty`]); the tooltip carries
/// the untrimmed signature.
fn params_block(f: &FnDef) -> Option<El> {
    if f.params.is_empty() {
        return None;
    }
    let rows: Vec<El> = f
        .params
        .iter()
        .map(|(name, ty)| {
            row([
                text(name.clone()).mono().font_size(SUB_SIZE).nowrap_text(),
                text(short_ty(ty)).mono().muted().font_size(SUB_SIZE).nowrap_text(),
            ])
            .gap(tokens::SPACE_2)
            .align(Align::Center)
        })
        .collect();
    Some(column(rows).gap(2.0).padding(tokens::SPACE_1))
}

/// The region tree for a fn's body, or `None` for a bodyless `sig` / a
/// measure-only form (annotation, no logic to chart).
fn body_region(f: &FnDef) -> Option<Region> {
    if f.body.is_empty() || f.body.iter().all(|form| form.head() == Some("measure")) {
        None
    } else {
        Some(Region::build(&f.body))
    }
}

/// Trim a return type for the card header (mirrors Board).
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn quantize_snaps_to_half_octave_bands() {
        // Band points are 2^(k/2): …, 0.5, ~0.707, 1.0, ~1.414, …
        assert_eq!(quantize(1.0), 1.0);
        assert_eq!(quantize(0.5), 0.5);
        assert!((quantize(0.6) - 0.707).abs() < 0.01);
        // Idempotent: a band point maps to itself, so the layout key is stable
        // frame-over-frame while the zoom sits inside one band.
        for z in [0.05, 0.13, 0.3, 0.62, 1.7] {
            let q = quantize(z);
            assert_eq!(quantize(q), q, "quantize not idempotent at {z}");
        }
    }

    #[test]
    fn reanchor_pins_the_file_nearest_the_target() {
        let prev = MapMemo {
            scope: Scope::Project,
            selected_fn: None,
            z_used: 0.5,
            offset: (10.0, 0.0),
            // Applied positions: file 1 at (110,100), file 2 at (510,100).
            anchors: vec![(1, 100.0, 100.0), (2, 500.0, 100.0)],
        };
        // Target sits by file 2's applied position → it's the anchor. Its new
        // pre-offset center is (250,50); the offset must pin it at (510,100).
        let new_anchors = vec![(1, 50.0, 50.0), (2, 250.0, 50.0)];
        let off = reanchor(&prev, &new_anchors, (520.0, 90.0));
        assert_eq!(off, (260.0, 50.0));
        assert_eq!((250.0 + off.0, 50.0 + off.1), (510.0, 100.0));
    }

    #[test]
    fn reanchor_keeps_the_old_offset_when_the_anchor_vanished() {
        let prev = MapMemo {
            scope: Scope::Project,
            selected_fn: None,
            z_used: 0.5,
            offset: (7.0, -3.0),
            anchors: vec![(1, 100.0, 100.0)],
        };
        // File 1 isn't in the new layout (can't happen for same-scope reflows,
        // but the fallback must stay sane rather than teleport to (0,0)).
        assert_eq!(reanchor(&prev, &[(2, 0.0, 0.0)], (100.0, 100.0)), (7.0, -3.0));
    }
}
