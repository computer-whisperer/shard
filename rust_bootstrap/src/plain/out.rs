//! What a loaded closure is handed on as: the canonical text of frontend
//! parity (`eval dump`; `v3/kernel/dump.shard` prints the same for the V3
//! loader's program, and its header states the format), and the evaluator's
//! Module, every name an identity.

use crate::ast;

use super::{Decl, ETy, Lit, Pat, Tables, Tm, last_component};

/// A constructor prints under its type: the last two components of its identity
/// (dump.shard dm_ctor) — Init has several constructors named `mk`.
fn ctor_name(n: &str) -> &str {
    match n.rmatch_indices('.').nth(1) {
        Some((i, _)) => &n[i + 1..],
        None => n,
    }
}

/// A head prints as its identity beyond the module it was declared in.
fn head(t: &Tables, n: &str) -> String {
    match t.module_of(n) {
        None => last_component(n).to_string(),
        Some(m) => {
            let mc: Vec<&str> = if m.is_empty() { Vec::new() } else { m.split('.').collect() };
            let nc: Vec<&str> = n.split('.').collect();
            let mut k = 0;
            while k < mc.len() && k < nc.len() && mc[k] == nc[k] {
                k += 1;
            }
            if k == nc.len() { "anon".to_string() } else { nc[k..].join(".") }
        }
    }
}

fn ty(t: &ETy, order: &[usize]) -> String {
    match t {
        ETy::Var(i) => format!("?{}", order.iter().position(|x| x == i).unwrap_or(*i)),
        ETy::Con(n, args) if args.is_empty() => last_component(n).to_string(),
        ETy::Con(n, args) => {
            let inner: Vec<String> = args.iter().map(|a| ty(a, order)).collect();
            format!("({} {})", last_component(n), inner.join(" "))
        }
    }
}

fn vars_of(t: &ETy, acc: &mut Vec<usize>) {
    match t {
        ETy::Var(i) => {
            if !acc.contains(i) {
                acc.push(*i);
            }
        }
        ETy::Con(_, args) => args.iter().for_each(|a| vars_of(a, acc)),
    }
}

/// `n (T…) T` with the type variables renumbered by first occurrence.
fn signature(ps: &[ETy], rt: &ETy) -> String {
    let mut order = Vec::new();
    ps.iter().for_each(|p| vars_of(p, &mut order));
    vars_of(rt, &mut order);
    let inner: Vec<String> = ps.iter().map(|p| ty(p, &order)).collect();
    format!("{} ({}) {}", order.len(), inner.join(" "), ty(rt, &order))
}

fn lit(l: &Lit) -> String {
    match l {
        Lit::Nat(k) | Lit::Int(k) => k.to_string(),
        Lit::Sym(s) => format!("'{s}"),
        Lit::Str(nl, cs, bs) => {
            let (nl, cs) = (ctor_name(nl), ctor_name(cs));
            let mut s = format!("({nl})");
            for b in bs.iter().rev() {
                s = format!("({cs} {b} {s})");
            }
            s
        }
    }
}

fn pat(p: &Pat) -> String {
    match p {
        Pat::Var => "_".to_string(),
        Pat::Ctor(n, subs) if subs.is_empty() => ctor_name(n).to_string(),
        Pat::Ctor(n, subs) => {
            let inner: Vec<String> = subs.iter().map(pat).collect();
            format!("({} {})", ctor_name(n), inner.join(" "))
        }
        Pat::Lit(l) => lit(l),
    }
}

fn app(t: &Tables, h: &str, args: &[Tm]) -> String {
    let mut s = format!("({h}");
    for a in args {
        s.push(' ');
        s.push_str(&term(t, a));
    }
    s.push(')');
    s
}

fn term(t: &Tables, tm: &Tm) -> String {
    match tm {
        Tm::BVar(i) => format!("#{i}"),
        Tm::Lit(l) => lit(l),
        Tm::Ctor(n, args) => app(t, ctor_name(n), args),
        Tm::Call(n, args) | Tm::Ext(n, args) => app(t, &head(t, n), args),
        Tm::Prim(n, args) => app(t, n, args),
        Tm::Match(s, arms) => {
            let mut out = format!("(match {}", term(t, s));
            for (p, b) in arms {
                out.push_str(&format!(" ({} {})", pat(p), term(t, b)));
            }
            out.push(')');
            out
        }
        Tm::Let(rs, b) => {
            let inner: Vec<String> = rs.iter().map(|r| term(t, r)).collect();
            format!("(let ({}) {})", inner.join(" "), term(t, b))
        }
        Tm::If(c, a, b) => format!("(if {} {} {})", term(t, c), term(t, a), term(t, b)),
        Tm::Absurd => "(absurd)".to_string(),
    }
}

/// The closure's program as the canonical text: one line per declaration,
/// sorted bytewise. A fn whose body has no reading has no line.
pub fn dump(t: &Tables) -> String {
    let mut lines: Vec<String> = Vec::new();
    for n in &t.order {
        match &t.decls[n] {
            Decl::Ind { name, nparams, ctors } => {
                let mut s = format!("type {} {}", last_component(name), nparams);
                for (c, fts) in ctors {
                    s.push_str(&format!(" ({}", ctor_name(c)));
                    for ft in fts {
                        s.push(' ');
                        s.push_str(&ty(ft, &[]));
                    }
                    s.push(')');
                }
                lines.push(s);
            }
            Decl::Fn { name, params, ret, body: Some(b), .. } => {
                lines.push(format!("fn {} {} {}", head(t, name), signature(params, ret), term(t, b)));
            }
            Decl::Fn { body: None, .. } => {}
            Decl::Extern { name, params, ret, .. } => {
                lines.push(format!("extern {} {}", head(t, name), signature(params, ret)));
            }
            Decl::Sig { name, params, ret, .. } => {
                lines.push(format!("sig {} {}", head(t, name), signature(params, ret)));
            }
            Decl::SigType { name, arity } => lines.push(format!("sigtype {} {}", last_component(name), arity)),
        }
    }
    lines.sort();
    let mut out = lines.join("\n");
    out.push('\n');
    out
}

/// The runtime's cells go to the evaluator by the names the runtime gives them: `True`,
/// `False`, `Nil`, `Cons`, `None`, `Some`, `Pair`. They are the constructors of Init's `Bool`,
/// `List`, `Option` and `Prod`, by identity (lower.shard's wire roles are the same rule). The
/// evaluator's comparisons return `True` and `False`, its symbol bridge and the host speak
/// `Cons` and `Nil` cells. Every other constructor goes by its identity.
fn cell(id: &str) -> String {
    match id {
        "Bool.true" => "True".into(),
        "Bool.false" => "False".into(),
        "List.nil" => "Nil".into(),
        "List.cons" => "Cons".into(),
        "Option.none" => "None".into(),
        "Option.some" => "Some".into(),
        "Prod.mk" => "Pair".into(),
        _ => id.to_string(),
    }
}

fn ast_ty(t: &ETy) -> ast::Type {
    match t {
        ETy::Var(i) => ast::Type::TVar(format!("T{i}")),
        ETy::Con(n, args) => ast::Type::TCon(n.clone(), args.iter().map(ast_ty).collect()),
    }
}

fn ast_lit(l: &Lit) -> ast::Expr {
    match l {
        Lit::Nat(k) | Lit::Int(k) => ast::Expr::IntLit(k.clone()),
        Lit::Sym(s) => ast::Expr::SymLit(s.clone()),
        Lit::Str(nl, cs, bs) => {
            let mut acc = ast::Expr::Ctor(cell(nl), Vec::new());
            for b in bs.iter().rev() {
                acc = ast::Expr::Ctor(cell(cs), vec![ast::Expr::IntLit((*b).into()), acc]);
            }
            acc
        }
    }
}

fn ast_pat(p: &Pat) -> ast::Pat {
    match p {
        Pat::Var => ast::Pat::PVar,
        Pat::Ctor(n, subs) => ast::Pat::PCtor(cell(n), subs.iter().map(ast_pat).collect()),
        Pat::Lit(Lit::Nat(k)) | Pat::Lit(Lit::Int(k)) => ast::Pat::PInt(k.clone()),
        Pat::Lit(Lit::Sym(s)) => ast::Pat::PSym(s.clone()),
        // the reader gives a pattern no string
        Pat::Lit(Lit::Str(..)) => ast::Pat::PSym(String::new()),
    }
}

fn ast_term(tm: &Tm) -> ast::Expr {
    let all = |ts: &[Tm]| -> Vec<ast::Expr> { ts.iter().map(ast_term).collect() };
    match tm {
        Tm::BVar(i) => ast::Expr::BVar(*i as u32),
        Tm::Lit(l) => ast_lit(l),
        Tm::Ctor(n, args) => ast::Expr::Ctor(cell(n), all(args)),
        // a call's head is its identity, a primitive's its table entry's name, an extern's the
        // name the host is asked by
        Tm::Call(n, args) | Tm::Ext(n, args) | Tm::Prim(n, args) => ast::Expr::Call(n.clone(), all(args)),
        Tm::Match(s, arms) => ast::Expr::Match(
            Box::new(ast_term(s)),
            arms.iter().map(|(p, b)| ast::Arm { pat: ast_pat(p), body: ast_term(b) }).collect(),
        ),
        Tm::Let(rs, b) => ast::Expr::Let(all(rs), Box::new(ast_term(b))),
        Tm::If(c, a, b) => ast::Expr::If(Box::new(ast_term(c)), Box::new(ast_term(a)), Box::new(ast_term(b))),
        // a refuted row: stuck if reached
        Tm::Absurd => ast::Expr::Call("absurd".into(), Vec::new()),
    }
}

/// The closure as the evaluator runs it. A fn whose body has no reading is
/// a call the evaluator has no answer for: reaching it stops the run with
/// its name.
pub fn module(t: &Tables) -> ast::Module {
    let mut m = ast::Module::default();
    for n in &t.order {
        match &t.decls[n] {
            Decl::Ind { name, nparams, ctors } => m.types.push(ast::TypeDef {
                name: name.clone(),
                params: (0..*nparams).map(|i| format!("T{i}")).collect(),
                ctors: ctors
                    .iter()
                    .map(|(c, fts)| ast::CtorDef { name: cell(c), fields: fts.iter().map(ast_ty).collect() })
                    .collect(),
            }),
            Decl::Fn { name, params, ret, body, .. } => m.fns.push(ast::FnDef {
                name: name.clone(),
                params: params.iter().map(ast_ty).collect(),
                ret: ast_ty(ret),
                body: match body {
                    Some(b) => ast_term(b),
                    None => ast::Expr::Call(format!("unread:{name}"), Vec::new()),
                },
            }),
            Decl::Extern { name, params, ret, .. } => {
                m.externs.push(ast::ExternDef { name: name.clone(), params: params.iter().map(ast_ty).collect(), ret: ast_ty(ret) })
            }
            Decl::Sig { .. } | Decl::SigType { .. } => {}
        }
    }
    m
}

/// What a run of the closure starts from: its first root module's `main`, and the
/// constructor of the World its one parameter is typed at.
pub fn entry(l: &super::Loader) -> Result<(String, String), String> {
    let Some(root) = l.roots.first() else { return Err("no root file".into()) };
    let main = super::append(root, "main");
    let Some(Decl::Fn { params, .. }) = l.t.find_decl(&main) else {
        return Err(format!("{main}: the root module declares no fn main"));
    };
    let world = match params.as_slice() {
        [ETy::Con(w, args)] if args.is_empty() => w,
        _ => return Err(format!("{main}: a program's main takes the World alone")),
    };
    match l.t.struct_ctor(world) {
        Some(c) => Ok((main, c)),
        None => Err(format!("{main}: {world} is not a one-constructor type")),
    }
}
