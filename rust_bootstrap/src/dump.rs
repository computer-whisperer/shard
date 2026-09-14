//! `eval dump FILE` — the bootstrap's side of frontend parity
//! (v3/LANGUAGE.md §10 item 1, slice 6): the loaded closure printed as the
//! canonical text `v3/kernel/dump.shard` prints for the V3 reader's `Prog`
//! over the same files, one line per declaration, sorted bytewise:
//!
//!   type NAME n (CTOR T…)…        n type parameters; names as written
//!   fn NAME n (T…) T BODY         n = the distinct type variables of the signature,
//!   extern NAME n (T…) T            numbered ?i by first occurrence across the params then the result
//!   T   ::= Int | Symbol | NAME | (NAME T…) | ?i
//!   E   ::= #i | INT | 'sym | (NAME E…) | (match E (PAT E)…) | (let (E…) E) | (if E E E)
//!   PAT ::= _ | NAME | (NAME PAT…) | INT | 'sym
//!
//! `#i` is the SEQUENTIAL de Bruijn index: this loader binds a `let` group
//! in parallel (every right-hand side in the outer scope), the V3 reader
//! sequentially (each sees the bindings before it), so the printer first
//! resolves every variable to its binder under the parallel rule and then
//! numbers it under the sequential one (§13 item 25). A bare type name that
//! is neither a declared type of the closure nor `Int`/`Symbol` is a type
//! variable, as the V3 reader auto-binds it. The measure clause is not in
//! the AST (`load_fn_def` steps over it) and not in the text.
//!
//! The text is a projection (dump.shard's header; GPT-6 R53): last name
//! components, one rendering for a constructor, a call and an extern, no
//! measure clause, one literal kind. The parity harness checks each closure
//! for one declaration per short name per kind before comparing, so byte
//! agreement is evidence where the projection is injective.

use crate::ast::{Expr, ExternDef, FnDef, Module, Pat, Type, TypeDef};
use std::collections::HashSet;

pub fn dump(m: &Module) -> String {
    let types: HashSet<&str> = m.types.iter().map(|t| t.name.as_str()).collect();
    let mut lines: Vec<String> = Vec::new();
    for t in &m.types {
        lines.push(dump_type(t, &types));
    }
    for f in &m.fns {
        lines.push(dump_fn(f, &types));
    }
    for e in &m.externs {
        lines.push(dump_extern(e, &types));
    }
    lines.sort();
    let mut out = lines.join("\n");
    out.push('\n');
    out
}

fn dump_type(t: &TypeDef, types: &HashSet<&str>) -> String {
    // the type's own parameters are ?0.. in declared order
    let mut vars: Vec<String> = t.params.clone();
    let mut s = format!("type {} {}", t.name, t.params.len());
    for c in &t.ctors {
        s.push_str(" (");
        s.push_str(&c.name);
        for f in &c.fields {
            s.push(' ');
            s.push_str(&ty(f, &mut vars, types));
        }
        s.push(')');
    }
    s
}

fn signature(params: &[Type], ret: &Type, types: &HashSet<&str>) -> String {
    let mut vars: Vec<String> = Vec::new();
    let ps: Vec<String> = params.iter().map(|p| ty(p, &mut vars, types)).collect();
    let r = ty(ret, &mut vars, types);
    format!("{} ({}) {}", vars.len(), ps.join(" "), r)
}

fn dump_fn(f: &FnDef, types: &HashSet<&str>) -> String {
    // parameters are the outermost binders: ids 0..n, the last innermost
    let mut ctx: Vec<u32> = (0..f.params.len() as u32).collect();
    let mut next = f.params.len() as u32;
    let body = resolve(&f.body, &mut ctx, &mut next);
    format!("fn {} {} {}", f.name, signature(&f.params, &f.ret, types), print(&body, &mut ctx))
}

fn dump_extern(e: &ExternDef, types: &HashSet<&str>) -> String {
    format!("extern {} {}", e.name, signature(&e.params, &e.ret, types))
}

fn ty(t: &Type, vars: &mut Vec<String>, types: &HashSet<&str>) -> String {
    match t {
        Type::TVar(n) => var(n, vars),
        Type::TCon(n, args) if args.is_empty() => {
            if types.contains(n.as_str()) || n == "Int" || n == "Symbol" {
                n.clone()
            } else {
                var(n, vars)
            }
        }
        Type::TCon(n, args) => {
            let inner: Vec<String> = args.iter().map(|a| ty(a, vars, types)).collect();
            format!("({} {})", n, inner.join(" "))
        }
    }
}

fn var(n: &str, vars: &mut Vec<String>) -> String {
    let i = match vars.iter().position(|v| v == n) {
        Some(i) => i,
        None => {
            vars.push(n.to_string());
            vars.len() - 1
        }
    };
    format!("?{}", i)
}

/// The body with every variable resolved to its binder's id under the
/// loader's (parallel `let`) scoping.
enum R {
    Var(u32),
    Int(String),
    Sym(String),
    App(String, Vec<R>),
    Match(Box<R>, Vec<(Pat, Vec<u32>, R)>),
    Let(Vec<R>, Vec<u32>, Box<R>),
    If(Box<R>, Box<R>, Box<R>),
}

fn resolve(e: &Expr, ctx: &mut Vec<u32>, next: &mut u32) -> R {
    match e {
        Expr::BVar(i) => R::Var(ctx[ctx.len() - 1 - *i as usize]),
        Expr::FVar(n) => R::App(format!("free:{}", n), Vec::new()),
        Expr::IntLit(n) => R::Int(n.to_string()),
        Expr::SymLit(s) => R::Sym(s.clone()),
        Expr::Ctor(n, args) | Expr::Call(n, args) => {
            R::App(n.clone(), args.iter().map(|a| resolve(a, ctx, next)).collect())
        }
        Expr::Match(s, arms) => {
            let sr = resolve(s, ctx, next);
            let mut out = Vec::new();
            for arm in arms {
                let k = pat_vars(&arm.pat);
                let ids: Vec<u32> = (0..k).map(|_| fresh(next)).collect();
                let saved = ctx.len();
                ctx.extend(ids.iter().cloned());
                let b = resolve(&arm.body, ctx, next);
                ctx.truncate(saved);
                out.push((arm.pat.clone(), ids, b));
            }
            R::Match(Box::new(sr), out)
        }
        Expr::Let(rhss, body) => {
            // parallel: every right-hand side in the outer scope
            let rs: Vec<R> = rhss.iter().map(|r| resolve(r, ctx, next)).collect();
            let ids: Vec<u32> = (0..rhss.len()).map(|_| fresh(next)).collect();
            let saved = ctx.len();
            ctx.extend(ids.iter().cloned());
            let b = resolve(body, ctx, next);
            ctx.truncate(saved);
            R::Let(rs, ids, Box::new(b))
        }
        Expr::If(c, a, b) => R::If(
            Box::new(resolve(c, ctx, next)),
            Box::new(resolve(a, ctx, next)),
            Box::new(resolve(b, ctx, next)),
        ),
    }
}

fn fresh(next: &mut u32) -> u32 {
    let id = *next;
    *next += 1;
    id
}

fn pat_vars(p: &Pat) -> usize {
    match p {
        Pat::PVar => 1,
        Pat::PCtor(_, subs) => subs.iter().map(pat_vars).sum(),
        Pat::PInt(_) | Pat::PSym(_) => 0,
    }
}

/// The resolved body printed under SEQUENTIAL scoping: a `let` pushes its
/// binders one at a time, each right-hand side under the ones before it.
fn print(r: &R, ctx: &mut Vec<u32>) -> String {
    match r {
        R::Var(id) => {
            let pos = ctx.iter().rposition(|x| x == id).expect("bound variable");
            format!("#{}", ctx.len() - 1 - pos)
        }
        R::Int(s) => s.clone(),
        R::Sym(s) => format!("'{}", s),
        R::App(n, args) => {
            let mut s = format!("({}", n);
            for a in args {
                s.push(' ');
                s.push_str(&print(a, ctx));
            }
            s.push(')');
            s
        }
        R::Match(scrut, arms) => {
            let mut s = format!("(match {}", print(scrut, ctx));
            for (p, ids, b) in arms {
                let saved = ctx.len();
                ctx.extend(ids.iter().cloned());
                s.push_str(&format!(" ({} {})", pat(p), print(b, ctx)));
                ctx.truncate(saved);
            }
            s.push(')');
            s
        }
        R::Let(rs, ids, body) => {
            let saved = ctx.len();
            let mut s = String::from("(let (");
            for (j, r) in rs.iter().enumerate() {
                if j > 0 {
                    s.push(' ');
                }
                s.push_str(&print(r, ctx));
                ctx.push(ids[j]);
            }
            s.push_str(") ");
            s.push_str(&print(body, ctx));
            s.push(')');
            ctx.truncate(saved);
            s
        }
        R::If(c, a, b) => format!("(if {} {} {})", print(c, ctx), print(a, ctx), print(b, ctx)),
    }
}

fn pat(p: &Pat) -> String {
    match p {
        Pat::PVar => "_".to_string(),
        Pat::PCtor(n, subs) if subs.is_empty() => n.clone(),
        Pat::PCtor(n, subs) => {
            let inner: Vec<String> = subs.iter().map(pat).collect();
            format!("({} {})", n, inner.join(" "))
        }
        Pat::PInt(n) => n.to_string(),
        Pat::PSym(s) => format!("'{}", s),
    }
}
