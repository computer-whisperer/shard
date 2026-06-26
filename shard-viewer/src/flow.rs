//! Intra-fn **dataflow / decision-tree** model: turns one fn body's s-expr into
//! a left-to-right graph so the nesting becomes spatial instead of parenthetical.
//!
//! The shape is a *hybrid* (see the viewer's view layer for the rendering):
//!
//! - **Control structures** (`match` / `if` / `let`) become branch nodes; each
//!   arm / branch / body hangs to the right as a child, headed by the pattern or
//!   branch label that selects it. This is the skeleton that de-nests the soup.
//! - **Leaf expressions** (applications) become **operation** nodes: the head
//!   symbol is the title, *simple* operands (vars / literals) sit inline, and
//!   *compound* operands (nested applications) expand into their own op nodes
//!   wired in as data children. So `(int_eq th 59)` is one box, but
//!   `(head_code (head_atom line))` is two.
//!
//! This module is semantics-light and damascene-free (like `layout`): it builds
//! the typed node/edge lists; the view layer sizes and draws them.

use crate::model::pretty;
use crate::sexpr::Sexpr;

/// What a flow node represents — drives its color and chrome in the view.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FlowKind {
    /// `match` on a scrutinee; control children are its arms.
    Match,
    /// `if`; control children are the then/else branches.
    If,
    /// `let`; data children are binding values, the control child is the body.
    Let,
    /// A function application (leaf computation).
    Op,
    /// A bare variable / parameter reference.
    Source,
    /// A literal (int, string, or quoted datum).
    Lit,
}

/// Whether an edge carries control (which branch runs) or data (a value feeds a
/// computation). Rendered with distinct styling so the two flows stay legible.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum EdgeKind {
    Control,
    Data,
}

/// One node in the flow diagram.
#[derive(Clone, Debug)]
pub struct FlowNode {
    pub kind: FlowKind,
    /// The branch/binding label that selects this node, shown as a header chip:
    /// a `match` pattern, `then`/`else`, or a `let` binding name. Empty for the
    /// root and for plain operands.
    pub label: String,
    /// Primary content: the head symbol, or `match <scrutinee>` / `if <cond>`.
    pub title: String,
    /// Secondary content: an op's inline simple operands, or a `let`'s names.
    pub subtitle: String,
}

/// A directed parent→child edge (parent sits left, child right).
#[derive(Clone, Copy, Debug)]
pub struct FlowEdge {
    pub from: usize,
    pub to: usize,
    pub kind: EdgeKind,
}

/// The whole diagram for one fn body.
#[derive(Clone, Debug, Default)]
pub struct FlowGraph {
    pub nodes: Vec<FlowNode>,
    pub edges: Vec<FlowEdge>,
}

/// Max characters shown for an inline scrutinee / operand string before it is
/// elided; the full text lives in the source panel.
const MAX_INLINE: usize = 30;

impl FlowGraph {
    /// Build the flow diagram for a fn body (the forms after the return type).
    /// A multi-form body (rare — usually one expression) is treated as an
    /// implicit sequence: each form becomes a root child.
    pub fn build(body: &[Sexpr]) -> FlowGraph {
        // The `(measure …)` totality clause is an annotation, not body logic —
        // drop it so the diagram shows only the computation.
        let forms: Vec<&Sexpr> = body
            .iter()
            .filter(|f| f.head() != Some("measure"))
            .collect();
        let mut g = FlowGraph::default();
        match forms.as_slice() {
            [] => {}
            [single] => {
                g.go(single, String::new());
            }
            many => {
                // Several top-level forms: a synthetic `do` groups them.
                let root = g.push(FlowKind::Op, String::new(), "do".into(), String::new());
                for form in many {
                    let c = g.go(form, String::new());
                    g.edges.push(FlowEdge { from: root, to: c, kind: EdgeKind::Control });
                }
            }
        }
        g
    }

    /// Recursively lower `expr` into nodes, returning the id of the node that
    /// stands for its value. `label` is the branch/binding chip leading here.
    fn go(&mut self, expr: &Sexpr, label: String) -> usize {
        match expr {
            Sexpr::Sym(s) => self.push(FlowKind::Source, label, s.clone(), String::new()),
            Sexpr::Int(_) | Sexpr::Str(_) => {
                self.push(FlowKind::Lit, label, lit_text(expr), String::new())
            }
            Sexpr::List(items) => self.go_list(expr, items, label),
        }
    }

    fn go_list(&mut self, expr: &Sexpr, items: &[Sexpr], label: String) -> usize {
        match expr.head() {
            Some("match") => self.go_match(items, label),
            Some("if") => self.go_if(items, label),
            Some("let") => self.go_let(items, label),
            Some("quote") => self.push(FlowKind::Lit, label, lit_text(expr), String::new()),
            // An application with a symbol head, or any other list: an op node.
            _ => self.go_op(items, label),
        }
    }

    fn go_match(&mut self, items: &[Sexpr], label: String) -> usize {
        let scrut = items.get(1).map(short).unwrap_or_default();
        let id = self.push(FlowKind::Match, label, format!("match {scrut}"), String::new());
        // items[2..] are arms: each a `(pattern body)` list.
        for arm in &items[2.min(items.len())..] {
            if let Sexpr::List(parts) = arm {
                let pat = parts.first().map(pretty).unwrap_or_default();
                let body = parts.get(1).cloned().unwrap_or(Sexpr::List(vec![]));
                let c = self.go(&body, pat);
                self.edges.push(FlowEdge { from: id, to: c, kind: EdgeKind::Control });
            }
        }
        id
    }

    fn go_if(&mut self, items: &[Sexpr], label: String) -> usize {
        let cond = items.get(1).map(short).unwrap_or_default();
        let id = self.push(FlowKind::If, label, format!("if {cond}"), String::new());
        if let Some(then) = items.get(2) {
            let c = self.go(then, "then".into());
            self.edges.push(FlowEdge { from: id, to: c, kind: EdgeKind::Control });
        }
        if let Some(els) = items.get(3) {
            let c = self.go(els, "else".into());
            self.edges.push(FlowEdge { from: id, to: c, kind: EdgeKind::Control });
        }
        id
    }

    fn go_let(&mut self, items: &[Sexpr], label: String) -> usize {
        // (let ((name val) ...) body)
        let binds: Vec<(String, Sexpr)> = match items.get(1) {
            Some(Sexpr::List(bs)) => bs
                .iter()
                .filter_map(|b| match b {
                    Sexpr::List(p) => {
                        let name = p.first()?.as_sym()?.to_string();
                        Some((name, p.get(1).cloned().unwrap_or(Sexpr::List(vec![]))))
                    }
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        };
        let names: Vec<&str> = binds.iter().map(|(n, _)| n.as_str()).collect();
        let id = self.push(FlowKind::Let, label, "let".into(), names.join(" "));
        // Each binding's value is a data child, headed by the binding name.
        for (name, val) in &binds {
            let c = self.go(val, name.clone());
            self.edges.push(FlowEdge { from: id, to: c, kind: EdgeKind::Data });
        }
        // The body is the control child (where evaluation continues).
        if let Some(body) = items.get(2) {
            let c = self.go(body, String::new());
            self.edges.push(FlowEdge { from: id, to: c, kind: EdgeKind::Control });
        }
        id
    }

    fn go_op(&mut self, items: &[Sexpr], label: String) -> usize {
        let head = match items.first() {
            Some(Sexpr::Sym(s)) => s.clone(),
            Some(Sexpr::List(_)) if items.first().map(|h| h.head()) == Some(Some("::")) => {
                // A qualified head `(:: a b name)` — show its short name.
                qualified_short(items.first().unwrap())
            }
            _ => "·".into(),
        };
        // Partition operands: simple (var/lit) stay inline; compound expand.
        let mut inline: Vec<String> = Vec::new();
        let mut compound: Vec<&Sexpr> = Vec::new();
        for arg in &items[1.min(items.len())..] {
            match arg {
                Sexpr::Sym(s) => inline.push(s.clone()),
                Sexpr::Int(_) | Sexpr::Str(_) => inline.push(lit_text(arg)),
                Sexpr::List(_) if arg.head() == Some("quote") => inline.push(lit_text(arg)),
                Sexpr::List(_) if arg.head() == Some("::") => inline.push(qualified_short(arg)),
                _ => compound.push(arg),
            }
        }
        let id = self.push(FlowKind::Op, label, head, elide(inline.join(" "), MAX_INLINE));
        for arg in compound {
            let c = self.go(arg, String::new());
            self.edges.push(FlowEdge { from: id, to: c, kind: EdgeKind::Data });
        }
        id
    }

    fn push(&mut self, kind: FlowKind, label: String, title: String, subtitle: String) -> usize {
        let id = self.nodes.len();
        self.nodes.push(FlowNode {
            kind,
            label,
            title,
            subtitle,
        });
        id
    }
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
        // Wrap the expression in a trivial fn and take its body.
        let form = parse_top(&format!("(fn f () T {src})")).unwrap().pop().unwrap();
        match form {
            Sexpr::List(items) => items[4..].to_vec(),
            _ => vec![],
        }
    }

    #[test]
    fn op_inlines_simple_operands_only() {
        let g = FlowGraph::build(&body("(int_eq th 59)"));
        // One op node, no children — both operands are inline.
        assert_eq!(g.nodes.len(), 1);
        assert_eq!(g.nodes[0].title, "int_eq");
        assert_eq!(g.nodes[0].subtitle, "th 59");
        assert!(g.edges.is_empty());
    }

    #[test]
    fn op_expands_compound_operand() {
        let g = FlowGraph::build(&body("(head_code (head_atom line))"));
        assert_eq!(g.nodes.len(), 2);
        assert_eq!(g.nodes[0].title, "head_code");
        assert_eq!(g.nodes[1].title, "head_atom");
        assert_eq!(g.nodes[1].subtitle, "line");
        assert_eq!(g.edges.len(), 1);
        assert_eq!(g.edges[0].kind, EdgeKind::Data);
    }

    #[test]
    fn match_arms_become_labelled_control_children() {
        let g = FlowGraph::build(&body(
            "(match (trim_left line) (Nil (Pair 6 cur)) ((Cons th tt) (Pair 5 cur)))",
        ));
        assert_eq!(g.nodes[0].kind, FlowKind::Match);
        assert_eq!(g.nodes[0].title, "match (trim_left line)");
        // Two arms → two control edges from the match node.
        let ctrl: Vec<_> = g.edges.iter().filter(|e| e.from == 0).collect();
        assert_eq!(ctrl.len(), 2);
        assert!(ctrl.iter().all(|e| e.kind == EdgeKind::Control));
        // Arm children are headed by their patterns.
        let labels: Vec<&str> = ctrl.iter().map(|e| g.nodes[e.to].label.as_str()).collect();
        assert!(labels.contains(&"Nil"));
        assert!(labels.contains(&"(Cons th tt)"));
    }

    #[test]
    fn measure_clause_is_skipped() {
        // A structural body `(measure (struct xs)) (match xs ...)` should chart
        // only the match — the measure clause is a totality annotation.
        let g = FlowGraph::build(&body("(measure (struct xs)) (match xs (Nil 0))"));
        assert_eq!(g.nodes[0].kind, FlowKind::Match);
        assert!(g.nodes.iter().all(|n| n.title != "measure" && n.title != "do"));
    }

    #[test]
    fn let_binding_is_data_body_is_control() {
        let g = FlowGraph::build(&body("(let ((code (head_atom line))) (Pair code code))"));
        assert_eq!(g.nodes[0].kind, FlowKind::Let);
        assert_eq!(g.nodes[0].subtitle, "code");
        let kinds: Vec<EdgeKind> = g.edges.iter().map(|e| e.kind).collect();
        assert!(kinds.contains(&EdgeKind::Data)); // the binding value
        assert!(kinds.contains(&EdgeKind::Control)); // the body
    }
}
