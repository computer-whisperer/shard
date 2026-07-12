//! `shard-viewer` — the graphical navigator.
//!
//! Two-pane workbench: a sidebar listing every `.shard` file in the project,
//! and a `viewport()` canvas drawing the selected file's fns as boxes with
//! their intra-file call edges as arrows. The viewport handles pan (drag the
//! background) and zoom (wheel toward the cursor) natively. A fn box selects it
//! and opens a detail panel (source + clickable callers/callees).
//!
//!   shard-viewer [PROJECT_ROOT]   (defaults to the current directory)

use damascene_core::prelude::*;
use shard_viewer::model::Project;
use shard_viewer::scope::Scope;
use shard_viewer::view::{self, CANVAS_KEY, Sel, ViewMode, ViewParams};

struct Viewer {
    project: Project,
    mode: ViewMode,
    /// The canvas subject (what the view is about). See [`Scope`].
    scope: Scope,
    /// The inspector cursor (fn or file). See [`Sel`].
    selected: Option<Sel>,
    /// Viewport commands queued from clicks, drained once per frame by the host.
    pending: Vec<ViewportRequest>,
    /// Sidebar filter text + its (app-owned) text-selection state.
    filter: String,
    selection: Selection,
    /// Whether the source lightbox is open over the selected fn.
    source_modal: bool,
    /// The Map's per-scope committed-layout cache (see `view::map::Committed`).
    /// Interior-mutable because the pure `build` is what commits a scope the
    /// first time it's shown.
    map_cache: view::MapCache,
    /// Zoom past which Map fn slots draw their flow innards (legend −/+).
    flow_z: f32,
}

impl Viewer {
    fn fit(&mut self) {
        self.pending.push(ViewportRequest::FitContent {
            key: CANVAS_KEY.into(),
            padding: 32.0,
            // Instant: fit() accompanies a content swap (new scope/mode), and
            // flying between two unrelated layouts is meaningless motion.
            behavior: ViewportBehavior::Instant,
        });
    }

    /// Fly the Map camera to a region of the committed plane (scope-as-camera:
    /// the layout stays put, the viewport travels).
    fn fly_to(&mut self, rect: Rect) {
        self.pending.push(ViewportRequest::FrameRect {
            key: CANVAS_KEY.into(),
            rect,
            padding: 48.0,
            behavior: ViewportBehavior::Smooth,
        });
    }

    /// Open a file's call graph (the Methods drill-down), framing it.
    fn open_file(&mut self, i: usize) {
        self.scope = Scope::File(i);
        self.selected = None;
        self.source_modal = false;
        self.mode = ViewMode::Methods;
        // Frame the newly shown graph (the viewport's pan/zoom persists across
        // rebuilds, so without this the new graph could open off-screen).
        self.fit();
    }
}

impl App for Viewer {
    fn build(&self, cx: &BuildCx) -> El {
        let view = cx.viewport_view(CANVAS_KEY).unwrap_or_default();
        // Whether the viewport is still fitted by the armed policy (vs taken
        // over by a user pan/zoom) — the Map keys its cull + LOD pricing off
        // this. A queued FitContent/ResetView counts as home too: it applies
        // during this frame's layout (after build), so the readbacks still
        // hold the *previous* scope's pan/zoom — culling the fresh layout
        // against that stale window would blank regions of the fitted frame.
        let at_home = cx.viewport_at_home(CANVAS_KEY).unwrap_or(true) || !self.pending.is_empty();
        // The detail panel is user-resizable; read its current (dragged) width
        // so the manually-wrapped source re-wraps to fill it.
        let panel_w = cx.user_size(view::PANEL_KEY).unwrap_or(view::DEFAULT_PANEL_W);
        // Estimated canvas size: the window minus the sidebar, open panel, and
        // fixed chrome. Only feeds the Map's (padded) cull window and at-home
        // fit computation — a rough estimate is fine.
        let sidebar_w = cx.user_size(view::SIDEBAR_KEY).unwrap_or(view::DEFAULT_SIDEBAR_W);
        let (win_w, win_h) = cx.viewport().unwrap_or((1280.0, 800.0));
        let panel = if self.selected.is_some() { panel_w + 16.0 } else { 0.0 };
        let canvas = (
            (win_w - sidebar_w - 32.0 - panel - 16.0).max(200.0),
            (win_h - 122.0 - 16.0).max(200.0),
        );
        view::app_root(
            &self.project,
            &ViewParams {
                mode: self.mode,
                scope: self.scope.clone(),
                selected: self.selected,
                zoom: view.zoom,
                pan: view.pan,
                canvas,
                at_home,
                flow_z: self.flow_z,
                hovered: cx.hovered_key().map(str::to_string),
                filter: self.filter.clone(),
                selection: self.selection.clone(),
                source_modal: self.source_modal,
                panel_w,
            },
            Some(&self.map_cache),
        )
    }

    fn selection(&self) -> Selection {
        self.selection.clone()
    }

    fn on_event(&mut self, event: UiEvent, _cx: &EventCx) {
        // Sidebar filter editing: keystrokes / focus / pointer within the field
        // arrive as non-click events routed to "filter". Handle (and the global
        // selection-clear) before the click gate below.
        if event.kind == UiEventKind::SelectionChanged {
            if let Some(sel) = event.selection.as_ref() {
                self.selection = sel.clone();
            }
            return;
        }
        if event.target_key() == Some("filter") {
            text_input::apply_event(&mut self.filter, &mut self.selection, &event, "filter");
            return;
        }
        if !matches!(event.kind, UiEventKind::Click | UiEventKind::Activate) {
            return;
        }
        if event.is_route("src_expand") {
            self.source_modal = true;
        } else if event.is_route("src_close") || event.is_route("src_modal:dismiss") {
            self.source_modal = false;
        } else if event.is_route("filter_clear") {
            self.filter.clear();
            self.selection = Selection::default();
        } else if event.is_route("flowz_down") {
            // Half-octave steps, mirroring how zoom itself is felt.
            self.flow_z = (self.flow_z / std::f32::consts::SQRT_2).max(0.05);
        } else if event.is_route("flowz_up") {
            self.flow_z = (self.flow_z * std::f32::consts::SQRT_2).min(1.0);
        } else if event.is_route("fit") {
            self.fit();
        } else if event.is_route("reset") {
            self.pending.push(ViewportRequest::ResetView {
                key: CANVAS_KEY.into(),
                behavior: ViewportBehavior::Instant,
            });
        } else if event.is_route("mode_methods") {
            self.mode = ViewMode::Methods;
            self.fit();
        } else if event.is_route("mode_systems") {
            self.mode = ViewMode::Systems;
            self.fit();
        } else if event.is_route("goto_card") {
            // "Read this fn large": fly the Map camera to the selected fn's
            // committed flow card (scope-as-camera, like the dir/file cases
            // below). Off the Map — or when the card isn't on the current
            // plane — scope the Map to the fn's file instead; the selected
            // card always draws its innards, so it reads on arrival.
            if let Some(Sel::Fn(g)) = self.selected {
                if self.mode == ViewMode::Map
                    && let Some(r) =
                        view::region_rect(&self.map_cache, &self.scope, view::MapTarget::Fn(g))
                {
                    self.fly_to(r);
                } else {
                    self.scope = Scope::File(self.project.fns[g].file);
                    self.mode = ViewMode::Map;
                    self.fit();
                }
            }
        } else if event.is_route("mode_map") {
            self.mode = ViewMode::Map;
            self.fit();
        } else if event.is_route("scope_project") {
            if self.mode == ViewMode::Map && self.scope == Scope::Project {
                // Already on the project plane: fly home rather than snap.
                // (A smooth FitContent re-arms the fit policy on arrival.)
                self.pending.push(ViewportRequest::FitContent {
                    key: CANVAS_KEY.into(),
                    padding: 32.0,
                    behavior: ViewportBehavior::Smooth,
                });
            } else {
                // Map the whole project at once (sidebar "Whole project").
                self.scope = Scope::Project;
                self.selected = None;
                self.mode = ViewMode::Map;
                self.fit();
            }
        } else if event.is_route("scope_tree") {
            // Map the selected fn's call neighborhood (detail-panel "Tree ▸").
            // Keep it focused as the tree's root. One up, two down: immediate
            // callers plus the transitive implementation it drives.
            if let Some(Sel::Fn(root)) = self.selected {
                self.scope = Scope::CallTree { root, up: 1, down: 2 };
                self.mode = ViewMode::Map;
                self.fit();
            }
        } else if let Some(dir) = event.route_suffix("dir") {
            // Scope-as-camera: when the Map is up and this dir already sits on
            // the committed plane on screen, fly the camera to its box — the
            // topology never re-roots under the user. Otherwise fall back to
            // scoping the canvas to the subtree (a dir spans many files, so it
            // can only be shown on the Map — switch to it; single-file views
            // have no anchor for a Dir scope).
            if self.mode == ViewMode::Map
                && let Some(r) =
                    view::region_rect(&self.map_cache, &self.scope, view::MapTarget::Dir(dir))
            {
                self.fly_to(r);
            } else {
                self.scope = Scope::Dir(dir.to_string());
                self.selected = None;
                self.mode = ViewMode::Map;
                self.fit();
            }
        } else if let Some(i) = event.route_index::<usize>("sysfile")
            && i < self.project.files.len()
        {
            // Select the file in the systems graph: opens its breakdown panel
            // (with an "Open call graph" button to drill) without leaving the
            // import view. Don't refit — the graph itself is unchanged.
            self.scope = Scope::File(i);
        } else if let Some(i) = event.route_index::<usize>("open")
            && i < self.project.files.len()
        {
            // Drill from the systems breakdown panel into the call graph.
            self.open_file(i);
        } else if let Some(i) = event.route_index::<usize>("file")
            && i < self.project.files.len()
        {
            // Scope-as-camera, same as the dir case: a sidebar file click
            // while its box is on the Map's plane flies there instead of
            // tearing the user out into the Methods view.
            if self.mode == ViewMode::Map
                && let Some(r) =
                    view::region_rect(&self.map_cache, &self.scope, view::MapTarget::File(i))
            {
                // Also open the file inspector — the click names the file as
                // the subject of interest, not just a place to look at.
                self.selected = Some(Sel::File(i));
                self.fly_to(r);
            } else {
                self.open_file(i);
            }
        } else if let Some(i) = event.route_index::<usize>("filebox")
            && i < self.project.files.len()
        {
            // A file box's label on the Map: open the file inspector in
            // place. No camera move — the box is already under the pointer.
            self.selected = Some(Sel::File(i));
        } else if let Some(i) = event.route_index::<usize>("fn")
            && i < self.project.fns.len()
        {
            if self.mode == ViewMode::Map {
                // In the Map, a fn click selects it *in place* — opening the
                // detail panel over the same canvas. Don't switch views or
                // collapse the (possibly multi-file) scope to one file; the
                // whole point of the Map is to keep the surrounding structure.
                self.selected = Some(Sel::Fn(i));
            } else {
                // Elsewhere, following a cross-file callee/caller switches the
                // canvas to that fn's file.
                let file = self.project.fns[i].file;
                if Some(file) != self.scope.focus_file(&self.project)
                    || self.mode != ViewMode::Methods
                {
                    self.open_file(file);
                }
                self.selected = Some(Sel::Fn(i));
            }
        }
    }

    fn drain_viewport_requests(&mut self) -> Vec<ViewportRequest> {
        std::mem::take(&mut self.pending)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let root = std::env::args().nth(1).unwrap_or_else(|| ".".to_string());
    let project = Project::load(std::path::Path::new(&root))?;

    // Default the canvas to the file with the most fns, so the window opens on
    // something worth looking at rather than an empty pane.
    let selected_file = project
        .files
        .iter()
        .enumerate()
        .filter(|(_, f)| !f.fns.is_empty())
        .max_by_key(|(_, f)| f.fns.len())
        .map(|(i, _)| i);

    // Open already framed on the default file's graph.
    let pending = if selected_file.is_some() {
        vec![ViewportRequest::FitContent {
            key: CANVAS_KEY.into(),
            padding: 32.0,
            behavior: ViewportBehavior::Instant,
        }]
    } else {
        Vec::new()
    };

    let viewport_rect = Rect::new(0.0, 0.0, 1280.0, 800.0);
    damascene_winit_wgpu::run(
        "shard-viewer",
        viewport_rect,
        Viewer {
            project,
            mode: ViewMode::Methods,
            scope: selected_file.map_or(Scope::None, Scope::File),
            selected: None,
            pending,
            filter: String::new(),
            selection: Selection::default(),
            source_modal: false,
            map_cache: view::MapCache::default(),
            flow_z: view::DEFAULT_FLOW_Z,
        },
    )
}
