//! `shard-viewer` — the graphical navigator.
//!
//! Two-pane workbench: a sidebar listing every `.shard` file in the project,
//! and a canvas drawing the selected file's fns as boxes with their intra-file
//! call edges as arrows. A fn box selects it and opens a detail panel (source +
//! clickable callers/callees). The canvas pans by dragging an empty area and
//! zooms with the wheel or the toolbar buttons.
//!
//!   shard-viewer [PROJECT_ROOT]   (defaults to the current directory)

use damascene_core::prelude::*;
use shard_viewer::model::Project;
use shard_viewer::view::{self, CANVAS_KEY, ViewParams};

const ZOOM_MIN: f32 = 0.2;
const ZOOM_MAX: f32 = 3.0;

struct Drag {
    start_pointer: (f32, f32),
    start_pan: (f32, f32),
}

struct Viewer {
    project: Project,
    selected_file: Option<usize>,
    selected_fn: Option<usize>,
    zoom: f32,
    pan: (f32, f32),
    drag: Option<Drag>,
}

impl Viewer {
    fn params(&self) -> ViewParams {
        ViewParams {
            selected_file: self.selected_file,
            selected_fn: self.selected_fn,
            zoom: self.zoom,
            pan: self.pan,
        }
    }

    fn zoom_by(&mut self, factor: f32) {
        self.zoom = (self.zoom * factor).clamp(ZOOM_MIN, ZOOM_MAX);
    }

    /// Switching files resets the view so the new graph starts framed.
    fn select_file(&mut self, i: usize) {
        self.selected_file = Some(i);
        self.selected_fn = None;
        self.pan = (0.0, 0.0);
        self.zoom = 1.0;
    }
}

impl App for Viewer {
    fn build(&self, _cx: &BuildCx) -> El {
        view::app_root(&self.project, &self.params())
    }

    fn on_event(&mut self, event: UiEvent, _cx: &EventCx) {
        match event.kind {
            UiEventKind::Click | UiEventKind::Activate => {
                if event.is_route("zoom_in") {
                    self.zoom_by(1.25);
                } else if event.is_route("zoom_out") {
                    self.zoom_by(0.8);
                } else if event.is_route("zoom_reset") {
                    self.pan = (0.0, 0.0);
                    self.zoom = 1.0;
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
            UiEventKind::PointerDown if event.is_route(CANVAS_KEY) => {
                if let Some(pos) = event.pointer_pos() {
                    self.drag = Some(Drag {
                        start_pointer: pos,
                        start_pan: self.pan,
                    });
                }
            }
            UiEventKind::Drag => {
                if let (Some(pos), Some(d)) = (event.pointer_pos(), self.drag.as_ref()) {
                    self.pan = (
                        d.start_pan.0 + pos.0 - d.start_pointer.0,
                        d.start_pan.1 + pos.1 - d.start_pointer.1,
                    );
                }
            }
            UiEventKind::PointerUp => self.drag = None,
            _ => {}
        }
    }

    fn on_wheel_event(&mut self, event: UiEvent, _cx: &EventCx) -> bool {
        // Wheel over the graph zooms; elsewhere (the sidebar) fall through to
        // damascene's default scroll handling.
        let over_graph = event
            .route()
            .is_some_and(|r| r == CANVAS_KEY || r.starts_with("fn:"));
        if over_graph {
            if let Some(dy) = event.wheel_dy() {
                // dy > 0 is scroll-down (damascene wheel convention) → zoom out.
                self.zoom_by(if dy > 0.0 { 1.0 / 1.1 } else { 1.1 });
            }
            return true;
        }
        false
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
            zoom: 1.0,
            pan: (0.0, 0.0),
            drag: None,
        },
    )
}
