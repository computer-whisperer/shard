//! A generic layered ("Sugiyama") graph layout engine.
//!
//! This is deliberately *semantics-agnostic*: it knows nothing about shard,
//! fns, or s-exprs. Callers build a [`Graph`] of sized nodes (each with input /
//! output [ports]) and edges between ports, and get back a [`Layout`] of placed
//! rectangles, resolved port positions, and routed edge polylines. The
//! call-graph view is its first client; the planned LabVIEW-style dataflow view
//! is the second (it uses multiple ports per node and, eventually, the `sub`
//! nesting hook).
//!
//! Pipeline (left-to-right layering):
//!   1. SCC condensation + longest-path layering, then an edge-length-minimizing
//!      balancing pass (cycles share a column; sources/sinks slide toward their
//!      neighbours so long edges don't spawn needless dummy chains)
//!   2. dummy-node insertion for edges spanning more than one layer
//!   3. barycenter crossing reduction (iterated up/down sweeps)
//!   4. priority coordinate assignment (pull nodes toward neighbours, no overlap;
//!      dummies take a thin gap and win ties so long-edge chains run straight)
//!   5. port resolution + polyline edge routing, then a bounds pass that re-homes
//!      the whole drawing — edges included — into a positive box
//!
//! [ports]: GNode::n_in

pub type NodeId = usize;
/// Index of a port within a node's input or output list.
pub type PortId = usize;

/// A node to lay out. Size is intrinsic (the caller measures its content);
/// `n_in` / `n_out` are the port counts (1 each for a plain call-graph node).
#[derive(Clone, Debug)]
pub struct GNode {
    pub w: f32,
    pub h: f32,
    pub n_in: usize,
    pub n_out: usize,
    /// A nested sub-diagram laid out inside this node (LabVIEW "structures":
    /// `if` / `match` / `let`). Unused by the call-graph view; reserved so the
    /// dataflow view can plug in without an engine rewrite.
    pub sub: Option<Box<Graph>>,
}

impl GNode {
    /// A node with one input and one output port (the call-graph shape).
    pub fn simple(w: f32, h: f32) -> Self {
        GNode {
            w,
            h,
            n_in: 1,
            n_out: 1,
            sub: None,
        }
    }
}

/// One endpoint of an edge: a specific port on a specific node.
#[derive(Clone, Copy, Debug)]
pub struct EndPoint {
    pub node: NodeId,
    pub port: PortId,
}

/// A directed edge from a source out-port to a target in-port.
#[derive(Clone, Copy, Debug)]
pub struct GEdge {
    pub from: EndPoint,
    pub to: EndPoint,
}

#[derive(Clone, Debug, Default)]
pub struct Graph {
    pub nodes: Vec<GNode>,
    pub edges: Vec<GEdge>,
}

#[derive(Clone, Copy, Debug)]
pub struct LayoutConfig {
    /// Horizontal gap between layers (columns).
    pub layer_gap: f32,
    /// Minimum vertical gap between nodes in the same layer.
    pub node_gap: f32,
    pub margin: f32,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        LayoutConfig {
            layer_gap: 90.0,
            node_gap: 22.0,
            margin: 40.0,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct PlacedNode {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
    /// Resolved absolute positions of the input ports (left edge, top→bottom).
    pub in_ports: Vec<(f32, f32)>,
    /// Resolved absolute positions of the output ports (right edge).
    pub out_ports: Vec<(f32, f32)>,
}

/// A routed edge as a polyline from source out-port to target in-port, with
/// intermediate bend points where it threads the inter-column gaps.
#[derive(Clone, Debug)]
pub struct RoutedEdge {
    pub points: Vec<(f32, f32)>,
    /// True for a same-column (mutual-recursion / SCC) edge, which loops back on
    /// the right side rather than flowing left-to-right. Renderers style these
    /// distinctly so a cycle reads as a cycle, not a stray right-angle.
    pub back: bool,
}

#[derive(Clone, Debug, Default)]
pub struct Layout {
    /// Placed nodes, index-aligned with the input `Graph::nodes`.
    pub nodes: Vec<PlacedNode>,
    /// Routed edges, index-aligned with the input `Graph::edges`.
    pub edges: Vec<RoutedEdge>,
    pub width: f32,
    pub height: f32,
}

const DUMMY_H: f32 = 8.0;
/// Vertical breathing room reserved beside a routing dummy. Far smaller than
/// `node_gap`: dummies are 1px-thin waypoints, and long-edge-heavy call graphs
/// spawn more dummies than real nodes, so charging each a full node gap is what
/// inflated tall files to a ~10000px strip.
const DUMMY_GAP: f32 = 5.0;
const CROSSING_SWEEPS: usize = 8;
const COORD_SWEEPS: usize = 6;

/// Aspect the component-packing pass steers the overall drawing toward
/// (screen-shaped). Layering itself is left-to-right; a graph of many weakly
/// connected pieces would otherwise stack them all into layer 0 — one enormous
/// column whose fit zoom is uselessly small.
const PACK_ASPECT: f32 = 1.6;

/// Lay out `graph`. The result's `nodes`/`edges` are index-aligned with the
/// input so callers can map placement back to their own data.
///
/// The graph's weakly-connected components are laid out independently and
/// shelf-packed toward [`PACK_ASPECT`]; a connected graph takes the layered
/// pipeline directly.
pub fn layout(graph: &Graph, cfg: &LayoutConfig) -> Layout {
    let comps = components(graph);
    if comps.len() <= 1 {
        return layout_connected(graph, cfg);
    }

    // Lay each component out with a slim margin (the pack gap spaces them; the
    // real margin goes around the packed whole), tracking which original node /
    // edge indices each sub-layout speaks about.
    let sub_cfg = LayoutConfig { margin: cfg.node_gap * 0.5, ..*cfg };
    let mut comp_of: Vec<usize> = vec![0; graph.nodes.len()];
    let mut node_of: Vec<usize> = vec![0; graph.nodes.len()]; // orig node -> comp-local idx
    for (c, comp) in comps.iter().enumerate() {
        for (li, &g) in comp.iter().enumerate() {
            comp_of[g] = c;
            node_of[g] = li;
        }
    }
    let mut placed: Vec<(Layout, Vec<usize>, Vec<usize>)> = Vec::with_capacity(comps.len());
    for (c, comp) in comps.iter().enumerate() {
        let mut edge_ids = Vec::new();
        let mut edges = Vec::new();
        for (ei, e) in graph.edges.iter().enumerate() {
            if comp_of[e.from.node] == c {
                edge_ids.push(ei);
                edges.push(GEdge {
                    from: EndPoint { node: node_of[e.from.node], port: e.from.port },
                    to: EndPoint { node: node_of[e.to.node], port: e.to.port },
                });
            }
        }
        let nodes = comp.iter().map(|&g| graph.nodes[g].clone()).collect();
        placed.push((layout_connected(&Graph { nodes, edges }, &sub_cfg), comp.clone(), edge_ids));
    }

    // Shelf-pack the component boxes: rows filled left-to-right up to a target
    // width chosen so the packed area lands near PACK_ASPECT (never narrower
    // than the widest component). Tallest-first keeps shelves dense.
    let total_area: f32 = placed.iter().map(|(l, _, _)| l.width * l.height).sum();
    let widest = placed.iter().map(|(l, _, _)| l.width).fold(0.0, f32::max);
    let target_w = (total_area * PACK_ASPECT).sqrt().max(widest);
    let mut order: Vec<usize> = (0..placed.len()).collect();
    order.sort_by(|&a, &b| placed[b].0.height.total_cmp(&placed[a].0.height));

    let mut out = Layout {
        nodes: vec![PlacedNode::default(); graph.nodes.len()],
        edges: vec![RoutedEdge { points: Vec::new(), back: false }; graph.edges.len()],
        width: 0.0,
        height: 0.0,
    };
    let gap = cfg.node_gap;
    let (mut x, mut y, mut shelf_h) = (cfg.margin, cfg.margin, 0.0_f32);
    for &c in &order {
        let (lay, comp, edge_ids) = &placed[c];
        if x > cfg.margin && x + lay.width > cfg.margin + target_w {
            x = cfg.margin;
            y += shelf_h + gap;
            shelf_h = 0.0;
        }
        for (li, &g) in comp.iter().enumerate() {
            let mut pn = lay.nodes[li].clone();
            pn.x += x;
            pn.y += y;
            for p in pn.in_ports.iter_mut().chain(pn.out_ports.iter_mut()) {
                p.0 += x;
                p.1 += y;
            }
            out.nodes[g] = pn;
        }
        for (li, &ei) in edge_ids.iter().enumerate() {
            let mut re = lay.edges[li].clone();
            for p in &mut re.points {
                p.0 += x;
                p.1 += y;
            }
            out.edges[ei] = re;
        }
        out.width = out.width.max(x + lay.width + cfg.margin);
        out.height = out.height.max(y + lay.height + cfg.margin);
        shelf_h = shelf_h.max(lay.height);
        x += lay.width + gap;
    }
    out
}

/// The graph's weakly-connected components (each a sorted list of node ids),
/// in first-seen order.
fn components(graph: &Graph) -> Vec<Vec<usize>> {
    let n = graph.nodes.len();
    let mut adj = vec![Vec::new(); n];
    for e in &graph.edges {
        adj[e.from.node].push(e.to.node);
        adj[e.to.node].push(e.from.node);
    }
    let mut comp = vec![usize::MAX; n];
    let mut out: Vec<Vec<usize>> = Vec::new();
    for start in 0..n {
        if comp[start] != usize::MAX {
            continue;
        }
        let c = out.len();
        let mut members = vec![start];
        comp[start] = c;
        let mut stack = vec![start];
        while let Some(v) = stack.pop() {
            for &w in &adj[v] {
                if comp[w] == usize::MAX {
                    comp[w] = c;
                    members.push(w);
                    stack.push(w);
                }
            }
        }
        members.sort_unstable();
        out.push(members);
    }
    out
}

/// The layered pipeline over one weakly-connected graph (the historical
/// `layout` body; [`layout`] wraps it with component packing).
fn layout_connected(graph: &Graph, cfg: &LayoutConfig) -> Layout {
    let n = graph.nodes.len();
    if n == 0 {
        return Layout::default();
    }

    let node_layer = assign_layers(n, &node_pairs(&graph.edges));
    let nlayers = node_layer.iter().copied().max().unwrap_or(0) + 1;

    // ---- vertices: real nodes (0..n) + routing dummies (n..) ----
    let mut vlayer: Vec<usize> = node_layer.clone();
    let mut vh: Vec<f32> = graph.nodes.iter().map(|nd| nd.h).collect();
    let mut vw: Vec<f32> = graph.nodes.iter().map(|nd| nd.w).collect();

    // Per-edge routing chain of vertex ids (source .. dummies .. target), and
    // the consecutive-layer links that the ordering/coordinate phases consume.
    struct Route {
        chain: Vec<usize>,
        flat: bool,
    }
    let mut routes: Vec<Route> = Vec::with_capacity(graph.edges.len());
    let mut links: Vec<(usize, usize)> = Vec::new(); // (upper-layer vertex, lower-layer vertex)

    for e in &graph.edges {
        let (a, b) = (e.from.node, e.to.node);
        let (la, lb) = (node_layer[a], node_layer[b]);
        if la == lb {
            // Intra-SCC (same column) edge — routed directly, kept out of the
            // layered ordering machinery.
            routes.push(Route {
                chain: vec![a, b],
                flat: true,
            });
            continue;
        }
        // SCC condensation guarantees la < lb for cross-component edges, but we
        // stay direction-agnostic just in case.
        let (lo, hi) = (la.min(lb), la.max(lb));
        let mut chain = vec![a];
        let mut mids: Vec<usize> = ((lo + 1)..hi)
            .map(|l| {
                let d = vlayer.len();
                vlayer.push(l);
                vh.push(DUMMY_H);
                vw.push(0.0);
                d
            })
            .collect();
        if la > lb {
            mids.reverse();
        }
        chain.extend(mids);
        chain.push(b);
        for w in chain.windows(2) {
            let (u, v) = (w[0], w[1]);
            if vlayer[u] <= vlayer[v] {
                links.push((u, v));
            } else {
                links.push((v, u));
            }
        }
        routes.push(Route { chain, flat: false });
    }

    let nv = vlayer.len();
    let mut layers: Vec<Vec<usize>> = vec![Vec::new(); nlayers];
    for v in 0..nv {
        layers[vlayer[v]].push(v);
    }

    // Up/down adjacency between adjacent layers (for ordering + coordinates).
    let mut up = vec![Vec::new(); nv];
    let mut down = vec![Vec::new(); nv];
    for &(u, v) in &links {
        down[u].push(v);
        up[v].push(u);
    }

    reduce_crossings(&mut layers, &up, &down);

    // ---- X: one column per layer, width = widest node in the layer ----
    let mut layer_w = vec![0.0_f32; nlayers];
    for v in 0..nv {
        layer_w[vlayer[v]] = layer_w[vlayer[v]].max(vw[v]);
    }
    let mut layer_x = vec![0.0_f32; nlayers];
    let mut acc = cfg.margin;
    for l in 0..nlayers {
        layer_x[l] = acc;
        acc += layer_w[l] + cfg.layer_gap;
    }

    let vy = assign_y(&layers, &up, &down, &vh, n, cfg);

    // ---- assemble placed nodes ----
    let mut placed = vec![PlacedNode::default(); n];
    for i in 0..n {
        let (x, y, w, h) = (layer_x[node_layer[i]], vy[i], graph.nodes[i].w, graph.nodes[i].h);
        placed[i] = PlacedNode {
            x,
            y,
            w,
            h,
            in_ports: port_positions(x, y, h, graph.nodes[i].n_in),
            out_ports: port_positions(x + w, y, h, graph.nodes[i].n_out),
        };
    }

    // ---- route edges ----
    let mut edges: Vec<RoutedEdge> = routes
        .iter()
        .zip(&graph.edges)
        .map(|(r, e)| route_edge(r.flat, &r.chain, e, &placed, &layer_x, &vlayer, &vy, cfg))
        .collect();

    // Bounds must cover the routed edges too, not just the node rects: flat
    // (mutual-recursion) edges bow out past the right column, and long edges
    // thread a half-gap left of their dummy column — both land outside the node
    // box. If the bounds (and thus the edge-overlay viewBox) ignored them, those
    // curves would be clipped at a fixed content coordinate (invariant to pan /
    // zoom). Shift any negative extent back so the whole drawing sits at >= 0.
    let mut min_x = placed.iter().map(|p| p.x).fold(f32::INFINITY, f32::min);
    let mut min_y = placed.iter().map(|p| p.y).fold(f32::INFINITY, f32::min);
    let mut max_x = placed.iter().map(|p| p.x + p.w).fold(0.0, f32::max);
    let mut max_y = placed.iter().map(|p| p.y + p.h).fold(0.0, f32::max);
    for e in &edges {
        for &(x, y) in &e.points {
            min_x = min_x.min(x);
            min_y = min_y.min(y);
            max_x = max_x.max(x);
            max_y = max_y.max(y);
        }
    }
    // Re-home the drawing so its top-left padding is exactly `margin`.
    let (sx, sy) = (cfg.margin - min_x, cfg.margin - min_y);
    if sx != 0.0 || sy != 0.0 {
        for p in &mut placed {
            shift_placed(p, sx, sy);
        }
        for e in &mut edges {
            for pt in &mut e.points {
                pt.0 += sx;
                pt.1 += sy;
            }
        }
    }

    let width = (max_x + sx + cfg.margin).max(cfg.margin * 2.0);
    let height = (max_y + sy + cfg.margin).max(cfg.margin * 2.0);

    Layout {
        nodes: placed,
        edges,
        width,
        height,
    }
}

/// Translate a placed node and its resolved ports by `(sx, sy)`.
fn shift_placed(p: &mut PlacedNode, sx: f32, sy: f32) {
    p.x += sx;
    p.y += sy;
    for q in p.in_ports.iter_mut().chain(p.out_ports.iter_mut()) {
        q.0 += sx;
        q.1 += sy;
    }
}

fn port_positions(edge_x: f32, y: f32, h: f32, count: usize) -> Vec<(f32, f32)> {
    (0..count)
        .map(|k| (edge_x, y + h * (k as f32 + 1.0) / (count as f32 + 1.0)))
        .collect()
}

#[allow(clippy::too_many_arguments)]
fn route_edge(
    flat: bool,
    chain: &[usize],
    e: &GEdge,
    placed: &[PlacedNode],
    layer_x: &[f32],
    vlayer: &[usize],
    vy: &[f32],
    cfg: &LayoutConfig,
) -> RoutedEdge {
    let src = &placed[e.from.node];
    let dst = &placed[e.to.node];
    let start = *src
        .out_ports
        .get(e.from.port)
        .unwrap_or(&(src.x + src.w, src.y + src.h / 2.0));
    let end = *dst
        .in_ports
        .get(e.to.port)
        .unwrap_or(&(dst.x, dst.y + dst.h / 2.0));

    if flat {
        // Same-column edge (mutual recursion): both endpoints share a column, so
        // a left-port target would force the line back *across* the node. Loop on
        // the right instead — leave the source's right port, bow into the right
        // gap, and arrow into the target's right edge. Reads as a return arc.
        let dst_right = (dst.x + dst.w, dst.y + dst.h / 2.0);
        let bow = start.0.max(dst_right.0) + cfg.layer_gap * 0.45;
        return RoutedEdge {
            points: vec![start, (bow, start.1), (bow, dst_right.1), dst_right],
            back: true,
        };
    }

    let mut points = vec![start];
    for &d in &chain[1..chain.len() - 1] {
        // Thread the gap just left of the dummy's column.
        points.push((layer_x[vlayer[d]] - cfg.layer_gap * 0.5, vy[d] + DUMMY_H / 2.0));
    }
    points.push(end);
    RoutedEdge { points, back: false }
}

/// Reduce edge crossings by iterated barycenter ordering, keeping the best.
fn reduce_crossings(layers: &mut [Vec<usize>], up: &[Vec<usize>], down: &[Vec<usize>]) {
    let nv: usize = layers.iter().map(|l| l.len()).sum();
    let mut pos = vec![0usize; nv];
    recompute_pos(layers, &mut pos);

    let mut best = layers.to_vec();
    let mut best_cross = count_crossings(layers, down, &pos);

    for it in 0..CROSSING_SWEEPS {
        if it % 2 == 0 {
            for layer in layers[1..].iter_mut() {
                sort_by_barycenter(layer, up, &pos);
                index_layer(layer, &mut pos);
            }
        } else {
            let last = layers.len().saturating_sub(1);
            for layer in layers[..last].iter_mut().rev() {
                sort_by_barycenter(layer, down, &pos);
                index_layer(layer, &mut pos);
            }
        }
        let c = count_crossings(layers, down, &pos);
        if c < best_cross {
            best_cross = c;
            best = layers.to_vec();
        }
    }
    layers.clone_from_slice(&best);
}

fn recompute_pos(layers: &[Vec<usize>], pos: &mut [usize]) {
    for layer in layers {
        index_layer(layer, pos);
    }
}

fn index_layer(layer: &[usize], pos: &mut [usize]) {
    for (i, &v) in layer.iter().enumerate() {
        pos[v] = i;
    }
}

fn sort_by_barycenter(layer: &mut [usize], refs: &[Vec<usize>], pos: &[usize]) {
    let bary = |v: usize| -> f32 {
        let ns = &refs[v];
        if ns.is_empty() {
            pos[v] as f32
        } else {
            ns.iter().map(|&u| pos[u] as f32).sum::<f32>() / ns.len() as f32
        }
    };
    // Stable sort so vertices with no neighbours (key = current pos) stay put.
    layer.sort_by(|&a, &b| bary(a).partial_cmp(&bary(b)).unwrap_or(std::cmp::Ordering::Equal));
}

fn count_crossings(layers: &[Vec<usize>], down: &[Vec<usize>], pos: &[usize]) -> usize {
    let mut total = 0;
    for layer in layers {
        let mut segs: Vec<(usize, usize)> = Vec::new();
        for &u in layer {
            for &v in &down[u] {
                segs.push((pos[u], pos[v]));
            }
        }
        segs.sort_by_key(|&(a, _)| a);
        for i in 0..segs.len() {
            for j in (i + 1)..segs.len() {
                if segs[i].1 > segs[j].1 {
                    total += 1;
                }
            }
        }
    }
    total
}

/// Iterative coordinate assignment: pull each vertex toward the mean centre of
/// its neighbours while keeping layer order and a minimum gap (no overlap).
/// `n` is the real-node count; vertices `>= n` are routing dummies, which get a
/// much smaller vertical gap (see [`DUMMY_GAP`]) and a higher placement priority
/// so long-edge chains run straight instead of bowing.
fn assign_y(
    layers: &[Vec<usize>],
    up: &[Vec<usize>],
    down: &[Vec<usize>],
    vh: &[f32],
    n: usize,
    cfg: &LayoutConfig,
) -> Vec<f32> {
    let nv: usize = layers.iter().map(|l| l.len()).sum();
    let mut vy = vec![0.0_f32; nv];
    // Min gap *below* vertex v: thin for dummies (and below dummies), so a layer
    // packed with routing waypoints stays compact.
    let gap_below = |v: usize| if v >= n { DUMMY_GAP } else { cfg.node_gap };
    // Placement priority: dummies win (straighten long edges), then by degree.
    let prio = |v: usize| -> usize {
        if v >= n {
            usize::MAX
        } else {
            up[v].len() + down[v].len()
        }
    };

    // Initial top-down stack within each layer.
    for layer in layers {
        let mut y = cfg.margin;
        for &v in layer {
            vy[v] = y;
            y += vh[v] + gap_below(v);
        }
    }

    for it in 0..COORD_SWEEPS {
        let order: Vec<usize> = if it % 2 == 0 {
            (0..layers.len()).collect()
        } else {
            (0..layers.len()).rev().collect()
        };
        for l in order {
            place_layer_priority(&layers[l], up, down, vh, &mut vy, &gap_below, &prio);
        }
    }

    // Normalise so the topmost vertex sits at the margin.
    let min_y = vy.iter().copied().fold(f32::INFINITY, f32::min);
    if min_y.is_finite() {
        let shift = cfg.margin - min_y;
        for y in &mut vy {
            *y += shift;
        }
    }
    vy
}

/// Place one layer's vertices at their neighbour barycentres using the priority
/// method: process high-priority vertices first and let them push lower-priority
/// neighbours aside (never higher ones), keeping left-to-right order and the
/// per-vertex minimum gaps. Dummies have max priority, so long-edge chains end
/// up vertically aligned (straight) rather than bowed by averaging.
fn place_layer_priority(
    layer: &[usize],
    up: &[Vec<usize>],
    down: &[Vec<usize>],
    vh: &[f32],
    vy: &mut [f32],
    gap_below: &impl Fn(usize) -> f32,
    prio: &impl Fn(usize) -> usize,
) {
    let desired = |v: usize, vy: &[f32]| -> f32 {
        let ns = up[v].len() + down[v].len();
        if ns == 0 {
            return vy[v];
        }
        let sum: f32 = up[v]
            .iter()
            .chain(&down[v])
            .map(|&u| vy[u] + vh[u] / 2.0)
            .sum();
        sum / ns as f32 - vh[v] / 2.0
    };

    // Minimum top-of-`layer[j]` given `layer[i]` sits at y (i < j): stack the
    // intervening vertices with their gaps.
    let min_top_after = |i: usize, yi: f32| -> Vec<f32> {
        let mut tops = vec![0.0_f32; layer.len()];
        let mut run = yi;
        for j in (i + 1)..layer.len() {
            run += vh[layer[j - 1]] + gap_below(layer[j - 1]);
            tops[j] = run;
        }
        tops
    };
    // Symmetric: maximum top-of-`layer[j]` given `layer[i]` sits at y (j < i).
    let max_top_before = |i: usize, yi: f32| -> Vec<f32> {
        let mut tops = vec![0.0_f32; layer.len()];
        let mut run = yi;
        for j in (0..i).rev() {
            run -= vh[layer[j]] + gap_below(layer[j]);
            tops[j] = run;
        }
        tops
    };

    // Process indices in decreasing priority (stable on ties by position).
    let mut order: Vec<usize> = (0..layer.len()).collect();
    order.sort_by(|&a, &b| prio(layer[b]).cmp(&prio(layer[a])).then(a.cmp(&b)));

    for &i in &order {
        let v = layer[i];
        let want = desired(v, vy);
        let pv = prio(v);

        // How far down we may push: bounded by the nearest higher-priority
        // vertex below (we may shove equal/lower ones, never higher).
        let mut lo = f32::NEG_INFINITY; // upper bound from above (higher-prio)
        let mut hi = f32::INFINITY; // lower bound from below (higher-prio)
        let downs = min_top_after(i, want);
        for j in (i + 1)..layer.len() {
            if prio(layer[j]) > pv {
                hi = hi.min(vy[layer[j]] - (downs[j] - want));
                break;
            }
        }
        let ups = max_top_before(i, want);
        for j in (0..i).rev() {
            if prio(layer[j]) > pv {
                lo = lo.max(vy[layer[j]] - (ups[j] - want));
                break;
            }
        }

        // When two higher-priority vertices crowd this one from both sides the
        // band can invert (forced overlap); centre it in that case.
        let y = if lo > hi { 0.5 * (lo + hi) } else { want.clamp(lo, hi) };
        vy[v] = y;

        // Push lower/equal-priority neighbours out of the way to keep order+gap.
        let mut run = y;
        for j in (i + 1)..layer.len() {
            run += vh[layer[j - 1]] + gap_below(layer[j - 1]);
            if vy[layer[j]] < run {
                vy[layer[j]] = run;
            } else {
                break;
            }
        }
        let mut run = y;
        for j in (0..i).rev() {
            run -= vh[layer[j]] + gap_below(layer[j]);
            if vy[layer[j]] > run {
                vy[layer[j]] = run;
            } else {
                break;
            }
        }
    }
}

fn node_pairs(edges: &[GEdge]) -> Vec<(usize, usize)> {
    let mut pairs: Vec<(usize, usize)> = edges
        .iter()
        .filter(|e| e.from.node != e.to.node)
        .map(|e| (e.from.node, e.to.node))
        .collect();
    pairs.sort_unstable();
    pairs.dedup();
    pairs
}

/// Test/diagnostic hook: the per-node layer assignment for a graph.
#[doc(hidden)]
pub fn debug_layers(graph: &Graph) -> Vec<usize> {
    assign_layers(graph.nodes.len(), &node_pairs(&graph.edges))
}

/// Assign each node a column by longest path through the *condensed* graph:
/// strongly-connected components (shard's mutual-recursion SCCs) collapse to a
/// single node first, so every cycle member shares one column and the
/// longest-path relaxation converges on the acyclic condensation.
fn assign_layers(n: usize, edges: &[(usize, usize)]) -> Vec<usize> {
    let mut adj = vec![Vec::new(); n];
    for &(u, v) in edges {
        if u != v {
            adj[u].push(v);
        }
    }
    let scc = tarjan_scc(n, &adj);
    let ncomp = scc.iter().copied().max().map(|m| m + 1).unwrap_or(0);

    let mut cedges: Vec<(usize, usize)> = edges
        .iter()
        .filter(|&&(u, v)| scc[u] != scc[v])
        .map(|&(u, v)| (scc[u], scc[v]))
        .collect();
    cedges.sort_unstable();
    cedges.dedup();

    let mut clayer = vec![0usize; ncomp];
    for _ in 0..ncomp {
        let mut changed = false;
        for &(cu, cv) in &cedges {
            if clayer[cv] < clayer[cu] + 1 {
                clayer[cv] = clayer[cu] + 1;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    balance_layers(ncomp, &cedges, &mut clayer);

    (0..n).map(|i| clayer[scc[i]]).collect()
}

/// Shorten edges by pulling each component toward its neighbours within its
/// feasible band. Longest-path layering pins every source at layer 0 even when
/// its only successor is many layers right, spawning a long chain of routing
/// dummies (the sweeping-arc spaghetti). Sources have no predecessor, so we may
/// slide them right to just-left-of their earliest successor; sinks slide left;
/// interior components move to the median of their neighbours. Iterated to a
/// fixpoint-ish, then layers are repacked so none are empty.
fn balance_layers(ncomp: usize, cedges: &[(usize, usize)], clayer: &mut [usize]) {
    if ncomp == 0 {
        return;
    }
    let mut preds = vec![Vec::new(); ncomp];
    let mut succs = vec![Vec::new(); ncomp];
    for &(u, v) in cedges {
        succs[u].push(v);
        preds[v].push(u);
    }

    for _ in 0..8 {
        let mut changed = false;
        for c in 0..ncomp {
            if preds[c].is_empty() && succs[c].is_empty() {
                continue;
            }
            let low = preds[c].iter().map(|&p| clayer[p] + 1).max().unwrap_or(0);
            let high = succs[c]
                .iter()
                .map(|&s| clayer[s].saturating_sub(1))
                .min()
                .unwrap_or(usize::MAX);
            if low > high {
                continue; // band pinned by neighbours — no slack
            }
            let target = if preds[c].is_empty() {
                high // root: sit just left of the earliest successor
            } else if succs[c].is_empty() {
                low // leaf: sit just right of the latest predecessor
            } else {
                let mut ns: Vec<usize> = preds[c]
                    .iter()
                    .chain(&succs[c])
                    .map(|&x| clayer[x])
                    .collect();
                ns.sort_unstable();
                ns[ns.len() / 2].clamp(low, high) // median of neighbours
            };
            if target != clayer[c] {
                clayer[c] = target;
                changed = true;
            }
        }
        if !changed {
            break;
        }
    }

    // Repack: drop now-empty layers so columns stay contiguous.
    let mut used: Vec<usize> = clayer.to_vec();
    used.sort_unstable();
    used.dedup();
    let remap: std::collections::HashMap<usize, usize> =
        used.iter().enumerate().map(|(i, &l)| (l, i)).collect();
    for l in clayer.iter_mut() {
        *l = remap[l];
    }
}

/// Tarjan's SCC algorithm: a component id per node. The condensation (one node
/// per component) is a DAG, which makes layering well-defined on cyclic graphs.
fn tarjan_scc(n: usize, adj: &[Vec<usize>]) -> Vec<usize> {
    const UNVISITED: usize = usize::MAX;
    let mut state = Tarjan {
        adj,
        index: vec![UNVISITED; n],
        low: vec![0; n],
        on_stack: vec![false; n],
        stack: Vec::new(),
        comp: vec![UNVISITED; n],
        next_index: 0,
        next_comp: 0,
    };
    for v in 0..n {
        if state.index[v] == UNVISITED {
            state.strongconnect(v);
        }
    }
    state.comp
}

struct Tarjan<'a> {
    adj: &'a [Vec<usize>],
    index: Vec<usize>,
    low: Vec<usize>,
    on_stack: Vec<bool>,
    stack: Vec<usize>,
    comp: Vec<usize>,
    next_index: usize,
    next_comp: usize,
}

impl Tarjan<'_> {
    fn strongconnect(&mut self, v: usize) {
        self.index[v] = self.next_index;
        self.low[v] = self.next_index;
        self.next_index += 1;
        self.stack.push(v);
        self.on_stack[v] = true;

        for w in self.adj[v].clone() {
            if self.index[w] == usize::MAX {
                self.strongconnect(w);
                self.low[v] = self.low[v].min(self.low[w]);
            } else if self.on_stack[w] {
                self.low[v] = self.low[v].min(self.index[w]);
            }
        }

        if self.low[v] == self.index[v] {
            loop {
                let w = self.stack.pop().unwrap();
                self.on_stack[w] = false;
                self.comp[w] = self.next_comp;
                if w == v {
                    break;
                }
            }
            self.next_comp += 1;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn edge(a: usize, b: usize) -> GEdge {
        GEdge {
            from: EndPoint { node: a, port: 0 },
            to: EndPoint { node: b, port: 0 },
        }
    }

    #[test]
    fn chain_layers_increase() {
        let layer = assign_layers(3, &[(0, 1), (1, 2)]);
        assert_eq!(layer, vec![0, 1, 2]);
    }

    #[test]
    fn cycle_compacts_to_one_column() {
        let layer = assign_layers(3, &[(0, 1), (1, 2), (2, 0)]);
        assert_eq!(layer, vec![0, 0, 0]);
    }

    #[test]
    fn cycle_then_tail() {
        let layer = assign_layers(3, &[(0, 1), (1, 0), (1, 2)]);
        assert_eq!(layer[0], layer[1]);
        assert_eq!(layer[2], layer[0] + 1);
    }

    #[test]
    fn nodes_in_a_layer_do_not_overlap() {
        // A hub fanning out to several leaves: all leaves share layer 1 and
        // must be stacked without vertical overlap.
        let nodes = vec![GNode::simple(120.0, 40.0); 5];
        let edges = vec![edge(0, 1), edge(0, 2), edge(0, 3), edge(0, 4)];
        let g = Graph { nodes, edges };
        let l = layout(&g, &LayoutConfig::default());
        let leaves = [1usize, 2, 3, 4];
        for &a in &leaves {
            for &b in &leaves {
                if a < b {
                    let (ra, rb) = (&l.nodes[a], &l.nodes[b]);
                    let overlap = ra.y < rb.y + rb.h && rb.y < ra.y + ra.h;
                    assert!(!overlap, "leaves {a} and {b} overlap vertically");
                }
            }
        }
    }

    #[test]
    fn crossing_reduction_helps_a_swap() {
        // Two layers wired in reverse order (0->3, 1->2 with 2 above 3) — the
        // identity ordering crosses; the reducer should reach zero crossings.
        let nv = 4;
        let mut up = vec![Vec::new(); nv];
        let mut down = vec![Vec::new(); nv];
        // 0 -> 3, 1 -> 2
        down[0].push(3);
        up[3].push(0);
        down[1].push(2);
        up[2].push(1);
        let mut pos = vec![0usize; nv];
        let mut layers = vec![vec![0usize, 1], vec![2usize, 3]];
        recompute_pos(&layers, &mut pos);
        let before = count_crossings(&layers, &down, &pos);
        reduce_crossings(&mut layers, &up, &down);
        recompute_pos(&layers, &mut pos);
        let after = count_crossings(&layers, &down, &pos);
        assert_eq!(before, 1);
        assert_eq!(after, 0);
    }

    #[test]
    fn routed_edges_stay_within_bounds() {
        // Long forward edges (spanning many layers → dummies threaded a half-gap
        // left of each column) and a same-column mutual-recursion pair (bowing
        // out past the right column) both leave the node box. The reported
        // bounds must still cover every routed point, or the edge overlay's
        // viewBox clips those curves at a fixed content coordinate.
        let nodes = vec![GNode::simple(120.0, 40.0); 6];
        let edges = vec![
            edge(0, 1),
            edge(1, 2),
            edge(2, 3),
            edge(3, 4),
            edge(0, 4), // long forward edge → dummies
            edge(4, 5),
            edge(5, 4), // 4<->5 mutual recursion → same column, flat bow
        ];
        let g = Graph { nodes, edges };
        let l = layout(&g, &LayoutConfig::default());
        for (ei, e) in l.edges.iter().enumerate() {
            for &(x, y) in &e.points {
                assert!(
                    x >= -0.5 && x <= l.width + 0.5 && y >= -0.5 && y <= l.height + 0.5,
                    "edge {ei} point ({x},{y}) outside bounds {}x{}",
                    l.width,
                    l.height
                );
            }
        }
    }

    #[test]
    fn disconnected_components_pack_toward_screen_aspect() {
        // 24 isolated nodes: without component packing they all share layer 0 —
        // one 24-node column. Packed, the drawing should land near PACK_ASPECT,
        // not degenerate into a strip in either direction.
        let g = Graph { nodes: vec![GNode::simple(120.0, 40.0); 24], edges: Vec::new() };
        let l = layout(&g, &LayoutConfig::default());
        let aspect = l.width / l.height;
        assert!((0.5..=4.0).contains(&aspect), "degenerate aspect {aspect} ({}x{})", l.width, l.height);
        // Placement stays index-aligned and inside bounds, and no two nodes
        // overlap (packing must not stack shelves onto each other).
        for (i, a) in l.nodes.iter().enumerate() {
            assert!(a.x >= 0.0 && a.y >= 0.0 && a.x + a.w <= l.width && a.y + a.h <= l.height);
            for b in &l.nodes[i + 1..] {
                let apart = a.x + a.w <= b.x || b.x + b.w <= a.x || a.y + a.h <= b.y || b.y + b.h <= a.y;
                assert!(apart, "packed nodes overlap");
            }
        }
    }

    #[test]
    fn component_packing_keeps_edges_aligned_and_local() {
        // Two components — a 3-chain and a 2-chain — plus an isolated node.
        // Edges must come back index-aligned, with each polyline connecting its
        // own endpoints' placed ports (never bridging components).
        let g = Graph {
            nodes: vec![GNode::simple(100.0, 36.0); 6],
            edges: vec![edge(0, 1), edge(1, 2), edge(3, 4)],
        };
        let l = layout(&g, &LayoutConfig::default());
        assert_eq!(l.edges.len(), 3);
        for (ei, e) in g.edges.iter().enumerate() {
            let pts = &l.edges[ei].points;
            assert!(pts.len() >= 2, "edge {ei} unrouted");
            let (sx, sy) = l.nodes[e.from.node].out_ports[0];
            let (tx, ty) = l.nodes[e.to.node].in_ports[0];
            assert_eq!(pts[0], (sx, sy), "edge {ei} start not at its out-port");
            assert_eq!(*pts.last().unwrap(), (tx, ty), "edge {ei} end not at its in-port");
        }
    }

    #[test]
    fn balancing_shortens_a_dangling_source() {
        // A source whose only successor sits deep should slide right to just
        // before it (edge length 1), not stay pinned at layer 0.
        // 0->1->2->3->4 chain, plus 5->4: node 5's only edge targets layer 4,
        // so balancing must place 5 at layer 3, not 0.
        let layers = debug_layers(&Graph {
            nodes: vec![GNode::simple(80.0, 30.0); 6],
            edges: vec![edge(0, 1), edge(1, 2), edge(2, 3), edge(3, 4), edge(5, 4)],
        });
        assert_eq!(layers[4], 4);
        assert_eq!(layers[5], 3, "dangling source should slide next to its callee");
    }
}
