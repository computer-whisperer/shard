# shard-viewer

A graphical navigator for shard source. It parses a shard project itself (a
lightweight structural s-expr reader — *not* the kernel elaborator, and with no
coupling to `rust_bootstrap`) and draws its **methods** as a call-graph flow
chart, built on the [damascene](https://github.com/computer-whisperer/damascene)
UI library.

## Status

Five views, toggled in the toolbar. The **subject** a view is about — its
*scope* — is one value (`scope.rs`): a fn, a file, a directory subtree, a fn's
call-neighborhood, or the whole project. Each view projects that scope down to
what it needs (the single-file views read its *focus file*; the Map view reads
the full fn/file sets), so selection is unified rather than per-view.

**The cascade convention**: every dependency graph in the viewer — calls,
imports, proof citations, claim subjects — reads left→right from
*dependencies to dependents*. Callees, cited lemmas, and imported files layer
left; **arrows point at their users**; trust and control build rightward.

**Map** *(experimental — being built)* — the unified view: any scope's fns
**and proof-layer forms**, grouped by origin **dir ⊃ file** into nested
bounding boxes, each fn drawn in the Flow form. Pick a file or a directory in
the sidebar to scope it. The layout is a **recursive graph-placement** pass
mirroring the program's own tree, and at *every* level it is **link-based**:
inside a file box the fns are placed by the file's intra-file **call graph**
(and the claims by their citations — see below), and inside a dir box the
child file/subdir boxes are placed by the **import DAG** among them
(aggregated — a subdir imports another when any of its files do). Each level
is sized bottom-up by the *measured* intrinsic size of the level below
(`layout::intrinsic`): a card measures itself, a file box measures its
laid-out call graph, a dir box measures its laid-out import graph — so the
engine always has real sizes and there is **no estimation** (contrast the
Board's `est()`).

**The proof layer is first-class**: every `(claim …)`, `(axiom …)`,
`(requirement …)`, and `(fulfills …)` form is a **claim card** beside the fn
cards in its file box — kind tag + name + the goal statement. The model
(`model.rs::ClaimDef`) resolves two edge families the same shallow
same-file-first way calls resolve: **citations** (claim → the claims/axioms
its proof mentions; a fulfills cites its requirement, which is what marks it
fulfilled) and **subjects** (claim → the fns its `(goal …)` statement
mentions), and both feed the file's Sugiyama graph. Colors keep the
project-wide convention
(**amber = proof**, same as the Systems heat): axioms are the loud amber
(assumed, not proven — the trust roots), plain claims a faint wash,
requirements **green once fulfilled and red while open** — an unmet
obligation is visible from across the room. Citation edges draw amber,
subject edges muted; the kind tint stays on the slab at any zoom, so the wide
view still reads *where the proof mass and the assumptions live*. A
statements-only file (`kernel/facts.shard`) earns its box from its claims
alone. Scopes follow suit (`Scope::claims`): file/dir/project scopes carry
their files' claims, and fn-anchored scopes (`Fn`, `CallTree`) pull in the
claims *about* their fns from wherever those claims live.

**The shape layer is first-class too**: every `(type …)`, `(record …)`, and
`(sig type …)` form is a **type card** beside the fn and claim cards — kind
tag + name (+ type params), then one row per ctor with its field types and
the author's trailing `;` note when the source carries one (those notes are
half the definition). Colors extend the convention: **blue = shape**
(`tokens::INFO`), next to amber = proof. The model (`model.rs::TypeDef`)
resolves three edge families: **composition** (type → the types its ctor
fields mention — `Type → CtorDef → TypeDef → Module`), **ctor-use** (type →
the fns that construct or pattern-match it — the strong dependency: the fn
breaks if the shape changes), and **sig-use** (type → fns that merely name it
in a signature — the weak tier). Resolution is same-file-first, then
*transitively imported* files, then a unique project-wide match — a
still-ambiguous name is dropped (dozens of example files define their own
`Bool`; wrong-file answers are worse than none). The premise: **edges
supported maximally, shown selectively.** Composition edges draw always (the
web is sparse and load-bearing). Ctor-use edges are *committed* — they inform
placement, so a file's types layer left of the code that works over them —
but draw only when the **hovered or selected member** is an endpoint (hover
is render input, like zoom; the committed plane never hears it). Sig-use
stays out of the routed graph entirely and lives in the deck. The call and
citation webs tier the same way: the focus member's edges always draw at
full strength (hover is the *trace* gesture), while at rest an edge fades by
**horizontal reach** (crossing most of the file = context, not local
structure) and by **fan redundancy** (a hub's thirtieth caller-line repeats
what its committed leftmost position already says) — counted within the
edge's own class, so a much-cited lemma keeps its crisp call web:

**The shape deck**: hover (or select) a fn and a screen-space overlay docks
at the canvas edge listing its types' full definition cards — strong shapes
first, then signature mentions, each captioned with its home file. This is
the answer to "I'm zoomed into one fn and its types are defined three files
away": the committed plane can only draw intra-file edges, but the deck is an
overlay, exempt from the cartographic rule, so it follows focus freely and
shows cross-file definitions on the same screen. Hovering a type shows what
*it* is composed of, wherever those parts live. (Headless: set
`SHARD_RENDER_HOVER=fn_name` — or a raw member key like `type:7` — to
simulate the pointer for review renders.)

**The proof card** shows each proof's *structure* in the same Flow vocabulary
the fn cards taught (`proof.rs` lowers the tactic tree into the shared
`Region` type): case-split tactics (`induct` / `fin-split` / `case-on` /
`wf-induct`) are blue frames with case-selector chips — a case split *is* a
match; `have`/`chain` are green frames binding named facts, each shown as its
goal statement over the proof that establishes it — a `have` *is* a let of
facts; step ladders (`steps`) are railed columns read top→bottom, where a
lemma rewrite makes the **lemma name the bold hero** (`bor_le_pow2 ←
rewrite-with`), premise/hyp rewrites and unfolds keep the verb visible, and
pure computation moves (`simp`/`compute`/`reduce`, `refl`, `by arith`) are
dim tags. The rule: render the skeleton — where it branches, what it cuts in,
what it cites — and drop the checker food (directions, positions, farkas
coefficient lists). Proof footprints are committed like everything else, so
proof *size on the plane* is an honest complexity signal; below the flow
threshold a claim shows as its kind-tinted name slab.

**Blocks, not receipts (reshaping)**: proofs and fn bodies are almost pure
sequence-and-fork, and rendering both dimensions vertically produced cards
many screens tall. The vertical shapes now reshape toward a target aspect
(`view/flow.rs`): fork tactics lay their cases **side by side** (chip above
its case, shelf-wrapped — cases are parallel subproofs), and tall vertical
runs — step ladders, list elements, op operand stacks — wrap into balanced
**galley columns** read down-then-right, a `↳` marker heading each
continuation column. Fn-body `match`/`if` arms and `let`/`have` binding
stacks keep the vertical read (arm order is code order). The partitions are
pure in the region tree, sized by the same `est()` the Board uses, so the
committed topology stays deterministic.

**One committed topology per scope (the cartographic rule)**: the layout is a
pure function of the scope — zoom, selection, and the pointer never move
anything. Every fn owns a footprint sized for its full flow card, laid out
once (cached per scope in an app-owned `MapCache`); zooming only changes
*what is drawn inside* the fixed footprints, the way a map reveals streets
and labels as you approach while no city ever moves. Rendering is priced in
**screen px** per frame: a fn slot draws its flow innards when it's in view
and the zoom affords legible text (the selected fn always — so selecting
highlights in place, nothing reflows), else a slab with the fn name at a
**screen-constant (cartographic) font** clamped into the slot; file/dir boxes
draw their contents only past ~48px on screen and carry their names as
cartographic labels over the box (child labels yield to an ancestor's label
that still overhangs them — country names before city names); a file's
call-edge overlay gates at ~140px on screen; box strokes and edge splines
draw at **hairline (screen-constant) weight**, since a project-fit zoom of
~0.005 makes any content-space stroke invisible. Everything outside the
(unprojected) viewport is culled per frame. While the viewport is *at home*
(armed `FitPolicy`, or headless) the effective zoom is computed exactly from
the committed extent — which no longer depends on zoom, so the old
fit⇄extent feedback loop cannot exist.

**Scope-as-camera (fly-to navigation)**: while the Map is up, clicking a
sidebar dir or file whose box is already on the committed plane flies the
camera to it (`ViewportRequest::FrameRect`, damascene #122, smooth van
Wijk–Nuij zoom-out/translate/zoom-in) instead of re-rooting the layout — so
from `Whole project` you navigate the entire codebase on one unchanging
plane, and spatial memory holds: kernel is always *over there*. Clicking
`Whole project` again flies home. Targets not on the current plane fall back
to the usual scope switch + instant fit. (Until the next damascene release,
`FrameRect` rides a temporary `[patch.crates-io]` onto the local damascene
main checkout — see Cargo.toml.)
The interface/implementation
split reads at a glance: a `mod.req.shard` of signature-only cards sits beside
the `.shard` that implements them, with an import arrow between. The whole tree
is placed without a viewport (`shared::placed_graph`); only the outermost result
is wrapped in one pan/zoom viewport. The per-level router is Sugiyama
(`layout.rs`) for now — swappable per scale, since each level is one layout call.
This is the view the others are converging into — see *Direction*.

**Methods** — one file's call graph, with a **triage overlay**:

- The sidebar is a **scope picker**: a **`◆ Whole project`** entry at the top
  (maps every fn at once), then every `.shard` file grouped under a clickable
  **directory header** (`▸ dir/ (N)` — selecting it scopes the Map to the whole
  subtree); files show as basenames with their fn count, and selecting one scopes
  to that file. A **filter box** narrows the list (case-insensitive substring,
  with a `shown/total` count), and files that fail to parse are flagged (`⚠`, the
  error on hover). **Drag the sidebar's right edge** to widen it when paths get
  cramped (`user_resizable`, 220–620px — the runtime keeps the width). A selected
  fn's detail panel also offers **`Tree ▸`** — scope the Map to that fn's call
  neighborhood (`CallTree`: one caller level up, two callee levels down).
- The canvas draws the selected file's fns as boxes (name + `N args → Ret`)
  with intra-file call edges as curved arrows.
- **Triage colors/sizes** encode the dead-code / complexity signal: a node is
  **red** when it's an *orphan* — nothing in the project calls it, it isn't
  reasoned about in any claim/fulfills/requirement, and it isn't an entry
  point (a cut candidate); **warm/orange** scales with call degree (hubs stand
  out from leaves); and node **height** grows with the fn's source-line count.
  The detail panel shows `lines · calls · callers` and tags orphans. Resolution
  is a short-name heuristic — verify a candidate with grep before cutting.

**Systems** — the project-wide file import dependency graph, with a
**category heat map**:

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
  dummy nodes for long edges (ordering only) → barycenter crossing reduction →
  iterative coordinate assignment over the cards alone → direct port-to-port
  routing. (The Flow view does *not* use this — a nested tree is a different
  problem; see below.) Coordinate sweeps are **column-anchored**: after every
  sweep each column is rigidly re-centered on the global height-weighted mean,
  because barycenter iteration is otherwise indifferent to shear — any
  staircase where each column rests at its neighbours' average is a fixpoint,
  and dense files converged to diagonal drifts that wasted half the drawing.
  **Edges draw under the card layer** and route as near-straight curves;
  routing dummies don't reserve vertical slots between cards (sharing the
  column stack used to fence low-degree cards thousands of px from their
  neighbours and swept long edges into swooping waypoint chains). Two aspect
  guards keep real shard graphs screen-shaped: overfull layers **split into
  sub-columns** (fan-heavy files pile dozens of leaf helpers into one rank —
  legal to split, since same-rank nodes never have edges between them), and
  disconnected components shelf-pack with short pieces **stacking vertically**
  beside tall ones instead of trailing across the top.
- **Pan** by dragging an empty area of the canvas; **zoom** with the mouse wheel
  (toward the cursor). The canvas is damascene's native `viewport()` widget, so
  the transform follows hit-test for free. `Fit` frames the whole graph;
  `Reset view` snaps to 1:1. The graph auto-fits when you switch files.
- Click a fn box to open a **detail panel**: a fixed header (signature, triage
  metrics, view-jump buttons) over a **scrolling body** that holds the fn's real
  source text and the clickable **Calls** / **Called by** lists (so a fn with
  many calls/callers doesn't overflow the panel). The source is
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

## Direction

The **Map** view is where the others are heading: one unified canvas that
renders any *scope* (one fn → a file → a dir → a call-neighborhood → the whole
project), with fns grouped by origin file/dir into bounding boxes and call usage
traced between them. The point is to iterate *one* view instead of four, and to
support richer module-level reading at a glance.

The intended layout is **recursive and compositional**, mirroring the program's
own containment tree (`dir ⊃ file ⊃ fn ⊃ body`): each level lays out its
children in its own frame and reports an intrinsic size plus the ports it
imports/exports; the module tree is laid out against the declared import DAG;
function-usage wires are routed last, bundled along those import edges. Sugiyama
(`layout.rs`) survives as *one* intra-level router, swappable per scale rather
than being the view itself. Stable, identity-anchored placement (so the map is a
thing you learn, not a fresh diagram each time) and a proof/requirement lens are
later experiments. Methods/Board/Flow stay as-is until Map subsumes them — at
which point the plain name box is just a *collapsed* flow card.

This is being built additively: Map is a fifth tab, and the existing four are
untouched while it grows.

## Binaries

| Bin | What it does |
|---|---|
| `shard-viewer [ROOT]` | The GUI (native window via `damascene-winit-wgpu`). Defaults to the most fn-dense file. |
| `shard-graph [ROOT] [FN]` | Text dump of the extracted model: per-file counts, a project-wide **lines-by-category tally** (mirrors `tools/loc`), the most-called fns, a **cut-candidate (orphan) list**, or one fn's callers/callees. No GUI deps. |
| `shard-render ROOT FILE_SUBSTRING [OUT.svg]` | Headless render of one file's graph to SVG + a lint report. No GPU/window. Use `systems` for the import graph, `flow:FN_NAME` for a fn's dataflow diagram, `board:FILE_SUBSTRING` for the expanded call-DAG board, or `map:FILE_SUBSTRING` for the unified Map scoped to that file (`map:DIR/` with a trailing slash scopes to a whole directory subtree, `tree:FN_NAME` to a fn's call neighborhood, `project` to every fn in the project). |

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
  model.rs   structural extraction (fns / claims / types) + edge resolution
             (calls, citations, composition, shape use — same-file-first)
  scope.rs   the subject selection: a fn / file / dir / call-tree / project
  flow.rs    intra-fn structured model (one fn body -> a region/containment tree)
  layout.rs  layered SCC-aware graph layout (+ weakly-connected components
             shelf-packed toward a screen aspect, so sparse graphs don't
             degenerate into one enormous column)
  view/      damascene view tree (pure: project state -> El), one file per variant
    mod.rs     shell: sidebar scope-picker / toolbar / pane dispatch + ViewMode
    shared.rs  pan/zoom viewport, laid-out-graph canvas, edge + legend primitives
    methods.rs call graph + triage overlay (+ the shared per-fn detail panel)
    systems.rs import graph + proof/impl heat map (+ its breakdown panel)
    flow.rs    one fn body as a region card (render_region, reused by board/map)
    board.rs   the call DAG with each node in expanded flow form
    map.rs     the unified scope view (recursive graph placement: calls + imports)
  bin/       viewer.rs (GUI) · graph.rs (text) · render.rs (headless SVG)
```
