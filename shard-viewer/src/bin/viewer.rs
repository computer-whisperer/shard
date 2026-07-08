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
use shard_viewer::view::{self, CANVAS_KEY, ViewMode, ViewParams};

struct Viewer {
    project: Project,
    mode: ViewMode,
    /// The canvas subject (what the view is about). See [`Scope`].
    scope: Scope,
    selected_fn: Option<usize>,
    /// Viewport commands queued from clicks, drained once per frame by the host.
    pending: Vec<ViewportRequest>,
    /// Sidebar filter text + its (app-owned) text-selection state.
    filter: String,
    selection: Selection,
    /// Whether the source lightbox is open over the selected fn.
    source_modal: bool,
    /// The Map's cross-frame anchoring state (see `view::map::MapMemo`).
    /// Interior-mutable because the pure `build` is what learns each frame's
    /// layout.
    map_memo: view::MapMemoCell,
}

impl Viewer {
    fn fit(&mut self) {
        self.pending.push(ViewportRequest::FitContent {
            key: CANVAS_KEY.into(),
            padding: 32.0,
        });
    }

    /// Open a file's call graph (the Methods drill-down), framing it.
    fn open_file(&mut self, i: usize) {
        self.scope = Scope::File(i);
        self.selected_fn = None;
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
        // over by a user pan/zoom) — the Map keys its level of detail off this.
        let at_home = cx.viewport_at_home(CANVAS_KEY).unwrap_or(true);
        // The detail panel is user-resizable; read its current (dragged) width
        // so the manually-wrapped source re-wraps to fill it.
        let panel_w = cx.user_size(view::PANEL_KEY).unwrap_or(view::DEFAULT_PANEL_W);
        view::app_root(
            &self.project,
            &ViewParams {
                mode: self.mode,
                scope: self.scope.clone(),
                selected_fn: self.selected_fn,
                zoom: view.zoom,
                pan: view.pan,
                at_home,
                filter: self.filter.clone(),
                selection: self.selection.clone(),
                source_modal: self.source_modal,
                panel_w,
            },
            Some(&self.map_memo),
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
        } else if event.is_route("fit") {
            self.fit();
        } else if event.is_route("reset") {
            self.pending.push(ViewportRequest::ResetView {
                key: CANVAS_KEY.into(),
            });
        } else if event.is_route("mode_methods") {
            self.mode = ViewMode::Methods;
            self.fit();
        } else if event.is_route("mode_systems") {
            self.mode = ViewMode::Systems;
            self.fit();
        } else if event.is_route("mode_board") {
            self.mode = ViewMode::Board;
            self.fit();
        } else if event.is_route("mode_flow") {
            self.mode = ViewMode::Flow;
            self.fit();
        } else if event.is_route("mode_map") {
            self.mode = ViewMode::Map;
            self.fit();
        } else if event.is_route("scope_project") {
            // Map the whole project at once (sidebar "Whole project").
            self.scope = Scope::Project;
            self.selected_fn = None;
            self.mode = ViewMode::Map;
            self.fit();
        } else if event.is_route("scope_tree") {
            // Map the selected fn's call neighborhood (detail-panel "Tree ▸").
            // Keep it focused as the tree's root. One up, two down: immediate
            // callers plus the transitive implementation it drives.
            if let Some(root) = self.selected_fn {
                self.scope = Scope::CallTree { root, up: 1, down: 2 };
                self.mode = ViewMode::Map;
                self.fit();
            }
        } else if let Some(dir) = event.route_suffix("dir") {
            // Sidebar dir header: scope the canvas to the whole subtree. A dir
            // spans many files, so it can only be shown on the Map — switch to
            // it (single-file views have no anchor for a Dir scope).
            self.scope = Scope::Dir(dir.to_string());
            self.selected_fn = None;
            self.mode = ViewMode::Map;
            self.fit();
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
            self.open_file(i);
        } else if let Some(i) = event.route_index::<usize>("fn")
            && i < self.project.fns.len()
        {
            if self.mode == ViewMode::Map {
                // In the Map, a fn click selects it *in place* — opening the
                // detail panel over the same canvas. Don't switch views or
                // collapse the (possibly multi-file) scope to one file; the
                // whole point of the Map is to keep the surrounding structure.
                self.selected_fn = Some(i);
            } else {
                // Elsewhere, following a cross-file callee/caller switches the
                // canvas to that fn's file.
                let file = self.project.fns[i].file;
                if Some(file) != self.scope.focus_file(&self.project)
                    || self.mode != ViewMode::Methods
                {
                    self.open_file(file);
                }
                self.selected_fn = Some(i);
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
            selected_fn: None,
            pending,
            filter: String::new(),
            selection: Selection::default(),
            source_modal: false,
            map_memo: view::MapMemoCell::default(),
        },
    )
}
