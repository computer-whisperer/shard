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
        Region::Seq(steps) => render_seq(steps),
        Region::Var(name) => var_pill(name),
        Region::Lit(value) => lit_tag(value),
    }
}

/// The width/height a reshaped block aims for. Proof spines are almost pure
/// sequence-and-fork; rendered strictly vertically they come out as receipts
/// many screens tall, so both the step ladder and the case fork RESHAPE
/// toward this aspect (galley columns / branch shelves, below).
const TARGET_ASPECT: f32 = 1.7;
/// Ladders shorter than this stay a single column (wrapping a short proof
/// only adds reading friction).
const WRAP_MIN_H: f32 = 260.0;

/// An ordered sequence (a proof's step ladder): amber-railed rungs read
/// top→bottom. A ladder taller than [`WRAP_MIN_H`] wraps into balanced
/// newspaper columns (read down, then next column — the `↳` marker heads
/// each continuation), so long spines fill a block instead of a receipt.
fn render_seq(steps: &[Region]) -> El {
    if steps.is_empty() {
        return lit_tag("·");
    }
    let ladders: Vec<El> = seq_columns(steps)
        .into_iter()
        .enumerate()
        .map(|(i, chunk)| {
            let rows = column(chunk.iter().map(render_region).collect::<Vec<_>>()).gap(4.0);
            let ladder = row([seq_bar(), rows]).gap(tokens::SPACE_2).align(Align::Stretch);
            if i == 0 {
                ladder
            } else {
                column([cont_marker(), ladder]).gap(2.0)
            }
        })
        .collect();
    if ladders.len() == 1 {
        ladders.into_iter().next().expect("len 1")
    } else {
        row(ladders).gap(tokens::SPACE_3).align(Align::Start)
    }
}

/// Partition a vertical run of regions (ladder rungs, list elements) into
/// contiguous galley columns: pick the column count that lands the block
/// nearest [`TARGET_ASPECT`], then break greedily at item boundaries so
/// columns balance. `gap` is the view's inter-item spacing, `col_extra_w` the
/// per-column chrome (rail/bracket + gaps). Pure in the region tree (sizes
/// come from [`est`]) — safe under the Map's committed-topology rule.
fn galley_columns(items: &[Region], gap: f32, col_extra_w: f32) -> Vec<&[Region]> {
    let sizes: Vec<(f32, f32)> = items.iter().map(est).collect();
    galley_partition(&sizes, gap, col_extra_w)
        .into_iter()
        .map(|r| &items[r])
        .collect()
}

/// The partition itself, on pre-measured sizes — [`est`] goes through this
/// directly (measuring children exactly once; routing it through
/// [`galley_columns`] would re-measure per level, exponential in depth).
fn galley_partition(
    sizes: &[(f32, f32)],
    gap: f32,
    col_extra_w: f32,
) -> Vec<std::ops::Range<usize>> {
    let total: f32 =
        sizes.iter().map(|s| s.1).sum::<f32>() + gap * sizes.len().saturating_sub(1) as f32;
    let w = sizes.iter().map(|s| s.0).fold(40.0_f32, f32::max) + col_extra_w;
    let k = if total < WRAP_MIN_H {
        1
    } else {
        ((TARGET_ASPECT * total / w).sqrt().round() as usize).clamp(1, 8)
    };
    if k <= 1 {
        return std::iter::once(0..sizes.len()).collect();
    }
    let target = total / k as f32;
    let mut cols = Vec::new();
    let mut start = 0;
    let mut acc = 0.0;
    for (i, (_, h)) in sizes.iter().enumerate() {
        if acc > 0.0 && acc + h > target && cols.len() + 1 < k {
            cols.push(start..i);
            start = i;
            acc = 0.0;
        }
        acc += h + gap;
    }
    cols.push(start..sizes.len());
    cols
}

/// Ladder-flavoured galley: rail chrome width, rung gap.
fn seq_columns(steps: &[Region]) -> Vec<&[Region]> {
    galley_columns(steps, 4.0, 13.5)
}

/// List-flavoured galley: bracket-bar chrome width, element gap.
fn list_columns(elems: &[Region]) -> Vec<&[Region]> {
    galley_columns(elems, 5.0, 15.0)
}

/// The reading-order cue heading each continuation column of a wrapped ladder.
fn cont_marker() -> El {
    text("↳")
        .mono()
        .font_size(11.0)
        .text_color(tokens::WARNING.mix(tokens::MUTED_FOREGROUND, 0.35))
        .nowrap_text()
}

/// The thin vertical rule tying a step ladder's rungs into one sequence.
fn seq_bar() -> El {
    column(Vec::<El>::new())
        .width(Size::Fixed(2.5))
        .height(Size::Fill(1.0))
        .fill(tokens::WARNING.mix(tokens::MUTED, 0.5))
        .radius(2.0)
}

/// A collapsed `Cons` spine: a bracketed column of element regions (data
/// construction, distinct from the control frames and op cards). A non-`Nil`
/// terminator is shown as a trailing `⋯ tail` row (cons-onto-an-existing-list).
fn render_list(elems: &[Region], tail: Option<&Region>) -> El {
    let count = elems.len();
    let mut tail_row = tail.map(|t| {
        row([
            text("⋯").mono().muted().font_size(12.0).nowrap_text(),
            render_region(t),
        ])
        .gap(tokens::SPACE_2)
        .align(Align::Center)
    });
    // An empty closed list is just `[]`; render it as a literal-style tag.
    if elems.is_empty() && tail_row.is_none() {
        return lit_tag("[]");
    }
    // Long element runs wrap into galley columns like the proof ladders do
    // (a wasm emit list is hundreds of instructions — the same receipt shape);
    // each column keeps its bracket bar, the tail row closes the last one.
    let cols = list_columns(elems);
    let last = cols.len() - 1;
    let brackets: Vec<El> = cols
        .into_iter()
        .enumerate()
        .map(|(i, chunk)| {
            let mut rows: Vec<El> = chunk.iter().map(render_region).collect();
            if i == last
                && let Some(t) = tail_row.take()
            {
                rows.push(t);
            }
            let body = column(rows).gap(5.0).padding(6.0);
            let bracket = row([bracket_bar(), body]).align(Align::Stretch);
            if i == 0 {
                bracket
            } else {
                column([cont_marker(), bracket]).gap(2.0)
            }
        })
        .collect();
    let body = if brackets.len() == 1 {
        brackets.into_iter().next().expect("len 1")
    } else {
        row(brackets).gap(tokens::SPACE_3).align(Align::Start)
    };
    let header = text(format!("list · {count}"))
        .mono()
        .muted()
        .font_size(10.0)
        .nowrap_text();
    column([header, body])
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
        // Branching (a proof case split IS a match — same band, same read).
        FrameKind::Match
        | FrameKind::If
        | FrameKind::Induct
        | FrameKind::FinSplit
        | FrameKind::CaseOn
        | FrameKind::WfInduct
        | FrameKind::SubtermInduct => (tokens::INFO, tokens::INFO_FOREGROUND),
        // Binding (`have` binds facts the way `let` binds values).
        FrameKind::Let | FrameKind::Have => (tokens::SUCCESS, tokens::SUCCESS_FOREGROUND),
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

    // Proof case splits fork into PARALLEL subproofs — they read side by side
    // (chip above its case), shelf-wrapped toward the target aspect. Fn-body
    // match/if arms keep the vertical read (arm order is the code's order and
    // arms share flow into one result), as do `let`/`have` binding stacks.
    let body = if forks(kind) && branches.len() > 1 {
        let shelves: Vec<El> = branch_shelves(branches)
            .into_iter()
            .map(|shelf| {
                row(shelf.iter().map(branch_column).collect::<Vec<_>>())
                    .gap(tokens::SPACE_3)
                    .align(Align::Start)
            })
            .collect();
        column(shelves).gap(9.0).padding(8.0)
    } else {
        column(branches.iter().map(render_branch).collect::<Vec<_>>()).gap(7.0).padding(8.0)
    };

    column([band, body])
        .fill(tokens::CARD)
        .stroke(accent)
        .radius(7.0)
}

/// The frame kinds whose branches are parallel alternatives (a case analysis),
/// eligible for the side-by-side fork layout.
fn forks(kind: FrameKind) -> bool {
    matches!(
        kind,
        FrameKind::Induct | FrameKind::FinSplit | FrameKind::CaseOn | FrameKind::SubtermInduct
    )
}

/// Partition fork branches into shelves (rows of side-by-side cases): pick the
/// shelf count landing nearest [`TARGET_ASPECT`], fill greedily in case order.
fn branch_shelves(branches: &[Branch]) -> Vec<&[Branch]> {
    let sizes: Vec<(f32, f32)> = branches.iter().map(branch_column_est).collect();
    shelf_partition(&sizes).into_iter().map(|r| &branches[r]).collect()
}

/// Shelf partition on pre-measured case sizes (same once-only-measure rule as
/// [`galley_partition`]).
fn shelf_partition(sizes: &[(f32, f32)]) -> Vec<std::ops::Range<usize>> {
    if sizes.is_empty() {
        // Callers guard len > 1, but `clamp(1, 0)` below would panic — match
        // galley_partition's one-empty-range behavior instead of trapping.
        return std::iter::once(0..0).collect();
    }
    let total: f32 =
        sizes.iter().map(|s| s.0).sum::<f32>() + 12.0 * sizes.len().saturating_sub(1) as f32;
    let mean_h = sizes.iter().map(|s| s.1).sum::<f32>() / sizes.len() as f32;
    let s = ((total / (TARGET_ASPECT * mean_h.max(24.0))).sqrt().round() as usize)
        .clamp(1, sizes.len());
    if s <= 1 {
        return std::iter::once(0..sizes.len()).collect();
    }
    let target = total / s as f32;
    let mut shelves = Vec::new();
    let mut start = 0;
    let mut acc = 0.0;
    for (i, (w, _)) in sizes.iter().enumerate() {
        if acc > 0.0 && acc + w > target && shelves.len() + 1 < s {
            shelves.push(start..i);
            start = i;
            acc = 0.0;
        }
        acc += w + 12.0;
    }
    shelves.push(start..sizes.len());
    shelves
}

/// One case of a fork, standing upright: its selector chip over its subproof.
fn branch_column(b: &Branch) -> El {
    if b.label.is_empty() {
        return render_region(&b.region);
    }
    column([selector_chip(&b.label), render_region(&b.region)]).gap(3.0).align(Align::Start)
}

/// One labelled branch inside a frame: its selector chip + the contained
/// region. An empty label (a single-body induction frame) draws no chip.
fn render_branch(b: &Branch) -> El {
    if b.label.is_empty() {
        return render_region(&b.region);
    }
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
    // A long operand run (a literal instruction list feeding `list`, say)
    // galley-wraps like the other vertical shapes; the gather bar still spans
    // the whole block, so every column visibly feeds the same op.
    let cols: Vec<El> = galley_columns(args, 6.0, 6.0)
        .into_iter()
        .enumerate()
        .map(|(i, chunk)| {
            let col = column(chunk.iter().map(render_region).collect::<Vec<_>>()).gap(6.0);
            if i == 0 {
                col
            } else {
                column([cont_marker(), col]).gap(2.0)
            }
        })
        .collect();
    let inputs = if cols.len() == 1 {
        cols.into_iter().next().expect("len 1")
    } else {
        row(cols).gap(tokens::SPACE_3).align(Align::Start)
    };
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

/// Approximate the rendered (w, h) of a region, mirroring the element
/// structure closely enough that a box sized to it holds its content — a
/// deliberate slight over-estimate (no clipping is better than overlap).
/// Feeds the Board's node sizing AND this view's own reshaping decisions
/// ([`seq_columns`] / [`branch_shelves`]), which must agree with what gets
/// rendered — that's why it lives beside the renderer.
pub(crate) fn est(r: &Region) -> (f32, f32) {
    match r {
        Region::Var(name) => (text_w(name, 8.0) + 14.0, 26.0),
        Region::Lit(value) => {
            let chars = if value.is_empty() { 1 } else { value.chars().count() };
            (chars as f32 * 7.5 + 12.0, 24.0)
        }
        Region::Op { head, inline, args } => {
            let card_w = text_w(head, 8.5).max(text_w(inline, 7.0)) + 16.0;
            let card_h = if inline.is_empty() { 32.0 } else { 48.0 };
            if args.is_empty() {
                return (card_w, card_h);
            }
            // Compound operands gather to the op's left, galley-wrapped.
            let sizes: Vec<(f32, f32)> = args.iter().map(est).collect();
            let mut block_w = 0.0_f32;
            let mut block_h = 0.0_f32;
            for (i, r) in galley_partition(&sizes, 6.0, 6.0).into_iter().enumerate() {
                let mut cw = 0.0_f32;
                let mut ch = if i > 0 { 17.0 } else { 0.0 }; // ↳ marker line + gap
                for (j, (aw, ah)) in sizes[r].iter().enumerate() {
                    cw = cw.max(*aw);
                    ch += ah;
                    if j > 0 {
                        ch += 6.0;
                    }
                }
                if i > 0 {
                    block_w += 12.0; // column gap
                }
                block_w += cw;
                block_h = block_h.max(ch);
            }
            block_w += 26.0; // gather bar + feed arrow
            (block_w + card_w + 6.0, card_h.max(block_h))
        }
        Region::Seq(steps) => {
            if steps.is_empty() {
                return (20.0, 24.0);
            }
            // Mirror the galley wrap: side-by-side railed columns.
            let sizes: Vec<(f32, f32)> = steps.iter().map(est).collect();
            let mut w = 0.0_f32;
            let mut h = 0.0_f32;
            for (i, r) in galley_partition(&sizes, 4.0, 13.5).into_iter().enumerate() {
                let mut cw = 0.0_f32;
                let mut ch = if i > 0 { 17.0 } else { 0.0 }; // ↳ marker line + gap
                for (j, (sw, sh)) in sizes[r].iter().enumerate() {
                    cw = cw.max(*sw);
                    ch += sh;
                    if j > 0 {
                        ch += 4.0;
                    }
                }
                if i > 0 {
                    w += 12.0; // column gap
                }
                w += cw + 11.0; // rail + gap
                h = h.max(ch);
            }
            (w, h.max(20.0))
        }
        Region::Frame { kind, detail, branches } => {
            let band_w = text_w(kind.keyword(), 7.5) + text_w(detail, 7.0) + 20.0;
            let (body_w, body_h) = if forks(*kind) && branches.len() > 1 {
                // Shelves of upright cases.
                let sizes: Vec<(f32, f32)> = branches.iter().map(branch_column_est).collect();
                let mut w = 0.0_f32;
                let mut h = 0.0_f32;
                for (i, r) in shelf_partition(&sizes).into_iter().enumerate() {
                    let mut sw = 0.0_f32;
                    let mut sh = 0.0_f32;
                    for (j, (bw, bh)) in sizes[r].iter().enumerate() {
                        sw += bw;
                        if j > 0 {
                            sw += 12.0;
                        }
                        sh = sh.max(*bh);
                    }
                    w = w.max(sw);
                    h += sh;
                    if i > 0 {
                        h += 9.0;
                    }
                }
                (w, h)
            } else {
                // Branches stack in a column; each is its selector chip + region.
                let mut w = 0.0_f32;
                let mut h = 0.0_f32;
                for (i, b) in branches.iter().enumerate() {
                    let (rw, rh) = est(&b.region);
                    let chip_w = text_w(&b.label, 7.0) + 10.0;
                    w = w.max(chip_w + 8.0 + rw);
                    h += rh.max(24.0);
                    if i > 0 {
                        h += 7.0;
                    }
                }
                (w, h)
            };
            (band_w.max(body_w + 16.0), 26.0 + body_h + 16.0) // band + paddings
        }
        Region::List { elems, tail } => {
            // header (`list · N`) over galley columns of bracketed elements.
            let sizes: Vec<(f32, f32)> = elems.iter().map(est).collect();
            let cols = galley_partition(&sizes, 5.0, 15.0);
            let last = cols.len() - 1;
            let mut body_w = 0.0_f32;
            let mut body_h = 0.0_f32;
            for (i, r) in cols.into_iter().enumerate() {
                let mut cw = 0.0_f32;
                let mut ch = if i > 0 { 17.0 } else { 0.0 }; // ↳ marker line + gap
                for (j, (ew, eh)) in sizes[r].iter().enumerate() {
                    cw = cw.max(*ew);
                    ch += eh;
                    if j > 0 {
                        ch += 5.0;
                    }
                }
                if i == last && let Some(t) = tail {
                    let (tw, th) = est(t);
                    cw = cw.max(tw + 16.0); // "⋯ " lead
                    ch += th.max(20.0) + 5.0;
                }
                if i > 0 {
                    body_w += 12.0; // column gap
                }
                body_w += cw + 15.0; // bracket bar + paddings
                body_h = body_h.max(ch);
            }
            let w = (body_w + 12.0).max(54.0);
            let h = 16.0 + body_h + 12.0; // header + body + paddings
            (w, h)
        }
    }
}

/// [`est`] for one upright fork case (chip over region).
fn branch_column_est(b: &Branch) -> (f32, f32) {
    let (rw, rh) = est(&b.region);
    if b.label.is_empty() {
        return (rw, rh);
    }
    (rw.max(text_w(&b.label, 7.0) + 10.0), rh + 21.0)
}

pub(crate) fn text_w(s: &str, per: f32) -> f32 {
    s.chars().count() as f32 * per
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn short_runs_stay_one_column() {
        let sizes = vec![(60.0, 30.0); 5]; // 170px total < WRAP_MIN_H
        assert_eq!(galley_partition(&sizes, 4.0, 13.5), vec![0..5]);
    }

    #[test]
    fn tall_runs_wrap_into_balanced_columns() {
        let sizes = vec![(60.0, 30.0); 40]; // ~1360px tall, ~74px wide
        let cols = galley_partition(&sizes, 4.0, 13.5);
        assert!(cols.len() > 1, "a 1360px ladder must wrap");
        // Contiguous, in order, covering everything.
        assert_eq!(cols.first().unwrap().start, 0);
        assert_eq!(cols.last().unwrap().end, 40);
        for w in cols.windows(2) {
            assert_eq!(w[0].end, w[1].start);
        }
        // Balanced: no column more than ~2x the mean.
        let mean = 40.0 / cols.len() as f32;
        assert!(cols.iter().all(|r| (r.len() as f32) < mean * 2.0));
    }

    #[test]
    fn one_oversized_rung_does_not_starve_the_rest() {
        let mut sizes = vec![(60.0, 24.0); 10];
        sizes[0] = (200.0, 900.0); // a huge nested frame as rung 0
        let cols = galley_partition(&sizes, 4.0, 13.5);
        assert_eq!(cols.last().unwrap().end, 10);
        assert!(!cols.iter().any(|r| r.is_empty()), "no empty columns");
    }

    #[test]
    fn few_wide_cases_stay_on_one_shelf() {
        let sizes = vec![(80.0, 400.0), (90.0, 380.0)];
        assert_eq!(shelf_partition(&sizes), vec![0..2]);
    }

    #[test]
    fn many_cases_wrap_shelves() {
        let sizes = vec![(300.0, 60.0); 12]; // 3732px of shelf for 60px height
        let shelves = shelf_partition(&sizes);
        assert!(shelves.len() > 1, "a 3700px shelf must wrap");
        assert_eq!(shelves.last().unwrap().end, 12);
        for w in shelves.windows(2) {
            assert_eq!(w[0].end, w[1].start);
        }
    }
}
