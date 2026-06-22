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
//!   1. SCC condensation + longest-path layer assignment (cycles share a column)
//!   2. dummy-node insertion for edges spanning more than one layer
//!   3. barycenter crossing reduction (iterated up/down sweeps)
//!   4. iterative coordinate assignment (pull nodes toward neighbours, no overlap)
//!   5. port resolution + polyline edge routing through the dummy chain
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
const CROSSING_SWEEPS: usize = 8;
const COORD_SWEEPS: usize = 6;

/// Lay out `graph`. The result's `nodes`/`edges` are index-aligned with the
/// input so callers can map placement back to their own data.
pub fn layout(graph: &Graph, cfg: &LayoutConfig) -> Layout {
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

    let vy = assign_y(&layers, &up, &down, &vh, cfg);

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
    let edges = routes
        .iter()
        .zip(&graph.edges)
        .map(|(r, e)| route_edge(r.flat, &r.chain, e, &placed, &layer_x, &vlayer, &vy, cfg))
        .collect();

    let width = placed.iter().map(|p| p.x + p.w).fold(0.0, f32::max) + cfg.margin;
    let height = placed.iter().map(|p| p.y + p.h).fold(0.0, f32::max) + cfg.margin;

    Layout {
        nodes: placed,
        edges,
        width: width.max(cfg.margin * 2.0),
        height: height.max(cfg.margin * 2.0),
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
        // Same-column edge (mutual recursion): bow out into the right gap and
        // come back to the target's left port.
        let bow = start.0.max(end.0) + cfg.layer_gap * 0.4;
        return RoutedEdge {
            points: vec![start, (bow, start.1), (bow, end.1), end],
        };
    }

    let mut points = vec![start];
    for &d in &chain[1..chain.len() - 1] {
        // Thread the gap just left of the dummy's column.
        points.push((layer_x[vlayer[d]] - cfg.layer_gap * 0.5, vy[d] + DUMMY_H / 2.0));
    }
    points.push(end);
    RoutedEdge { points }
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
fn assign_y(
    layers: &[Vec<usize>],
    up: &[Vec<usize>],
    down: &[Vec<usize>],
    vh: &[f32],
    cfg: &LayoutConfig,
) -> Vec<f32> {
    let nv: usize = layers.iter().map(|l| l.len()).sum();
    let mut vy = vec![0.0_f32; nv];
    // Initial top-down stack within each layer.
    for layer in layers {
        let mut y = cfg.margin;
        for &v in layer {
            vy[v] = y;
            y += vh[v] + cfg.node_gap;
        }
    }

    for it in 0..COORD_SWEEPS {
        let order: Vec<usize> = if it % 2 == 0 {
            (0..layers.len()).collect()
        } else {
            (0..layers.len()).rev().collect()
        };
        for l in order {
            place_layer_toward_neighbours(&layers[l], up, down, vh, &mut vy, cfg.node_gap);
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

fn place_layer_toward_neighbours(
    layer: &[usize],
    up: &[Vec<usize>],
    down: &[Vec<usize>],
    vh: &[f32],
    vy: &mut [f32],
    gap: f32,
) {
    let desired: Vec<f32> = layer
        .iter()
        .map(|&v| {
            let centre = |u: usize| vy[u] + vh[u] / 2.0;
            let ns: Vec<usize> = up[v].iter().chain(&down[v]).copied().collect();
            if ns.is_empty() {
                vy[v]
            } else {
                ns.iter().map(|&u| centre(u)).sum::<f32>() / ns.len() as f32 - vh[v] / 2.0
            }
        })
        .collect();
    // Place in layer order, clamping each down so it can't overlap the previous.
    let mut running = f32::NEG_INFINITY;
    for (i, &v) in layer.iter().enumerate() {
        let y = desired[i].max(running);
        vy[v] = y;
        running = y + vh[v] + gap;
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

    (0..n).map(|i| clayer[scc[i]]).collect()
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
}
