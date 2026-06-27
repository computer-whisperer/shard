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
use shard_viewer::view::{self, CANVAS_KEY, ViewMode, ViewParams};

struct Viewer {
    project: Project,
    mode: ViewMode,
    selected_file: Option<usize>,
    selected_fn: Option<usize>,
    /// Viewport commands queued from clicks, drained once per frame by the host.
    pending: Vec<ViewportRequest>,
    /// Sidebar filter text + its (app-owned) text-selection state.
    filter: String,
    selection: Selection,
    /// Whether the source lightbox is open over the selected fn.
    source_modal: bool,
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
        self.selected_file = Some(i);
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
        let zoom = cx.viewport_view(CANVAS_KEY).map_or(1.0, |v| v.zoom);
        // The detail panel is user-resizable; read its current (dragged) width
        // so the manually-wrapped source re-wraps to fill it.
        let panel_w = cx.user_size(view::PANEL_KEY).unwrap_or(view::DEFAULT_PANEL_W);
        view::app_root(
            &self.project,
            &ViewParams {
                mode: self.mode,
                selected_file: self.selected_file,
                selected_fn: self.selected_fn,
                zoom,
                filter: self.filter.clone(),
                selection: self.selection.clone(),
                source_modal: self.source_modal,
                panel_w,
            },
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
        } else if let Some(i) = event.route_index::<usize>("sysfile")
            && i < self.project.files.len()
        {
            // Select the file in the systems graph: opens its breakdown panel
            // (with an "Open call graph" button to drill) without leaving the
            // import view. Don't refit — the graph itself is unchanged.
            self.selected_file = Some(i);
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
            // Following a cross-file callee/caller switches the canvas.
            let file = self.project.fns[i].file;
            if Some(file) != self.selected_file || self.mode != ViewMode::Methods {
                self.open_file(file);
            }
            self.selected_fn = Some(i);
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
            selected_file,
            selected_fn: None,
            pending,
            filter: String::new(),
            selection: Selection::default(),
            source_modal: false,
        },
    )
}
