//! Intra-fn **structured** model: turns one fn body's s-expr into a *region
//! tree* for the LabVIEW-style flow view.
//!
//! Unlike a flat node/edge graph, this is a containment hierarchy:
//!
//! - **Control structures** (`match` / `if` / `let`) become [`Region::Frame`]s
//!   that *contain* their arms / branches / body as labelled [`Branch`]es. Paren
//!   nesting becomes box enclosure.
//! - **Leaf computations** become [`Region::Op`] (a function application — head
//!   symbol + inline simple operands + nested regions for compound operands),
//!   [`Region::Var`], or [`Region::Lit`]. Inside a frame these are wired
//!   left-to-right (the view draws the connectors).
//!
//! The view layer renders this tree as nested damascene elements — containment
//! and sizing fall out of the element layout; only the intra-op wires are drawn.
//! This module stays damascene-free: it just builds the typed tree.

use crate::model::pretty;
use crate::sexpr::Sexpr;

/// A control structure's flavor — drives its band color/keyword in the view.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FrameKind {
    Match,
    If,
    Let,
}

impl FrameKind {
    pub fn keyword(self) -> &'static str {
        match self {
            FrameKind::Match => "match",
            FrameKind::If => "if",
            FrameKind::Let => "let",
        }
    }
}

/// A region of the diagram: either a containing control frame or a leaf value.
#[derive(Clone, Debug)]
pub enum Region {
    /// A `match` / `if` / `let` frame containing labelled child branches.
    Frame {
        kind: FrameKind,
        /// The scrutinee / condition shown in the header (empty for `let`).
        detail: String,
        branches: Vec<Branch>,
    },
    /// A function application: `head` + inline simple operands + compound
    /// operand sub-regions (wired in from the left by the view).
    Op {
        head: String,
        inline: String,
        args: Vec<Region>,
    },
    /// A bare variable / parameter reference.
    Var(String),
    /// A literal (int / string / quoted datum).
    Lit(String),
}

/// A labelled child of a frame: the arm pattern / `then` / `else` / binding name
/// that selects `region`.
#[derive(Clone, Debug)]
pub struct Branch {
    pub label: String,
    pub region: Region,
}

/// Max characters shown for an inline scrutinee / operand before eliding.
const MAX_INLINE: usize = 28;

impl Region {
    /// Build the region tree for a fn body (the forms after the return type).
    /// The `(measure …)` totality clause is dropped (annotation, not logic).
    pub fn build(body: &[Sexpr]) -> Region {
        let forms: Vec<&Sexpr> = body
            .iter()
            .filter(|f| f.head() != Some("measure"))
            .collect();
        match forms.as_slice() {
            [] => Region::Lit(String::new()),
            [single] => lower(single),
            many => Region::Op {
                head: "do".into(),
                inline: String::new(),
                args: many.iter().map(|f| lower(f)).collect(),
            },
        }
    }
}

/// Lower one expression into a region.
fn lower(expr: &Sexpr) -> Region {
    match expr {
        Sexpr::Sym(s) => Region::Var(s.clone()),
        Sexpr::Int(_) | Sexpr::Str(_) => Region::Lit(lit_text(expr)),
        Sexpr::List(items) => match expr.head() {
            Some("match") => lower_match(items),
            Some("if") => lower_if(items),
            Some("let") => lower_let(items),
            Some("quote") => Region::Lit(lit_text(expr)),
            _ => lower_op(items),
        },
    }
}

fn lower_match(items: &[Sexpr]) -> Region {
    let detail = items.get(1).map(short).unwrap_or_default();
    let branches = items[2.min(items.len())..]
        .iter()
        .filter_map(|arm| match arm {
            Sexpr::List(parts) => {
                let label = parts.first().map(pretty).unwrap_or_default();
                let body = parts.get(1).cloned().unwrap_or(Sexpr::List(vec![]));
                Some(Branch { label, region: lower(&body) })
            }
            _ => None,
        })
        .collect();
    Region::Frame { kind: FrameKind::Match, detail, branches }
}

fn lower_if(items: &[Sexpr]) -> Region {
    let detail = items.get(1).map(short).unwrap_or_default();
    let mut branches = Vec::new();
    if let Some(then) = items.get(2) {
        branches.push(Branch { label: "then".into(), region: lower(then) });
    }
    if let Some(els) = items.get(3) {
        branches.push(Branch { label: "else".into(), region: lower(els) });
    }
    Region::Frame { kind: FrameKind::If, detail, branches }
}

fn lower_let(items: &[Sexpr]) -> Region {
    // (let ((name val) ...) body)
    let mut branches = Vec::new();
    if let Some(Sexpr::List(bs)) = items.get(1) {
        for b in bs {
            if let Sexpr::List(p) = b
                && let Some(name) = p.first().and_then(|s| s.as_sym())
            {
                let val = p.get(1).cloned().unwrap_or(Sexpr::List(vec![]));
                branches.push(Branch { label: name.to_string(), region: lower(&val) });
            }
        }
    }
    if let Some(body) = items.get(2) {
        branches.push(Branch { label: "in".into(), region: lower(body) });
    }
    Region::Frame { kind: FrameKind::Let, detail: String::new(), branches }
}

fn lower_op(items: &[Sexpr]) -> Region {
    let head = match items.first() {
        Some(Sexpr::Sym(s)) => s.clone(),
        Some(h) if h.head() == Some("::") => qualified_short(items.first().unwrap()),
        _ => "·".into(),
    };
    let mut inline: Vec<String> = Vec::new();
    let mut args: Vec<Region> = Vec::new();
    for arg in &items[1.min(items.len())..] {
        match arg {
            Sexpr::Sym(s) => inline.push(s.clone()),
            Sexpr::Int(_) | Sexpr::Str(_) => inline.push(lit_text(arg)),
            Sexpr::List(_) if arg.head() == Some("quote") => inline.push(lit_text(arg)),
            Sexpr::List(_) if arg.head() == Some("::") => inline.push(qualified_short(arg)),
            _ => args.push(lower(arg)),
        }
    }
    Region::Op { head, inline: elide(inline.join(" "), MAX_INLINE), args }
}

/// A compact one-line rendering of an expression for an inline scrutinee/cond.
fn short(e: &Sexpr) -> String {
    elide(pretty(e), MAX_INLINE)
}

/// The short (last) name of a `(:: a b … name)` qualified path.
fn qualified_short(e: &Sexpr) -> String {
    match e {
        Sexpr::List(items) => items.last().map(pretty).unwrap_or_else(|| pretty(e)),
        _ => pretty(e),
    }
}

fn lit_text(e: &Sexpr) -> String {
    match e {
        Sexpr::Int(n) => n.to_string(),
        Sexpr::Str(s) => format!("{s:?}"),
        Sexpr::List(items) if e.head() == Some("quote") => {
            format!("'{}", items.get(1).map(pretty).unwrap_or_default())
        }
        _ => pretty(e),
    }
}

fn elide(s: String, max: usize) -> String {
    if s.chars().count() > max {
        let mut t: String = s.chars().take(max - 1).collect();
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

    fn body(src: &str) -> Vec<Sexpr> {
        let form = parse_top(&format!("(fn f () T {src})")).unwrap().pop().unwrap();
        match form {
            Sexpr::List(items) => items[4..].to_vec(),
            _ => vec![],
        }
    }

    fn frame(r: &Region) -> (&FrameKind, &str, &[Branch]) {
        match r {
            Region::Frame { kind, detail, branches } => (kind, detail.as_str(), branches),
            _ => panic!("expected a frame, got {r:?}"),
        }
    }

    #[test]
    fn op_inlines_simple_operands_only() {
        match Region::build(&body("(int_eq th 59)")) {
            Region::Op { head, inline, args } => {
                assert_eq!(head, "int_eq");
                assert_eq!(inline, "th 59");
                assert!(args.is_empty());
            }
            other => panic!("expected op, got {other:?}"),
        }
    }

    #[test]
    fn op_nests_compound_operand_as_region() {
        match Region::build(&body("(head_code (head_atom line))")) {
            Region::Op { head, args, .. } => {
                assert_eq!(head, "head_code");
                assert_eq!(args.len(), 1);
                match &args[0] {
                    Region::Op { head, inline, .. } => {
                        assert_eq!(head, "head_atom");
                        assert_eq!(inline, "line");
                    }
                    other => panic!("expected nested op, got {other:?}"),
                }
            }
            other => panic!("expected op, got {other:?}"),
        }
    }

    #[test]
    fn match_arms_become_labelled_branches() {
        let r = Region::build(&body(
            "(match (trim_left line) (Nil (Pair 6 cur)) ((Cons th tt) (Pair 5 cur)))",
        ));
        let (kind, detail, branches) = frame(&r);
        assert_eq!(*kind, FrameKind::Match);
        assert_eq!(detail, "(trim_left line)");
        assert_eq!(branches.len(), 2);
        assert_eq!(branches[0].label, "Nil");
        assert_eq!(branches[1].label, "(Cons th tt)");
    }

    #[test]
    fn nested_control_in_arm_is_a_child_frame() {
        let r = Region::build(&body(
            "(match x (Nil (if c (Pair 1 a) (Pair 2 a))))",
        ));
        let (_, _, branches) = frame(&r);
        let (kind, _, inner) = frame(&branches[0].region);
        assert_eq!(*kind, FrameKind::If);
        assert_eq!(inner[0].label, "then");
        assert_eq!(inner[1].label, "else");
    }

    #[test]
    fn let_bindings_and_body_are_branches() {
        let r = Region::build(&body("(let ((code (head_atom line))) (Pair code code))"));
        let (kind, _, branches) = frame(&r);
        assert_eq!(*kind, FrameKind::Let);
        assert_eq!(branches[0].label, "code");
        assert_eq!(branches.last().unwrap().label, "in");
    }

    #[test]
    fn measure_clause_is_skipped() {
        let r = Region::build(&body("(measure (struct xs)) (match xs (Nil 0))"));
        assert!(matches!(r, Region::Frame { kind: FrameKind::Match, .. }));
    }
}
