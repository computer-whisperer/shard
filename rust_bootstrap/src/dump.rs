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
//! `#i` is the de Bruijn index as loaded: since phase 3's slice 3.2 this
//! loader binds a `let` sequentially, as the V3 reader does (§5.4), so the
//! index is printed as it stands. A bare type name that is neither a
//! declared type of the closure nor `Int`/`Nat`/`Symbol` (the built-in E
//! types, §8.1 rule 1) is a type variable —
//! declared by a `(T Type)` binder (§8.1 rule 3) or auto-bound as the old
//! spelling had it; either way numbered by first occurrence, as the V3
//! dump numbers it. The measure clause is not in the AST (`load_fn_def`
//! steps over it) and not in the text.
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
    format!("fn {} {} {}", f.name, signature(&f.params, &f.ret, types), print(&f.body))
}

fn dump_extern(e: &ExternDef, types: &HashSet<&str>) -> String {
    format!("extern {} {}", e.name, signature(&e.params, &e.ret, types))
}

fn ty(t: &Type, vars: &mut Vec<String>, types: &HashSet<&str>) -> String {
    match t {
        Type::TVar(n) => var(n, vars),
        Type::TCon(n, args) if args.is_empty() => {
            // the built-in E types (v3/LANGUAGE.md §8.1 rule 1) print as
            // themselves; a declared type by its last component, as the V3
            // dump prints an identity (a dotted citation, `lib.Stack`)
            let short = short_name(n);
            if types.contains(short) || n == "Int" || n == "Nat" || n == "Symbol" {
                short.to_string()
            } else {
                var(n, vars)
            }
        }
        Type::TCon(n, args) => {
            let inner: Vec<String> = args.iter().map(|a| ty(a, vars, types)).collect();
            format!("({} {})", short_name(n), inner.join(" "))
        }
    }
}

fn short_name(n: &str) -> &str {
    n.rsplit('.').next().unwrap_or(n)
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

/// The body as loaded: `#i` the de Bruijn index, `_` a pattern variable.
fn print(e: &Expr) -> String {
    match e {
        Expr::BVar(i) => format!("#{}", i),
        Expr::FVar(n) => format!("(free:{})", n),
        Expr::IntLit(n) => n.to_string(),
        Expr::SymLit(s) => format!("'{}", s),
        Expr::Ctor(n, args) | Expr::Call(n, args) => {
            let mut s = format!("({}", n);
            for a in args {
                s.push(' ');
                s.push_str(&print(a));
            }
            s.push(')');
            s
        }
        Expr::Match(scrut, arms) => {
            let mut s = format!("(match {}", print(scrut));
            for arm in arms {
                s.push_str(&format!(" ({} {})", pat(&arm.pat), print(&arm.body)));
            }
            s.push(')');
            s
        }
        Expr::Let(rhss, body) => {
            let rs: Vec<String> = rhss.iter().map(print).collect();
            format!("(let ({}) {})", rs.join(" "), print(body))
        }
        Expr::If(c, a, b) => format!("(if {} {} {})", print(c), print(a), print(b)),
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
