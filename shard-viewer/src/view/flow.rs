//! Flow view: one fn body as a structured (LabVIEW-style) diagram. Control
//! structures are FRAMES that contain their branches; leaf computations are op
//! cards WIRED to their operands. Rendered as nested elements — containment,
//! sizing, and text wrapping fall out of the element layout — inside the
//! pan/zoom viewport. (The Sugiyama engine stays for the call/import graphs.)
//!
//! [`render_region`] is the reusable fn-card renderer: the Board view drops it
//! into each node to show every fn in this same expanded form.

use super::shared::{legend_chip, pan_zoom_viewport};
use super::SUB_SIZE;
use crate::flow::{Branch, FrameKind, Region};
use crate::model::Project;
use damascene_core::prelude::*;

/// The Flow-view key.
pub(crate) fn legend() -> El {
    row([
        text("flow").mono().muted().font_size(SUB_SIZE),
        legend_chip(tokens::INFO, "match / if"),
        legend_chip(tokens::SUCCESS, "let"),
        legend_chip(tokens::CARD, "op"),
        legend_chip(tokens::CARD.mix(tokens::WARNING, 0.32), "var"),
        legend_chip(tokens::BACKGROUND, "literal"),
        text("·  frames contain · operands wired in →")
            .mono()
            .muted()
            .font_size(SUB_SIZE),
    ])
    .gap(tokens::SPACE_3)
    .padding(tokens::SPACE_2)
}

pub(crate) fn canvas(project: &Project, fn_idx: usize) -> El {
    let f = &project.fns[fn_idx];
    if f.body.iter().all(|form| form.head() == Some("measure")) {
        return column([text("This fn has no body to chart (a signature).").muted()])
            .padding(tokens::SPACE_8);
    }
    let region = Region::build(&f.body);
    // The nested-element tree hugs its own content; a little padding keeps it
    // off the viewport edges. The viewport frames it (Fit / pan / zoom).
    let content = row([render_region(&region)]).padding(tokens::SPACE_6);
    pan_zoom_viewport(content)
}

/// Render a region as a nested element. Frames contain labelled branches; op
/// cards wire to their operand sub-regions; vars/lits are leaf pills/tags.
///
/// Reused by the Board view, which renders every fn in this same form.
pub(crate) fn render_region(r: &Region) -> El {
    match r {
        Region::Frame { kind, detail, branches } => render_frame(*kind, detail, branches),
        Region::List { elems, tail } => render_list(elems, tail.as_deref()),
        Region::Op { head, inline, args } => render_op(head, inline, args),
        Region::Var(name) => var_pill(name),
        Region::Lit(value) => lit_tag(value),
    }
}

/// A collapsed `Cons` spine: a bracketed column of element regions (data
/// construction, distinct from the control frames and op cards). A non-`Nil`
/// terminator is shown as a trailing `⋯ tail` row (cons-onto-an-existing-list).
fn render_list(elems: &[Region], tail: Option<&Region>) -> El {
    let count = elems.len();
    let mut rows: Vec<El> = elems.iter().map(render_region).collect();
    if let Some(t) = tail {
        rows.push(
            row([
                text("⋯").mono().muted().font_size(12.0).nowrap_text(),
                render_region(t),
            ])
            .gap(tokens::SPACE_2)
            .align(Align::Center),
        );
    }
    // An empty closed list is just `[]`; render it as a literal-style tag.
    if rows.is_empty() {
        return lit_tag("[]");
    }
    let header = text(format!("list · {count}"))
        .mono()
        .muted()
        .font_size(10.0)
        .nowrap_text();
    let body = column(rows).gap(5.0).padding(6.0);
    // A bracket bar down the left edge marks the elements as one ordered list.
    column([header, row([bracket_bar(), body]).align(Align::Stretch)])
        .gap(3.0)
        .padding(6.0)
        .fill(tokens::CARD)
        .stroke(tokens::MUTED)
        .radius(7.0)
}

/// The thin vertical rule down the left of a list, tying its elements together.
fn bracket_bar() -> El {
    column(Vec::<El>::new())
        .width(Size::Fixed(3.0))
        .height(Size::Fill(1.0))
        .fill(tokens::MUTED)
        .radius(2.0)
}

/// A control structure: a colored keyword band over a body that *contains* its
/// branches, each headed by its selector chip. Nesting = box enclosure.
fn render_frame(kind: FrameKind, detail: &str, branches: &[Branch]) -> El {
    let (accent, fg) = match kind {
        FrameKind::Match | FrameKind::If => (tokens::INFO, tokens::INFO_FOREGROUND),
        FrameKind::Let => (tokens::SUCCESS, tokens::SUCCESS_FOREGROUND),
    };
    let mut band_kids = vec![
        text(kind.keyword().to_string())
            .mono()
            .semibold()
            .font_size(12.0)
            .text_color(fg)
            .nowrap_text(),
    ];
    if !detail.is_empty() {
        band_kids.push(
            text(detail.to_string())
                .mono()
                .font_size(11.0)
                .text_color(fg)
                .nowrap_text()
                .ellipsis(),
        );
    }
    let band = row(band_kids)
        .gap(tokens::SPACE_2)
        .padding(5.0)
        .width(Size::Fill(1.0))
        .fill(accent);

    let body = column(branches.iter().map(render_branch).collect::<Vec<_>>())
        .gap(7.0)
        .padding(8.0);

    column([band, body])
        .fill(tokens::CARD)
        .stroke(accent)
        .radius(7.0)
}

/// One labelled branch inside a frame: its selector chip + the contained region.
fn render_branch(b: &Branch) -> El {
    // Top-align: the chip sits beside its region's header, not floating at the
    // vertical centre of a tall nested frame.
    row([selector_chip(&b.label), render_region(&b.region)])
        .gap(tokens::SPACE_2)
        .align(Align::Start)
}

/// The selector pill that heads a branch (arm pattern / `then`/`else` / binding
/// name). Blue ties it to the control vocabulary; it sits left of its region.
fn selector_chip(label: &str) -> El {
    row([text(label.to_string())
        .mono()
        .semibold()
        .font_size(10.0)
        .text_color(tokens::INFO_FOREGROUND)
        .nowrap_text()
        .ellipsis()])
    .padding(3.0)
    .radius(5.0)
    .fill(tokens::INFO)
}

/// A function application: the op card, with any compound operands gathered on a
/// connector to its left (data flows left→right into the op, LabVIEW-style).
///
/// Compound operands stack in a column whose right edge is a full-height
/// **gather bar**; a single arrow runs from the bar into the card. (The earlier
/// per-operand stub arrows pointed into empty space — only the vertically
/// centered operand ever lined up with the card; the bar always spans them.)
fn render_op(head: &str, inline: &str, args: &[Region]) -> El {
    let card = op_card(head, inline);
    if args.is_empty() {
        return card;
    }
    let inputs = column(args.iter().map(render_region).collect::<Vec<_>>()).gap(6.0);
    // The bar spans the full operand-column height (Stretch), so every operand
    // visibly belongs to the same gather, regardless of how tall the column is.
    let gathered = row([inputs, gather_bar()]).gap(tokens::SPACE_2).align(Align::Stretch);
    row([gathered, feed_arrow(), card]).gap(tokens::SPACE_1).align(Align::Center)
}

/// The full-height vertical rule down the right of an op's operand column: it
/// gathers every operand into one bus feeding the op.
fn gather_bar() -> El {
    column(Vec::<El>::new())
        .width(Size::Fixed(2.5))
        .height(Size::Fill(1.0))
        .fill(tokens::INFO)
        .radius(2.0)
}

/// The single arrow from the gather bar into the op card.
fn feed_arrow() -> El {
    let line = PathBuilder::new()
        .move_to(0.0, 6.0)
        .line_to(11.0, 6.0)
        .stroke_solid(tokens::INFO, 1.6)
        .build();
    let head = super::shared::arrowhead(4.0, 6.0, 17.0, 6.0, tokens::INFO);
    vector(VectorAsset::from_paths([0.0, 0.0, 18.0, 12.0], vec![line, head]))
        .width(Size::Fixed(18.0))
        .height(Size::Fixed(12.0))
}

/// The op card itself: the function name as a bold hero, inline simple operands
/// a quiet second line.
fn op_card(head: &str, inline: &str) -> El {
    let mut kids = vec![
        text(head.to_string())
            .mono()
            .semibold()
            .font_size(13.0)
            .text_color(tokens::FOREGROUND)
            .nowrap_text()
            .ellipsis(),
    ];
    if !inline.is_empty() {
        kids.push(
            text(inline.to_string())
                .mono()
                .muted()
                .font_size(11.0)
                .nowrap_text()
                .ellipsis(),
        );
    }
    column(kids)
        .gap(1.0)
        .padding(6.0)
        .fill(tokens::CARD)
        .stroke(tokens::BORDER)
        .radius(6.0)
}

/// A variable reference: a small warm pill (a data input).
fn var_pill(name: &str) -> El {
    column([text(name.to_string())
        .mono()
        .semibold()
        .font_size(12.0)
        .text_color(tokens::FOREGROUND)
        .center_text()
        .nowrap_text()])
    .padding(5.0)
    .fill(tokens::CARD.mix(tokens::WARNING, 0.32))
    .stroke(tokens::WARNING)
    .radius(13.0)
}

/// A literal: a dim mono tag (a constant).
fn lit_tag(value: &str) -> El {
    let show = if value.is_empty() { "·".to_string() } else { value.to_string() };
    column([text(show)
        .mono()
        .muted()
        .font_size(11.0)
        .center_text()
        .nowrap_text()])
    .padding(4.0)
    .fill(tokens::BACKGROUND)
    .stroke(tokens::BORDER)
    .radius(4.0)
}

