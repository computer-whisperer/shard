# shard-viewer

A graphical navigator for shard source. It parses a shard project itself (a
lightweight structural s-expr reader — *not* the kernel elaborator, and with no
coupling to `rust_bootstrap`) and draws its **methods** as a call-graph flow
chart, built on the [damascene](https://github.com/computer-whisperer/damascene)
UI library.

> **Dependency note:** this currently uses damascene's native `viewport()`
> pan/zoom widget, which isn't in the published 0.4.3 yet, so `Cargo.toml`
> points at a local damascene checkout via a path dependency. Switch back to a
> crates.io version once a release ships `viewport()`.

## Status

Two views, toggled in the toolbar:

**Methods** — one file's call graph, with a **triage overlay**:

- Sidebar lists every `.shard` file with its fn count; click to switch.
- The canvas draws the selected file's fns as boxes (name + `N args → Ret`)
  with intra-file call edges as curved arrows.
- **Triage colors/sizes** encode the dead-code / complexity signal: a node is
  **red** when it's an *orphan* — nothing in the project calls it, it isn't
  reasoned about in any claim/fulfills/requirement, and it isn't an entry
  point (a cut candidate); **warm/orange** scales with call degree (hubs stand
  out from leaves); and node **height** grows with the fn's source-line count.
  The detail panel shows `lines · calls · callers` and tags orphans. Resolution
  is a short-name heuristic — verify a candidate with grep before cutting.

**Systems** — the project-wide file import dependency graph (each file → the
files it imports), with a **category heat map**:

- Each file node is **tinted by its proof-vs-impl share** — cool for
  implementation-heavy files, warm for proof-heavy ones — and carries a thin
  **composition bar** (implementation · proof burden · comment/blank track), so
  you can read a large tree's verification weight at a glance.
- Lines are classified into shard-specific categories (impl / measure / proof /
  reqproof / req / sidecar / comment / blank) by the same column-0-head-atom
  state machine as `tools/loc` — the Rust port is verified byte-identical to
  that shard tool across the corpus.
- **Click a file node** to open its **breakdown panel** (per-category line
  counts + import in/out degree); the panel's **Open call graph ▸** button
  drills into the file's Methods view.
- Layout is a generic, semantics-agnostic **layered (Sugiyama) engine**
  (`layout.rs`): SCC condensation (cycles share a column) → dummy nodes for
  long edges → barycenter crossing reduction → iterative coordinate assignment
  → port-aware polyline routing. Nodes carry input/output **ports** and a
  reserved `sub` nesting hook, so the planned s-expr **dataflow (LabVIEW-style)
  view** plugs into the same engine. The call-graph view is its first client.
- **Pan** by dragging an empty area of the canvas; **zoom** with the mouse wheel
  (toward the cursor). The canvas is damascene's native `viewport()` widget, so
  the transform follows hit-test for free. `Fit` frames the whole graph;
  `Reset view` snaps to 1:1. The graph auto-fits when you switch files.
- Click a fn box to open a **detail panel**: signature, the fn's real source
  text, and clickable **Calls** / **Called by** lists. Clicking a callee/caller
  (including cross-file) navigates to it, switching the canvas as needed.

**Flow** — one fn body as a **dataflow / decision-tree** diagram, so s-expr
nesting becomes spatial instead of parenthetical. Select a fn (in Methods),
then hit **Flow**. It's a *hybrid* rendering:

Each node *kind* gets a distinct visual form so you read the shape, not the
text:

- **Control structures** (`match` / `if`) are cards with a blue **keyword band**
  over their scrutinee/condition — the recognizable skeleton; **`let`** gets a
  green band. Each arm / branch / body hangs to the right, headed by a **selector
  chip** showing the pattern or branch (`Nil`, `then`, a binding name).
- **Leaf expressions** become **operation** cards: the function name is the bold
  hero, operands a quiet second line. *Simple* operands (vars / literals) sit
  inline; *compound* operands (nested applications) expand into their own op
  cards wired in. So `(int_eq th 59)` is one card; `(head_code (head_atom line))`
  is two.
- **Variables** are warm amber **pills** (data inputs); **literals** are dim mono
  **tags** (constants).
- **Control wires** (which branch runs) are thick and neutral; **data wires** (a
  value feeds a computation) are thin and accent-tinted, so the decision
  skeleton reads apart from the value wiring. The `(measure …)` totality clause
  is skipped (it's an annotation, not logic).

The model (`flow.rs`) lowers the body into typed node/edge lists; the view sizes,
shapes, and colors them and reuses the same layered layout engine. Today `match`
scrutinees and `if` conditions are shown inline (not expanded into op trees);
that's the obvious next tuning knob.

## Binaries

| Bin | What it does |
|---|---|
| `shard-viewer [ROOT]` | The GUI (native window via `damascene-winit-wgpu`). Defaults to the most fn-dense file. |
| `shard-graph [ROOT] [FN]` | Text dump of the extracted model: per-file counts, a project-wide **lines-by-category tally** (mirrors `tools/loc`), the most-called fns, a **cut-candidate (orphan) list**, or one fn's callers/callees. No GUI deps. |
| `shard-render ROOT FILE_SUBSTRING [OUT.svg]` | Headless render of one file's graph to SVG + a lint report. No GPU/window. Use `systems` for the import graph, or `flow:FN_NAME` for a fn's dataflow diagram. |

`ROOT` defaults to the current directory; run from a shard checkout.

## Headless review loop

`shard-render` is the cheap build-time way to *see* the graph without a window.
To rasterize the SVG with the same fonts damascene bundles (so labels render),
point `resvg` at the font files in your damascene checkout:

```bash
shard-render . calc/calc.shard /tmp/g.svg
D=~/workspace/damascene/damascene.main
resvg \
  --use-font-file "$D/crates/damascene-fonts-inter/fonts/InterVariable.ttf" \
  --use-font-file "$D/crates/damascene-fonts-jetbrains-mono/fonts/JetBrainsMonoVariable.ttf" \
  /tmp/g.svg /tmp/g.png
```

## Layout

```
src/
  sexpr.rs   s-expr reader (paren tree only)
  model.rs   structural extraction + call-graph resolution (same-file-first)
  flow.rs    intra-fn dataflow / decision-tree model (one fn body -> nodes/edges)
  layout.rs  layered SCC-aware graph layout
  view.rs    damascene view tree (pure: project state -> El)
  bin/       viewer.rs (GUI) · graph.rs (text) · render.rs (headless SVG)
```
