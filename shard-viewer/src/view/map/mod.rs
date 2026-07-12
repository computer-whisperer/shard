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

mod cards;
mod commit;
mod deck;
mod els;

pub use commit::debug_file_graph;

use cards::{claim_colors, orphan_colors, type_colors};
use commit::commit;
use deck::shape_deck;
use els::render_walk;

use super::shared::{legend_chip, pan_zoom_viewport_min, EdgeClass};
use super::{SUB_SIZE, ViewParams};
use crate::layout::Layout;
use crate::model::{ClaimKind, Project};
use crate::scope::Scope;
use damascene_core::prelude::*;
use std::collections::BTreeMap;

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

pub(crate) fn legend(flow_z: f32) -> El {
    row([
        text("map").mono().muted().font_size(SUB_SIZE),
        text("one committed layout per scope · boxes by imports, fns by calls, claims by citations, types by composition · distant edges fade · hover traces a member")
            .muted()
            .font_size(SUB_SIZE),
        legend_chip(claim_colors(ClaimKind::Axiom, false).0, "axiom"),
        legend_chip(claim_colors(ClaimKind::Claim, false).0, "claim"),
        legend_chip(claim_colors(ClaimKind::Requirement, false).0, "unmet req"),
        legend_chip(type_colors().0, "type"),
        legend_chip(orphan_colors().0, "orphan fn"),
        legend_chip(tokens::CARD.mix(tokens::ACCENT, 0.3), "impl-heavy box"),
        legend_chip(tokens::CARD.mix(tokens::WARNING, 0.3), "proof-heavy box"),
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
    /// Per-edge slot endpoints `(from, to)`, index-aligned like `classes`
    /// (empty for dir levels). The render pass needs incidence to reveal a
    /// focused member's [`EdgeClass::Use`] edges; the routed layout alone
    /// no longer knows which nodes an edge connects.
    ends: Vec<(usize, usize)>,
    /// The member behind each slot, index-aligned with the layout's nodes
    /// (empty for dir levels — their slots are boxes, not members). Makes the
    /// committed geometry self-describing, so [`region_rect`] can resolve a
    /// fn's card rect without re-deriving the scope's member gathering.
    members: Vec<Member>,
}

/// One slot in a file box: a fn (flow card), a proof-layer form (claim
/// card), a datastructure definition (type card), or the file's own
/// `;;;`-header docstring (one prose card per documented file). Slot order
/// per file = fns in definition order, then claims, then types, then the
/// doc card.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Member {
    Fn(usize),
    Claim(usize),
    Type(usize),
    /// The file's `;;;` header as a card (the index is the *file*). No
    /// edges — it describes the whole box, not any member.
    Doc(usize),
}

impl Member {
    /// Parse a member's element key ("fn:12" / "claim:3" / "type:7") — the
    /// bridge from damascene hover readback to the reveal/deck focus.
    fn from_key(key: &str) -> Option<Member> {
        let (kind, idx) = key.split_once(':')?;
        let i: usize = idx.parse().ok()?;
        match kind {
            "fn" => Some(Member::Fn(i)),
            "claim" => Some(Member::Claim(i)),
            "type" => Some(Member::Type(i)),
            _ => None,
        }
    }
}

/// The app-owned per-scope cache of committed maps, lent to the pure view.
/// Most-recently-used last. Headless render passes `None`.
pub type MapCache = std::cell::RefCell<Vec<Committed>>;

/// A region on a committed plane — what scope-as-camera navigation flies to.
pub enum MapTarget<'a> {
    File(usize),
    /// A dir path as [`crate::scope::Scope::Dir`] spells it (no trailing `/`).
    Dir(&'a str),
    /// One fn's committed flow card (the "read this fn large" gesture — the
    /// successor of the old standalone Flow view).
    Fn(usize),
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
        // A fn's card: its slot inside its file's committed level, made
        // absolute through the file box's rect + content offset. Searching
        // every file is fine — the member lists are the scope's own size.
        MapTarget::Fn(g) => com.files.iter().find_map(|(file, geom)| {
            let slot = geom.members.iter().position(|&m| m == Member::Fn(g))?;
            let fr = com.file_rects.get(file)?;
            let n = &geom.lay.nodes[slot];
            Some(Rect::new(fr.x + geom.off.0 + n.x, fr.y + geom.off.1 + n.y, n.w, n.h))
        }),
    }
}

pub(crate) fn canvas(project: &Project, p: &ViewParams, cache: Option<&MapCache>) -> El {
    let fns = p.scope.fns(project);
    let claims = p.scope.claims(project);
    let types = p.scope.types(project);
    if fns.is_empty() && claims.is_empty() && types.is_empty() {
        return column([
            h3("Map"),
            text("Pick a file or directory in the sidebar to scope the map.").muted(),
        ])
        .gap(tokens::SPACE_3)
        .padding(tokens::SPACE_8);
    }

    // The in-scope members of each file (a scope may include only some of a
    // file's fns, e.g. a call-tree), keyed by file: fns in definition order,
    // then the proof-layer forms, then the datastructure definitions. A
    // statements-only file (kernel/facts.shard) gets its box from the claim
    // loop alone.
    let mut by_file: BTreeMap<usize, Vec<Member>> = BTreeMap::new();
    for &fi in &fns {
        by_file.entry(project.fns[fi].file).or_default().push(Member::Fn(fi));
    }
    for &ci in &claims {
        by_file.entry(project.claims[ci].file).or_default().push(Member::Claim(ci));
    }
    for &ti in &types {
        by_file.entry(project.types[ti].file).or_default().push(Member::Type(ti));
    }
    // A documented file also carries its `;;;` header as a member card — the
    // author's account of the module, committed into the file's layout like
    // any other card (file boxes usually have the slack, and it's the single
    // most informative card a box can spend it on).
    for (&file, members) in by_file.iter_mut() {
        if !project.files[file].doc.is_empty() {
            members.push(Member::Doc(file));
        }
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

    // The reveal/deck focus: the hovered member (any kind), else the selected
    // fn. Hover is pure render input — the committed topology can't hear it.
    let focus = p
        .hovered
        .as_deref()
        .and_then(Member::from_key)
        .or(p.selected_fn().map(Member::Fn));

    let rctx = RCtx {
        project,
        by_file: &by_file,
        selected_fn: p.selected_fn(),
        focus,
        zoom,
        flow_z: p.flow_z,
        cull,
    };
    let pad = tokens::SPACE_6;
    let content = row([render_walk(&rctx, com, &root, "", (pad, pad), &[])]).padding(pad);

    // Let the user zoom out to just past the fit of *this* scope's extent —
    // a fixed floor either strands a project map half-fitted or lets a small
    // file map zoom out into nothing.
    let viewport = pan_zoom_viewport_min(content, (fit * 0.6).clamp(0.0005, 0.04));

    // The shape deck: a screen-space overlay listing the focused member's
    // datastructure definitions — the answer to "I'm zoomed into one fn and
    // its types are defined three files away". Overlay, not topology: it
    // follows hover/selection freely without touching the committed plane.
    // It shares the flow-innards gate: far out, fn slots are name slabs and
    // the map is being read as a map — a definition card popping up on every
    // hover sweep is noise there. The deck earns its screen space exactly
    // when the reading zoom does (zoom ≥ flow_z, the same threshold that
    // turns the innards on).
    //
    // The stack wrapper is UNCONDITIONAL (blank deck slot when nothing is
    // focused or the zoom hasn't earned it): a node's `computed_id` is its
    // full tree path, and the viewport's pan/zoom + fit-takeover state lives
    // under that id. Toggling the wrapper with hover changed the viewport's
    // path every time the deck appeared, so the armed Contain policy stopped
    // recognizing the user's takeover and snapped to fit — hover on/off
    // oscillated the zoom.
    let deck = if zoom >= p.flow_z {
        shape_deck(project, focus).unwrap_or_else(blank)
    } else {
        blank()
    };
    stack([
        viewport,
        row([spacer(), deck])
            .width(Size::Fill(1.0))
            .align(Align::Start)
            .padding(tokens::SPACE_4),
    ])
    .width(Size::Fill(1.0))
    .height(Size::Fill(1.0))
}

/// Per-frame inputs to the render walk.
struct RCtx<'a> {
    project: &'a Project,
    by_file: &'a BTreeMap<usize, Vec<Member>>,
    selected_fn: Option<usize>,
    /// The hover-else-selection member whose shape-usage edges the render
    /// reveals (see [`EdgeClass::Use`]). Render-only, like zoom.
    focus: Option<Member>,
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

/// An invisible placeholder for a culled slot — [`placed_graph`] rects are
/// index-aligned with the level's nodes, so every slot must yield an element.
/// Unkeyed and unfilled: draws nothing, intercepts nothing.
fn blank() -> El {
    column(Vec::<El>::new())
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

#[cfg(test)]
mod tests {
    use super::*;

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
