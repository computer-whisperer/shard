//! A small layered ("Sugiyama-lite") layout for a fn call graph.
//!
//! Nodes are placed in columns by their longest-path depth from a source
//! (callers on the left, callees on the right); within a column they stack
//! vertically. Cycles (shard's mutual-recursion SCCs) don't break it — layer
//! assignment relaxes a bounded number of times, so a back edge just spans
//! columns rather than looping forever.
//!
//! The layout is computed in abstract canvas coordinates; the GUI maps node
//! rects and edge paths straight into that space.

use crate::model::Project;

pub const NODE_W: f32 = 200.0;
pub const NODE_H: f32 = 44.0;
const H_GAP: f32 = 90.0;
const V_GAP: f32 = 22.0;
const MARGIN: f32 = 40.0;

#[derive(Debug, Clone)]
pub struct Node {
    /// Index into `Project::fns`.
    pub fn_idx: usize,
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Clone, Default)]
pub struct GraphLayout {
    pub nodes: Vec<Node>,
    /// Edges as `(from, to)` indices into `nodes`.
    pub edges: Vec<(usize, usize)>,
    pub width: f32,
    pub height: f32,
}

impl GraphLayout {
    pub fn center(&self) -> (f32, f32) {
        (self.width / 2.0, self.height / 2.0)
    }
}

/// Lay out the intra-file call graph of one file: its fns as nodes, and the
/// calls whose callee is also defined in the same file as edges.
pub fn layout_file(project: &Project, file_idx: usize) -> GraphLayout {
    let local_fns = &project.files[file_idx].fns;
    layout_subgraph(project, local_fns, |callee_file| callee_file == file_idx)
}

/// Lay out an explicit set of fns (`fn_indices`), keeping only the edges whose
/// callee passes `keep_callee` (called with the callee's file index). This is
/// the shared core for both the file view and a future fn-neighborhood view.
pub fn layout_subgraph(
    project: &Project,
    fn_indices: &[usize],
    keep_callee: impl Fn(usize) -> bool,
) -> GraphLayout {
    let n = fn_indices.len();
    let mut layout = GraphLayout::default();
    if n == 0 {
        return layout;
    }

    // global fn index -> local node index
    let mut local_of = std::collections::HashMap::new();
    for (li, &gi) in fn_indices.iter().enumerate() {
        local_of.insert(gi, li);
    }

    let mut edges = Vec::new();
    for &gi in fn_indices {
        let li = local_of[&gi];
        for &callee in &project.fns[gi].calls {
            if !keep_callee(project.fns[callee].file) {
                continue;
            }
            if let Some(&lj) = local_of.get(&callee)
                && li != lj
            {
                edges.push((li, lj));
            }
        }
    }
    edges.sort_unstable();
    edges.dedup();

    let layer = assign_layers(n, &edges);
    let max_layer = layer.iter().copied().max().unwrap_or(0);

    // Order nodes within each layer; track running y per column.
    let mut col_count = vec![0usize; max_layer + 1];
    let mut nodes = vec![
        Node {
            fn_idx: 0,
            x: 0.0,
            y: 0.0,
            w: NODE_W,
            h: NODE_H,
        };
        n
    ];
    for li in 0..n {
        let l = layer[li];
        let row = col_count[l];
        col_count[l] += 1;
        nodes[li] = Node {
            fn_idx: fn_indices[li],
            x: MARGIN + l as f32 * (NODE_W + H_GAP),
            y: MARGIN + row as f32 * (NODE_H + V_GAP),
            w: NODE_W,
            h: NODE_H,
        };
    }

    let width = MARGIN + (max_layer as f32 + 1.0) * (NODE_W + H_GAP) - H_GAP + MARGIN;
    let tallest = col_count.iter().copied().max().unwrap_or(0);
    let height = MARGIN + tallest as f32 * (NODE_H + V_GAP) - V_GAP + MARGIN;

    layout.nodes = nodes;
    layout.edges = edges;
    layout.width = width.max(NODE_W + 2.0 * MARGIN);
    layout.height = height.max(NODE_H + 2.0 * MARGIN);
    layout
}

/// Assign each node a column ("layer") by longest path through the *condensed*
/// graph: strongly-connected components (shard's mutual-recursion SCCs) collapse
/// to a single node first, so every member of a cycle shares one column instead
/// of being smeared across `n` columns. The condensation is acyclic, so the
/// longest-path relaxation converges.
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

/// Tarjan's SCC algorithm: returns a component id per node. The condensation
/// (one node per component) is a DAG, which is what makes layering well-defined
/// on cyclic call graphs.
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

    #[test]
    fn chain_layers_increase() {
        // 0 -> 1 -> 2 should land in three distinct columns.
        let layer = assign_layers(3, &[(0, 1), (1, 2)]);
        assert_eq!(layer, vec![0, 1, 2]);
    }

    #[test]
    fn cycle_compacts_to_one_column() {
        // A 3-cycle is one SCC, so all members share a single column.
        let layer = assign_layers(3, &[(0, 1), (1, 2), (2, 0)]);
        assert_eq!(layer, vec![0, 0, 0]);
    }

    #[test]
    fn cycle_then_tail() {
        // {0,1} is a cycle (col 0); 2 hangs off it (col 1).
        let layer = assign_layers(3, &[(0, 1), (1, 0), (1, 2)]);
        assert_eq!(layer[0], layer[1]);
        assert_eq!(layer[2], layer[0] + 1);
    }
}
