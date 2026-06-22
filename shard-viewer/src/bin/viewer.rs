//! `shard-viewer` — the graphical navigator.
//!
//! Two-pane workbench: a sidebar listing every `.shard` file in the project,
//! and a canvas drawing the selected file's fns as boxes with their intra-file
//! call edges as arrows. Click a fn box to highlight it; click a file to switch
//! the canvas. The view tree itself lives in `shard_viewer::view`.
//!
//!   shard-viewer [PROJECT_ROOT]   (defaults to the current directory)

use damascene_core::prelude::*;
use shard_viewer::model::Project;
use shard_viewer::view;

struct Viewer {
    project: Project,
    selected_file: Option<usize>,
    selected_fn: Option<usize>,
}

impl App for Viewer {
    fn build(&self, _cx: &BuildCx) -> El {
        view::app_root(&self.project, self.selected_file, self.selected_fn)
    }

    fn on_event(&mut self, event: UiEvent, _cx: &EventCx) {
        if !matches!(event.kind, UiEventKind::Click | UiEventKind::Activate) {
            return;
        }
        if let Some(i) = event.route_index::<usize>("file")
            && i < self.project.files.len()
        {
            self.selected_file = Some(i);
            self.selected_fn = None;
        } else if let Some(i) = event.route_index::<usize>("fn")
            && i < self.project.fns.len()
        {
            self.selected_fn = Some(i);
            // Following a cross-file callee also switches the canvas.
            self.selected_file = Some(self.project.fns[i].file);
        }
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

    let viewport = Rect::new(0.0, 0.0, 1280.0, 800.0);
    damascene_winit_wgpu::run(
        "shard-viewer",
        viewport,
        Viewer {
            project,
            selected_file,
            selected_fn: None,
        },
    )
}
