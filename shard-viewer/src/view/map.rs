//! Map view (experimental): the unified canvas we're growing toward — any
//! [`Scope`](crate::scope::Scope)'s fns, grouped by origin file/dir into
//! bounding boxes, each fn drawn in the expanded flow form. The layout becomes
//! a recursive pass over `dir ⊃ file ⊃ fn ⊃ body` (each level sized by its
//! contents, intrinsic — no `est()` estimation), with call usage traced on top.
//!
//! Slice 1 ships only the scaffold: a readout of what the current scope
//! resolves to, so the subject-selection plumbing can be seen end to end. The
//! recursive block layout lands in the next slice; this file is the seam it
//! grows into.

use super::SUB_SIZE;
use crate::model::Project;
use crate::view::ViewParams;
use damascene_core::prelude::*;
use std::collections::BTreeMap;

pub(crate) fn legend() -> El {
    row([
        text("map").mono().muted().font_size(SUB_SIZE),
        text("scaffold — subject selection wired; recursive file/dir layout next")
            .muted()
            .font_size(SUB_SIZE),
    ])
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_2)
}

pub(crate) fn canvas(project: &Project, p: &ViewParams) -> El {
    let fns = p.scope.fns(project);
    if fns.is_empty() {
        return column([
            h3("Map"),
            text("Pick a file or directory in the sidebar to scope the map.").muted(),
        ])
        .gap(tokens::SPACE_3)
        .padding(tokens::SPACE_8);
    }

    // Group the in-scope fns by their origin file (the boxes the real layout
    // will draw); files in turn cluster by directory.
    let mut by_file: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for &fi in &fns {
        by_file.entry(project.fns[fi].file).or_default().push(fi);
    }

    let n_files = by_file.len();
    let summary = text(format!(
        "{} — {} fns across {} {}",
        p.scope.label(project),
        fns.len(),
        n_files,
        if n_files == 1 { "file" } else { "files" }
    ))
    .mono()
    .muted()
    .font_size(SUB_SIZE);

    let mut file_rows: Vec<El> = Vec::new();
    for (file, members) in &by_file {
        let f = &project.files[*file];
        file_rows.push(
            column([
                text(f.rel.clone()).mono().semibold().font_size(SUB_SIZE).nowrap_text().ellipsis(),
                text(
                    members
                        .iter()
                        .map(|&fi| project.fns[fi].name.as_str())
                        .collect::<Vec<_>>()
                        .join("  "),
                )
                .mono()
                .muted()
                .font_size(SUB_SIZE)
                .wrap_text(),
            ])
            .gap(tokens::SPACE_1)
            .padding(tokens::SPACE_3)
            .fill(tokens::CARD)
            .stroke(tokens::BORDER)
            .radius(8.0),
        );
    }

    scroll([column(
        std::iter::once(summary).chain(file_rows).collect::<Vec<_>>(),
    )
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_6)])
    .height(Size::Fill(1.0))
}
