//! The damascene view: pure functions from project state to an `El` tree.
//!
//! Kept separate from the `App` impl (in the `shard-viewer` bin) so the same
//! tree can be rendered headlessly — to SVG + a lint report — without a GPU or
//! a window. That headless render is the build-time review loop.

use crate::flow::{Branch, FrameKind, Region};
use crate::layout::{self, EndPoint, GEdge, GNode, Graph, Layout};
use crate::model::Project;
use damascene_core::prelude::*;
use std::collections::HashMap;

/// Which graph the canvas is showing.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum ViewMode {
    /// One file's fns and their intra-file call edges.
    Methods,
    /// The project-wide file import dependency graph.
    Systems,
    /// One fn's body as a dataflow / decision-tree diagram.
    Flow,
}

/// Everything the view needs from the running app, snapshotted per frame.
pub struct ViewParams {
    pub mode: ViewMode,
    pub selected_file: Option<usize>,
    pub selected_fn: Option<usize>,
    /// Current viewport zoom (read back from the runtime), for display only.
    pub zoom: f32,
}

/// Key of the pan/zoom viewport — also the target of `ViewportRequest`s.
pub const CANVAS_KEY: &str = "canvas";

// Low enough that even the densest file (driver.shard ~ 4000×6000 content)
// fits the frame on `Fit`; FitContent never zooms below the true fit, so this
// only governs how far the user may manually zoom out.
const MIN_ZOOM: f32 = 0.04;
const MAX_ZOOM: f32 = 3.0;

const TITLE_SIZE: f32 = 13.0;
const SUB_SIZE: f32 = 11.0;

/// The whole window: sidebar + main pane + (when a fn is selected) detail panel.
pub fn app_root(project: &Project, p: &ViewParams) -> El {
    let mut panes = vec![
        sidebar(project, p.selected_file),
        main_pane(project, p),
    ];
    match p.mode {
        ViewMode::Methods | ViewMode::Flow => {
            if let Some(fni) = p.selected_fn {
                panes.push(detail_panel(project, fni));
            }
        }
        ViewMode::Systems => {
            if let Some(fi) = p.selected_file {
                panes.push(systems_detail_panel(project, fi));
            }
        }
    }
    page([row(panes).gap(tokens::SPACE_4).height(Size::Fill(1.0))])
}

fn sidebar(project: &Project, selected_file: Option<usize>) -> El {
    let rows: Vec<El> = project
        .files
        .iter()
        .enumerate()
        .map(|(i, f)| {
            let mut b = button(format!("{}  ({})", f.rel, f.fns.len()))
                .key(format!("file:{i}"))
                .ghost();
            if selected_file == Some(i) {
                b = b.selected();
            }
            b
        })
        .collect();
    column([h3("Files"), scroll(rows).height(Size::Fill(1.0))])
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .width(Size::Fixed(320.0))
        .height(Size::Fill(1.0))
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .radius(10.0)
}

fn main_pane(project: &Project, p: &ViewParams) -> El {
    let body = match p.mode {
        ViewMode::Systems => systems_canvas(project, p),
        ViewMode::Methods => match p.selected_file {
            None => column([text("Select a file to see its call graph.").muted()])
                .padding(tokens::SPACE_8),
            Some(fi) => methods_canvas(project, fi, p),
        },
        ViewMode::Flow => match p.selected_fn {
            None => column([text("Select a fn (in Methods) to chart its body.").muted()])
                .padding(tokens::SPACE_8),
            Some(fni) => flow_canvas(project, fni),
        },
    };
    let mut head = vec![toolbar(project, p)];
    match p.mode {
        ViewMode::Methods if p.selected_file.is_some() => head.push(triage_legend()),
        ViewMode::Systems => head.push(heat_legend()),
        ViewMode::Flow if p.selected_fn.is_some() => head.push(flow_legend()),
        _ => {}
    }
    head.push(body);
    column(head)
        .gap(tokens::SPACE_3)
        .width(Size::Fill(1.0))
        .height(Size::Fill(1.0))
}

/// A small colored square (an empty box used purely as a swatch).
fn swatch(color: Color, side: f32) -> El {
    column(Vec::<El>::new())
        .width(Size::Fixed(side))
        .height(Size::Fixed(side))
        .radius(4.0)
        .fill(color)
        .stroke(tokens::BORDER)
}

/// A small colored chip + label, for a legend.
fn legend_chip(color: Color, label: &str) -> El {
    row([swatch(color, 14.0), text(label).muted().font_size(SUB_SIZE)]).gap(tokens::SPACE_2)
}

/// Heat tint for a file node: cool (ACCENT) for implementation-heavy files,
/// warm (WARNING) for proof-heavy ones, blended over CARD so labels stay
/// readable. Files with no substantive code read neutral (plain CARD).
fn heat_fill(share: Option<f32>) -> Color {
    match share {
        None => tokens::CARD,
        Some(s) => tokens::CARD.mix(tokens::ACCENT.mix(tokens::WARNING, s), 0.4),
    }
}

/// A thin stacked bar showing a file's line composition: implementation, then
/// proof burden, over a track that shows the comment/blank remainder. `inner_w`
/// is the available width inside the node's padding.
fn composition_bar(c: &crate::model::Counts, inner_w: f32) -> El {
    let total = c.total().max(1) as f32;
    let seg = |n: u32, color: Color| -> Option<El> {
        let w = (n as f32 / total) * inner_w;
        (w >= 0.5).then(|| {
            column(Vec::<El>::new())
                .width(Size::Fixed(w))
                .height(Size::Fixed(6.0))
                .fill(color)
        })
    };
    let mut segs = Vec::new();
    segs.extend(seg(c.impl_lines(), tokens::ACCENT));
    segs.extend(seg(c.proof_lines(), tokens::WARNING));
    row(segs)
        .gap(0.0)
        .width(Size::Fixed(inner_w))
        .height(Size::Fixed(6.0))
        .radius(3.0)
        .fill(tokens::BORDER) // the uncovered track = comment/blank remainder
}

/// The triage-overlay key (Methods view): how node color and size encode the
/// dead-code / complexity signal.
fn triage_legend() -> El {
    row([
        text("triage").mono().muted().font_size(SUB_SIZE),
        legend_chip(tokens::CARD.mix(tokens::DESTRUCTIVE, 0.5), "orphan — cut candidate"),
        legend_chip(tokens::CARD.mix(tokens::WARNING, 0.6), "hub — many callers"),
        legend_chip(tokens::CARD, "leaf"),
        legend_chip(tokens::MUTED, "sig"),
        text("· taller = more source lines").muted().font_size(SUB_SIZE),
    ])
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_2)
}

/// The Systems-view heat key: node tint encodes proof-vs-impl share, and each
/// node carries a stacked composition bar.
fn heat_legend() -> El {
    let warm = tokens::ACCENT.mix(tokens::WARNING, 1.0);
    let cool = tokens::ACCENT.mix(tokens::WARNING, 0.0);
    row([
        text("heat").mono().muted().font_size(SUB_SIZE),
        legend_chip(tokens::CARD.mix(cool, 0.4), "impl-heavy"),
        legend_chip(tokens::CARD.mix(tokens::ACCENT.mix(tokens::WARNING, 0.5), 0.4), "mixed"),
        legend_chip(tokens::CARD.mix(warm, 0.4), "proof-heavy"),
        text("·  bar").mono().muted().font_size(SUB_SIZE),
        legend_chip(tokens::ACCENT, "impl"),
        legend_chip(tokens::WARNING, "proof"),
        legend_chip(tokens::BORDER, "comment/blank"),
    ])
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_2)
}

fn toolbar(project: &Project, p: &ViewParams) -> El {
    let title = match p.mode {
        ViewMode::Systems => format!("Systems · {} files", project.files.len()),
        ViewMode::Methods => match p.selected_file {
            Some(fi) => project.files[fi].rel.clone(),
            None => "shard-viewer".to_string(),
        },
        ViewMode::Flow => match p.selected_fn {
            Some(fni) => format!("{}  ·  flow", project.fns[fni].name),
            None => "Flow".to_string(),
        },
    };
    let mode_btn = |label: &str, key: &str, active: bool| {
        let b = button(label).key(key.to_string());
        if active { b.selected() } else { b.ghost() }
    };
    row([
        h3(title),
        spacer(),
        mode_btn("Methods", "mode_methods", p.mode == ViewMode::Methods),
        mode_btn("Systems", "mode_systems", p.mode == ViewMode::Systems),
        mode_btn("Flow", "mode_flow", p.mode == ViewMode::Flow),
        text(format!("{:.0}%", p.zoom * 100.0))
            .mono()
            .muted()
            .center_text()
            .width(Size::Fixed(52.0)),
        button("Fit").key("fit").secondary(),
        button("Reset view").key("reset").ghost(),
    ])
    .gap(tokens::SPACE_2)
    .padding(tokens::SPACE_2)
}

fn methods_canvas(project: &Project, file_idx: usize, p: &ViewParams) -> El {
    let (graph, fn_of) = build_call_graph(project, file_idx);
    if graph.nodes.is_empty() {
        return column([text("This file defines no fns.").muted()]).padding(tokens::SPACE_8);
    }
    let lay = layout::layout(&graph, &layout::LayoutConfig::default());
    let node_els: Vec<El> = lay
        .nodes
        .iter()
        .enumerate()
        .map(|(i, pn)| node_box(project, fn_of[i], p.selected_fn, pn.w, pn.h))
        .collect();
    graph_canvas(&lay, node_els)
}

fn systems_canvas(project: &Project, p: &ViewParams) -> El {
    let (graph, file_of) = build_systems_graph(project);
    if graph.nodes.is_empty() {
        return column([text("No in-project imports to graph.").muted()]).padding(tokens::SPACE_8);
    }
    let lay = layout::layout(&graph, &layout::LayoutConfig::default());
    let node_els: Vec<El> = lay
        .nodes
        .iter()
        .enumerate()
        .map(|(i, pn)| sys_node_box(project, file_of[i], p.selected_file, pn.w, pn.h))
        .collect();
    graph_canvas(&lay, node_els)
}

/// Wrap a laid-out graph in the pan/zoom viewport: an edge overlay plus the
/// per-node elements (index-aligned with `lay.nodes`), placed at their content
/// coordinates via the `El::layout` escape hatch.
fn graph_canvas(lay: &Layout, node_els: Vec<El>) -> El {
    graph_canvas_edges(lay, node_els, edges_asset(lay))
}

/// Like [`graph_canvas`] but with a caller-supplied edge overlay, so views that
/// style edges by kind (e.g. flow's control vs data) can build their own.
fn graph_canvas_edges(lay: &Layout, node_els: Vec<El>, edges: VectorAsset) -> El {
    let mut children: Vec<El> = Vec::with_capacity(node_els.len() + 1);
    // Edge overlay, drawn in content coordinates; the viewport transform scales
    // it for free. Unkeyed so it never intercepts the background pan drag.
    children.push(vector(edges));
    children.extend(node_els);

    let positions: Vec<(f32, f32, f32, f32)> =
        lay.nodes.iter().map(|n| (n.x, n.y, n.w, n.h)).collect();
    let (cw, ch) = (lay.width, lay.height);

    // The content layer: nodes placed at their absolute graph coordinates. No
    // pan/zoom math here — the `viewport()` wrapper bakes the transform into
    // descendant rects (hit-test included) and scales per-node chrome.
    let content = stack(children)
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
        });

    pan_zoom_viewport(content)
}

/// Wrap `content` in the shared pan/zoom viewport. The content sizes itself
/// (absolute-positioned graph stack, or a self-sizing nested flow tree); the
/// `viewport()` bakes the pan/zoom transform into descendant rects + hit-test.
fn pan_zoom_viewport(content: El) -> El {
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

/// Build the project-wide import dependency graph (file → files it imports).
/// Only files that import or are imported participate, so isolated files don't
/// clutter the canvas. Returns the file index behind each graph node.
fn build_systems_graph(project: &Project) -> (Graph, Vec<usize>) {
    let imported: std::collections::HashSet<usize> = project
        .files
        .iter()
        .flat_map(|f| f.import_targets.iter().copied())
        .collect();
    let participating: Vec<usize> = (0..project.files.len())
        .filter(|&i| !project.files[i].import_targets.is_empty() || imported.contains(&i))
        .collect();
    let local: HashMap<usize, usize> =
        participating.iter().enumerate().map(|(li, &fi)| (fi, li)).collect();

    let nodes: Vec<GNode> = participating
        .iter()
        .map(|&fi| {
            let (w, h) = file_node_size(project, fi);
            GNode::simple(w, h)
        })
        .collect();

    let mut edges = Vec::new();
    for (&fi, &li) in &local {
        for &target in &project.files[fi].import_targets {
            if let Some(&lj) = local.get(&target)
                && li != lj
            {
                edges.push(GEdge {
                    from: EndPoint { node: li, port: 0 },
                    to: EndPoint { node: lj, port: 0 },
                });
            }
        }
    }
    (Graph { nodes, edges }, participating)
}

fn file_node_size(project: &Project, file_idx: usize) -> (f32, f32) {
    let (stem, dir) = file_label(&project.files[file_idx].rel);
    let chars = stem.chars().count().max(dir.chars().count()) as f32;
    let w = (chars * 7.0 + 24.0).clamp(130.0, 280.0);
    (w, 58.0) // extra height for the composition bar
}

/// Split a rel path into (file stem, parent dir) for a compact node label.
fn file_label(rel: &str) -> (String, String) {
    let (dir, file) = rel.rsplit_once('/').unwrap_or(("", rel));
    let stem = file.strip_suffix(".shard").unwrap_or(file);
    (stem.to_string(), dir.to_string())
}

fn sys_node_box(project: &Project, file_idx: usize, selected_file: Option<usize>, w: f32, h: f32) -> El {
    let f = &project.files[file_idx];
    let (stem, dir) = file_label(&f.rel);
    let selected = selected_file == Some(file_idx);
    let sub = if dir.is_empty() {
        format!("{} fns", f.fns.len())
    } else {
        format!("{dir}  ·  {} fns", f.fns.len())
    };
    let b = column([
        text(stem).mono().font_size(TITLE_SIZE).nowrap_text().ellipsis(),
        text(sub).muted().font_size(SUB_SIZE).nowrap_text().ellipsis(),
        composition_bar(&f.counts, w - 16.0),
    ])
    .gap(3.0)
    .padding(8.0)
    .radius(8.0)
    .width(Size::Fixed(w))
    .height(Size::Fixed(h))
    .key(format!("sysfile:{file_idx}"));
    // Tint by proof-vs-impl share so the verification-heavy corners of the tree
    // stand out at a glance; selection still wins for the focused node.
    if selected {
        b.fill(tokens::ACCENT).stroke(tokens::RING)
    } else {
        b.fill(heat_fill(f.counts.proof_share())).stroke(tokens::BORDER)
    }
}

/// Build the engine `Graph` for a file's intra-file call graph, plus the fn
/// index behind each graph node (index-aligned with the layout result).
fn build_call_graph(project: &Project, file_idx: usize) -> (Graph, Vec<usize>) {
    let fn_of: Vec<usize> = project.files[file_idx].fns.clone();
    let local: HashMap<usize, usize> = fn_of.iter().enumerate().map(|(i, &g)| (g, i)).collect();

    let nodes: Vec<GNode> = fn_of
        .iter()
        .map(|&g| {
            let (w, h) = node_size(project, g);
            GNode::simple(w, h)
        })
        .collect();

    let mut seen = std::collections::HashSet::new();
    let mut edges = Vec::new();
    for (i, &g) in fn_of.iter().enumerate() {
        for &callee in &project.fns[g].calls {
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
    (Graph { nodes, edges }, fn_of)
}

/// Intrinsic node size: width tracks the longer of the two label lines so the
/// engine can pack columns tightly. Height grows with the fn's source-line
/// count (a cheap complexity proxy) so large fns read as visually massive —
/// the "where's the weight" half of the triage overlay.
fn node_size(project: &Project, fn_idx: usize) -> (f32, f32) {
    let f = &project.fns[fn_idx];
    let title_len = f.name.chars().count() + if f.is_sig { 6 } else { 0 };
    let sub_len = format!("{} args → {}", f.params.len(), short_ty(&f.ret))
        .chars()
        .count();
    let chars = title_len.max(sub_len) as f32;
    let w = (chars * 7.5 + 24.0).clamp(140.0, 300.0);
    let h = (40.0 + f.src_lines() as f32 * 1.4).clamp(46.0, 130.0);
    (w, h)
}

fn node_box(project: &Project, fn_idx: usize, selected_fn: Option<usize>, w: f32, h: f32) -> El {
    let f = &project.fns[fn_idx];
    let selected = selected_fn == Some(fn_idx);
    let title = if f.is_sig {
        format!("{}  (sig)", f.name)
    } else {
        f.name.clone()
    };
    let sub = format!("{} args → {}", f.params.len(), short_ty(&f.ret));
    let b = column([
        text(title).mono().font_size(TITLE_SIZE).nowrap_text().ellipsis(),
        text(sub).muted().font_size(SUB_SIZE).nowrap_text().ellipsis(),
    ])
    .gap(2.0)
    .padding(8.0)
    .radius(8.0)
    .width(Size::Fixed(w))
    .height(Size::Fixed(h))
    // Keyed (so clicks route and pan-drag skips them) but NOT focusable: the
    // auto hover/press envelope on focusable nodes flashes the fill as the
    // cursor sweeps across the dense graph. Selection highlight (below) is the
    // only per-node visual state we want.
    .key(format!("fn:{fn_idx}"));
    // Triage overlay (when not selected): orphans flag as cut candidates;
    // everything else warms with connectivity so hubs stand out from leaves.
    if selected {
        b.fill(tokens::ACCENT).stroke(tokens::RING)
    } else if f.is_orphan() {
        b.fill(tokens::CARD.mix(tokens::DESTRUCTIVE, 0.5))
            .stroke(tokens::DESTRUCTIVE)
    } else if f.is_sig {
        b.fill(tokens::MUTED).stroke(tokens::BORDER)
    } else {
        let warmth = (f.degree() as f32 / 14.0).min(1.0);
        b.fill(tokens::CARD.mix(tokens::WARNING, warmth * 0.6))
            .stroke(tokens::BORDER)
    }
}

// ---------------------------------------------------------------------------
// Flow view: one fn body as a structured (LabVIEW-style) diagram. Control
// structures are FRAMES that contain their branches; leaf computations are op
// cards WIRED to their operands. Rendered as nested elements — containment,
// sizing, and text wrapping fall out of the element layout — inside the
// pan/zoom viewport. (The Sugiyama engine stays for the call/import graphs.)
// ---------------------------------------------------------------------------

/// The Flow-view key.
fn flow_legend() -> El {
    row([
        text("flow").mono().muted().font_size(SUB_SIZE),
        legend_chip(tokens::INFO, "match / if"),
        legend_chip(tokens::SUCCESS, "let"),
        legend_chip(tokens::CARD, "op"),
        legend_chip(tokens::CARD.mix(tokens::WARNING, 0.32), "var"),
        legend_chip(tokens::BACKGROUND, "literal"),
        text("·  frames contain · operands wired in →")
            .mono()
            .muted()
            .font_size(SUB_SIZE),
    ])
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_2)
}

fn flow_canvas(project: &Project, fn_idx: usize) -> El {
    let f = &project.fns[fn_idx];
    if f.body.iter().all(|form| form.head() == Some("measure")) {
        return column([text("This fn has no body to chart (a signature).").muted()])
            .padding(tokens::SPACE_8);
    }
    let region = Region::build(&f.body);
    // The nested-element tree hugs its own content; a little padding keeps it
    // off the viewport edges. The viewport frames it (Fit / pan / zoom).
    let content = row([render_region(&region)]).padding(tokens::SPACE_6);
    pan_zoom_viewport(content)
}

/// Render a region as a nested element. Frames contain labelled branches; op
/// cards wire to their operand sub-regions; vars/lits are leaf pills/tags.
fn render_region(r: &Region) -> El {
    match r {
        Region::Frame { kind, detail, branches } => render_frame(*kind, detail, branches),
        Region::Op { head, inline, args } => render_op(head, inline, args),
        Region::Var(name) => var_pill(name),
        Region::Lit(value) => lit_tag(value),
    }
}

/// A control structure: a colored keyword band over a body that *contains* its
/// branches, each headed by its selector chip. Nesting = box enclosure.
fn render_frame(kind: FrameKind, detail: &str, branches: &[Branch]) -> El {
    let (accent, fg) = match kind {
        FrameKind::Match | FrameKind::If => (tokens::INFO, tokens::INFO_FOREGROUND),
        FrameKind::Let => (tokens::SUCCESS, tokens::SUCCESS_FOREGROUND),
    };
    let mut band_kids = vec![
        text(kind.keyword().to_string())
            .mono()
            .semibold()
            .font_size(12.0)
            .text_color(fg)
            .nowrap_text(),
    ];
    if !detail.is_empty() {
        band_kids.push(
            text(detail.to_string())
                .mono()
                .font_size(11.0)
                .text_color(fg)
                .nowrap_text()
                .ellipsis(),
        );
    }
    let band = row(band_kids)
        .gap(tokens::SPACE_2)
        .padding(5.0)
        .width(Size::Fill(1.0))
        .fill(accent);

    let body = column(branches.iter().map(render_branch).collect::<Vec<_>>())
        .gap(7.0)
        .padding(8.0);

    column([band, body])
        .fill(tokens::CARD)
        .stroke(accent)
        .radius(7.0)
}

/// One labelled branch inside a frame: its selector chip + the contained region.
fn render_branch(b: &Branch) -> El {
    // Top-align: the chip sits beside its region's header, not floating at the
    // vertical centre of a tall nested frame.
    row([selector_chip(&b.label), render_region(&b.region)])
        .gap(tokens::SPACE_2)
        .align(Align::Start)
}

/// The selector pill that heads a branch (arm pattern / `then`/`else` / binding
/// name). Blue ties it to the control vocabulary; it sits left of its region.
fn selector_chip(label: &str) -> El {
    row([text(label.to_string())
        .mono()
        .semibold()
        .font_size(10.0)
        .text_color(tokens::INFO_FOREGROUND)
        .nowrap_text()
        .ellipsis()])
    .padding(3.0)
    .radius(5.0)
    .fill(tokens::INFO)
}

/// A function application: the op card, with any compound operands wired in
/// from the left (data flows left→right into the op, LabVIEW-style).
fn render_op(head: &str, inline: &str, args: &[Region]) -> El {
    let card = op_card(head, inline);
    if args.is_empty() {
        return card;
    }
    let arg_rows: Vec<El> = args
        .iter()
        .map(|a| row([render_region(a), wire_stub()]).align(Align::Center))
        .collect();
    row([column(arg_rows).gap(6.0), card]).align(Align::Center)
}

/// The op card itself: the function name as a bold hero, inline simple operands
/// a quiet second line.
fn op_card(head: &str, inline: &str) -> El {
    let mut kids = vec![
        text(head.to_string())
            .mono()
            .semibold()
            .font_size(13.0)
            .text_color(tokens::FOREGROUND)
            .nowrap_text()
            .ellipsis(),
    ];
    if !inline.is_empty() {
        kids.push(
            text(inline.to_string())
                .mono()
                .muted()
                .font_size(11.0)
                .nowrap_text()
                .ellipsis(),
        );
    }
    column(kids)
        .gap(1.0)
        .padding(6.0)
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .radius(6.0)
}

/// A variable reference: a small warm pill (a data input).
fn var_pill(name: &str) -> El {
    column([text(name.to_string())
        .mono()
        .semibold()
        .font_size(12.0)
        .text_color(tokens::FOREGROUND)
        .center_text()
        .nowrap_text()])
    .padding(5.0)
    .fill(tokens::CARD.mix(tokens::WARNING, 0.32))
    .stroke(tokens::WARNING)
    .radius(13.0)
}

/// A literal: a dim mono tag (a constant).
fn lit_tag(value: &str) -> El {
    let show = if value.is_empty() { "·".to_string() } else { value.to_string() };
    column([text(show)
        .mono()
        .muted()
        .font_size(11.0)
        .center_text()
        .nowrap_text()])
    .padding(4.0)
    .fill(tokens::BACKGROUND)
    .stroke(tokens::BORDER)
    .radius(4.0)
}

/// A short right-pointing wire stub: an operand feeding into an op card.
fn wire_stub() -> El {
    let line = PathBuilder::new()
        .move_to(0.0, 6.0)
        .line_to(15.0, 6.0)
        .stroke_solid(tokens::INFO, 1.6)
        .build();
    let head = arrowhead(8.0, 6.0, 21.0, 6.0, tokens::INFO);
    vector(VectorAsset::from_paths([0.0, 0.0, 22.0, 12.0], vec![line, head]))
        .width(Size::Fixed(22.0))
        .height(Size::Fixed(12.0))
}

fn detail_panel(project: &Project, fn_idx: usize) -> El {
    let f = &project.fns[fn_idx];
    let sig: Vec<String> = f.params.iter().map(|(n, t)| format!("({n} {t})")).collect();

    // Callees (within project) and callers (reverse edges, precomputed).
    let callees = &f.calls;
    let callers = &f.callers;

    // Triage metrics + a cut-candidate / proof-subject tag.
    let mut metrics = format!(
        "{} lines · {} calls · {} callers",
        f.src_lines(),
        f.calls.len(),
        f.callers.len()
    );
    if f.is_orphan() {
        metrics.push_str("  ·  ⚠ orphan — cut candidate");
    } else if f.proof_refd && f.callers.is_empty() {
        metrics.push_str("  ·  proof subject");
    }

    let mut items = vec![
        row([h3(f.name.clone()), spacer()]).gap(tokens::SPACE_2),
        text(format!("({}) → {}", sig.join(" "), f.ret))
            .mono()
            .muted()
            .font_size(tokens::TEXT_SM.size),
        text(format!("in {}", project.files[f.file].rel))
            .caption()
            .muted(),
        text(metrics).caption().muted(),
        separator(),
        text("Source").label(),
        scroll([code_block(if f.src.is_empty() {
            "(signature only)".to_string()
        } else {
            f.src.clone()
        })])
        .height(Size::Fill(1.0))
        .fill(tokens::BACKGROUND)
        .stroke(tokens::BORDER)
        .radius(8.0),
    ];

    items.push(separator());
    items.push(text(format!("Calls ({})", callees.len())).label());
    items.push(fn_link_list(project, callees));
    items.push(text(format!("Called by ({})", callers.len())).label());
    items.push(fn_link_list(project, callers));

    column(items)
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .width(Size::Fixed(420.0))
        .height(Size::Fill(1.0))
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .radius(10.0)
}

/// Systems-mode side panel: the selected file's line-category breakdown plus
/// its import in/out degree, with a button to drill into its call graph.
fn systems_detail_panel(project: &Project, file_idx: usize) -> El {
    let f = &project.files[file_idx];
    let c = &f.counts;
    let imported_by = project
        .files
        .iter()
        .filter(|g| g.import_targets.contains(&file_idx))
        .count();

    // One labelled, swatched, right-aligned count row.
    let cat_row = |label: &str, n: u32, color: Color| -> El {
        row([
            swatch(color, 12.0),
            text(label.to_string()).font_size(SUB_SIZE),
            spacer(),
            text(n.to_string()).mono().muted().font_size(SUB_SIZE),
        ])
        .gap(tokens::SPACE_2)
    };

    let items = vec![
        row([h3(file_label(&f.rel).0), spacer()]).gap(tokens::SPACE_2),
        text(f.rel.clone()).caption().muted(),
        button("Open call graph ▸").key(format!("open:{file_idx}")).secondary(),
        separator(),
        text(format!("{} lines · {} fns", c.total(), f.fns.len()))
            .caption()
            .muted(),
        composition_bar(c, 384.0),
        separator(),
        cat_row("impl", c.impl_, tokens::ACCENT),
        cat_row("measure", c.measure, tokens::WARNING),
        cat_row("proof", c.proof, tokens::WARNING),
        cat_row("reqproof", c.reqproof, tokens::WARNING),
        cat_row("req", c.req, tokens::ACCENT),
        cat_row("sidecar", c.sidecar, tokens::WARNING),
        cat_row("comment", c.comment, tokens::BORDER),
        cat_row("blank", c.blank, tokens::BORDER),
        separator(),
        text(format!(
            "imports {} · imported by {imported_by}",
            f.import_targets.len()
        ))
        .caption()
        .muted(),
    ];

    column(items)
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .width(Size::Fixed(420.0))
        .height(Size::Fill(1.0))
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .radius(10.0)
}

/// A wrapped list of clickable fn links (jump targets for navigation).
fn fn_link_list(project: &Project, fns: &[usize]) -> El {
    if fns.is_empty() {
        return text("—").muted().font_size(tokens::TEXT_SM.size);
    }
    let chips: Vec<El> = fns
        .iter()
        .map(|&j| {
            let g = &project.fns[j];
            // Disambiguate cross-file targets with their module.
            let label = g.name.clone();
            button(label).key(format!("fn:{j}")).ghost()
        })
        .collect();
    column(chips).gap(2.0)
}

fn edges_asset(lay: &Layout) -> VectorAsset {
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
fn edge_curve(pts: &[(f32, f32)], back: bool, color: Color, width: f32) -> VectorPath {
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
fn arrowhead(from_x: f32, from_y: f32, tip_x: f32, tip_y: f32, color: Color) -> VectorPath {
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

/// Trim a string so it fits a node box / signature line.
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
