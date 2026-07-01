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

use super::flow::render_region;
use super::shared::{edges_asset, pan_zoom_viewport, placed_graph};
use super::SUB_SIZE;
use crate::flow::Region;
use crate::layout::{self, EndPoint, GEdge, GNode, Graph};
use crate::model::{FnDef, Project};
use crate::view::ViewParams;
use damascene_core::layout::intrinsic;
use damascene_core::prelude::*;
use std::collections::BTreeMap;

pub(crate) fn legend() -> El {
    row([
        text("map").mono().muted().font_size(SUB_SIZE),
        text("dir/file boxes placed by imports · fns by calls · drag to pan, wheel to zoom")
            .muted()
            .font_size(SUB_SIZE),
    ])
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_2)
}

pub(crate) fn canvas(project: &Project, p: &ViewParams) -> El {
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
    let ctx = Ctx { project, by_file: &by_file, selected_fn: p.selected_fn };
    let placed = layout_children(&ctx, &root);

    pan_zoom_viewport(row([placed.el]).padding(tokens::SPACE_6))
}

/// Shared inputs threaded through the recursion (project data + the in-scope fn
/// sets + the focus cursor), so the layout fns take one context not five args.
struct Ctx<'a> {
    project: &'a Project,
    by_file: &'a BTreeMap<usize, Vec<usize>>,
    selected_fn: Option<usize>,
}

/// A laid-out, measured sub-tree: its element plus the intrinsic size the parent
/// graph needs to place it.
struct Block {
    el: El,
    w: f32,
    h: f32,
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

    let inner = place_with_els(Graph { nodes, edges }, cards);
    let f = &ctx.project.files[file];
    let base = f.rel.rsplit_once('/').map(|(_, b)| b).unwrap_or(&f.rel);
    let header = row([
        text(base.to_string()).mono().semibold().font_size(12.0).nowrap_text(),
        text(format!("{} fns", members.len()))
            .mono()
            .muted()
            .font_size(SUB_SIZE)
            .nowrap_text(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);

    boxed(
        column([header, inner.el]),
        tokens::CARD.mix(tokens::BACKGROUND, 0.4),
        tokens::BORDER,
    )
}

/// Wrap a laid-out dir's content in a folder-labelled bounding box.
fn dir_box(name: &str, inner: Block) -> Block {
    let band = text(format!("▸ {name}/"))
        .mono()
        .semibold()
        .font_size(11.0)
        .muted()
        .nowrap_text();
    boxed(column([band, inner.el]), tokens::BACKGROUND, tokens::MUTED)
}

/// Common box chrome (padding/fill/stroke/radius) + the bottom-up measure: pin
/// the result to its intrinsic size so the parent graph places it exactly.
fn boxed(body: El, fill: Color, stroke: Color) -> Block {
    let el = body
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .fill(fill)
        .stroke(stroke)
        .radius(8.0)
        .align(Align::Start);
    let (w, h) = intrinsic(&el);
    Block { el: el.width(Size::Fixed(w)).height(Size::Fixed(h)), w, h }
}

/// Place pre-measured child blocks by `edges` and return the placed content as a
/// measured block (no box) — used at the dir level.
fn place(blocks: Vec<Block>, edges: Vec<GEdge>) -> Block {
    let nodes: Vec<GNode> = blocks.iter().map(|b| GNode::simple(b.w, b.h)).collect();
    let els: Vec<El> = blocks.into_iter().map(|b| b.el).collect();
    place_with_els(Graph { nodes, edges }, els)
}

/// Run the engine over `graph` (node sizes already set) and place `els` at the
/// computed coordinates, returning the placed content sized to the layout.
fn place_with_els(graph: Graph, els: Vec<El>) -> Block {
    let lay = layout::layout(&graph, &layout::LayoutConfig::default());
    let el = placed_graph(&lay, els, edges_asset(&lay));
    Block { el, w: lay.width, h: lay.height }
}

/// One fn as an intrinsic flow card: a name/signature header, its named
/// arguments (LabVIEW-style inputs), then its region tree (the same renderer
/// the Flow/Board views use). No fixed size — it hugs.
fn fn_card(ctx: &Ctx, fn_idx: usize) -> El {
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
