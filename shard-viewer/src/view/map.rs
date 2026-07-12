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
    composition_bar, edges_asset_filtered, edges_asset_scaled, heat_color, legend_chip,
    pan_zoom_viewport_min, placed_graph, EdgeClass,
};
use super::SUB_SIZE;
use crate::flow::Region;
use crate::layout::{self, EndPoint, GEdge, GNode, Graph, Layout};
use crate::model::{ClaimDef, ClaimKind, FnDef, Project, TypeDef, TypeKind};
use crate::scope::Scope;
use crate::view::ViewParams;
use damascene_core::layout::intrinsic;
use damascene_core::prelude::*;
use std::collections::BTreeMap;

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
    out.dirs.insert(
        path.to_string(),
        LevelGeom {
            lay,
            off: (0.0, 0.0),
            classes: Vec::new(),
            ends: Vec::new(),
            members: Vec::new(),
        },
    );
    size
}

/// The intra-file graph exactly as committed: every fn measured as a full
/// flow card, claims as claim cards, types as type cards, with the classed
/// edge set among them. Factored from [`commit_file`] so layout diagnostics
/// can run experiments on the true committed topology.
fn file_graph(project: &Project, members: &[Member]) -> (Graph, Vec<EdgeClass>) {
    let nodes: Vec<GNode> = members
        .iter()
        .map(|&m| {
            let (w, h) = match m {
                Member::Fn(fi) => intrinsic(&flow_card(project, fi, false)),
                Member::Claim(ci) => intrinsic(&claim_card(project, ci)),
                Member::Type(ti) => intrinsic(&type_card(project, ti, true)),
                Member::Doc(fi) => intrinsic(&filedoc_card(project, fi)),
            };
            // The doc card leads the packing order: the file's own account
            // takes the top-left slot, where reading starts.
            GNode { lead: matches!(m, Member::Doc(_)), ..GNode::simple(w, h) }
        })
        .collect();

    // Slot indices per layer, for edge resolution within this file.
    let mut fn_slot: BTreeMap<usize, usize> = BTreeMap::new();
    let mut claim_slot: BTreeMap<usize, usize> = BTreeMap::new();
    let mut type_slot: BTreeMap<usize, usize> = BTreeMap::new();
    for (i, &m) in members.iter().enumerate() {
        match m {
            Member::Fn(g) => fn_slot.insert(g, i),
            Member::Claim(c) => claim_slot.insert(c, i),
            Member::Type(t) => type_slot.insert(t, i),
            Member::Doc(_) => None, // edge-less; no slot map needed
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
                // Shape usage: the types this fn constructs/matches layer
                // left of it, committed as [`EdgeClass::Use`] — informing
                // placement always, drawn only under a hover/selection
                // reveal. Only the strong (ctor) tier earns routed edges;
                // weak signature mentions live in the shape deck alone —
                // committing them too doubles the dummy load for edges that
                // say "passes through", not "depends on the shape".
                for &t in &project.fns[g].shapes {
                    if let Some(&j) = type_slot.get(&t) {
                        push(i, j, EdgeClass::Use, &mut edges);
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
            Member::Type(t) => {
                // Composition: field types layer left of the aggregates
                // built from them (always drawn — the shape web is sparse).
                for &dep in &project.types[t].composed {
                    if let Some(&j) = type_slot.get(&dep) {
                        push(i, j, EdgeClass::Shape, &mut edges);
                    }
                }
            }
            Member::Doc(_) => {}
        }
    }
    (Graph { nodes, edges }, classes)
}

/// Diagnostic hook: the committed graph for `file` under a File scope (all
/// members, the same fns/claims/types order [`canvas`] gathers). Lets the
/// layout diag bin measure the real committed topology — card-true node
/// sizes — instead of approximating them.
#[doc(hidden)]
pub fn debug_file_graph(project: &Project, file: usize) -> Graph {
    let f = &project.files[file];
    let mut members: Vec<Member> = (f.fns.iter().map(|&g| Member::Fn(g)))
        .chain(f.claims.iter().map(|&c| Member::Claim(c)))
        .chain(f.types.iter().map(|&t| Member::Type(t)))
        .collect();
    if !f.doc.is_empty() {
        members.push(Member::Doc(file));
    }
    file_graph(project, &members).0
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
    let mut counts = (0usize, 0usize, 0usize);
    for &m in members {
        match m {
            Member::Fn(_) => counts.0 += 1,
            Member::Claim(_) => counts.1 += 1,
            Member::Type(_) => counts.2 += 1,
            Member::Doc(_) => {}
        }
    }
    let (graph, classes) = file_graph(project, members);
    let ends: Vec<(usize, usize)> =
        graph.edges.iter().map(|e| (e.from.node, e.to.node)).collect();
    let cfg = layout::LayoutConfig::for_nodes(&graph.nodes);
    let lay = layout::layout(&graph, &cfg);
    let (nfns, nclaims, ntypes) = counts;
    let m =
        box_metrics(|| file_header(project, file, nfns, nclaims, ntypes), lay.width, lay.height);
    out.files
        .insert(file, LevelGeom { lay, off: m.off, classes, ends, members: members.clone() });
    m.size
}

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
fn file_header(project: &Project, file: usize, nfns: usize, nclaims: usize, ntypes: usize) -> El {
    let f = &project.files[file];
    let base = f.rel.rsplit_once('/').map(|(_, b)| b).unwrap_or(&f.rel);
    let mut parts: Vec<String> = Vec::new();
    if nfns > 0 {
        parts.push(format!("{nfns} fns"));
    }
    if nclaims > 0 {
        parts.push(format!("{nclaims} claims"));
    }
    if ntypes > 0 {
        parts.push(format!("{ntypes} types"));
    }
    let what = parts.join(" · ");
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
    let mut tip =
        format!("{tag} {}\n{}\ncites {} · about {} fns", c.name, c.goal, c.cites.len(), c.about.len());
    if !c.doc.is_empty() {
        tip = format!("{tip}\n\n{}", c.doc);
    }
    tip
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
    if let Some(summary) = doc_summary(&c.doc, 52) {
        parts.push(summary);
    }
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

/// The one-line docstring summary a card carries: the doc's first line,
/// clipped. `None` for an undocumented member — cards don't spend a row on
/// absence. Prose (non-mono) and muted, so it reads as commentary beside the
/// mono code text; the full block lives in the tooltip and detail panel.
fn doc_summary(doc: &str, max: usize) -> Option<El> {
    let line = doc.lines().next()?.trim();
    if line.is_empty() {
        return None;
    }
    Some(text(ellipt(line, max)).muted().font_size(SUB_SIZE).nowrap_text())
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

/// The shape-layer card colors `(fill, stroke)` — the blue family
/// (`tokens::INFO`), keeping amber for proof.
fn type_colors() -> (Color, Color) {
    (tokens::CARD.mix(tokens::INFO, 0.10), tokens::INFO.mix(tokens::BORDER, 0.5))
}

/// The kind tag a type card leads with.
fn type_tag(kind: TypeKind) -> &'static str {
    match kind {
        TypeKind::Data => "type",
        TypeKind::Record => "record",
        TypeKind::Opaque => "opaque",
    }
}

fn type_tip(t: &TypeDef) -> String {
    let mut tip = format!(
        "{} {}\n{} ctors · composed of {} types",
        type_tag(t.kind),
        t.name,
        t.ctors.len(),
        t.composed.len()
    );
    if !t.doc.is_empty() {
        tip = format!("{tip}\n\n{}", t.doc);
    }
    tip
}

/// A datastructure definition as an intrinsic card: kind tag + name (+ type
/// params), then one row per ctor — ctor name, its field types, and the
/// author's trailing `;` note when the source carries one. A record reads the
/// same with field names as the rows; an opaque `sig type` is just the head
/// (its ctors are the module impl's business). Like [`flow_card`] it hugs —
/// the commit pass measures exactly this construction.
fn type_card(project: &Project, ti: usize, keyed: bool) -> El {
    let t = &project.types[ti];
    let (fill, stroke) = type_colors();
    let mut head = t.name.clone();
    if !t.params.is_empty() {
        head = format!("({} {})", t.name, t.params.join(" "));
    }
    let title = row([
        text(type_tag(t.kind))
            .mono()
            .semibold()
            .font_size(SUB_SIZE)
            .text_color(tokens::INFO.mix(tokens::MUTED_FOREGROUND, 0.3))
            .nowrap_text(),
        text(head).mono().semibold().font_size(super::TITLE_SIZE).nowrap_text().ellipsis(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);

    let mut parts = vec![title];
    if let Some(summary) = doc_summary(&t.doc, 52) {
        parts.push(summary);
    }
    if t.kind == TypeKind::Opaque {
        parts.push(text("ctors private to impl").muted().font_size(SUB_SIZE).nowrap_text());
    }
    for c in &t.ctors {
        let mut cells = vec![
            text(c.name.clone())
                .mono()
                .semibold()
                .font_size(SUB_SIZE)
                .text_color(tokens::FOREGROUND)
                .nowrap_text(),
        ];
        if !c.fields.is_empty() {
            cells.push(
                text(ellipt(&c.fields.join(" "), 46)).mono().muted().font_size(SUB_SIZE).nowrap_text(),
            );
        }
        if !c.comment.is_empty() {
            cells.push(
                text(format!("· {}", ellipt(&c.comment, 40)))
                    .font_size(SUB_SIZE)
                    .text_color(tokens::MUTED_FOREGROUND.mix(tokens::INFO, 0.25))
                    .nowrap_text(),
            );
        }
        parts.push(row(cells).gap(tokens::SPACE_2).align(Align::Center));
    }
    let card = column(parts)
        .gap(tokens::SPACE_1)
        .padding(8.0)
        .radius(7.0)
        .fill(fill)
        .stroke(stroke);
    // The deck draws unkeyed copies of cards the plane may also hold — two
    // Els with one key would collide in hit-test. Unkeyed nodes are never
    // hover targets, so the deck variant also drops the tooltip (it would
    // be dead, and the lint would rightly flag it).
    if keyed { card.key(format!("type:{ti}")).tooltip(type_tip(t)) } else { card }
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

/// The file's `;;;` header as a prose card — the author's account of the
/// module, spending the box's free space on the most informative thing it
/// can hold. Leads with a `;;;` marker (the corpus's own syntax) and the
/// file's basename; the body keeps the author's line breaks (headers are
/// already hand-formatted to a comment column).
fn filedoc_card(project: &Project, file: usize) -> El {
    let f = &project.files[file];
    let base = f.rel.rsplit_once('/').map(|(_, b)| b).unwrap_or(&f.rel);
    let title = row([
        text(";;;")
            .mono()
            .semibold()
            .font_size(SUB_SIZE)
            .text_color(tokens::MUTED_FOREGROUND)
            .nowrap_text(),
        text(base.to_string())
            .mono()
            .semibold()
            .font_size(super::TITLE_SIZE)
            .nowrap_text()
            .ellipsis(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);
    let mut parts = vec![title];
    for line in f.doc.lines() {
        // Keep blank lines as paragraph breaks (an empty text measures away).
        let shown = if line.trim().is_empty() { " ".to_string() } else { ellipt(line, 88) };
        parts.push(text(shown).muted().font_size(SUB_SIZE).nowrap_text());
    }
    column(parts)
        .gap(2.0)
        .padding(8.0)
        .radius(7.0)
        .fill(tokens::BACKGROUND.mix(tokens::CARD, 0.5))
        .stroke(tokens::BORDER.mix(tokens::BACKGROUND, 0.4))
}

/// The screen-space shape deck: the focused member's datastructure
/// definitions as compact cards, docked at the canvas edge. This is the only
/// way to see a shape's composition while zoomed into a fn whose types live
/// in another file — the committed plane can't show cross-file members, an
/// overlay can. `None` when the focus has no shapes to show.
fn shape_deck(project: &Project, focus: Option<Member>) -> Option<El> {
    let (title, type_ids): (String, Vec<usize>) = match focus? {
        Member::Fn(g) => {
            let f = &project.fns[g];
            let ids: Vec<usize> =
                f.shapes.iter().chain(&f.sig_types).copied().collect();
            (f.name.clone(), ids)
        }
        // A type's own deck: what it's composed of (its definition is already
        // under the cursor; its parts may be anywhere).
        Member::Type(t) => (project.types[t].name.clone(), project.types[t].composed.clone()),
        Member::Claim(_) | Member::Doc(_) => return None,
    };
    if type_ids.is_empty() {
        return None;
    }
    const DECK_CAP: usize = 4;
    let shown = &type_ids[..type_ids.len().min(DECK_CAP)];
    let mut cards: Vec<El> = vec![
        row([
            text("shapes of").muted().font_size(SUB_SIZE).nowrap_text(),
            text(title).mono().semibold().font_size(SUB_SIZE).nowrap_text().ellipsis(),
        ])
        .gap(tokens::SPACE_2),
    ];
    for &ti in shown {
        let home = &project.files[project.types[ti].file].rel;
        cards.push(
            column([
                type_card(project, ti, false),
                text(home.clone()).muted().font_size(9.0).nowrap_text().ellipsis(),
            ])
            .gap(2.0)
            .align(Align::End),
        );
    }
    if type_ids.len() > shown.len() {
        cards.push(
            text(format!("+{} more", type_ids.len() - shown.len()))
                .muted()
                .font_size(SUB_SIZE)
                .nowrap_text(),
        );
    }
    // Unkeyed throughout: the deck is a read-only lens that must never
    // intercept a pan drag or steal hover from the plane beneath it.
    Some(
        column(cards)
            .gap(tokens::SPACE_2)
            .padding(tokens::SPACE_3)
            .radius(8.0)
            .fill(tokens::BACKGROUND.mix(tokens::CARD, 0.6))
            .stroke(tokens::INFO.mix(tokens::BORDER, 0.7))
            .align(Align::End),
    )
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
    if let Some(summary) = doc_summary(&f.doc, 56) {
        parts.push(summary);
    }
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
    } else if f.is_orphan() {
        let (fill, stroke) = orphan_colors();
        card.fill(fill).stroke(stroke)
    } else {
        card.fill(tokens::CARD).stroke(tokens::BORDER)
    }
}

/// The triage lens carried over from the old Methods overlay: a fn nothing
/// calls (and no proof reasons about) is a cut candidate, flagged red at any
/// zoom — on the full card and on the distant slab alike. Fill/stroke only:
/// cards are committed-measured, so the lens must never touch geometry.
fn orphan_colors() -> (Color, Color) {
    (tokens::CARD.mix(tokens::DESTRUCTIVE, 0.30), tokens::DESTRUCTIVE.mix(tokens::BORDER, 0.35))
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
            ends: Vec::new(),
            members: Vec::new(),
        }
    }

    #[test]
    fn region_pass_resolves_absolute_rects_and_region_rect_reads_them() {
        // Root level: subdir "a" box then file 3's box. Inside a/: file 7.
        let mut root = DirNode::default();
        root.insert(&["a"], 7);
        root.insert(&[], 3);
        // File 7 holds fn 9's card at slot 0 (for the MapTarget::Fn check).
        let mut fn_level = level(vec![placed(2.0, 3.0, 20.0, 10.0)], (4.0, 10.0));
        fn_level.members = vec![Member::Fn(9)];
        let mut com = Committed {
            scope: Scope::Project,
            files: BTreeMap::from([(7, fn_level)]),
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
        // A fn's card: its file's rect + content offset + its slot position.
        let card = region_rect(&cache, &Scope::Project, MapTarget::Fn(9));
        assert_eq!(card, Some(Rect::new(57.0, 93.0, 20.0, 10.0)));
        // Misses: a dir not on the plane; a fn on no committed level; a plane
        // not in the cache.
        assert_eq!(region_rect(&cache, &Scope::Project, MapTarget::Dir("b")), None);
        assert_eq!(region_rect(&cache, &Scope::Project, MapTarget::Fn(4)), None);
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
