//! Map view: the unified canvas — any [`Scope`](crate::scope::Scope)'s fns,
//! grouped by origin **dir ⊃ file** into nested bounding boxes, on **one
//! committed topology per scope**.
//!
//! ## The cartographic rule
//! The layout is a pure function of the scope — zoom, selection, and the
//! pointer never move anything. Every fn owns a footprint sized for its full
//! flow card, laid out once ([`commit`]); zooming only changes *what is drawn
//! inside* the fixed footprints, the way a map reveals streets and labels as
//! you approach while no city ever moves. Spatial memory stays valid for the
//! whole session; every level-of-detail choice is a rendering decision priced
//! in screen px, never a layout input. (The previous design let the LOD tiers
//! feed the layout, and every threshold crossing reflowed the world under the
//! user — re-anchoring softened it, but the topology churn itself was the
//! problem.)
//!
//! ## Commit pass (expensive, cached)
//! [`commit`] measures every fn's flow card ([`intrinsic`]), lays out each
//! file's intra-file **call graph** and each dir's **import DAG** (the
//! Sugiyama engine in `layout.rs`), and keeps the per-level [`Layout`]s plus
//! each box's content offset. It runs in seconds at project width, so the app
//! owns a small per-scope cache ([`MapCache`]); headless render passes `None`
//! and computes fresh (one frame, one commit).
//!
//! ## Render pass (cheap, every frame)
//! [`render_walk`] walks the committed layouts, tracking absolute content
//! coordinates, culls against the unprojected viewport, and picks each
//! footprint's drawing by its size **on screen**:
//! - a fn slot draws its full flow innards when it is in view and the zoom
//!   clears the flow threshold (`ViewParams::flow_z`, user-tunable from the
//!   legend, default [`DEFAULT_FLOW_Z`]; the selected fn always) — else a
//!   slab carrying the fn name at a screen-constant (cartographic) font
//!   clamped into the slot, or a bare slab when even that would be illegible;
//! - a file/dir box always occupies its committed rect; its contents draw
//!   only past [`CONTENTS_PX`] on screen, and its name draws over the box at
//!   a screen-constant font (the successor of the old collapse-to-chip);
//! - a file's call-edge overlay gates on screen size ([`EDGE_PX`]); dir-level
//!   import edges always draw (they are the wide-view structure);
//! - box strokes and edge splines draw at hairline (screen-constant) weight —
//!   at a project-fit zoom of ~0.005 a content-space 1px stroke is invisible.
//!
//! While the viewport is at home (armed `FitPolicy`, incl. headless) the
//! effective zoom is computed exactly from the committed extent — which no
//! longer depends on zoom, so the old fit⇄extent feedback loop cannot exist.

use super::flow::render_region;
use super::shared::{
    edges_asset_classed, edges_asset_scaled, legend_chip, pan_zoom_viewport_min, placed_graph,
    EdgeClass,
};
use super::SUB_SIZE;
use crate::flow::Region;
use crate::layout::{self, EndPoint, GEdge, GNode, Graph, Layout};
use crate::model::{ClaimDef, ClaimKind, FnDef, Project};
use crate::scope::Scope;
use crate::view::ViewParams;
use damascene_core::layout::intrinsic;
use damascene_core::prelude::*;
use std::collections::BTreeMap;

pub(crate) fn legend(flow_z: f32) -> El {
    row([
        text("map").mono().muted().font_size(SUB_SIZE),
        text("one committed layout per scope · boxes by imports, fns by calls, claims by citations · zoom reveals detail in place")
            .muted()
            .font_size(SUB_SIZE),
        legend_chip(claim_colors(ClaimKind::Axiom, false).0, "axiom"),
        legend_chip(claim_colors(ClaimKind::Claim, false).0, "claim"),
        legend_chip(claim_colors(ClaimKind::Requirement, false).0, "unmet req"),
        spacer(),
        text("innards ≥").muted().font_size(SUB_SIZE),
        button("−")
            .key("flowz_down")
            .ghost()
            .tooltip("Show fn internals from further out (lower zoom threshold)"),
        text(format!("{:.0}%", flow_z * 100.0))
            .mono()
            .muted()
            .font_size(SUB_SIZE)
            .center_text()
            .width(Size::Fixed(36.0))
            .tooltip("Zoom at which fn slots draw their flow internals"),
        button("+")
            .key("flowz_up")
            .ghost()
            .tooltip("Show fn internals only when closer (higher zoom threshold)"),
    ])
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_2)
    .align(Align::Center)
}

/// Default zoom past which fn slots draw their flow innards (the live value
/// rides in `ViewParams::flow_z` — the Map legend has −/+ controls). Well
/// below text legibility (~0.55): the innards read as *structure* long before
/// their text does, and structure-first is what a map is for.
pub const DEFAULT_FLOW_Z: f32 = 0.25;
/// A box (file or dir) smaller than this on screen (either dimension) draws
/// label-only: its committed footprint stays, its contents don't spend els.
const CONTENTS_PX: f32 = 48.0;
/// Below this on-screen size a file's intra-file call-edge overlay is dropped
/// (the fn slots stay — placement still encodes the call structure).
const EDGE_PX: f32 = 140.0;
/// Cartographic label sizes, in *screen* px: what a label wants to occupy on
/// screen regardless of zoom (clamped into its box, dropped when illegible).
const NAME_PX: f32 = 13.0;
const FILE_LABEL_PX: f32 = 13.0;
const DIR_LABEL_PX: f32 = 15.0;
/// A label whose screen size lands under this is noise — drop it.
const LEGIBLE_PX: f32 = 7.0;
/// Mono advance width as a fraction of font size (JetBrains Mono ≈ 0.6em).
const MONO_ADV: f32 = 0.62;
/// Scopes the app-owned cache keeps committed (LRU; a project-wide commit is
/// seconds of work and a few MB of geometry — cheap to keep, dear to redo).
const CACHE_CAP: usize = 6;

/// The committed geometry of one scope: every level's [`Layout`] (fn slots in
/// a file, child boxes in a dir) plus where each box's content sits inside
/// its chrome. Positions/sizes only — elements are rebuilt from it per frame.
pub struct Committed {
    scope: Scope,
    /// Per file: its intra-file call-graph layout + content offset in the box.
    files: BTreeMap<usize, LevelGeom>,
    /// Per dir path (`""` = root, `"kernel/"`, `"examples/io/"` …): the import
    /// layout of its children. Child index order = subdirs (name order), then
    /// files (ascending index) — [`DirNode`] iteration, identical every walk.
    dirs: BTreeMap<String, LevelGeom>,
    /// Every file box's absolute rect on this plane (canvas content coords) —
    /// the fly-to targets for scope-as-camera navigation ([`region_rect`]).
    file_rects: BTreeMap<usize, Rect>,
    /// Every dir box's absolute rect, keyed like [`Self::dirs`].
    dir_rects: BTreeMap<String, Rect>,
    /// Content extent (root layout + canvas padding).
    w: f32,
    h: f32,
}

struct LevelGeom {
    lay: Layout,
    /// Where the level's content starts inside its enclosing box (chrome
    /// padding + label band). Zero for the boxless root.
    off: (f32, f32),
    /// Per-edge class, index-aligned with the layout's edges (empty = all
    /// [`EdgeClass::Flow`] — dir levels, whose edges are all imports).
    classes: Vec<EdgeClass>,
}

/// One slot in a file box: a fn (flow card) or a proof-layer form (claim
/// card). Slot order per file = fns in definition order, then claims.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Member {
    Fn(usize),
    Claim(usize),
}

/// The app-owned per-scope cache of committed maps, lent to the pure view.
/// Most-recently-used last. Headless render passes `None`.
pub type MapCache = std::cell::RefCell<Vec<Committed>>;

/// A region on a committed plane — what scope-as-camera navigation flies to.
pub enum MapTarget<'a> {
    File(usize),
    /// A dir path as [`crate::scope::Scope::Dir`] spells it (no trailing `/`).
    Dir(&'a str),
}

/// The committed rect of a region on an **already-committed** plane, in the
/// Map canvas's content coordinates — push it as a `ViewportRequest::FrameRect`
/// to fly there. `None` when the plane isn't in the cache (the Map hasn't
/// drawn this scope yet) or the target isn't on it; the caller falls back to
/// a scope switch. Never commits: any plane on screen was committed by its
/// own build.
pub fn region_rect(cache: &MapCache, plane: &Scope, target: MapTarget) -> Option<Rect> {
    let g = cache.borrow();
    let com = g.iter().find(|c| c.scope == *plane)?;
    match target {
        MapTarget::File(i) => com.file_rects.get(&i).copied(),
        MapTarget::Dir(d) => com.dir_rects.get(&format!("{d}/")).copied(),
    }
}

pub(crate) fn canvas(project: &Project, p: &ViewParams, cache: Option<&MapCache>) -> El {
    let fns = p.scope.fns(project);
    let claims = p.scope.claims(project);
    if fns.is_empty() && claims.is_empty() {
        return column([
            h3("Map"),
            text("Pick a file or directory in the sidebar to scope the map.").muted(),
        ])
        .gap(tokens::SPACE_3)
        .padding(tokens::SPACE_8);
    }

    // The in-scope members of each file (a scope may include only some of a
    // file's fns, e.g. a call-tree), keyed by file: fns in definition order,
    // then the proof-layer forms. A statements-only file (kernel/facts.shard)
    // gets its box from the claim loop alone.
    let mut by_file: BTreeMap<usize, Vec<Member>> = BTreeMap::new();
    for &fi in &fns {
        by_file.entry(project.fns[fi].file).or_default().push(Member::Fn(fi));
    }
    for &ci in &claims {
        by_file.entry(project.claims[ci].file).or_default().push(Member::Claim(ci));
    }
    // The dir tree over the spanned files — the walk order both passes share.
    let mut root = DirNode::default();
    for &file in by_file.keys() {
        root.insert(&dir_segments(&project.files[file].rel), file);
    }

    // The committed topology: from the cache, or committed now. The borrow is
    // held across the render walk — sound because app_root builds exactly ONE
    // Map canvas per frame. A second canvas() call in one build (a future
    // minimap / split pane) would panic here; give it its own cache cell.
    let fresh;
    let guard;
    let com: &Committed = match cache {
        Some(cell) => {
            let mut g = cell.borrow_mut();
            if let Some(i) = g.iter().position(|c| c.scope == p.scope) {
                let hit = g.remove(i);
                g.push(hit);
            } else {
                g.push(commit(project, &p.scope, &by_file, &root));
                if g.len() > CACHE_CAP {
                    g.remove(0);
                }
            }
            guard = g;
            guard.last().expect("just ensured non-empty")
        }
        None => {
            fresh = commit(project, &p.scope, &by_file, &root);
            &fresh
        }
    };

    // Effective zoom for the screen-space LOD pricing. At home the armed fit
    // frames the committed extent — exactly computable, no readback needed
    // (and the headless readback would lie). Off home, the live readback.
    let (cw, ch) = p.canvas;
    let fit = ((cw - 48.0) / com.w).min((ch - 48.0) / com.h).min(1.0);
    let zoom = if p.at_home { fit } else { p.zoom };
    // Visible content rect, unprojected (content-local coords: screen s maps
    // to content (s − pan)/zoom), padded 25% each side so the canvas-size
    // estimate and pan-bounds slack never cull a partially-visible card.
    let cull = if p.at_home {
        None
    } else {
        let (mx, my) = (cw * 0.25, ch * 0.25);
        Some(Rect::new(
            (-p.pan.0 - mx) / zoom,
            (-p.pan.1 - my) / zoom,
            (cw + 2.0 * mx) / zoom,
            (ch + 2.0 * my) / zoom,
        ))
    };

    let rctx =
        RCtx { project, by_file: &by_file, selected_fn: p.selected_fn, zoom, flow_z: p.flow_z, cull };
    let pad = tokens::SPACE_6;
    let content = row([render_walk(&rctx, com, &root, "", (pad, pad), &[])]).padding(pad);

    // Let the user zoom out to just past the fit of *this* scope's extent —
    // a fixed floor either strands a project map half-fitted or lets a small
    // file map zoom out into nothing.
    pan_zoom_viewport_min(content, (fit * 0.6).clamp(0.0005, 0.04))
}

// ---------------------------------------------------------------------------
// Commit pass: scope → geometry. No zoom, no selection, no screen anywhere.
// ---------------------------------------------------------------------------

/// Lay the whole scope out at full (flow-card) footprints and keep every
/// level's geometry. Pure in `(project, scope)` — this is what makes the
/// topology committable: nothing the user does while navigating is an input.
fn commit(
    project: &Project,
    scope: &Scope,
    by_file: &BTreeMap<usize, Vec<Member>>,
    root: &DirNode,
) -> Committed {
    let mut out = Committed {
        scope: scope.clone(),
        files: BTreeMap::new(),
        dirs: BTreeMap::new(),
        file_rects: BTreeMap::new(),
        dir_rects: BTreeMap::new(),
        w: 0.0,
        h: 0.0,
    };
    let (w, h) = commit_children(project, by_file, root, "", &mut out);
    let pad = tokens::SPACE_6;
    out.w = w + 2.0 * pad;
    out.h = h + 2.0 * pad;
    // Resolve every box's absolute rect (positions are per-level until the
    // whole tree is placed) — the region index fly-to navigation reads.
    region_pass(&mut out, root, "", (pad, pad));
    out
}

/// Accumulate absolute rects for every dir/file box, mirroring the render
/// walk's iteration (subdirs in name order, then files) over the committed
/// per-level layouts.
fn region_pass(com: &mut Committed, node: &DirNode, path: &str, origin: (f32, f32)) {
    let rects: Vec<(f32, f32, f32, f32)> =
        com.dirs[path].lay.nodes.iter().map(|n| (n.x, n.y, n.w, n.h)).collect();
    let mut i = 0;
    for (name, sub) in &node.subdirs {
        let (x, y, w, h) = rects[i];
        let child_path = format!("{path}{name}/");
        let abs = (origin.0 + x, origin.1 + y);
        com.dir_rects.insert(child_path.clone(), Rect::new(abs.0, abs.1, w, h));
        let off = com.dirs[&child_path].off;
        region_pass(com, sub, &child_path, (abs.0 + off.0, abs.1 + off.1));
        i += 1;
    }
    for &file in &node.files {
        let (x, y, w, h) = rects[i];
        com.file_rects.insert(file, Rect::new(origin.0 + x, origin.1 + y, w, h));
        i += 1;
    }
}

/// Commit one dir level: children (subdir boxes, then file boxes) sized
/// bottom-up, placed by the import DAG among them. Returns the level's size.
fn commit_children(
    project: &Project,
    by_file: &BTreeMap<usize, Vec<Member>>,
    node: &DirNode,
    path: &str,
    out: &mut Committed,
) -> (f32, f32) {
    let mut sizes: Vec<(f32, f32)> = Vec::new();
    let mut files_of: Vec<Vec<usize>> = Vec::new();
    for (name, sub) in &node.subdirs {
        let child_path = format!("{path}{name}/");
        let inner = commit_children(project, by_file, sub, &child_path, out);
        let m = box_metrics(|| dir_band(name), inner.0, inner.1);
        // The child level was inserted boxless; now that its box chrome is
        // measured, record where its content sits inside the box.
        out.dirs.get_mut(&child_path).expect("just committed").off = m.off;
        sizes.push(m.size);
        files_of.push(sub.all_files());
    }
    for &file in &node.files {
        sizes.push(commit_file(project, by_file, file, out));
        files_of.push(vec![file]);
    }

    // A child B → A edge when any file in A imports any in-scope file in B:
    // dependency → dependent, so foundational children layer left and arrows
    // point at their users.
    let in_scope = |g: usize| by_file.contains_key(&g);
    let mut edges = Vec::new();
    for (i, fa) in files_of.iter().enumerate() {
        for (j, fb) in files_of.iter().enumerate() {
            if i != j && imports_between(project, fa, fb, &in_scope) {
                edges.push(GEdge {
                    from: EndPoint { node: j, port: 0 },
                    to: EndPoint { node: i, port: 0 },
                });
            }
        }
    }
    let nodes: Vec<GNode> = sizes.iter().map(|&(w, h)| GNode::simple(w, h)).collect();
    let cfg = layout::LayoutConfig::for_nodes(&nodes);
    let lay = layout::layout(&Graph { nodes, edges }, &cfg);
    let size = (lay.width, lay.height);
    out.dirs
        .insert(path.to_string(), LevelGeom { lay, off: (0.0, 0.0), classes: Vec::new() });
    size
}

/// Commit one file: every in-scope fn measured as a full flow card and every
/// in-scope claim as a claim card, placed together by the intra-file call +
/// proof-citation graph. Returns the file *box* size (content + chrome).
fn commit_file(
    project: &Project,
    by_file: &BTreeMap<usize, Vec<Member>>,
    file: usize,
    out: &mut Committed,
) -> (f32, f32) {
    let members = &by_file[&file];
    let nodes: Vec<GNode> = members
        .iter()
        .map(|&m| {
            let (w, h) = match m {
                Member::Fn(fi) => intrinsic(&flow_card(project, fi, false)),
                Member::Claim(ci) => intrinsic(&claim_card(project, ci)),
            };
            GNode::simple(w, h)
        })
        .collect();

    // Slot indices per layer, for edge resolution within this file.
    let mut fn_slot: BTreeMap<usize, usize> = BTreeMap::new();
    let mut claim_slot: BTreeMap<usize, usize> = BTreeMap::new();
    for (i, &m) in members.iter().enumerate() {
        match m {
            Member::Fn(g) => fn_slot.insert(g, i),
            Member::Claim(c) => claim_slot.insert(c, i),
        };
    }

    // Intra-file edges between in-scope members (dedup, no self-loops):
    // fn→fn calls, claim→claim citations, claim→fn subject links. All three
    // run dependency → dependent — callees, cited lemmas, and subject fns
    // layer LEFT of their users, so trust and control build rightward and
    // every arrow points at the thing leaning on its source.
    let mut seen = std::collections::HashSet::new();
    let mut edges = Vec::new();
    let mut classes = Vec::new();
    // `push(user, used, …)` — the edge is emitted used → user.
    let mut push = |i: usize, j: usize, class: EdgeClass, edges: &mut Vec<GEdge>| {
        if i != j && seen.insert((i, j)) {
            edges.push(GEdge {
                from: EndPoint { node: j, port: 0 },
                to: EndPoint { node: i, port: 0 },
            });
            classes.push(class);
        }
    };
    for (i, &m) in members.iter().enumerate() {
        match m {
            Member::Fn(g) => {
                for &callee in &project.fns[g].calls {
                    if let Some(&j) = fn_slot.get(&callee) {
                        push(i, j, EdgeClass::Flow, &mut edges);
                    }
                }
            }
            Member::Claim(c) => {
                for &cited in &project.claims[c].cites {
                    if let Some(&j) = claim_slot.get(&cited) {
                        push(i, j, EdgeClass::Cite, &mut edges);
                    }
                }
                for &subject in &project.claims[c].about {
                    if let Some(&j) = fn_slot.get(&subject) {
                        push(i, j, EdgeClass::About, &mut edges);
                    }
                }
            }
        }
    }
    let (nfns, nclaims) = (fn_slot.len(), claim_slot.len());
    let cfg = layout::LayoutConfig::for_nodes(&nodes);
    let lay = layout::layout(&Graph { nodes, edges }, &cfg);
    let m = box_metrics(|| file_header(project, file, nfns, nclaims), lay.width, lay.height);
    out.files.insert(file, LevelGeom { lay, off: m.off, classes });
    m.size
}

/// A box's measured chrome: the size it takes wrapping `(iw, ih)` content
/// under a header band, and where the content starts inside it. Uses the same
/// element construction as the render pass, so the two can never disagree.
struct BoxMetrics {
    size: (f32, f32),
    off: (f32, f32),
}

fn box_metrics(header: impl Fn() -> El, iw: f32, ih: f32) -> BoxMetrics {
    let band_h = intrinsic(&header()).1;
    let stub = column(Vec::<El>::new()).width(Size::Fixed(iw)).height(Size::Fixed(ih));
    let body = column([header(), stub])
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .align(Align::Start);
    BoxMetrics {
        size: intrinsic(&body),
        off: (tokens::SPACE_3, tokens::SPACE_3 + band_h + tokens::SPACE_2),
    }
}

/// The header row a file box is sized around (also the committed band height —
/// the render-pass label may draw bigger *over* the box, never into layout).
fn file_header(project: &Project, file: usize, nfns: usize, nclaims: usize) -> El {
    let f = &project.files[file];
    let base = f.rel.rsplit_once('/').map(|(_, b)| b).unwrap_or(&f.rel);
    let what = match (nfns, nclaims) {
        (n, 0) => format!("{n} fns"),
        (0, m) => format!("{m} claims"),
        (n, m) => format!("{n} fns · {m} claims"),
    };
    row([
        text(base.to_string()).mono().semibold().font_size(12.0).nowrap_text(),
        text(what).mono().muted().font_size(SUB_SIZE).nowrap_text(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center)
}

/// The band a dir box is sized around.
fn dir_band(name: &str) -> El {
    text(format!("▸ {name}/")).mono().semibold().font_size(11.0).muted().nowrap_text()
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

// ---------------------------------------------------------------------------
// Render pass: geometry + screen → elements. Never feeds back into layout.
// ---------------------------------------------------------------------------

/// Per-frame inputs to the render walk.
struct RCtx<'a> {
    project: &'a Project,
    by_file: &'a BTreeMap<usize, Vec<Member>>,
    selected_fn: Option<usize>,
    /// Effective zoom — prices every screen-space LOD decision.
    zoom: f32,
    /// Zoom past which fn slots draw their flow innards (user-tunable).
    flow_z: f32,
    /// Visible content rect (absolute content coords), `None` = draw all.
    cull: Option<Rect>,
}

impl RCtx<'_> {
    fn in_view(&self, x: f32, y: f32, w: f32, h: f32) -> bool {
        overlaps(self.cull, x, y, w, h)
    }

    /// Cartographic line weight: content-space width that lands ~1px on
    /// screen below 1:1 (and the natural 1px at/above it). Strokes are part
    /// of the *rendering*, so unlike the geometry they may depend on zoom.
    fn hairline(&self) -> f32 {
        1.0 / self.zoom.min(1.0)
    }
}

/// Whether a rect overlaps the cull window (`None` = everything is visible).
fn overlaps(cull: Option<Rect>, x: f32, y: f32, w: f32, h: f32) -> bool {
    match cull {
        None => true,
        Some(c) => x < c.right() && x + w > c.x && y < c.bottom() && y + h > c.y,
    }
}

/// Render one committed dir level at `origin` (the level's absolute content
/// position): each child drawn per its screen size, import edges always.
/// `shade` carries the ancestor labels' absolute rects — a screen-constant
/// label overflows its committed band at far zooms, so a descendant whose own
/// label would land under an ancestor's yields to it (the map convention:
/// the enclosing name wins until you're close enough for both).
fn render_walk(
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
                }
            })
            .collect();
        // Edge LOD: below ~EDGE_PX on screen the intra-file web is unreadable
        // gray mass (placement already encodes the call structure — callers
        // left, callees right), so the overlay is dropped. Pure rendering:
        // the routes were committed with the layout and never change.
        let edge_overlay = if geom.lay.width.min(geom.lay.height) * ctx.zoom >= EDGE_PX {
            edges_asset_classed(&geom.lay, ctx.hairline(), &geom.classes)
        } else {
            VectorAsset::from_paths([0.0, 0.0, geom.lay.width, geom.lay.height], Vec::new())
        };
        let inner = placed_graph(&geom.lay, slots, edge_overlay);
        layers.push(at(inner, geom.off, geom.lay.width, geom.lay.height, w, h));
    }
    if let Some((lbl, _)) = carto_label(base, FILE_LABEL_PX, ctx.zoom, w, h, abs, shade) {
        layers.push(lbl);
    }
    let (nfns, nclaims) =
        members.iter().fold((0, 0), |(a, b), m| match m {
            Member::Fn(_) => (a + 1, b),
            Member::Claim(_) => (a, b + 1),
        });
    stack(layers)
        .width(Size::Fixed(w))
        .height(Size::Fixed(h))
        .fill(tokens::CARD.mix(tokens::BACKGROUND, 0.4))
        .stroke(tokens::BORDER)
        .stroke_width(ctx.hairline())
        .radius(8.0)
        .tooltip(format!(
            "{} · {} fns · {} claims · {} lines",
            f.rel,
            nfns,
            nclaims,
            f.counts.total()
        ))
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
    let name = &ctx.project.fns[fn_idx].name;
    let chars = name.chars().count().max(1) as f32;
    let font = (NAME_PX / ctx.zoom).min(h * 0.45).min(w * 0.92 / (chars * MONO_ADV));
    let body: Vec<El> = if font * ctx.zoom >= LEGIBLE_PX {
        vec![text(name.clone()).mono().semibold().font_size(font).nowrap_text()]
    } else {
        Vec::new()
    };
    column(body)
        .align(Align::Center)
        .justify(Justify::Center)
        .width(Size::Fixed(w))
        .height(Size::Fixed(h))
        .radius(7.0)
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .stroke_width(ctx.hairline())
        .key(format!("fn:{fn_idx}"))
        .tooltip(super::methods::node_tip(ctx.project, fn_idx))
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

/// The proof-layer card colors `(fill, stroke)` by kind — the Map's arm of
/// the project-wide convention (Systems heat): **amber = proof**. Axioms are
/// the loud amber (assumed, not proven — trust roots), plain claims a faint
/// wash, requirements green once fulfilled and red while open.
fn claim_colors(kind: ClaimKind, fulfilled: bool) -> (Color, Color) {
    match (kind, fulfilled) {
        (ClaimKind::Axiom, _) => {
            (tokens::CARD.mix(tokens::WARNING, 0.22), tokens::WARNING.mix(tokens::BORDER, 0.35))
        }
        (ClaimKind::Requirement, false) => (
            tokens::CARD.mix(tokens::DESTRUCTIVE, 0.35),
            tokens::DESTRUCTIVE.mix(tokens::FOREGROUND, 0.35),
        ),
        (ClaimKind::Requirement, true) => {
            (tokens::CARD.mix(tokens::SUCCESS, 0.12), tokens::SUCCESS.mix(tokens::BORDER, 0.5))
        }
        (ClaimKind::Claim | ClaimKind::Fulfills, _) => {
            (tokens::CARD.mix(tokens::WARNING, 0.08), tokens::BORDER)
        }
    }
}

/// The kind tag a claim card leads with, and its text tint.
fn claim_tag(c: &ClaimDef) -> (&'static str, Color) {
    match (c.kind, c.fulfilled) {
        (ClaimKind::Axiom, _) => ("axiom", tokens::WARNING),
        (ClaimKind::Claim, _) => ("claim", tokens::WARNING.mix(tokens::MUTED_FOREGROUND, 0.5)),
        (ClaimKind::Requirement, true) => ("req ✓", tokens::SUCCESS),
        (ClaimKind::Requirement, false) => ("req ✗", tokens::DESTRUCTIVE.mix(tokens::FOREGROUND, 0.4)),
        (ClaimKind::Fulfills, _) => ("proof", tokens::WARNING.mix(tokens::MUTED_FOREGROUND, 0.5)),
    }
}

fn claim_tip(c: &ClaimDef) -> String {
    let (tag, _) = claim_tag(c);
    format!("{tag} {}\n{}\ncites {} · about {} fns", c.name, c.goal, c.cites.len(), c.about.len())
}

/// One proof-layer form as an intrinsic card: kind tag + name, the goal
/// statement, then the proof's structure in the Flow vocabulary (case-split
/// frames, have facts, step ladders with lemma citations as the bold heroes —
/// see `proof.rs`). Like [`flow_card`] it hugs — the commit pass measures
/// exactly this construction, so it lands in its footprint.
fn claim_card(project: &Project, ci: usize) -> El {
    let c = &project.claims[ci];
    let (tag, tag_color) = claim_tag(c);
    let (fill, stroke) = claim_colors(c.kind, c.fulfilled);
    let title = row([
        text(tag).mono().semibold().font_size(SUB_SIZE).text_color(tag_color).nowrap_text(),
        text(c.name.clone())
            .mono()
            .semibold()
            .font_size(super::TITLE_SIZE)
            .nowrap_text()
            .ellipsis(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);
    let mut parts = vec![title];
    if !c.goal.is_empty() {
        parts.push(
            text(ellipt(&c.goal, 52)).mono().muted().font_size(SUB_SIZE).nowrap_text(),
        );
    }
    if let Some(region) = crate::proof::build(&c.form) {
        parts.push(render_region(&region));
    }
    column(parts)
        .gap(tokens::SPACE_1)
        .padding(8.0)
        .radius(7.0)
        .fill(fill)
        .stroke(stroke)
        .key(format!("claim:{ci}"))
        .tooltip(claim_tip(c))
}

/// Truncate to `max` chars with an ellipsis (goal statements can be long).
fn ellipt(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max - 1).collect();
        format!("{cut}…")
    }
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

/// An invisible placeholder for a culled slot — [`placed_graph`] rects are
/// index-aligned with the level's nodes, so every slot must yield an element.
/// Unkeyed and unfilled: draws nothing, intercepts nothing.
fn blank() -> El {
    column(Vec::<El>::new())
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

/// One fn as an intrinsic flow card: a name/signature header, its named
/// arguments (LabVIEW-style inputs), then its region tree (the same renderer
/// the Flow/Board views use). No fixed size — it hugs; its intrinsic size is
/// what the commit pass measures the footprint from, so `selected` must never
/// change geometry (fill/stroke only).
fn flow_card(project: &Project, fn_idx: usize, selected: bool) -> El {
    let f = &project.fns[fn_idx];
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
        .tooltip(super::methods::node_tip(project, fn_idx));
    if selected {
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

    fn placed(x: f32, y: f32, w: f32, h: f32) -> layout::PlacedNode {
        layout::PlacedNode { x, y, w, h, in_ports: Vec::new(), out_ports: Vec::new() }
    }

    fn level(nodes: Vec<layout::PlacedNode>, off: (f32, f32)) -> LevelGeom {
        LevelGeom {
            lay: Layout { nodes, edges: Vec::new(), width: 500.0, height: 300.0 },
            off,
            classes: Vec::new(),
        }
    }

    #[test]
    fn region_pass_resolves_absolute_rects_and_region_rect_reads_them() {
        // Root level: subdir "a" box then file 3's box. Inside a/: file 7.
        let mut root = DirNode::default();
        root.insert(&["a"], 7);
        root.insert(&[], 3);
        let mut com = Committed {
            scope: Scope::Project,
            files: BTreeMap::new(),
            dirs: BTreeMap::from([
                (
                    String::new(),
                    level(vec![placed(10.0, 20.0, 300.0, 200.0), placed(350.0, 20.0, 100.0, 80.0)], (0.0, 0.0)),
                ),
                ("a/".to_string(), level(vec![placed(5.0, 6.0, 60.0, 40.0)], (12.0, 30.0))),
            ]),
            file_rects: BTreeMap::new(),
            dir_rects: BTreeMap::new(),
            w: 500.0,
            h: 300.0,
        };
        region_pass(&mut com, &root, "", (24.0, 24.0));
        assert_eq!(com.dir_rects["a/"], Rect::new(34.0, 44.0, 300.0, 200.0));
        assert_eq!(com.file_rects[&3], Rect::new(374.0, 44.0, 100.0, 80.0));
        // File 7 sits inside a/'s box: dir origin + the box's content offset.
        assert_eq!(com.file_rects[&7], Rect::new(51.0, 80.0, 60.0, 40.0));

        let cache = MapCache::new(vec![com]);
        let hit = region_rect(&cache, &Scope::Project, MapTarget::Dir("a"));
        assert_eq!(hit, Some(Rect::new(34.0, 44.0, 300.0, 200.0)));
        let file = region_rect(&cache, &Scope::Project, MapTarget::File(7));
        assert_eq!(file, Some(Rect::new(51.0, 80.0, 60.0, 40.0)));
        // Misses: a dir not on the plane; a plane not in the cache.
        assert_eq!(region_rect(&cache, &Scope::Project, MapTarget::Dir("b")), None);
        assert_eq!(region_rect(&cache, &Scope::Dir("a".into()), MapTarget::File(7)), None);
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

    #[test]
    fn cull_rect_overlap_is_open_on_both_sides() {
        let c = Some(Rect::new(100.0, 100.0, 200.0, 200.0));
        assert!(overlaps(c, 150.0, 150.0, 10.0, 10.0)); // inside
        assert!(overlaps(c, 50.0, 50.0, 100.0, 100.0)); // straddles the corner
        assert!(!overlaps(c, 0.0, 0.0, 50.0, 50.0)); // fully outside
        assert!(!overlaps(c, 301.0, 150.0, 10.0, 10.0)); // past the right edge
        assert!(overlaps(None, 1e9, 1e9, 1.0, 1.0)); // no window = draw all
    }
}
