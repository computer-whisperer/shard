# shard-viewer

A graphical navigator for shard source. It parses a shard project itself (a
lightweight structural s-expr reader — *not* the kernel elaborator, and with no
coupling to `rust_bootstrap`) and draws it as **one map**: every fn, claim,
type, and file-doc in scope, grouped by origin **dir ⊃ file** into nested
bounding boxes on a single committed plane. Built on the
[damascene](https://github.com/computer-whisperer/damascene) UI library.

## The shape of the app

There is **one view**. The application state is two values plus a camera:

- **Scope** (`scope.rs`) — what set of members the plane is about: one fn, a
  file, a directory subtree, a fn's call-neighborhood, or the whole project.
  Each scope commits its own layout, cached (`MapCache`, LRU).
- **Sel** (`view/mod.rs`) — the inspector cursor: a fn (detail panel: source,
  docstring, call lists) or a file (breakdown panel: counts, composition,
  header doc, imports). Orthogonal to scope — a focus *within* it.
- The **camera** does the rest: clicking a dir, file, or fn whose box is on
  the current plane flies the viewport to it; targets not on the plane
  re-scope. The toolbar is a clickable **scope breadcrumb**
  (`◆ project ▸ kernel/ ▸ reader.shard ▸ rd_form`) — the orientation device.

The viewer began as five separate views (Methods / Systems / Flow / Board /
Map); the Map subsumed their topology and their lenses one by one, and the
others were deleted. What survived of them: the triage overlay (orphan flags
on cards), the Systems heat + composition lenses (on file boxes), the per-fn
and per-file panels (the inspector), and the region-card renderer (the Map's
card innards).

**The cascade convention**: every dependency graph in the viewer — calls,
imports, proof citations, claim subjects, type composition — reads left→right
from *dependencies to dependents*. Callees, cited lemmas, and imported files
layer left; **arrows point at their users**; trust and control build
rightward.

## One committed topology per scope (the cartographic rule)

The layout is a pure function of `(project, scope)` — zoom, selection, and
the pointer never move anything. Every fn owns a footprint sized for its full
flow card, laid out once (`view/map/commit.rs`); zooming only changes *what
is drawn inside* the fixed footprints, the way a map reveals streets and
labels as you approach while no city ever moves. Rendering
(`view/map/els.rs`) is priced in **screen px** per frame:

- a fn slot draws its flow innards when in view and past the flow threshold
  (`innards ≥` in the legend, user-tunable; the selected fn always — so
  selecting highlights in place, nothing reflows), else a slab with the fn
  name at a **screen-constant (cartographic) font** clamped into the slot;
- file/dir boxes draw their contents only past ~48px on screen and carry
  their names as cartographic labels over the box (child labels yield to an
  ancestor's label that still overhangs them — country names before city
  names);
- a file's edge overlay gates at ~140px on screen; box strokes and edge
  splines draw at **hairline (screen-constant) weight**;
- everything outside the (unprojected) viewport is culled per frame.

While the viewport is *at home* (armed `FitPolicy`, or headless) the
effective zoom is computed exactly from the committed extent — which no
longer depends on zoom, so no fit⇄extent feedback loop can exist.

**Scope-as-camera (fly-to navigation)**: clicking a sidebar/breadcrumb dir or
file whose box is already on the committed plane flies the camera to it
(`ViewportRequest::FrameRect`, smooth van Wijk–Nuij zoom-out/translate/
zoom-in) instead of re-rooting the layout — from `Whole project` you navigate
the entire codebase on one unchanging plane, and spatial memory holds: kernel
is always *over there*. The fn panel's **Card ▸** flies to that fn's
committed card ("read this fn large"); **Tree ▸** re-scopes to its call
neighborhood. Clicking `◆ project` again flies home. Targets not on the
current plane fall back to a scope switch + instant fit. (Until the next
damascene release, `FrameRect` rides a temporary `[patch.crates-io]` onto the
local damascene main checkout — see Cargo.toml.)

## The layers on the plane

**Placement is link-based at every level**: inside a file box the members are
placed by the file's intra-file **call graph** (plus the proof and shape edge
families below); inside a dir box the child file/subdir boxes are placed by
the **import DAG** among them (aggregated — a subdir imports another when any
of its files do). Each level is sized bottom-up by the *measured* intrinsic
size of the level below — a card measures itself, a file box measures its
laid-out graph — so the engine always has real sizes and there is no
estimation.

**The proof layer is first-class**: every `(claim …)`, `(axiom …)`,
`(requirement …)`, and `(fulfills …)` form is a **claim card** beside the fn
cards in its file box. The model resolves **citations** (claim → the
claims/axioms its proof mentions; a fulfills cites its requirement, which is
what marks it fulfilled) and **subjects** (claim → the fns its goal mentions),
and both feed the file's graph. **Amber = proof**: axioms are the loud amber
(assumed, not proven — the trust roots), plain claims a faint wash,
requirements **green once fulfilled and red while open**. The kind tint stays
on the slab at any distance, so the wide view reads *where the proof mass and
the assumptions live*. A statements-only file (`kernel/facts.shard`) earns
its box from its claims alone. Fn-anchored scopes pull in the claims *about*
their fns from wherever those claims live.

**The proof card** shows each proof's *structure* in the same Flow vocabulary
the fn cards use (`proof.rs` lowers the tactic tree into the shared `Region`
type): case-split tactics are blue frames with case-selector chips (a case
split *is* a match), `have`/`chain` are green frames binding named facts (a
`have` *is* a let of facts), step ladders are railed columns where a lemma
rewrite makes the **lemma name the bold hero** and checker food (directions,
positions, farkas coefficients) is dropped. Proof footprints are committed
like everything else, so proof *size on the plane* is an honest complexity
signal.

**The shape layer**: every `(type …)`, `(record …)`, and `(sig type …)` form
is a **type card** — kind tag + name, one row per ctor with field types and
the author's trailing `;` note. **Blue = shape**, beside amber = proof. Three
edge families, resolved same-file-first, then transitively-imported, then
unique-project-wide (ambiguous names dropped): **composition** (field type →
aggregate; always drawn — sparse and load-bearing), **ctor-use** (type → fns
that construct/match it; committed so shapes layer left of the code over
them, drawn only under a hover/selection reveal), and **sig-use** (weak tier;
deck-only). The premise: **edges supported maximally, shown selectively.**

**Edge attention tiers**: the focus member's edges always draw at full
strength (hover is the *trace* gesture). At rest, a call/citation edge fades
by **horizontal reach** (crossing most of the file = context, not local
structure) and by **fan redundancy** (a hub's thirtieth caller-line repeats
what its committed leftmost position already says) — counted within the
edge's own class, so a much-cited lemma keeps its crisp call web.

**The shape deck**: hover (or select) a fn and a screen-space overlay docks
at the canvas edge listing its types' full definition cards, each captioned
with its home file — the answer to "I'm zoomed into one fn and its types are
defined three files away". The committed plane can only draw intra-file
edges; the deck is an overlay, exempt from the cartographic rule, so it
follows focus freely. Hovering a type shows what *it* is composed of. It
shares the flow-innards zoom gate (far out, the map is being read as a map).

**Docstrings are harvested, not invented**: the corpus already writes them —
LANGUAGE.md §1's comment tiers (`;` trailing, `;;` line, `;;;` file header).
The parser (`sexpr::TopForm`) carries each top-level form's lead comments;
the model distills the contiguous `;;` run into `doc` (a `;;;` line, a
single-`;` line, or a `;; ----` banner ends the block), and the `;;;` block
at the top of a file becomes the *file's* doc. Every fn/claim/type card
carries the doc's first line as a muted one-line summary; tooltips and the
inspector panels carry the full block. A documented file additionally
commits a **file-doc card** — its whole `;;;` header as a prose card, placed
like any member but flagged as the packing **lead** so it takes the box's
top-left slot, where reading starts. Undocumented members spend no space on
absence.

**The lenses** (carried over from the deleted views):

- **Orphan triage**: a fn nothing calls (and no proof reasons about, and that
  isn't an entry point) flags **red** — on its card and on its distant slab,
  so a wide sweep reads cut candidates as red specks. Resolution is a
  short-name heuristic — verify with grep before cutting.
- **Proof/impl heat**: each file box's fill is tinted by its proof-vs-impl
  share — cool implementation, warm proof — and in the label-only regime the
  box carries a thin **composition bar** (impl · proof · comment/blank track)
  along its bottom edge at screen-constant height. Line categories (impl /
  measure / proof / reqproof / req / sidecar / comment / blank) come from the
  same column-0-head-atom state machine as `tools/loc` — verified
  byte-identical to that shard tool across the corpus.

**Blocks, not receipts (reshaping)**: proofs and fn bodies are almost pure
sequence-and-fork, and rendering both dimensions vertically produced cards
many screens tall. The vertical shapes reshape toward a target aspect
(`view/flow.rs`): fork tactics lay their cases **side by side** (shelf-
wrapped), and tall vertical runs — step ladders, list elements, operand
stacks — wrap into balanced **galley columns** read down-then-right, a `↳`
marker heading each continuation. Fn-body `match`/`if` arms and binding
stacks keep the vertical read (arm order is code order). The partitions are
pure in the region tree, so the committed topology stays deterministic.

## The workbench

- The **sidebar** is the scope picker: `◆ Whole project` on top, then every
  `.shard` file grouped under clickable directory headers; a filter box
  narrows the list; parse failures are flagged (`⚠`, error on hover). Drag
  its right edge to widen (220–620px).
- The **fn inspector** (click any fn card/slab): fixed header (signature,
  triage metrics, docstring, **Card ▸ / Tree ▸**) over a scrolling body with
  the fn's **syntax-highlighted, line-numbered, wrapped source** and
  clickable **Calls / Called by** lists (cross-file targets tagged with their
  file stem). The panel is resizable (drag its left edge, 320–820px; the
  source re-wraps). **Expand ⤢** opens the **source lightbox** — a large
  modal for wide/long bodies (`driver.shard::run_decls`).
- The **file inspector** (click a file's box label, or a sidebar file): the
  file's `;;;` header, per-category line counts, composition bar, import
  in/out degree, and **Map this file ▸** to scope down to it.
- **Pan** by dragging empty canvas; **zoom** with the wheel (toward the
  cursor). `Fit` frames the plane; `Reset view` snaps to 1:1.

## The flow form (inside fn cards)

S-expr nesting becomes box *enclosure* instead of parenthesis-counting:

- **Control structures** (`match` / `if` / `let`) are **frames** that
  physically contain their branches, headed by a blue keyword band (green for
  `let`) and blue **selector chips** per arm.
- **Leaf computations** are **op cards**: the fn name bold, simple operands
  inline, compound operands gathered on a full-height connector bar and fed
  in by one arrow (data flows left→right, LabVIEW-style).
- **`Cons` spines collapse into `list · N` boxes** (data construction, not
  computation) — this tames the deeply right-nested s-expr builders that
  pervade the kernel. A non-`Nil` terminator shows as a trailing `⋯ rest`.
- **Variables** are warm pills; **literals** dim mono tags; `(measure …)`
  clauses are skipped (annotation, not logic).

This is laid out as **nested damascene elements** (`view/flow.rs`) —
containment, sizing, and wrapping fall out of element layout. The Sugiyama
engine is *not* involved inside a card.

## The layout engine

The per-level router (`layout.rs`) is a generic, semantics-agnostic layered
(Sugiyama) engine: SCC condensation (cycles share a column) → dummy nodes for
long edges (**ordering only**) → barycenter crossing reduction → iterative
coordinate assignment over the cards alone → direct port-to-port routing.
Coordinate sweeps are **column-anchored**: after every sweep each column is
rigidly re-centered on the global height-weighted mean, because barycenter
iteration is otherwise indifferent to shear — any staircase where each column
rests at its neighbours' average is a fixpoint. **Edges draw under the card
layer** and route as near-straight curves; routing dummies don't reserve
vertical slots between cards. Two aspect guards keep real shard graphs
screen-shaped: overfull layers **split into sub-columns**, and disconnected
components shelf-pack with short pieces stacking vertically beside tall ones
(a **lead** component — the file-doc card — packs first, top-left). The
whole tree is placed without a viewport (`shared::placed_graph`); only the
outermost result is wrapped in one pan/zoom viewport. The router is
swappable per scale, since each level is one layout call.

## Binaries

| Bin | What it does |
|---|---|
| `shard-viewer [ROOT]` | The GUI (native window via `damascene-winit-wgpu`). Opens on the whole-project map, fitted. |
| `shard-graph [ROOT] [FN]` | Text dump of the extracted model: per-file counts, a project-wide lines-by-category tally (mirrors `tools/loc`), most-called fns, a cut-candidate (orphan) list, or one fn's callers/callees. No GUI deps. |
| `shard-render ROOT SPEC [OUT.svg]` | Headless render to SVG + a lint report. No GPU/window. Specs: `SUBSTR` (file map, most-called fn selected) · `map:SUBSTR` (file map, nothing selected) · `map:DIR/` (directory subtree) · `project` (everything) · `fn:NAME` (one fn's map — "read this fn large") · `tree:NAME` (call neighborhood) · `inspect:SUBSTR` (file inspector open) · `src:NAME` (source lightbox). `SHARD_RENDER_HOVER=fn_name` (or a raw key like `type:7`) simulates hover; `SHARD_RENDER_W/H` size the frame. |
| `diag [ROOT] [FILE]` | Layout diagnostics over the true committed topology (layer histograms, column stats, edge spans). |

`ROOT` defaults to the current directory; run from a shard checkout.

## Headless review loop

`shard-render` is the cheap build-time way to *see* the map without a window.
To rasterize the SVG with the same fonts damascene bundles (so labels
render), point `resvg` at the font files in your damascene checkout:

```bash
shard-render . map:calc/calc.shard /tmp/g.svg
D=~/workspace/damascene/damascene.main
resvg \
  --use-font-file "$D/crates/damascene-fonts-inter/fonts/InterVariable.ttf" \
  --use-font-file "$D/crates/damascene-fonts-jetbrains-mono/fonts/JetBrainsMonoVariable.ttf" \
  /tmp/g.svg /tmp/g.png
```

## Layout

```
src/
  sexpr.rs   s-expr reader (paren tree + lead-comment capture)
  model.rs   structural extraction (fns / claims / types / docs) + edge
             resolution (calls, citations, composition, shape use)
  scope.rs   the subject selection: a fn / file / dir / call-tree / project
  flow.rs    intra-fn structured model (one fn body -> a region tree)
  proof.rs   proof-form lowering (tactic tree -> the same region vocabulary)
  layout.rs  layered SCC-aware graph layout (+ shelf-packed components,
             column anchoring, sub-column splitting, lead-first packing)
  view/      damascene view tree (pure: project state -> El)
    mod.rs       shell: sidebar / scope breadcrumb / inspector dispatch (Sel)
    shared.rs    pan/zoom viewport, placed-graph layer, edge vectors, lenses
    inspector.rs the fn detail panel + file breakdown panel + card tips
    highlight.rs syntax-highlighted source view
    flow.rs      the region-card renderer (fn bodies + proof spines)
    map/         THE view
      mod.rs       committed types, cache + region index, canvas, legend
      commit.rs    commit pass: scope -> geometry (pure, cached)
      els.rs       render pass: boxes, slots, labels, edge tiers (screen-priced)
      cards.rs     the intrinsic member cards + colors + tips
      deck.rs      the screen-space shape deck overlay
  bin/       viewer.rs (GUI) · graph.rs (text) · render.rs (headless SVG) ·
             diag.rs (layout diagnostics)
```
