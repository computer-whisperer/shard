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
    let proof: Vec<&Sexpr> = items
        .iter()
        .skip(2)
        .filter(|it| !matches!(it.head(), Some("goal") | Some("kind")))
        .collect();
    match proof.as_slice() {
        [] => None,
        [single] => Some(lower(single)),
        many => Some(Region::Seq(many.iter().map(|f| lower(f)).collect())),
    }
}

/// Lower one proof form.
fn lower(e: &Sexpr) -> Region {
    let Some(items) = e.as_list() else {
        return Region::Lit(pretty(e)); // `refl` and friends
    };
    match e.head() {
        Some("steps") => lower_steps(items),
        Some("chain") => lower_chain(items),
        Some("have") => lower_have(e),
        Some("induct") => cases_frame(FrameKind::Induct, items),
        Some("fin-split") => cases_frame(FrameKind::FinSplit, items),
        Some("case-on") => cases_frame(FrameKind::CaseOn, items),
        Some("wf-induct") => body_frame(FrameKind::WfInduct, items),
        Some("subterm-induct") => body_frame(FrameKind::SubtermInduct, items),
        _ => step(e),
    }
}

/// `(steps (STEP…) TERMINAL…)` — the ladder: each step in order, then the
/// terminal (`refl`, `(by arith …)`).
fn lower_steps(items: &[Sexpr]) -> Region {
    let mut out = Vec::new();
    if let Some(Sexpr::List(steps)) = items.get(1) {
        out.extend(steps.iter().map(step));
    }
    for t in items.get(2..).unwrap_or(&[]) {
        out.push(lower(t));
    }
    Region::Seq(out)
}

/// `(chain (have NAME GOAL PROOF)… FINAL)` — named facts proven in sequence,
/// then the goal discharged with them in scope. A let-of-facts.
fn lower_chain(items: &[Sexpr]) -> Region {
    let mut branches = Vec::new();
    for it in &items[1.min(items.len())..] {
        match it.as_list() {
            Some(h) if it.head() == Some("have") && h.len() >= 4 => {
                branches.push(Branch {
                    label: h[1].as_sym().unwrap_or("have").to_string(),
                    region: fact(&h[2], &h[3..]),
                });
            }
            _ => branches.push(Branch { label: "⊢".into(), region: lower(it) }),
        }
    }
    Region::Frame { kind: FrameKind::Have, detail: String::new(), branches }
}

/// The anonymous continuation form `(have GOAL PROOF CONT)`: flatten the
/// `have` spine into one frame — each fact a labelled branch, the final
/// continuation the `⊢` branch. (The named 4-ary `(have NAME GOAL PROOF)`
/// nested in a chain is handled there; a stray one still reads here.)
fn lower_have(e: &Sexpr) -> Region {
    let mut branches = Vec::new();
    let mut cur = e;
    loop {
        match cur.as_list() {
            Some(h) if cur.head() == Some("have") && h.len() >= 4 && h[1].as_list().is_some() => {
                branches.push(Branch {
                    label: "have".into(),
                    region: fact(&h[1], &h[2..h.len() - 1]),
                });
                cur = h.last().expect("len >= 4");
            }
            Some(h) if cur.head() == Some("have") && h.len() >= 4 => {
                // Named, no continuation: one bound fact.
                branches.push(Branch {
                    label: h[1].as_sym().unwrap_or("have").to_string(),
                    region: fact(&h[2], &h[3..]),
                });
                break;
            }
            _ => {
                branches.push(Branch { label: "⊢".into(), region: lower(cur) });
                break;
            }
        }
    }
    Region::Frame { kind: FrameKind::Have, detail: String::new(), branches }
}

/// A bound fact: its goal statement over the proof that establishes it.
fn fact(goal: &Sexpr, proof: &[Sexpr]) -> Region {
    let mut rows = vec![Region::Lit(elide(pretty(goal)))];
    rows.extend(proof.iter().map(lower));
    Region::Seq(rows)
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
            Some(Branch {
                label: parts.get(1).map(pretty).unwrap_or_default(),
                region: seq_or_single(parts.get(2..).unwrap_or(&[])),
            })
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
        // `(at K STEP)` — position plumbing around a real step.
        Some("at") => items.get(2).map(step).unwrap_or_else(|| generic(items)),
        Some(verb @ ("rewrite" | "rewrite-with")) => match citation(items.get(1)) {
            Cite::Lemma(name) => op(name, format!("← {verb}")),
            Cite::Other(what) => op(verb.to_string(), what),
        },
        Some("unfold") => {
            let target = items.get(1).map(pretty).unwrap_or_default();
            op(target, "unfold".into())
        }
        Some(v @ ("simp" | "compute" | "reduce")) => Region::Lit(v.to_string()),
        Some("by") => Region::Lit(by_text(items)),
        // A structural form in step position (nested steps inside a chain
        // final, a case split inside a have proof, …) keeps its structure.
        Some("steps" | "chain" | "have" | "induct" | "fin-split" | "case-on" | "wf-induct")
        | Some("subterm-induct") => lower(e),
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
    fn chain_flattens_to_a_have_frame() {
        let r = build(&claim(
            "(fulfills f (chain \
               (have hc (= (le k 0) False) (by arith (list 1 1))) \
               (steps ((rewrite (premise hc) lr lhs true ())) refl)))",
        ))
        .unwrap();
        let (kind, _, branches) = frame(&r);
        assert_eq!(*kind, FrameKind::Have);
        assert_eq!(branches.len(), 2);
        assert_eq!(branches[0].label, "hc");
        let Region::Seq(fact) = &branches[0].region else { panic!() };
        assert!(matches!(&fact[0], Region::Lit(g) if g == "(= (le k 0) False)"));
        assert!(matches!(&fact[1], Region::Lit(v) if v == "arith"));
        assert_eq!(branches[1].label, "⊢");
    }

    #[test]
    fn anonymous_have_spine_flattens() {
        let r = build(&claim(
            "(fulfills f (have (= a b) (by arith (list 1)) \
               (have (= c d) (by arith (list 2)) \
                 (steps ((compute lhs)) refl))))",
        ))
        .unwrap();
        let (kind, _, branches) = frame(&r);
        assert_eq!(*kind, FrameKind::Have);
        assert_eq!(branches.len(), 3);
        assert_eq!(branches[0].label, "have");
        assert_eq!(branches[1].label, "have");
        assert_eq!(branches[2].label, "⊢");
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
