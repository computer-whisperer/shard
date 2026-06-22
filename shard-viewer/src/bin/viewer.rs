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
use shard_viewer::view::{self, CANVAS_KEY, ViewParams};

struct Viewer {
    project: Project,
    selected_file: Option<usize>,
    selected_fn: Option<usize>,
    /// Viewport commands queued from clicks, drained once per frame by the host.
    pending: Vec<ViewportRequest>,
}

impl Viewer {
    fn fit(&mut self) {
        self.pending.push(ViewportRequest::FitContent {
            key: CANVAS_KEY.into(),
            padding: 32.0,
        });
    }

    fn select_file(&mut self, i: usize) {
        self.selected_file = Some(i);
        self.selected_fn = None;
        // Frame the newly shown graph (the viewport's pan/zoom persists across
        // rebuilds, so without this the new file could open off-screen).
        self.fit();
    }
}

impl App for Viewer {
    fn build(&self, cx: &BuildCx) -> El {
        let zoom = cx.viewport_view(CANVAS_KEY).map_or(1.0, |v| v.zoom);
        view::app_root(
            &self.project,
            &ViewParams {
                selected_file: self.selected_file,
                selected_fn: self.selected_fn,
                zoom,
            },
        )
    }

    fn on_event(&mut self, event: UiEvent, _cx: &EventCx) {
        if !matches!(event.kind, UiEventKind::Click | UiEventKind::Activate) {
            return;
        }
        if event.is_route("fit") {
            self.fit();
        } else if event.is_route("reset") {
            self.pending.push(ViewportRequest::ResetView {
                key: CANVAS_KEY.into(),
            });
        } else if let Some(i) = event.route_index::<usize>("file")
            && i < self.project.files.len()
        {
            self.select_file(i);
        } else if let Some(i) = event.route_index::<usize>("fn")
            && i < self.project.fns.len()
        {
            // Following a cross-file callee/caller switches the canvas.
            let file = self.project.fns[i].file;
            if Some(file) != self.selected_file {
                self.select_file(file);
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
            selected_file,
            selected_fn: None,
            pending,
        },
    )
}
