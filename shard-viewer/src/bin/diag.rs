//! Temporary layout diagnostics: dump layer/edge statistics for one file's
//! committed Map graph, to ground the layout/routing refinement round.

use shard_viewer::layout;
use shard_viewer::model::Project;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut args = std::env::args().skip(1);
    let root = args.next().unwrap_or_else(|| "..".to_string());
    let needle = args.next().unwrap_or_default();
    let project = Project::load(std::path::Path::new(&root))?;
    let file = project
        .files
        .iter()
        .position(|f| f.rel.contains(&needle))
        .ok_or_else(|| format!("no file matching `{needle}`"))?;
    println!("=== {} ===", project.files[file].rel);

    let graph = shard_viewer::view::debug_file_graph(&project, file);
    let n = graph.nodes.len();
    println!("nodes {}  edges {}", n, graph.edges.len());

    let layers = layout::debug_layers(&graph);
    let nlayers = layers.iter().copied().max().unwrap_or(0) + 1;
    let mut hist = vec![0usize; nlayers];
    for &l in &layers {
        hist[l] += 1;
    }
    println!("layers {nlayers}: {hist:?}");

    // Edge span distribution + dummy count.
    let mut spans = std::collections::BTreeMap::new();
    let mut dummies = 0usize;
    for e in &graph.edges {
        let s = layers[e.from.node].abs_diff(layers[e.to.node]);
        *spans.entry(s).or_insert(0usize) += 1;
        dummies += s.saturating_sub(1);
    }
    println!("edge spans (layers->count): {spans:?}");
    println!("total dummies {dummies} vs real nodes {n}");

    let cfg = layout::LayoutConfig::for_nodes(&graph.nodes);
    let lay = layout::layout(&graph, &cfg);
    println!("layout {}x{}  aspect {:.2}", lay.width, lay.height, lay.width / lay.height);

    // Node bbox vs full (edge-including) bbox.
    let (mut nx0, mut ny0, mut nx1, mut ny1) = (f32::MAX, f32::MAX, 0f32, 0f32);
    for p in &lay.nodes {
        nx0 = nx0.min(p.x);
        ny0 = ny0.min(p.y);
        nx1 = nx1.max(p.x + p.w);
        ny1 = ny1.max(p.y + p.h);
    }
    let (mut ex0, mut ey0, mut ex1, mut ey1) = (nx0, ny0, nx1, ny1);
    for e in &lay.edges {
        for &(x, y) in &e.points {
            ex0 = ex0.min(x);
            ey0 = ey0.min(y);
            ex1 = ex1.max(x);
            ey1 = ey1.max(y);
        }
    }
    println!(
        "node bbox {:.0}x{:.0}   with edges {:.0}x{:.0}  (edge overhang: {:.0} below, {:.0} above, {:.0} right, {:.0} left)",
        nx1 - nx0,
        ny1 - ny0,
        ex1 - ex0,
        ey1 - ey0,
        ey1 - ny1,
        ny0 - ey0,
        ex1 - nx1,
        nx0 - ex0
    );

    // Diagonal drift: group placed nodes into columns by x (the post-cap
    // layering), then print each column's real-node count, mean center-y and
    // y-extent left to right.
    let mut xs: Vec<f32> = lay.nodes.iter().map(|p| p.x).collect();
    xs.sort_by(f32::total_cmp);
    xs.dedup_by(|a, b| (*a - *b).abs() < 1.0);
    let col_of = |x: f32| xs.iter().position(|&c| (c - x).abs() < 1.0).unwrap();
    let ncols = xs.len();
    let mut acc: Vec<(f32, usize)> = vec![(0.0, 0); ncols];
    let mut ext: Vec<(f32, f32)> = vec![(f32::MAX, 0.0); ncols];
    for p in &lay.nodes {
        let l = col_of(p.x);
        acc[l].0 += p.y + p.h / 2.0;
        acc[l].1 += 1;
        ext[l].0 = ext[l].0.min(p.y);
        ext[l].1 = ext[l].1.max(p.y + p.h);
    }
    println!("columns {ncols} (post-cap):");
    for l in 0..ncols {
        println!(
            "  col {:2}  n {:3}  mean-cy {:6.0}  y [{:6.0} .. {:6.0}]",
            l,
            acc[l].1,
            acc[l].0 / acc[l].1.max(1) as f32,
            ext[l].0,
            ext[l].1
        );
    }

    // The monsters: tallest nodes (suspected flinging levers).
    let mut by_h: Vec<usize> = (0..n).collect();
    by_h.sort_by(|&a, &b| lay.nodes[b].h.total_cmp(&lay.nodes[a].h));
    println!("tallest nodes:");
    for &i in by_h.iter().take(8) {
        println!(
            "  node {:3}  {:.0}x{:.0}  at y {:.0} col {}",
            i,
            lay.nodes[i].w,
            lay.nodes[i].h,
            lay.nodes[i].y,
            col_of(lay.nodes[i].x)
        );
    }
    // The flung: lowest-bottom real nodes + their neighbours' mean center.
    let mut nb: Vec<Vec<usize>> = vec![Vec::new(); n];
    for e in &graph.edges {
        nb[e.from.node].push(e.to.node);
        nb[e.to.node].push(e.from.node);
    }
    let mut by_bot: Vec<usize> = (0..n).collect();
    by_bot.sort_by(|&a, &b| {
        (lay.nodes[b].y + lay.nodes[b].h).total_cmp(&(lay.nodes[a].y + lay.nodes[a].h))
    });
    println!("lowest nodes:");
    for &i in by_bot.iter().take(8) {
        let nmean: f32 = if nb[i].is_empty() {
            -1.0
        } else {
            nb[i].iter().map(|&j| lay.nodes[j].y + lay.nodes[j].h / 2.0).sum::<f32>()
                / nb[i].len() as f32
        };
        println!(
            "  node {:3}  h {:5.0}  y {:6.0} col {:2}  deg {:2}  neighbours-mean-cy {:6.0}",
            i,
            lay.nodes[i].h,
            lay.nodes[i].y,
            col_of(lay.nodes[i].x),
            nb[i].len(),
            nmean
        );
    }
    // The overhang: how many edges dip below the node mass, and how far.
    let node_bot = ny1;
    let mut over = 0usize;
    let mut worst = 0.0_f32;
    for e in &lay.edges {
        let m = e.points.iter().map(|p| p.1).fold(0.0, f32::max);
        if m > node_bot + 1.0 {
            over += 1;
            worst = worst.max(m - node_bot);
        }
    }
    println!("edges dipping below node mass: {over} (worst {worst:.0}px)");
    Ok(())
}
