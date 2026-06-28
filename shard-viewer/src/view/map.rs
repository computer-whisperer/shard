//! Map view (experimental): the unified canvas — any [`Scope`](crate::scope::Scope)'s
//! fns, grouped by origin **dir ⊃ file** into bounding boxes, each fn drawn in
//! the expanded flow form ([`super::flow::render_region`]).
//!
//! ## Layout = recursive containment, intrinsic sizing
//! The layout is a recursive pass mirroring the program's own containment tree:
//! a dir box stacks its subdir + file boxes; a file box packs its fn cards; a
//! card is a fn body's region tree. Every box **hugs its contents**, so sizing
//! falls out of damascene's element layout — there is no size *estimation* here
//! (contrast `board.rs::est`, which the call-DAG board still needs because its
//! Sugiyama engine wants sizes up front). The only measurement is the greedy
//! row-packer ([`wrap_pack`]), which reads each card's *real* intrinsic width to
//! flow cards into rows instead of one tall column.
//!
//! The intra-level arrangement is deliberately the simplest router (stack dirs/
//! files, wrap fns); swapping the module level for an import-DAG layout and
//! tracing call wires on top are the next slices. This file is the seam.

use super::flow::render_region;
use super::shared::pan_zoom_viewport;
use super::SUB_SIZE;
use crate::flow::Region;
use crate::model::{FnDef, Project};
use crate::view::ViewParams;
use damascene_core::layout::intrinsic;
use damascene_core::prelude::*;
use std::collections::BTreeMap;

/// Width budget a file box wraps its fn cards within (logical px, pre-zoom).
const ROW_BUDGET: f32 = 1180.0;
/// Gap between packed cards / stacked boxes.
const GAP: f32 = 10.0;

pub(crate) fn legend() -> El {
    row([
        text("map").mono().muted().font_size(SUB_SIZE),
        text("grouped by dir / file · each fn in flow form · drag to pan, wheel to zoom")
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

    // Build the dir tree over the spanned files, then render it recursively.
    let mut root = DirNode::default();
    for &file in by_file.keys() {
        root.insert(&dir_segments(&project.files[file].rel), file);
    }
    let content = render_dir(project, &root, p.selected_fn, &by_file);

    pan_zoom_viewport(row([content]).padding(tokens::SPACE_6))
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
}

/// The directory segments of a file's `rel` path (e.g. `examples/foo/a.shard`
/// → `["examples", "foo"]`; a root-level file → `[]`).
fn dir_segments(rel: &str) -> Vec<&str> {
    let mut segs: Vec<&str> = rel.split('/').collect();
    segs.pop(); // drop the file name
    segs
}

/// Render a dir node: its subdir boxes and file boxes, stacked. The synthetic
/// root (the whole scope) renders bare — no enclosing box, just the children —
/// so a single-file scope isn't wrapped in a redundant frame.
fn render_dir(
    project: &Project,
    node: &DirNode,
    selected_fn: Option<usize>,
    by_file: &BTreeMap<usize, Vec<usize>>,
) -> El {
    let mut kids: Vec<El> = Vec::new();
    for (name, sub) in &node.subdirs {
        kids.push(dir_box(name, render_dir(project, sub, selected_fn, by_file)));
    }
    for &file in &node.files {
        kids.push(render_file(project, file, &by_file[&file], selected_fn));
    }
    column(kids).gap(GAP).align(Align::Start)
}

/// A directory bounding box: a folder label band over its stacked children.
fn dir_box(name: &str, children: El) -> El {
    let band = text(format!("▸ {name}/"))
        .mono()
        .semibold()
        .font_size(11.0)
        .muted()
        .nowrap_text();
    column([band, children])
        .gap(GAP)
        .padding(tokens::SPACE_3)
        .fill(tokens::BACKGROUND)
        .stroke(tokens::MUTED)
        .radius(9.0)
        .align(Align::Start)
}

/// A file bounding box: a file label over its fn cards, wrapped into rows.
fn render_file(
    project: &Project,
    file: usize,
    fns: &[usize],
    selected_fn: Option<usize>,
) -> El {
    let f = &project.files[file];
    let base = f.rel.rsplit_once('/').map(|(_, b)| b).unwrap_or(&f.rel);
    let header = row([
        text(base.to_string()).mono().semibold().font_size(12.0).nowrap_text(),
        text(format!("{} fns", fns.len())).mono().muted().font_size(SUB_SIZE).nowrap_text(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);

    let cards: Vec<El> = fns.iter().map(|&fi| fn_card(project, fi, selected_fn)).collect();
    column([header, wrap_pack(cards, ROW_BUDGET)])
        .gap(GAP)
        .padding(tokens::SPACE_3)
        .fill(tokens::CARD.mix(tokens::BACKGROUND, 0.4))
        .stroke(tokens::BORDER)
        .radius(8.0)
        .align(Align::Start)
}

/// One fn as an intrinsic flow card: a name/signature header over its region
/// tree (the same renderer the Flow/Board views use). No fixed size — it hugs.
fn fn_card(project: &Project, fn_idx: usize, selected_fn: Option<usize>) -> El {
    let f = &project.fns[fn_idx];
    let title = row([
        text(f.name.clone())
            .mono()
            .semibold()
            .font_size(super::TITLE_SIZE)
            .nowrap_text()
            .ellipsis(),
        text(format!("{} → {}", f.params.len(), short_ty(&f.ret)))
            .mono()
            .muted()
            .font_size(SUB_SIZE)
            .nowrap_text(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);

    let body = match body_region(f) {
        Some(region) => render_region(&region),
        None => text("(signature only)").muted().font_size(SUB_SIZE),
    };

    let card = column([title, body])
        .gap(tokens::SPACE_2)
        .padding(8.0)
        .radius(7.0)
        .key(format!("fn:{fn_idx}"))
        .tooltip(super::methods::node_tip(project, fn_idx));
    if selected_fn == Some(fn_idx) {
        card.fill(tokens::CARD.mix(tokens::ACCENT, 0.18)).stroke(tokens::RING)
    } else {
        card.fill(tokens::CARD).stroke(tokens::BORDER)
    }
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

/// Greedy row-packer: flow `cards` into rows no wider than `budget`, measuring
/// each card's *real* intrinsic width. Rows top-align (cards vary in height).
fn wrap_pack(cards: Vec<El>, budget: f32) -> El {
    let mut rows: Vec<El> = Vec::new();
    let mut cur: Vec<El> = Vec::new();
    let mut width = 0.0_f32;
    for card in cards {
        let cw = intrinsic(&card).0;
        if !cur.is_empty() && width + cw > budget {
            rows.push(row(std::mem::take(&mut cur)).gap(GAP).align(Align::Start));
            width = 0.0;
        }
        width += cw + GAP;
        cur.push(card);
    }
    if !cur.is_empty() {
        rows.push(row(cur).gap(GAP).align(Align::Start));
    }
    column(rows).gap(GAP).align(Align::Start)
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
