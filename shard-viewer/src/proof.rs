//! Intra-claim structured model: turns a proof-layer form's tactic tree into
//! the same [`Region`] tree the Flow view renders fn bodies with — so proof
//! structure reads in the exact visual vocabulary the fn cards taught.
//!
//! The design rule: render the proof's **skeleton** — where it branches
//! (induct / fin-split / case-on frames), what facts it cuts in (`have`
//! frames, goals shown), and what each step **cites** (lemma names as the
//! bold heroes) — and drop the tactic plumbing (rewrite directions/positions,
//! farkas coefficient lists), which is for the checker, not the reader.
//! Unrecognized forms degrade to a generic op card, exactly like unknown
//! forms in a fn body.

use crate::flow::{Branch, FrameKind, Region};
use crate::model::pretty;
use crate::sexpr::Sexpr;

/// Max chars for an inline goal statement / operand tail before eliding.
const MAX_GOAL: usize = 44;

/// The proof body of a claim/fulfills form, lowered — `None` when the form
/// carries no proof (an axiom or requirement is statement-only).
pub fn build(form: &Sexpr) -> Option<Region> {
    let items = form.as_list()?;
    // Everything after the head + name that isn't the statement is proof.
    let mut out = Vec::new();
    for it in items.iter().skip(2) {
        if !matches!(it.head(), Some("goal") | Some("kind")) {
            rungs(it, &mut out);
        }
    }
    match out.len() {
        0 => None,
        1 => out.pop(),
        _ => Some(Region::Seq(out)),
    }
}

/// Lower one proof form to a single region (a frame body, an op operand).
fn lower(e: &Sexpr) -> Region {
    let mut out = Vec::new();
    rungs(e, &mut out);
    match out.len() {
        1 => out.pop().expect("len 1"),
        _ => Region::Seq(out),
    }
}

/// Append `e`'s rungs to the current ladder. The continuation-taking forms
/// (§10.3: `steps` / `rewrite-with` / `have` / `div-facts` / `refine-fact` /
/// `inject`) each contribute their own rung and *flatten* their trailing
/// PROOF into the same ladder — sequential proofs stay one column instead of
/// a nesting pyramid. `chain` (the reader sugar for exactly these spines)
/// lowers identically: its items are the same forms without the trailing
/// argument. Consecutive `have` facts merge into one green frame.
fn rungs(e: &Sexpr, out: &mut Vec<Region>) {
    let Some(items) = e.as_list() else {
        out.push(Region::Lit(pretty(e))); // `refl` and friends
        return;
    };
    match e.head() {
        // `(steps (STEP…) TERMINAL?)` — each step a rung, then the terminal.
        Some("steps") => {
            if let Some(Sexpr::List(steps)) = items.get(1) {
                out.extend(steps.iter().map(step));
            }
            for t in items.get(2..).unwrap_or(&[]) {
                rungs(t, out);
            }
        }
        Some("chain") => {
            for it in &items[1.min(items.len())..] {
                rungs(it, out);
            }
        }
        // `(have EQ P [CONT])` / `(have NAME EQ P [CONT])` — the cut.
        Some("have") => {
            let (label, goal_i) = match items.get(1) {
                Some(Sexpr::Sym(n)) => (n.clone(), 2),
                _ => (String::new(), 1),
            };
            let (Some(goal), Some(p)) = (items.get(goal_i), items.get(goal_i + 1)) else {
                out.push(generic(items)); // malformed — never recurse on `e`
                return;
            };
            have_rung(out, label, goal, p);
            if let Some(cont) = items.get(goal_i + 2) {
                rungs(cont, out);
            }
        }
        // `(rewrite-with EQREF DIR SIDE (INST…) (PROOF…) [CONT])` — rewrite
        // by a premised equation; the sub-proofs discharging its premises
        // wire in as operands, the continuation is the rest of the ladder.
        Some("rewrite-with") => {
            let args = match items.get(5) {
                Some(Sexpr::List(subs)) => subs.iter().map(lower).collect(),
                _ => Vec::new(),
            };
            out.push(match citation(items.get(1)) {
                Cite::Lemma(name) => Region::Op { head: name, inline: "← rewrite-with".into(), args },
                Cite::Other(what) => Region::Op { head: "rewrite-with".into(), inline: what, args },
            });
            if let Some(cont) = items.get(6) {
                rungs(cont, out);
            }
        }
        // `(div-facts TERM D Q [CONT])` — inject the Euclidean triple.
        Some("div-facts") => {
            let inline = items
                .get(1..4.min(items.len()))
                .unwrap_or(&[])
                .iter()
                .map(pretty)
                .collect::<Vec<_>>()
                .join(" ");
            out.push(op("div-facts".into(), elide(inline)));
            if let Some(cont) = items.get(4) {
                rungs(cont, out);
            }
        }
        // `(refine-fact TYPE TERM [CONT])` — materialize a refinement fact.
        Some("refine-fact") => {
            let inline = items
                .get(1..3.min(items.len()))
                .unwrap_or(&[])
                .iter()
                .map(pretty)
                .collect::<Vec<_>>()
                .join(" ");
            out.push(op("refine-fact".into(), elide(inline)));
            if let Some(cont) = items.get(3) {
                rungs(cont, out);
            }
        }
        // `(inject EQREF (NAME…) [CONT])` — constructor injectivity.
        Some("inject") => {
            let inline = match citation(items.get(1)) {
                Cite::Lemma(n) | Cite::Other(n) => n,
            };
            out.push(op("inject".into(), inline));
            if let Some(cont) = items.get(3) {
                rungs(cont, out);
            }
        }
        Some("induct") => out.push(cases_frame(FrameKind::Induct, items)),
        Some("fin-split") => out.push(cases_frame(FrameKind::FinSplit, items)),
        Some("case-on") => out.push(cases_frame(FrameKind::CaseOn, items)),
        Some("wf-induct") => out.push(body_frame(FrameKind::WfInduct, items)),
        Some("subterm-induct") => out.push(body_frame(FrameKind::SubtermInduct, items)),
        _ => out.push(step(e)),
    }
}

/// One bound fact: goal statement over the proof establishing it. Merges into
/// a trailing `have` frame so a fact spine reads as one green block; an empty
/// label (an anonymous have) draws no chip — the goal line marks the row.
fn have_rung(out: &mut Vec<Region>, label: String, goal: &Sexpr, proof: &Sexpr) {
    let mut rows = vec![Region::Lit(elide(pretty(goal)))];
    rungs(proof, &mut rows);
    let branch = Branch { label, region: Region::Seq(rows) };
    if let Some(Region::Frame { kind: FrameKind::Have, branches, .. }) = out.last_mut() {
        branches.push(branch);
    } else {
        out.push(Region::Frame {
            kind: FrameKind::Have,
            detail: String::new(),
            branches: vec![branch],
        });
    }
}

/// A branching tactic: band = keyword + its subject (the induction variable /
/// scrutinized expression), branches = the labelled cases. The case list is
/// recognized structurally (the item whose every element is a `(case …)`),
/// so premise/type arguments between subject and cases don't matter.
fn cases_frame(kind: FrameKind, items: &[Sexpr]) -> Region {
    let detail = items.get(1).map(|s| elide(pretty(s))).unwrap_or_default();
    let branches = items
        .iter()
        .rev()
        .find_map(|it| match it {
            Sexpr::List(cs)
                if !cs.is_empty() && cs.iter().all(|c| c.head() == Some("case")) =>
            {
                Some(cs.as_slice())
            }
            _ => None,
        })
        .unwrap_or(&[])
        .iter()
        .filter_map(|c| {
            let parts = c.as_list()?;
            let mut label = parts.get(1).map(pretty).unwrap_or_default();
            let mut body = parts.get(2..).unwrap_or(&[]);
            // `(case CTOR (FIELD…) PROOF)` — field binders join the selector
            // chip (`Cons h t`), they aren't proof content.
            if body.len() >= 2
                && let Some(Sexpr::List(binders)) = body.first()
                && binders.iter().all(|b| b.as_sym().is_some())
            {
                for b in binders {
                    label.push(' ');
                    label.push_str(b.as_sym().expect("all syms"));
                }
                body = &body[1..];
            }
            Some(Branch { label, region: seq_or_single(body) })
        })
        .collect();
    Region::Frame { kind, detail, branches }
}

/// A single-body induction (`wf-induct` / `subterm-induct`): band + body.
fn body_frame(kind: FrameKind, items: &[Sexpr]) -> Region {
    let detail = items.get(1).map(|s| elide(pretty(s))).unwrap_or_default();
    let branches = vec![Branch {
        label: String::new(),
        region: seq_or_single(items.get(2..).unwrap_or(&[])),
    }];
    Region::Frame { kind, detail, branches }
}

fn seq_or_single(forms: &[Sexpr]) -> Region {
    match forms {
        [] => Region::Lit(String::new()),
        [single] => lower(single),
        many => Region::Seq(many.iter().map(lower).collect()),
    }
}

/// One rung of a step ladder. The reader-facing content is *what is cited*:
/// a lemma rewrite makes the lemma name the bold hero; premise/hyp rewrites
/// and unfolds keep the verb visible; pure computation moves are dim tags.
fn step(e: &Sexpr) -> Region {
    let Some(items) = e.as_list() else {
        return Region::Lit(pretty(e));
    };
    match e.head() {
        Some("rewrite") => match citation(items.get(1)) {
            Cite::Lemma(name) => op(name, "← rewrite".into()),
            Cite::Other(what) => op("rewrite".into(), what),
        },
        Some("unfold") => {
            let target = items.get(1).map(pretty).unwrap_or_default();
            op(target, "unfold".into())
        }
        Some(v @ ("simp" | "compute" | "reduce")) => Region::Lit(v.to_string()),
        Some("by") => Region::Lit(by_text(items)),
        // A structural / continuation form in leaf position (a case split as
        // a frame body, a spine as an op operand, …) keeps its structure.
        Some(
            "steps" | "chain" | "have" | "rewrite-with" | "div-facts" | "refine-fact"
            | "inject" | "induct" | "fin-split" | "case-on" | "wf-induct" | "subterm-induct",
        ) => lower(e),
        _ => generic(items),
    }
}

enum Cite {
    /// `(lemma NAME)` — a real citation; the name is the content.
    Lemma(String),
    /// `(premise X)` / `(hyp X)` / anything else — local plumbing, shown small.
    Other(String),
}

fn citation(arg: Option<&Sexpr>) -> Cite {
    let Some(a) = arg else {
        return Cite::Other(String::new());
    };
    match a.head() {
        Some("lemma") => match a.as_list().and_then(|l| l.get(1)) {
            Some(n) => Cite::Lemma(pretty(n)),
            None => Cite::Other("lemma".into()),
        },
        Some("premise") | Some("hyp") => Cite::Other(elide(pretty(a).replace(['(', ')'], ""))),
        _ => Cite::Other(elide(pretty(a))),
    }
}

/// `(by arith …)` → the solver name; coefficients are checker food.
fn by_text(items: &[Sexpr]) -> String {
    match items.get(1) {
        Some(Sexpr::Sym(s)) => s.clone(),
        _ => "by".into(),
    }
}

fn op(head: String, inline: String) -> Region {
    Region::Op { head, inline, args: Vec::new() }
}

/// Unrecognized form: head + elided argument tail, no pretense of structure.
fn generic(items: &[Sexpr]) -> Region {
    let head = items.first().map(pretty).unwrap_or_else(|| "·".into());
    let tail = items[1.min(items.len())..].iter().map(pretty).collect::<Vec<_>>().join(" ");
    op(head, elide(tail))
}

fn elide(s: String) -> String {
    if s.chars().count() > MAX_GOAL {
        let mut t: String = s.chars().take(MAX_GOAL - 1).collect();
        t.push('…');
        t
    } else {
        s
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::sexpr::parse_top;

    fn claim(src: &str) -> Sexpr {
        parse_top(src).unwrap().pop().unwrap()
    }

    fn frame(r: &Region) -> (&FrameKind, &str, &[Branch]) {
        match r {
            Region::Frame { kind, detail, branches } => (kind, detail.as_str(), branches),
            other => panic!("expected a frame, got {other:?}"),
        }
    }

    #[test]
    fn statement_only_forms_have_no_proof_region() {
        assert!(build(&claim("(axiom a (kind operational) (goal ((n Int)) () (= n n)))")).is_none());
        assert!(build(&claim("(requirement r (goal ((n Int)) () (= n n)))")).is_none());
    }

    #[test]
    fn steps_lower_to_a_ladder_with_lemma_heroes() {
        let r = build(&claim(
            "(fulfills f (steps ((unfold pow2 lhs) \
             (rewrite (lemma bshl_z) lr lhs true ()) \
             (rewrite (premise hc) lr lhs true ()) \
             (simp lhs)) refl))",
        ))
        .unwrap();
        let Region::Seq(steps) = r else { panic!("expected seq, got {r:?}") };
        assert_eq!(steps.len(), 5);
        assert!(matches!(&steps[0], Region::Op { head, inline, .. }
            if head == "pow2" && inline == "unfold"));
        assert!(matches!(&steps[1], Region::Op { head, inline, .. }
            if head == "bshl_z" && inline == "← rewrite"));
        assert!(matches!(&steps[2], Region::Op { head, inline, .. }
            if head == "rewrite" && inline == "premise hc"));
        assert!(matches!(&steps[3], Region::Lit(v) if v == "simp"));
        assert!(matches!(&steps[4], Region::Lit(v) if v == "refl"));
    }

    #[test]
    fn chain_lowers_to_one_ladder_with_a_fact_frame() {
        let r = build(&claim(
            "(fulfills f (chain \
               (have hc (= (le k 0) False) (by arith (list 1 1))) \
               (steps ((rewrite (premise hc) lr lhs true ())) refl)))",
        ))
        .unwrap();
        // One ladder: the fact frame, then the steps that use it.
        let Region::Seq(rungs) = r else { panic!("expected a ladder, got {r:?}") };
        assert_eq!(rungs.len(), 3);
        let (kind, _, branches) = frame(&rungs[0]);
        assert_eq!(*kind, FrameKind::Have);
        assert_eq!(branches.len(), 1);
        assert_eq!(branches[0].label, "hc");
        let Region::Seq(fact) = &branches[0].region else { panic!() };
        assert!(matches!(&fact[0], Region::Lit(g) if g == "(= (le k 0) False)"));
        assert!(matches!(&fact[1], Region::Lit(v) if v == "arith"));
        assert!(matches!(&rungs[1], Region::Op { head, .. } if head == "rewrite"));
        assert!(matches!(&rungs[2], Region::Lit(v) if v == "refl"));
    }

    #[test]
    fn have_spines_flatten_and_consecutive_facts_share_a_frame() {
        // Anonymous continuation spine: two facts, then the discharge.
        let r = build(&claim(
            "(fulfills f (have (= a b) (by arith (list 1)) \
               (have (= c d) (by arith (list 2)) \
                 (steps ((compute lhs)) refl))))",
        ))
        .unwrap();
        let Region::Seq(rungs) = r else { panic!("expected a ladder, got {r:?}") };
        assert_eq!(rungs.len(), 3); // fact frame + compute + refl
        let (kind, _, branches) = frame(&rungs[0]);
        assert_eq!(*kind, FrameKind::Have);
        assert_eq!(branches.len(), 2, "consecutive facts merge into one frame");
        assert!(branches.iter().all(|b| b.label.is_empty()), "anonymous facts draw no chip");

        // Named 5-ary have WITH continuation: the continuation is the
        // ladder's tail, not part of the fact's proof.
        let r = build(&claim(
            "(fulfills f (have h1 (= a b) (by arith (list 1)) \
               (steps ((rewrite (premise h1) lr lhs true ())) refl)))",
        ))
        .unwrap();
        let Region::Seq(rungs) = r else { panic!("expected a ladder, got {r:?}") };
        assert_eq!(rungs.len(), 3);
        let (_, _, branches) = frame(&rungs[0]);
        assert_eq!(branches[0].label, "h1");
        let Region::Seq(fact) = &branches[0].region else { panic!() };
        assert_eq!(fact.len(), 2, "fact holds goal + its own proof only");
    }

    #[test]
    fn short_chain_have_neither_crashes_nor_recurses() {
        // A chain item may be the anonymous have WITHOUT its trailing proof
        // (the sugar folds it in) — 3 items. Must not diverge.
        let r = build(&claim(
            "(fulfills f (chain (have (= a b) (by arith (list 1))) refl))",
        ))
        .unwrap();
        let Region::Seq(rungs) = r else { panic!("expected a ladder, got {r:?}") };
        assert_eq!(rungs.len(), 2);
        assert!(matches!(frame(&rungs[0]).0, FrameKind::Have));
        assert!(matches!(&rungs[1], Region::Lit(v) if v == "refl"));
    }

    #[test]
    fn rewrite_with_keeps_subproofs_and_continuation() {
        let r = build(&claim(
            "(fulfills f (rewrite-with (lemma board_w_min) lr lhs () \
               ((by arith (list 1)) (by arith (list 2))) \
               (steps ((compute lhs)) refl)))",
        ))
        .unwrap();
        let Region::Seq(rungs) = r else { panic!("expected a ladder, got {r:?}") };
        assert_eq!(rungs.len(), 3, "op + flattened continuation");
        let Region::Op { head, inline, args } = &rungs[0] else { panic!("{:?}", rungs[0]) };
        assert_eq!(head, "board_w_min");
        assert_eq!(inline, "← rewrite-with");
        assert_eq!(args.len(), 2, "premise-discharging subproofs wire in");
        assert!(matches!(&rungs[1], Region::Lit(v) if v == "compute"));
    }

    #[test]
    fn div_facts_keeps_its_continuation() {
        let r = build(&claim(
            "(fulfills f (div-facts n 256 q (steps ((compute lhs)) refl)))",
        ))
        .unwrap();
        let Region::Seq(rungs) = r else { panic!("expected a ladder, got {r:?}") };
        assert_eq!(rungs.len(), 3);
        assert!(matches!(&rungs[0], Region::Op { head, inline, .. }
            if head == "div-facts" && inline == "n 256 q"));
    }

    #[test]
    fn case_field_binders_join_the_selector_chip() {
        let r = build(&claim(
            "(fulfills f (induct xs ((case Nil refl) \
               (case Cons (h t) (steps ((compute lhs)) refl)))))",
        ))
        .unwrap();
        let (_, _, branches) = frame(&r);
        assert_eq!(branches[0].label, "Nil");
        assert_eq!(branches[1].label, "Cons h t");
        // The binder list is NOT proof content.
        let Region::Seq(body) = &branches[1].region else { panic!("{:?}", branches[1].region) };
        assert!(matches!(&body[0], Region::Lit(v) if v == "compute"));
    }

    #[test]
    fn case_splits_become_labelled_frames_wherever_the_case_list_sits() {
        // fin-split carries premise args between subject and cases.
        let r = build(&claim(
            "(claim c (goal ((p Int)) () (= p p)) \
              (fin-split p (premise 0) (premise 1) \
                ((case 0 (by arith (list 1))) (case 1 (by arith (list 2))))))",
        ))
        .unwrap();
        let (kind, detail, branches) = frame(&r);
        assert_eq!(*kind, FrameKind::FinSplit);
        assert_eq!(detail, "p");
        assert_eq!(branches.len(), 2);
        assert_eq!(branches[0].label, "0");
        // wf-induct wraps a single body.
        let r = build(&claim(
            "(fulfills f (wf-induct k (case-on (le k 0) Bool \
               ((case True (steps ((compute lhs)) refl)) \
                (case False (steps ((reduce lhs)) refl))))))",
        ))
        .unwrap();
        let (kind, detail, branches) = frame(&r);
        assert_eq!(*kind, FrameKind::WfInduct);
        assert_eq!(detail, "k");
        assert_eq!(branches.len(), 1);
        let (kind, _, inner) = frame(&branches[0].region);
        assert_eq!(*kind, FrameKind::CaseOn);
        assert_eq!(inner.len(), 2);
        assert_eq!(inner[0].label, "True");
    }
}
