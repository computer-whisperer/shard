//! The intrinsic member cards — fn flow cards, claim cards, type cards, the
//! file-doc card — plus their colors and hover tips. Every card hugs its
//! content: the commit pass measures exactly these constructions, so a card
//! always lands in its committed footprint, and styling (selection, orphan,
//! kind tints) must never change geometry.

use crate::flow::Region;
use crate::model::{ClaimDef, ClaimKind, FnDef, Project, TypeDef, TypeKind};
use crate::view::flow::render_region;
use crate::view::{SUB_SIZE, TITLE_SIZE};
use damascene_core::prelude::*;

/// The proof-layer card colors `(fill, stroke)` by kind — the Map's arm of
/// the project-wide convention (Systems heat): **amber = proof**. Axioms are
/// the loud amber (assumed, not proven — trust roots), plain claims a faint
/// wash, requirements green once fulfilled and red while open.
pub(super) fn claim_colors(kind: ClaimKind, fulfilled: bool) -> (Color, Color) {
    match (kind, fulfilled) {
        (ClaimKind::Axiom, _) => {
            (tokens::CARD.mix(tokens::WARNING, 0.22), tokens::WARNING.mix(tokens::BORDER, 0.35))
        }
        (ClaimKind::Requirement, false) => (
            tokens::CARD.mix(tokens::DESTRUCTIVE, 0.35),
            tokens::DESTRUCTIVE.mix(tokens::FOREGROUND, 0.35),
        ),
        (ClaimKind::Requirement, true) => {
            (tokens::CARD.mix(tokens::SUCCESS, 0.12), tokens::SUCCESS.mix(tokens::BORDER, 0.5))
        }
        (ClaimKind::Claim | ClaimKind::Fulfills, _) => {
            (tokens::CARD.mix(tokens::WARNING, 0.08), tokens::BORDER)
        }
    }
}

/// The kind tag a claim card leads with, and its text tint.
fn claim_tag(c: &ClaimDef) -> (&'static str, Color) {
    match (c.kind, c.fulfilled) {
        (ClaimKind::Axiom, _) => ("axiom", tokens::WARNING),
        (ClaimKind::Claim, _) => ("claim", tokens::WARNING.mix(tokens::MUTED_FOREGROUND, 0.5)),
        (ClaimKind::Requirement, true) => ("req ✓", tokens::SUCCESS),
        (ClaimKind::Requirement, false) => ("req ✗", tokens::DESTRUCTIVE.mix(tokens::FOREGROUND, 0.4)),
        (ClaimKind::Fulfills, _) => ("proof", tokens::WARNING.mix(tokens::MUTED_FOREGROUND, 0.5)),
    }
}

pub(super) fn claim_tip(c: &ClaimDef) -> String {
    let (tag, _) = claim_tag(c);
    let mut tip =
        format!("{tag} {}\n{}\ncites {} · about {} fns", c.name, c.goal, c.cites.len(), c.about.len());
    if !c.doc.is_empty() {
        tip = format!("{tip}\n\n{}", c.doc);
    }
    tip
}

/// One proof-layer form as an intrinsic card: kind tag + name, the goal
/// statement, then the proof's structure in the Flow vocabulary (case-split
/// frames, have facts, step ladders with lemma citations as the bold heroes —
/// see `proof.rs`). Like [`flow_card`] it hugs — the commit pass measures
/// exactly this construction, so it lands in its footprint.
pub(super) fn claim_card(project: &Project, ci: usize) -> El {
    let c = &project.claims[ci];
    let (tag, tag_color) = claim_tag(c);
    let (fill, stroke) = claim_colors(c.kind, c.fulfilled);
    let title = row([
        text(tag).mono().semibold().font_size(SUB_SIZE).text_color(tag_color).nowrap_text(),
        text(c.name.clone())
            .mono()
            .semibold()
            .font_size(TITLE_SIZE)
            .nowrap_text()
            .ellipsis(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);
    let mut parts = vec![title];
    if let Some(summary) = doc_summary(&c.doc, 52) {
        parts.push(summary);
    }
    if !c.goal.is_empty() {
        parts.push(
            text(ellipt(&c.goal, 52)).mono().muted().font_size(SUB_SIZE).nowrap_text(),
        );
    }
    if let Some(region) = crate::proof::build(&c.form) {
        parts.push(render_region(&region));
    }
    column(parts)
        .gap(tokens::SPACE_1)
        .padding(8.0)
        .radius(7.0)
        .fill(fill)
        .stroke(stroke)
        .key(format!("claim:{ci}"))
        .tooltip(claim_tip(c))
}

/// The one-line docstring summary a card carries: the doc's first line,
/// clipped. `None` for an undocumented member — cards don't spend a row on
/// absence. Prose (non-mono) and muted, so it reads as commentary beside the
/// mono code text; the full block lives in the tooltip and detail panel.
fn doc_summary(doc: &str, max: usize) -> Option<El> {
    let line = doc.lines().next()?.trim();
    if line.is_empty() {
        return None;
    }
    Some(text(ellipt(line, max)).muted().font_size(SUB_SIZE).nowrap_text())
}

/// Truncate to `max` chars with an ellipsis (goal statements can be long).
fn ellipt(s: &str, max: usize) -> String {
    if s.chars().count() <= max {
        s.to_string()
    } else {
        let cut: String = s.chars().take(max - 1).collect();
        format!("{cut}…")
    }
}

/// The shape-layer card colors `(fill, stroke)` — the blue family
/// (`tokens::INFO`), keeping amber for proof.
pub(super) fn type_colors() -> (Color, Color) {
    (tokens::CARD.mix(tokens::INFO, 0.10), tokens::INFO.mix(tokens::BORDER, 0.5))
}

/// The kind tag a type card leads with.
fn type_tag(kind: TypeKind) -> &'static str {
    match kind {
        TypeKind::Data => "type",
        TypeKind::Record => "record",
        TypeKind::Opaque => "opaque",
    }
}

pub(super) fn type_tip(t: &TypeDef) -> String {
    let mut tip = format!(
        "{} {}\n{} ctors · composed of {} types",
        type_tag(t.kind),
        t.name,
        t.ctors.len(),
        t.composed.len()
    );
    if !t.doc.is_empty() {
        tip = format!("{tip}\n\n{}", t.doc);
    }
    tip
}

/// A datastructure definition as an intrinsic card: kind tag + name (+ type
/// params), then one row per ctor — ctor name, its field types, and the
/// author's trailing `;` note when the source carries one. A record reads the
/// same with field names as the rows; an opaque `sig type` is just the head
/// (its ctors are the module impl's business). Like [`flow_card`] it hugs —
/// the commit pass measures exactly this construction.
pub(super) fn type_card(project: &Project, ti: usize, keyed: bool) -> El {
    let t = &project.types[ti];
    let (fill, stroke) = type_colors();
    let mut head = t.name.clone();
    if !t.params.is_empty() {
        head = format!("({} {})", t.name, t.params.join(" "));
    }
    let title = row([
        text(type_tag(t.kind))
            .mono()
            .semibold()
            .font_size(SUB_SIZE)
            .text_color(tokens::INFO.mix(tokens::MUTED_FOREGROUND, 0.3))
            .nowrap_text(),
        text(head).mono().semibold().font_size(TITLE_SIZE).nowrap_text().ellipsis(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);

    let mut parts = vec![title];
    if let Some(summary) = doc_summary(&t.doc, 52) {
        parts.push(summary);
    }
    if t.kind == TypeKind::Opaque {
        parts.push(text("ctors private to impl").muted().font_size(SUB_SIZE).nowrap_text());
    }
    for c in &t.ctors {
        let mut cells = vec![
            text(c.name.clone())
                .mono()
                .semibold()
                .font_size(SUB_SIZE)
                .text_color(tokens::FOREGROUND)
                .nowrap_text(),
        ];
        if !c.fields.is_empty() {
            cells.push(
                text(ellipt(&c.fields.join(" "), 46)).mono().muted().font_size(SUB_SIZE).nowrap_text(),
            );
        }
        if !c.comment.is_empty() {
            cells.push(
                text(format!("· {}", ellipt(&c.comment, 40)))
                    .font_size(SUB_SIZE)
                    .text_color(tokens::MUTED_FOREGROUND.mix(tokens::INFO, 0.25))
                    .nowrap_text(),
            );
        }
        parts.push(row(cells).gap(tokens::SPACE_2).align(Align::Center));
    }
    let card = column(parts)
        .gap(tokens::SPACE_1)
        .padding(8.0)
        .radius(7.0)
        .fill(fill)
        .stroke(stroke);
    // The deck draws unkeyed copies of cards the plane may also hold — two
    // Els with one key would collide in hit-test. Unkeyed nodes are never
    // hover targets, so the deck variant also drops the tooltip (it would
    // be dead, and the lint would rightly flag it).
    if keyed { card.key(format!("type:{ti}")).tooltip(type_tip(t)) } else { card }
}

/// The file's `;;;` header as a prose card — the author's account of the
/// module, spending the box's free space on the most informative thing it
/// can hold. Leads with a `;;;` marker (the corpus's own syntax) and the
/// file's basename; the body keeps the author's line breaks (headers are
/// already hand-formatted to a comment column).
pub(super) fn filedoc_card(project: &Project, file: usize) -> El {
    let f = &project.files[file];
    let base = f.rel.rsplit_once('/').map(|(_, b)| b).unwrap_or(&f.rel);
    let title = row([
        text(";;;")
            .mono()
            .semibold()
            .font_size(SUB_SIZE)
            .text_color(tokens::MUTED_FOREGROUND)
            .nowrap_text(),
        text(base.to_string())
            .mono()
            .semibold()
            .font_size(TITLE_SIZE)
            .nowrap_text()
            .ellipsis(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);
    let mut parts = vec![title];
    for line in f.doc.lines() {
        // Keep blank lines as paragraph breaks (an empty text measures away).
        let shown = if line.trim().is_empty() { " ".to_string() } else { ellipt(line, 88) };
        parts.push(text(shown).muted().font_size(SUB_SIZE).nowrap_text());
    }
    column(parts)
        .gap(2.0)
        .padding(8.0)
        .radius(7.0)
        .fill(tokens::BACKGROUND.mix(tokens::CARD, 0.5))
        .stroke(tokens::BORDER.mix(tokens::BACKGROUND, 0.4))
}

/// One fn as an intrinsic flow card: a name/signature header, its named
/// arguments (LabVIEW-style inputs), then its region tree (the same renderer
/// the Flow/Board views use). No fixed size — it hugs; its intrinsic size is
/// what the commit pass measures the footprint from, so `selected` must never
/// change geometry (fill/stroke only).
pub(super) fn flow_card(project: &Project, fn_idx: usize, selected: bool) -> El {
    let f = &project.fns[fn_idx];
    let title = row([
        text(f.name.clone())
            .mono()
            .semibold()
            .font_size(TITLE_SIZE)
            .nowrap_text()
            .ellipsis(),
        text(format!("→ {}", short_ty(&f.ret)))
            .mono()
            .muted()
            .font_size(SUB_SIZE)
            .nowrap_text(),
    ])
    .gap(tokens::SPACE_2)
    .align(Align::Center);

    // The fn's inputs, enumerated: `name  Type`, one per row. A LabVIEW panel
    // reads its wires by their terminals — the signature is first-class, not a
    // count. Omitted for a nullary fn (nothing to list).
    let mut parts = vec![title];
    if let Some(summary) = doc_summary(&f.doc, 56) {
        parts.push(summary);
    }
    if let Some(inputs) = params_block(f) {
        parts.push(inputs);
    }
    parts.push(match body_region(f) {
        Some(region) => render_region(&region),
        None => text("(signature only)").muted().font_size(SUB_SIZE),
    });

    let card = column(parts)
        .gap(tokens::SPACE_2)
        .padding(8.0)
        .radius(7.0)
        .key(format!("fn:{fn_idx}"))
        .tooltip(crate::view::inspector::node_tip(project, fn_idx));
    if selected {
        card.fill(tokens::CARD.mix(tokens::ACCENT, 0.18)).stroke(tokens::RING)
    } else if f.is_orphan() {
        let (fill, stroke) = orphan_colors();
        card.fill(fill).stroke(stroke)
    } else {
        card.fill(tokens::CARD).stroke(tokens::BORDER)
    }
}

/// The triage lens carried over from the old Methods overlay: a fn nothing
/// calls (and no proof reasons about) is a cut candidate, flagged red at any
/// zoom — on the full card and on the distant slab alike. Fill/stroke only:
/// cards are committed-measured, so the lens must never touch geometry.
pub(super) fn orphan_colors() -> (Color, Color) {
    (tokens::CARD.mix(tokens::DESTRUCTIVE, 0.30), tokens::DESTRUCTIVE.mix(tokens::BORDER, 0.35))
}

/// The fn's parameters as a small column of `name  Type` rows, or `None` for a
/// nullary fn. Names read in the foreground, types muted — the terminals of the
/// card. Types are trimmed like the return ([`short_ty`]); the tooltip carries
/// the untrimmed signature.
fn params_block(f: &FnDef) -> Option<El> {
    if f.params.is_empty() {
        return None;
    }
    let rows: Vec<El> = f
        .params
        .iter()
        .map(|(name, ty)| {
            row([
                text(name.clone()).mono().font_size(SUB_SIZE).nowrap_text(),
                text(short_ty(ty)).mono().muted().font_size(SUB_SIZE).nowrap_text(),
            ])
            .gap(tokens::SPACE_2)
            .align(Align::Center)
        })
        .collect();
    Some(column(rows).gap(2.0).padding(tokens::SPACE_1))
}

/// The region tree for a fn's body, or `None` for a bodyless `sig` / a
/// measure-only form (annotation, no logic to chart).
fn body_region(f: &FnDef) -> Option<Region> {
    if f.body.is_empty() || f.body.iter().all(|form| form.head() == Some("measure")) {
        None
    } else {
        Some(Region::build(&f.body))
    }
}

/// Trim a return type for the card header (mirrors Board).
fn short_ty(ty: &str) -> String {
    const MAX: usize = 22;
    if ty.chars().count() > MAX {
        let mut s: String = ty.chars().take(MAX - 1).collect();
        s.push('…');
        s
    } else {
        ty.to_string()
    }
}
