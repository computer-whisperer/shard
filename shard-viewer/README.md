# shard-viewer

A graphical navigator for shard source. It parses a shard project itself (a
lightweight structural s-expr reader — *not* the kernel elaborator, and with no
coupling to `rust_bootstrap`) and draws its **methods** as a call-graph flow
chart, built on the [damascene](https://github.com/computer-whisperer/damascene)
UI library.

> **Dependency note:** this points at a local damascene checkout via a path
> dependency — it needs fixes not yet in a crates.io release: wheel-zoom
> yielding to `block_pointer` overlays (so the source lightbox scrolls) and
> `viewport()` laying `Hug` content at full intrinsic (so tall flow diagrams
> aren't clamped). Switch back to a crates.io version once a release ships both.

## Status

Four views, toggled in the toolbar:

**Methods** — one file's call graph, with a **triage overlay**:

- Sidebar lists every `.shard` file with its fn count; a **filter box** narrows
  the list (case-insensitive substring, with a `shown/total` count), and files
  that fail to parse are flagged (`⚠`, the error on hover). Click to switch.
  **Drag the sidebar's right edge** to widen it when paths get cramped
  (`user_resizable`, 220–620px — no divider, the runtime keeps the width).
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
- Layout (for the call/import graphs) is a generic, semantics-agnostic **layered
  (Sugiyama) engine** (`layout.rs`): SCC condensation (cycles share a column) →
  dummy nodes for long edges → barycenter crossing reduction → iterative
  coordinate assignment → port-aware polyline routing. (The Flow view does *not*
  use this — a nested tree is a different problem; see below.)
- **Pan** by dragging an empty area of the canvas; **zoom** with the mouse wheel
  (toward the cursor). The canvas is damascene's native `viewport()` widget, so
  the transform follows hit-test for free. `Fit` frames the whole graph;
  `Reset view` snaps to 1:1. The graph auto-fits when you switch files.
- Click a fn box to open a **detail panel**: signature, the fn's real source
  text, and clickable **Calls** / **Called by** lists. The source is
  **syntax-highlighted** (a small s-expr tokenizer — blue special forms, amber
  constructors/types, green strings, muted comments/parens) and
  **line-numbered**, and long lines **wrap** (manually, at a monospace character
  budget) so nothing clips off the right edge. The panel is **resizable** —
  drag its left edge (`user_resizable`, 320–820px) and the source re-wraps to
  the new width (the live width is read back from the runtime and fed into the
  wrap budget). For a wide or long body (e.g. `driver.shard::run_decls`, 378
  lines) the **Expand ⤢** button additionally opens a **source lightbox** — a
  large centered modal showing the whole source at a much wider wrap budget,
  scrollable, over a dismiss scrim (click outside or **Close**). Cross-file
  links are tagged with
  their file (e.g. `main · check` vs `main · eval`) so homonym targets are
  distinguishable; clicking one navigates there, switching the canvas as needed.
  **Flow ▸ / Board ▸ / Graph ▸** buttons jump the selected fn between views.
  Hover any node for its full signature, home file, and triage metrics.

**Flow** — one fn body as a **structured (LabVIEW-style)** diagram, so s-expr
nesting becomes box *enclosure* instead of parenthesis-counting. Select a fn (in
Methods), then hit **Flow**. It's a containment hybrid:

- **Control structures** (`match` / `if` / `let`) are **frames** that physically
  *contain* their branches. A blue keyword band (green for `let`) heads the
  frame over its scrutinee/condition; inside, each arm / branch / body is a child
  region headed by a blue **selector chip** (`Nil`, `then`, a binding name).
  Nesting depth = box enclosure — a deeply nested `match`/`if` reads as nested
  rectangles, not a sprawl of wires.
- **Leaf computations** are **op cards**: the function name is the bold hero,
  *simple* operands (vars / literals) sit inline, and *compound* operands (nested
  applications) are gathered on a **full-height connector bar** to the left and
  fed into the op by a single arrow (data flows left→right, LabVIEW-style). So
  `(int_eq th 59)` is one card; `(head_code (head_atom line))` is two, the inner
  feeding the outer.
- **`Cons` spines collapse into lists.** A constructor chain like
  `(Cons a (Cons b (Cons c Nil)))` is *data construction*, not computation, so it
  reads as one **`list · N`** box (a bracket bar down a column of element
  regions) instead of three nested `Cons` cards. A non-`Nil` terminator
  (`(Cons x rest)`) shows as a trailing `⋯ rest` row. This is what tames the
  deeply right-nested s-expr builders that pervade the kernel (e.g.
  `driver.shard::rr_goal_s`, which otherwise sprawls into a row of tiny cards).
- **Variables** are warm amber **pills** (data inputs); **literals** are dim mono
  **tags** (constants). The `(measure …)` totality clause is skipped (annotation,
  not logic).

This is laid out *not* by the Sugiyama engine (which stays for the call/import
graphs) but as **nested damascene elements** — containment, sizing, and text
wrapping fall out of the element layout; only the intra-op operand wires are
drawn. The model (`flow.rs`) lowers the fn body into a **region tree** (frames
with labelled branches; ops with operand sub-regions); the view renders it into
the pan/zoom viewport. Today `match` scrutinees and `if` conditions are shown
inline in the band (not expanded into op cards) — the obvious next knob.

**Board** *(experimental)* — one file's **call DAG with each node rendered in the
Flow form**: instead of a name box, every fn appears as its full expanded flow
card, and the call arrows wire whole fn bodies together. You read both a fn's
internal structure *and* where its calls go at once. It reuses the same Sugiyama
layout as Methods, but the engine needs sizes up front while a flow card's true
size is intrinsic — so v1 *estimates* each card's size from its region tree
(`board.rs::est`). Large fns therefore make large nodes (pan/zoom copes);
tightening this (measured sizes, clamped thumbnails) is the next iteration. The
view files are split per-variant (`src/view/`) precisely so these are cheap to
try.

## Binaries

| Bin | What it does |
|---|---|
| `shard-viewer [ROOT]` | The GUI (native window via `damascene-winit-wgpu`). Defaults to the most fn-dense file. |
| `shard-graph [ROOT] [FN]` | Text dump of the extracted model: per-file counts, a project-wide **lines-by-category tally** (mirrors `tools/loc`), the most-called fns, a **cut-candidate (orphan) list**, or one fn's callers/callees. No GUI deps. |
| `shard-render ROOT FILE_SUBSTRING [OUT.svg]` | Headless render of one file's graph to SVG + a lint report. No GPU/window. Use `systems` for the import graph, `flow:FN_NAME` for a fn's dataflow diagram, or `board:FILE_SUBSTRING` for the expanded call-DAG board. |

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
  flow.rs    intra-fn structured model (one fn body -> a region/containment tree)
  layout.rs  layered SCC-aware graph layout
  view/      damascene view tree (pure: project state -> El), one file per variant
    mod.rs     shell: sidebar / toolbar / pane dispatch + ViewMode
    shared.rs  pan/zoom viewport, laid-out-graph canvas, edge + legend primitives
    methods.rs call graph + triage overlay (+ the shared per-fn detail panel)
    systems.rs import graph + proof/impl heat map (+ its breakdown panel)
    flow.rs    one fn body as a region card (render_region, reused by board)
    board.rs   the call DAG with each node in expanded flow form
  bin/       viewer.rs (GUI) · graph.rs (text) · render.rs (headless SVG)
```
