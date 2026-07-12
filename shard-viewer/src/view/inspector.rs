//! The inspector panels: the per-fn detail panel (source, docstring, call
//! lists, the source lightbox) and the per-file breakdown panel (counts,
//! composition, header doc, imports) — the [`super::Sel`]-dispatched right
//! pane over the Map, plus the hover tips the Map's cards share.

use super::shared::{composition_bar, swatch};
use super::SUB_SIZE;
use crate::model::Project;
use damascene_core::prelude::*;

/// Hover text for a fn card/slab: full signature, home file, triage metrics.
pub(crate) fn node_tip(project: &Project, fn_idx: usize) -> String {
    let f = &project.fns[fn_idx];
    let ps: Vec<String> = f.params.iter().map(|(n, t)| format!("({n} {t})")).collect();
    let kw = if f.is_sig { "sig " } else { "" };
    let mut tip = format!(
        "{kw}{}({}) → {}\n{}\n{} lines · {} calls · {} callers",
        f.name,
        ps.join(" "),
        f.ret,
        project.files[f.file].rel,
        f.src_lines(),
        f.calls.len(),
        f.callers.len()
    );
    if f.is_orphan() {
        tip.push_str("\n⚠ orphan — nothing calls it (cut candidate)");
    } else if f.proof_refd && f.callers.is_empty() {
        tip.push_str("\nproof subject — reasoned about, not called");
    }
    if !f.doc.is_empty() {
        tip.push_str("\n\n");
        tip.push_str(&f.doc);
    }
    tip
}

/// Navigation for the selected fn. "Card ▸" is scope-as-camera: fly to the
/// fn's committed flow card (large enough to read — the successor of the old
/// standalone Flow view), falling back to scoping the Map to the fn's file
/// when the card isn't on the current plane. "Tree ▸" is a scope change: map
/// the fn's call neighborhood (callers + transitive callees) around it.
fn nav_buttons() -> El {
    row([
        button("Card ▸")
            .key("goto_card")
            .secondary()
            .tooltip("Fly the Map to this fn's flow card"),
        button("Tree ▸")
            .key("scope_tree")
            .ghost()
            .tooltip("Map this fn's call neighborhood (callers + callees)"),
    ])
    .gap(tokens::SPACE_2)
}

pub(crate) fn detail_panel(project: &Project, fn_idx: usize, panel_w: f32) -> El {
    let f = &project.fns[fn_idx];
    let sig: Vec<String> = f.params.iter().map(|(n, t)| format!("({n} {t})")).collect();

    // Callees (within project) and callers (reverse edges, precomputed).
    let callees = &f.calls;
    let callers = &f.callers;

    // Triage metrics + a cut-candidate / proof-subject tag.
    let mut metrics = format!(
        "{} lines · {} calls · {} callers",
        f.src_lines(),
        f.calls.len(),
        f.callers.len()
    );
    if f.is_orphan() {
        metrics.push_str("  ·  ⚠ orphan — cut candidate");
    } else if f.proof_refd && f.callers.is_empty() {
        metrics.push_str("  ·  proof subject");
    }

    // Fixed header — identity + triage + nav stay put while the body scrolls.
    let mut header = vec![
        row([h3(f.name.clone()), spacer()]).gap(tokens::SPACE_2),
        text(format!("({}) → {}", sig.join(" "), f.ret))
            .mono()
            .muted()
            .font_size(tokens::TEXT_SM.size)
            .wrap_text(),
        text(format!("in {}", project.files[f.file].rel))
            .caption()
            .muted(),
        text(metrics).caption().muted(),
    ];
    // The full docstring — the author's own words outrank any derived metric,
    // so it sits right under the identity block.
    if !f.doc.is_empty() {
        header.push(text(f.doc.clone()).font_size(tokens::TEXT_SM.size).wrap_text());
    }
    header.push(nav_buttons());

    let src_header = {
        let mut head = vec![
            text("Source").label(),
            spacer(),
            text(format!("{} lines", f.src_lines())).caption().muted(),
        ];
        // The panel is a fixed width and shares space with the call lists, so a
        // wide/long body can be unreadable here — the lightbox opens it larger.
        if !f.src.is_empty() {
            head.push(
                button("Expand ⤢")
                    .key("src_expand")
                    .ghost()
                    .tooltip("Open the source in a larger view (click outside to close)"),
            );
        }
        row(head).gap(tokens::SPACE_2).align(Align::Center)
    };

    let source = if f.src.is_empty() {
        code_block("(signature only)")
    } else {
        // Syntax-highlighted + line-numbered, manually wrapped to a column
        // budget (the source is monospace, so a character count is an exact
        // width). `panel_w` is the live (possibly user-dragged) panel width, so
        // widening the panel re-wraps to fill it. Natural height — the body
        // scroll below owns the scrolling.
        let max_chars = source_budget(panel_w, tokens::SPACE_3);
        super::highlight::source_view(&f.src, max_chars)
    };

    // One scroll over source + call lists. Previously the source had its own
    // Fill scroll and the call lists sat un-scrolled beneath it, so a fn with
    // many calls/callers (e.g. driver.shard::run_decls) overflowed the panel's
    // bottom edge. Scrolling the whole lower section together fixes that.
    let body = scroll([column([
        src_header,
        source,
        separator(),
        text(format!("Calls ({})", callees.len())).label(),
        fn_link_list(project, callees, f.file),
        text(format!("Called by ({})", callers.len())).label(),
        fn_link_list(project, callers, f.file),
    ])
    .gap(tokens::SPACE_2)])
    .height(Size::Fill(1.0));

    let mut items = header;
    items.push(separator());
    items.push(body);

    column(items)
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .width(Size::Fixed(super::DEFAULT_PANEL_W))
        .height(Size::Fill(1.0))
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .radius(10.0)
        // Drag the left seam to widen the panel when the source is wide (the
        // body re-wraps to the new width via `panel_w` above). It's the last
        // row child, so the runtime anchors the grab band on its left edge.
        .key(super::PANEL_KEY)
        .user_resizable()
        .min_width(320.0)
        .max_width(820.0)
}

/// The character budget for the manually-wrapped source view inside a panel of
/// the given outer width and horizontal padding. Subtracts the panel padding
/// (both sides), the code-block chrome padding (2×`SPACE_3`), the line-number
/// gutter (~6 mono cols), and a scrollbar gutter, then divides by the mono glyph
/// advance. Shared by the detail panel and the lightbox so both wrap correctly.
fn source_budget(panel_w: f32, panel_pad: f32) -> usize {
    const MONO_CH: f32 = 7.8; // JetBrains Mono advance at TEXT_SM
    let avail = panel_w - 2.0 * panel_pad - 2.0 * tokens::SPACE_3 - 6.0 * MONO_CH - 12.0;
    (avail / MONO_CH).floor().max(8.0) as usize
}

/// The source lightbox: a large centered modal showing the selected fn's full
/// source, syntax-highlighted and line-numbered, wrapped to the modal's (much
/// wider) width. The fixed-width detail panel can't show a wide/long body
/// (run_decls is the motivating case); this is the "read it properly" escape
/// hatch. A dismiss scrim and a Close button both route to closing it.
pub(crate) fn source_modal(project: &Project, fn_idx: usize) -> El {
    const MODAL_W: f32 = 980.0;
    const MODAL_H: f32 = 720.0;
    let f = &project.fns[fn_idx];

    let body = if f.src.is_empty() {
        code_block("(signature only)")
    } else {
        let max_chars = source_budget(MODAL_W, tokens::SPACE_4);
        scroll([super::highlight::source_view(&f.src, max_chars)]).height(Size::Fill(1.0))
    };

    // The full signature is the first lines of the body itself, so the header
    // only needs the home file + length (a short line that won't clip).
    let meta = row([
        text(format!("{} · {} lines", project.files[f.file].rel, f.src_lines()))
            .mono()
            .muted()
            .font_size(tokens::TEXT_SM.size)
            .nowrap_text()
            .ellipsis(),
        spacer(),
        button("Close").key("src_close").secondary(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);

    let panel = modal_panel(f.name.clone(), [meta, body])
        .width(Size::Fixed(MODAL_W))
        .height(Size::Fixed(MODAL_H))
        .block_pointer();
    overlay([scrim("src_modal:dismiss"), panel])
}

/// A list of clickable fn links (jump targets for navigation). Cross-file
/// targets (file `!= home`) are disambiguated with their file stem, since
/// homonyms across files are common in shard and a bare name would be
/// ambiguous; the full path is on hover.
fn fn_link_list(project: &Project, fns: &[usize], home: usize) -> El {
    if fns.is_empty() {
        return text("—").muted().font_size(tokens::TEXT_SM.size);
    }
    let chips: Vec<El> = fns
        .iter()
        .map(|&j| {
            let g = &project.fns[j];
            let rel = &project.files[g.file].rel;
            let label = if g.file == home {
                g.name.clone()
            } else {
                format!("{}  · {}", g.name, file_stem(rel))
            };
            button(label)
                .key(format!("fn:{j}"))
                .ghost()
                .tooltip(format!("in {rel}"))
        })
        .collect();
    column(chips).gap(2.0)
}

/// The bare file name (no dir, no `.shard`) — a compact cross-file tag.
fn file_stem(rel: &str) -> &str {
    let file = rel.rsplit('/').next().unwrap_or(rel);
    file.strip_suffix(".shard").unwrap_or(file)
}

/// The file inspector: the selected file's line-category breakdown, its
/// `;;;` header, and its import in/out degree, with a button to scope the
/// Map to it. (Grew up in the Systems view; now the `Sel::File` panel.)
pub(crate) fn file_panel(project: &Project, file_idx: usize) -> El {
    let f = &project.files[file_idx];
    let c = &f.counts;
    let imported_by = project
        .files
        .iter()
        .filter(|g| g.import_targets.contains(&file_idx))
        .count();

    // One labelled, swatched, right-aligned count row.
    let cat_row = |label: &str, n: u32, color: Color| -> El {
        row([
            swatch(color, 12.0),
            text(label.to_string()).font_size(SUB_SIZE),
            spacer(),
            text(n.to_string()).mono().muted().font_size(SUB_SIZE),
        ])
        .gap(tokens::SPACE_2)
    };

    let mut items = vec![
        row([h3(file_stem(&f.rel).to_string()), spacer()]).gap(tokens::SPACE_2),
        text(f.rel.clone()).caption().muted(),
    ];
    // The file's `;;;` header block — the author's own account of the file.
    if !f.doc.is_empty() {
        items.push(text(f.doc.clone()).font_size(SUB_SIZE).wrap_text());
    }
    items.extend(vec![
        button("Map this file ▸")
            .key(format!("open:{file_idx}"))
            .secondary()
            .tooltip("Scope the Map to just this file"),
        separator(),
        text(format!("{} lines · {} fns", c.total(), f.fns.len()))
            .caption()
            .muted(),
        composition_bar(c, 384.0, 6.0),
        separator(),
        cat_row("impl", c.impl_, tokens::ACCENT),
        cat_row("measure", c.measure, tokens::WARNING),
        cat_row("proof", c.proof, tokens::WARNING),
        cat_row("reqproof", c.reqproof, tokens::WARNING),
        cat_row("req", c.req, tokens::ACCENT),
        cat_row("sidecar", c.sidecar, tokens::WARNING),
        cat_row("comment", c.comment, tokens::BORDER),
        cat_row("blank", c.blank, tokens::BORDER),
        separator(),
        text(format!(
            "imports {} · imported by {imported_by}",
            f.import_targets.len()
        ))
        .caption()
        .muted(),
    ]);

    column(items)
        .gap(tokens::SPACE_2)
        .padding(tokens::SPACE_3)
        .width(Size::Fixed(420.0))
        .height(Size::Fill(1.0))
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .radius(10.0)
}
