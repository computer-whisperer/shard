//! Commit pass: scope → geometry. No zoom, no selection, no screen anywhere.
//!
//! Lays the whole scope out at full flow-card footprints ([`commit`]) —
//! every fn measured as its flow card, every file's members placed by the
//! classed intra-file graph, every dir's children by the import DAG — and
//! resolves each box's absolute rect ([`region_pass`]). Pure in
//! `(project, scope)`: this is what makes the topology committable.

use super::cards::{claim_card, filedoc_card, flow_card, type_card};
use super::{Committed, DirNode, LevelGeom, Member};
use crate::layout::{self, EndPoint, GEdge, GNode, Graph};
use crate::model::Project;
use crate::scope::Scope;
use crate::view::shared::EdgeClass;
use crate::view::SUB_SIZE;
use damascene_core::layout::intrinsic;
use damascene_core::prelude::*;
use std::collections::BTreeMap;

/// Lay the whole scope out at full (flow-card) footprints and keep every
/// level's geometry. Pure in `(project, scope)` — this is what makes the
/// topology committable: nothing the user does while navigating is an input.
pub(super) fn commit(
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

#[cfg(test)]
mod tests {
    use super::super::{region_rect, MapCache, MapTarget};
    use super::*;
    use crate::layout::Layout;
    use crate::scope::Scope;

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
}
