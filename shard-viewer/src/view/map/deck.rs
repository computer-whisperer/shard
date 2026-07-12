//! The screen-space shape deck: the focused member's datastructure
//! definitions as compact cards docked at the canvas edge — the cross-file
//! lens the committed plane can't provide (an overlay follows hover freely;
//! the topology never hears it).

use super::cards::type_card;
use super::Member;
use crate::model::Project;
use crate::view::SUB_SIZE;
use damascene_core::prelude::*;

/// The screen-space shape deck: the focused member's datastructure
/// definitions as compact cards, docked at the canvas edge. This is the only
/// way to see a shape's composition while zoomed into a fn whose types live
/// in another file — the committed plane can't show cross-file members, an
/// overlay can. `None` when the focus has no shapes to show.
pub(super) fn shape_deck(project: &Project, focus: Option<Member>) -> Option<El> {
    let (title, type_ids): (String, Vec<usize>) = match focus? {
        Member::Fn(g) => {
            let f = &project.fns[g];
            let ids: Vec<usize> =
                f.shapes.iter().chain(&f.sig_types).copied().collect();
            (f.name.clone(), ids)
        }
        // A type's own deck: what it's composed of (its definition is already
        // under the cursor; its parts may be anywhere).
        Member::Type(t) => (project.types[t].name.clone(), project.types[t].composed.clone()),
        Member::Claim(_) | Member::Doc(_) => return None,
    };
    if type_ids.is_empty() {
        return None;
    }
    const DECK_CAP: usize = 4;
    let shown = &type_ids[..type_ids.len().min(DECK_CAP)];
    let mut cards: Vec<El> = vec![
        row([
            text("shapes of").muted().font_size(SUB_SIZE).nowrap_text(),
            text(title).mono().semibold().font_size(SUB_SIZE).nowrap_text().ellipsis(),
        ])
        .gap(tokens::SPACE_2),
    ];
    for &ti in shown {
        let home = &project.files[project.types[ti].file].rel;
        cards.push(
            column([
                type_card(project, ti, false),
                text(home.clone()).muted().font_size(9.0).nowrap_text().ellipsis(),
            ])
            .gap(2.0)
            .align(Align::End),
        );
    }
    if type_ids.len() > shown.len() {
        cards.push(
            text(format!("+{} more", type_ids.len() - shown.len()))
                .muted()
                .font_size(SUB_SIZE)
                .nowrap_text(),
        );
    }
    // Unkeyed throughout: the deck is a read-only lens that must never
    // intercept a pan drag or steal hover from the plane beneath it.
    Some(
        column(cards)
            .gap(tokens::SPACE_2)
            .padding(tokens::SPACE_3)
            .radius(8.0)
            .fill(tokens::BACKGROUND.mix(tokens::CARD, 0.6))
            .stroke(tokens::INFO.mix(tokens::BORDER, 0.7))
            .align(Align::End),
    )
}
