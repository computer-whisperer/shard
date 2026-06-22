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

First milestone — the **call-graph (methods) view**:

- Sidebar lists every `.shard` file with its fn count; click to switch.
- The canvas draws the selected file's fns as boxes (name + `N args → Ret`)
  with intra-file call edges as curved arrows.
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

Not yet: the import/"systems" graph and the intra-function control-flow view.
The call-scan parser already feeds both.

## Binaries

| Bin | What it does |
|---|---|
| `shard-viewer [ROOT]` | The GUI (native window via `damascene-winit-wgpu`). Defaults to the most fn-dense file. |
| `shard-graph [ROOT] [FN]` | Text dump of the extracted model: per-file counts, the most-called fns, or one fn's callers/callees. No GUI deps. |
| `shard-render ROOT FILE_SUBSTRING [OUT.svg]` | Headless render of one file's graph to SVG + a lint report. No GPU/window. |

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
  layout.rs  layered SCC-aware graph layout
  view.rs    damascene view tree (pure: project state -> El)
  bin/       viewer.rs (GUI) · graph.rs (text) · render.rs (headless SVG)
```
