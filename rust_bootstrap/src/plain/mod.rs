//! The reader: a closure read as the V3 loader's plain route reads it
//! (`v3/kernel/classify.shard`) — every name an identity (the module path
//! plus the declared name; an imported constant of Init its own name), a
//! citation resolved through the file's scope (`use`, the file's own types,
//! the modules its imports make visible), a body read with the types its
//! binders state: which written arguments run, an operator's entry by its
//! operands' type, a proof and a type passed over. The toolchain's own
//! sources and an ordinary program are read alike, and every file sees Init.
//!
//! What the classifier asks K about a constant of Init — its kind, its
//! binders' roles, an inductive's constructors — this reader takes from a
//! table the loader prints (`load.shard --facts`, `v3/kernel/facts.shard`):
//! it has no K.
//!
//! A directory module is its view; every view loaded is then linked to its
//! implementation, whose `fn`s and `type`s take the places of the view's
//! `sig`s (`Loader::link_views`), so the closure runs.
//!
//! It reads; it does not refuse. Exhaustiveness, the view's privacy, the
//! match of an implementation against its view, a proof nothing checked are
//! the loader's refusals, made where the same sources are loaded by it. A
//! form this reader has no reading for is recorded (`Tables::unread`) and its
//! program left out.
//!
//!   mod.rs     the data, the tables, resolution, the closure's files
//!   record.rs  the host parser's entry, records expanded
//!   facts.rs   the table of Init's constants
//!   read.rs    types, terms, patterns, declarations (classify.shard's read_*)
//!   settle.rs  static types, the operators by their operands' types, the rows
//!   out.rs     the canonical text (`eval dump`), the evaluator's Module

use std::collections::HashMap;
use std::path::{Path, PathBuf};

use lexpr::Value;

pub mod facts;
pub mod out;
pub mod read;
pub mod record;
pub mod settle;

use facts::{Fact, Flag, Role};

pub type Name = String;

/// A form or a citation this reader has no reading for: the reason (the
/// classifier's refusal where there is one) and what it was about.
#[derive(Clone, Debug)]
pub struct Unread(pub &'static str, pub String);
pub type R<T> = Result<T, Unread>;

pub fn unread<T>(reason: &'static str, about: impl Into<String>) -> R<T> {
    Err(Unread(reason, about.into()))
}

// -----------------------------------------------------------------------------
// Programs (v3/kernel/prog.shard)
// -----------------------------------------------------------------------------

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ETy {
    Con(Name, Vec<ETy>),
    Var(usize),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Lit {
    Nat(crate::ast::IntLit),
    Int(crate::ast::IntLit),
    /// "…": nil, cons, the UTF-8 bytes
    Str(Vec<u8>),
    Sym(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Pat {
    Var,
    Ctor(Name, Vec<Pat>),
    Lit(Lit),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Tm {
    BVar(usize),
    Lit(Lit),
    Ctor(Name, Vec<Tm>),
    Call(Name, Vec<Tm>),
    Prim(Name, Vec<Tm>),
    Ext(Name, Vec<Tm>),
    Match(Box<Tm>, Vec<(Pat, Tm)>),
    Let(Vec<Tm>, Box<Tm>),
    If(Box<Tm>, Box<Tm>, Box<Tm>),
    Absurd,
}

#[derive(Clone, Debug)]
pub enum Decl {
    Ind { name: Name, nparams: usize, ctors: Vec<(Name, Vec<ETy>)> },
    /// `body` is None until the body is read, and stays None where it has no reading
    Fn { name: Name, ntp: usize, params: Vec<ETy>, ret: ETy, body: Option<Tm> },
    Extern { name: Name, ntp: usize, params: Vec<ETy>, ret: ETy },
    Sig { name: Name, ntp: usize, params: Vec<ETy>, ret: ETy },
    SigType { name: Name, arity: usize },
}

impl Decl {
    pub fn name(&self) -> &Name {
        match self {
            Decl::Ind { name, .. }
            | Decl::Fn { name, .. }
            | Decl::Extern { name, .. }
            | Decl::Sig { name, .. }
            | Decl::SigType { name, .. } => name,
        }
    }
}

// -----------------------------------------------------------------------------
// Names and scopes (v3/kernel/scope.shard)
// -----------------------------------------------------------------------------

pub fn append(p: &str, n: &str) -> Name {
    if p.is_empty() {
        n.to_string()
    } else if n.is_empty() {
        p.to_string()
    } else {
        format!("{p}.{n}")
    }
}

pub fn first_component(n: &str) -> &str {
    n.split('.').next().unwrap_or(n)
}

pub fn last_component(n: &str) -> &str {
    n.rsplit('.').next().unwrap_or(n)
}

fn init_prefixed(n: &str) -> bool {
    first_component(n) == "Init"
}

/// The name without its leading `Init` component (`(use Init.List)` opens `List`).
fn drop_init(n: &str) -> Name {
    if n == "Init" {
        String::new()
    } else if let Some(r) = n.strip_prefix("Init.") {
        r.to_string()
    } else {
        n.to_string()
    }
}

#[derive(Clone, Debug)]
pub enum Open {
    All(Name),
    Some(Name, Vec<String>),
}

/// A file's scope at a form: the prefix its declarations take, its own
/// module, the opened prefixes in order, the visible modules. Every file sees Init.
#[derive(Clone, Debug)]
pub struct Scope {
    pub ident: Name,
    pub tag: Name,
    pub opened: Vec<Open>,
    pub vis: Vec<Name>,
}

impl Scope {
    pub fn qualify(&self, n: &str) -> Name {
        append(&self.ident, n)
    }

    /// Resolution's candidates: the module's own declarations, each opened
    /// prefix in order, `Init.NAME` as the bare imported name, the bare name.
    pub fn candidates(&self, n: &str) -> Vec<Name> {
        let mut out = Vec::new();
        if !self.ident.is_empty() {
            out.push(append(&self.ident, n));
        }
        let first = first_component(n);
        for o in &self.opened {
            match o {
                Open::All(p) => out.push(append(p, n)),
                Open::Some(p, ms) => {
                    if ms.iter().any(|m| m == first) {
                        out.push(append(p, n));
                    }
                }
            }
        }
        if init_prefixed(n) {
            out.push(drop_init(n));
        }
        out.push(n.to_string());
        out
    }

    /// Where among the candidates of the citation `n` the name `id` stands, if it is one
    /// of them: a rank that orders names as `candidates(n)` does, found without building
    /// them (a toolchain file opens fifty prefixes, and a body cites thousands of names).
    pub fn rank(&self, id: &str, n: &str) -> Option<usize> {
        let pre = if id == n {
            Some("")
        } else if id.len() > n.len() + 1 && id.ends_with(n) && id.as_bytes()[id.len() - n.len() - 1] == b'.' {
            Some(&id[..id.len() - n.len() - 1])
        } else {
            None
        };
        if let Some(pre) = pre {
            if !pre.is_empty() && pre == self.ident {
                return Some(0);
            }
            let first = first_component(n);
            for (i, o) in self.opened.iter().enumerate() {
                match o {
                    Open::All(p) if p == pre => return Some(1 + i),
                    Open::Some(p, ms) if p == pre && ms.iter().any(|m| m == first) => return Some(1 + i),
                    _ => {}
                }
            }
        }
        if init_prefixed(n) && id == drop_init(n) {
            return Some(1 + self.opened.len());
        }
        if id == n { Some(2 + self.opened.len()) } else { None }
    }

    fn sees(&self, module: &str) -> bool {
        module == "Init" || self.vis.iter().any(|m| m == module)
    }
}

// -----------------------------------------------------------------------------
// The tables: the E table (etable.shard), what K holds of Init (the facts)
// and of the closure's own declarations of the logic
// -----------------------------------------------------------------------------

#[derive(Clone, Debug)]
pub enum EInfo {
    Ctor { ind: Name, ord: usize, nfields: usize },
    Ind { nparams: usize, ctors: Vec<Name> },
    Fn(usize),
    Sig(usize),
    Ext(usize),
    SigType(usize),
    /// a fn with a function-typed binder: no program of its own
    Template,
}

/// A declaration of the logic the closure makes: what the plain reading
/// needs of it is its kind, whether its type ends in Prop, and for a `def`
/// its source (a type alias is read off its value).
#[derive(Clone, Debug)]
pub struct Native {
    pub module: Name,
    pub kind: NativeKind,
    pub prop: bool,
    pub form: Option<(std::rc::Rc<Scope>, Value)>,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NativeKind {
    Def,
    Theorem,
    Axiom,
    Opaque,
    Structure,
    Inductive,
}

pub enum Res {
    Name(Name),
    None,
    Ambiguous(Name, Name),
}

#[derive(Default)]
pub struct Tables {
    /// identity → the module it is registered under, its kind
    pub info: HashMap<Name, (Name, EInfo)>,
    /// the registered identities by their last component: a citation's candidates all
    /// end in the citation's
    by_last: HashMap<String, Vec<Name>>,
    /// the constants K holds beside those — a file's native declarations, Init's — by their
    /// last component
    k_last: HashMap<String, Vec<Name>>,
    pub decls: HashMap<Name, Decl>,
    /// the declarations in the order they were reached
    pub order: Vec<Name>,
    /// a native head's written binders: which a call writes, which run
    pub flags: HashMap<Name, Vec<Flag>>,
    pub facts: HashMap<Name, Fact>,
    /// Init's inductives that are E types and no program form has cited yet
    init_views: HashMap<Name, (usize, Vec<(Name, Vec<ETy>)>)>,
    pub natives: HashMap<Name, Native>,
    /// a view's parameter an implementation took the identity of: the view's module, which
    /// a consumer sees where it does not see the implementation's
    param_of: HashMap<Name, Name>,
    /// what was left unread: the identity (or the file), the reason
    pub unread: Vec<(Name, Unread)>,
}

impl Tables {
    /// An identity registered under its module. A `fn` or a `type` registered where a view's
    /// `sig` stands takes its place (etable.shard es_merge's substitution): the identity is
    /// then citable from the view's module as from the implementation's.
    pub fn register(&mut self, id: &str, module: &str, info: EInfo) {
        if let Some((m, EInfo::Sig(_) | EInfo::SigType(_))) = self.info.get(id) {
            if m != module {
                self.param_of.insert(id.to_string(), m.clone());
            }
        }
        if !self.info.contains_key(id) {
            self.by_last.entry(last_component(id).to_string()).or_default().push(id.to_string());
        }
        self.info.insert(id.to_string(), (module.to_string(), info));
    }

    /// A native declaration of the logic, recorded under its identity.
    pub fn add_native(&mut self, id: Name, nv: Native) {
        if !self.natives.contains_key(&id) {
            self.k_last.entry(last_component(&id).to_string()).or_default().push(id.clone());
        }
        self.natives.insert(id, nv);
    }

    /// Does the scope see the identity registered under `module`?
    fn in_sight(&self, sc: &Scope, id: &str, module: &str) -> bool {
        sc.sees(module) || self.param_of.get(id).is_some_and(|v| sc.sees(v))
    }

    pub fn add_decl(&mut self, d: Decl) {
        let n = d.name().clone();
        if !self.decls.contains_key(&n) {
            self.order.push(n.clone());
        }
        self.decls.insert(n, d);
    }

    /// An inductive of Init that is an E type, registered under the module Init on its first
    /// citation by a program form, with the inductives its fields are at (erase.shard
    /// ind_eligible).
    fn register_init(&mut self, n: &str) {
        let Some((np, ctors)) = self.init_views.remove(n) else { return };
        self.register(n, "Init", EInfo::Ind { nparams: np, ctors: ctors.iter().map(|(c, _)| c.clone()).collect() });
        for (i, (c, fts)) in ctors.iter().enumerate() {
            self.register(c, "Init", EInfo::Ctor { ind: n.to_string(), ord: i, nfields: fts.len() });
        }
        let mut cited = Vec::new();
        for (_, fts) in &ctors {
            for ft in fts {
                ety_names(ft, &mut cited);
            }
        }
        self.add_decl(Decl::Ind { name: n.to_string(), nparams: np, ctors });
        for c in cited {
            self.register_init(&c);
        }
    }

    /// The pre-scan of a program form: each of its names that
    /// resolves to an inductive of Init that is an E type, or to a constructor of one,
    /// registers it (erase.shard es_scan_init). A string is List cells.
    pub fn scan_init(&mut self, sc: &Scope, v: &Value) {
        if v.as_str().is_some() {
            return self.scan_name(sc, "List");
        }
        if let Some(x) = v.as_symbol() {
            return self.scan_name(sc, strip_at(x));
        }
        if let Some(items) = as_list(v) {
            for i in items {
                self.scan_init(sc, i);
            }
        }
    }

    fn scan_name(&mut self, sc: &Scope, n: &str) {
        if self.init_views.is_empty() {
            return;
        }
        let Some(k) = self.k_name(sc, n) else { return };
        let ind = match self.facts.get(&k) {
            Some(Fact::Ind { .. }) => k,
            Some(Fact::Ctor { ind, .. }) => ind.clone(),
            _ => return,
        };
        self.register_init(&ind);
    }

    pub fn einfo(&self, id: &str) -> Option<&EInfo> {
        self.info.get(id).map(|(_, i)| i)
    }

    pub fn module_of(&self, id: &str) -> Option<&Name> {
        self.info.get(id).map(|(m, _)| m)
    }

    /// etable.shard's eresolve: the E identities a citation names in this scope, in the
    /// order of the scope's candidates.
    pub fn e_hits(&self, sc: &Scope, n: &str) -> Vec<(Name, &EInfo)> {
        let Some(ids) = self.by_last.get(last_component(n)) else { return Vec::new() };
        let mut ranked: Vec<(usize, &Name, &EInfo)> = Vec::new();
        for id in ids {
            let Some(r) = sc.rank(id, n) else { continue };
            if let Some((m, i)) = self.info.get(id) {
                if self.in_sight(sc, id, m) {
                    ranked.push((r, id, i));
                }
            }
        }
        ranked.sort_by_key(|(r, _, _)| *r);
        ranked.into_iter().map(|(_, id, i)| (id.clone(), i)).collect()
    }

    /// The hits a term position sees: never a type.
    pub fn e_values(&self, sc: &Scope, n: &str) -> Vec<(Name, &EInfo)> {
        let mut hs = self.e_hits(sc, n);
        hs.retain(|(_, i)| !matches!(i, EInfo::Ind { .. } | EInfo::SigType(_)));
        hs
    }

    pub fn e_ctors(&self, sc: &Scope, n: &str) -> Vec<(Name, &EInfo)> {
        let mut hs = self.e_hits(sc, n);
        hs.retain(|(_, i)| matches!(i, EInfo::Ctor { .. }));
        hs
    }

    pub fn e_types(&self, sc: &Scope, n: &str) -> Vec<(Name, &EInfo)> {
        let mut hs = self.e_hits(sc, n);
        hs.retain(|(_, i)| matches!(i, EInfo::Ind { .. } | EInfo::SigType(_)));
        hs
    }

    /// Is `k` a constant K holds that this scope may cite? A constant of
    /// Init; a declaration of the closure in a visible module.
    fn k_known(&self, sc: &Scope, k: &str) -> bool {
        if let Some(nv) = self.natives.get(k) {
            if sc.sees(&nv.module) {
                return true;
            }
        }
        if let Some((m, _)) = self.info.get(k) {
            if m != "Init" && self.in_sight(sc, k, m) {
                return true;
            }
        }
        self.facts.contains_key(k)
    }

    fn k_inductive(&self, k: &str) -> bool {
        matches!(self.facts.get(k), Some(Fact::Ind { .. }))
            || matches!(self.einfo(k), Some(EInfo::Ind { .. }))
            || matches!(self.natives.get(k), Some(nv) if matches!(nv.kind, NativeKind::Structure | NativeKind::Inductive))
    }

    /// scope.shard's resolve, over the constants K holds: the candidates
    /// that are citable constants; two are ambiguous — except in a type
    /// position, where the one that is an inductive is the one meant.
    pub fn k_resolve(&self, sc: &Scope, n: &str, etype: bool) -> Res {
        // the candidates that are citable constants, in the candidates' order (`Scope::rank`),
        // found among the names that end as the citation does instead of built one by one
        let last = last_component(n);
        let mut ranked: Vec<(usize, &Name)> = Vec::new();
        for ids in [self.by_last.get(last), self.k_last.get(last)].into_iter().flatten() {
            for id in ids {
                if let Some(r) = sc.rank(id, n) {
                    if !ranked.iter().any(|(_, k)| *k == id) && self.k_known(sc, id) {
                        ranked.push((r, id));
                    }
                }
            }
        }
        ranked.sort_by_key(|(r, _)| *r);
        let mut found: Option<Name> = None;
        for (_, k) in ranked {
            let k = k.clone();
            match &found {
                None => found = Some(k),
                Some(f) if *f == k => {}
                Some(f) => {
                    if etype {
                        match (self.k_inductive(f), self.k_inductive(&k)) {
                            (true, false) => {}
                            (false, true) => found = Some(k),
                            _ => return Res::Ambiguous(f.clone(), k),
                        }
                    } else {
                        return Res::Ambiguous(f.clone(), k);
                    }
                }
            }
        }
        match found {
            Some(k) => Res::Name(k),
            None => Res::None,
        }
    }

    /// The citation as the constant of K it names, when it names exactly one.
    pub fn k_name(&self, sc: &Scope, n: &str) -> Option<Name> {
        match self.k_resolve(sc, n, false) {
            Res::Name(k) => Some(k),
            _ => None,
        }
    }

    /// Does the name resolve, through the scope, to the constant `k` of Init?
    pub fn init_const(&self, sc: &Scope, n: &str, k: &str) -> bool {
        self.k_name(sc, n).as_deref() == Some(k)
    }

    /// A head's binders' flags: K's constant — a
    /// constant of Init by the facts, a declaration of the closure by its
    /// written binders.
    pub fn k_flags(&self, id: &str) -> Option<Vec<Flag>> {
        match self.facts.get(id) {
            Some(Fact::Ctor { binders, .. }) | Some(Fact::Def { binders, .. }) => binders.clone(),
            Some(_) => None,
            None => {
                if let Some(f) = self.flags.get(id) {
                    return Some(f.clone());
                }
                // a constructor of a `type`: its type's parameters implicit, its fields written
                if let Some(EInfo::Ctor { ind, nfields, .. }) = self.einfo(id) {
                    if let Some(EInfo::Ind { nparams, .. }) = self.einfo(ind) {
                        let mut f = vec![Flag { explicit: false, role: Role::Drop }; *nparams];
                        f.extend(vec![Flag { explicit: true, role: Role::Keep }; *nfields]);
                        return Some(f);
                    }
                }
                None
            }
        }
    }

    /// A constructor's fields as a pattern writes them: true for a runtime
    /// field, false for an erased one.
    pub fn ctor_data_flags(&self, c: &str) -> Option<Vec<bool>> {
        match self.facts.get(c) {
            Some(Fact::Ctor { fields, .. }) => fields.clone(),
            Some(_) => None,
            None => match self.einfo(c) {
                Some(EInfo::Ctor { nfields, .. }) => Some(vec![true; *nfields]),
                _ => None,
            },
        }
    }

    /// The one constructor of a structure of K.
    pub fn struct_ctor(&self, sn: &str) -> Option<Name> {
        match self.facts.get(sn) {
            Some(Fact::Ind { ctors, .. }) if ctors.len() == 1 => Some(ctors[0].clone()),
            Some(_) => None,
            None => match self.einfo(sn) {
                Some(EInfo::Ind { ctors, .. }) if ctors.len() == 1 => Some(ctors[0].clone()),
                _ => None,
            },
        }
    }

    pub fn find_decl(&self, id: &str) -> Option<&Decl> {
        self.decls.get(id)
    }

    /// A constructor's field types (over its inductive's parameters).
    pub fn ctor_fields(&self, c: &str) -> Option<(&Name, usize, &Vec<ETy>)> {
        let Some(EInfo::Ctor { ind, .. }) = self.einfo(c) else { return None };
        let Some(Decl::Ind { name, nparams, ctors }) = self.decls.get(ind) else { return None };
        ctors.iter().find(|(n, _)| n == c).map(|(_, fts)| (name, *nparams, fts))
    }
}

fn ety_names(t: &ETy, acc: &mut Vec<Name>) {
    if let ETy::Con(n, args) = t {
        acc.push(n.clone());
        args.iter().for_each(|a| ety_names(a, acc));
    }
}

/// The registry's type rows (erase.shard rep_row): a structure represented
/// by its one runtime field.
pub fn rep_row(n: &str) -> bool {
    matches!(n, "Fin" | "Subtype" | "BitVec" | "UInt8" | "Array" | "ByteArray" | "String")
}

pub fn builtin_type(n: &str) -> bool {
    matches!(n, "Nat" | "Int" | "Symbol")
}

// -----------------------------------------------------------------------------
// The closure's files (v3/kernel/loader.shard's modules)
// -----------------------------------------------------------------------------

struct ModInfo {
    closure: Vec<Name>,
}

/// A view loaded and not yet linked: its directory, and the scope its last form was read in,
/// which its implementation's forms stand after.
struct Unlinked {
    dir: Vec<String>,
    scope: Scope,
}

pub struct Loader {
    root: PathBuf,
    pub t: Tables,
    mods: HashMap<Name, ModInfo>,
    stack: Vec<Name>,
    unlinked: Vec<Unlinked>,
    /// the modules of the files named on the command line, in order
    pub roots: Vec<Name>,
}

#[derive(Debug)]
pub struct LoadError(pub String);

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn err<T>(m: impl Into<String>) -> Result<T, LoadError> {
    Err(LoadError(m.into()))
}

enum ModPath {
    File(Name),
    View(Name),
    Dir,
}

/// The module path of a root-relative file (loader.shard module_of).
fn module_of(rel: &[String]) -> Result<ModPath, LoadError> {
    let mut m = String::new();
    for (i, p) in rel.iter().enumerate() {
        let last = i + 1 == rel.len();
        if last {
            if p == "mod.req.shard" {
                // DIR/mod.req.shard, or the dir form DIR/mod.req/mod.req.shard
                return if m.is_empty() { err("a view at the root: no directory") } else { Ok(ModPath::View(m)) };
            }
            return match p.strip_suffix(".shard") {
                Some(stem) => {
                    if stem.contains('.') {
                        return err(format!("a dot in a file name: {p}"));
                    }
                    Ok(ModPath::File(append(&m, stem)))
                }
                None => Ok(ModPath::Dir),
            };
        }
        if p == "mod.req" {
            if i + 2 == rel.len() && rel[i + 1] == "mod.req.shard" {
                continue;
            }
            return err("a file under mod.req/ other than mod.req.shard");
        }
        if p.contains('.') {
            return err(format!("a dot in a directory name: {p}"));
        }
        m = append(&m, p);
    }
    err("empty path")
}

fn split_path(p: &str) -> Vec<String> {
    p.split('/').map(|s| s.to_string()).collect()
}

/// Normalized: empty and `.` components dropped, `..` pops.
fn norm_parts(parts: Vec<String>) -> Option<Vec<String>> {
    let mut out: Vec<String> = Vec::new();
    for p in parts {
        if p.is_empty() || p == "." {
            continue;
        }
        if p == ".." {
            out.pop()?;
        } else {
            out.push(p);
        }
    }
    Some(out)
}

/// The source as the host parser takes it. Two spellings of the language are outside it: a
/// universe suffix (`PProd.{1 1}` — the logic's; a program takes none) is dropped, and an
/// integer literal past the parser's range is written `(__big "DIGITS")`, which `read::int_of`
/// reads back as the integer.
fn prelex(src: &str) -> String {
    let mut out = String::with_capacity(src.len());
    let mut tok = String::new();
    let flush = |tok: &mut String, out: &mut String| {
        let digits = tok.strip_prefix('-').unwrap_or(tok);
        if digits.len() > 18 && digits.bytes().all(|b| b.is_ascii_digit()) {
            out.push_str(&format!("(__big \"{tok}\")"));
        } else {
            out.push_str(tok);
        }
        tok.clear();
    };
    let mut cs = src.chars().peekable();
    while let Some(c) = cs.next() {
        match c {
            '"' => {
                flush(&mut tok, &mut out);
                out.push(c);
                while let Some(d) = cs.next() {
                    out.push(d);
                    if d == '\\' {
                        if let Some(e) = cs.next() {
                            out.push(e);
                        }
                    } else if d == '"' {
                        break;
                    }
                }
            }
            ';' => {
                flush(&mut tok, &mut out);
                out.push(c);
                for d in cs.by_ref() {
                    out.push(d);
                    if d == '\n' {
                        break;
                    }
                }
            }
            '.' if cs.peek() == Some(&'{') => {
                for d in cs.by_ref() {
                    if d == '}' {
                        break;
                    }
                }
            }
            '(' | ')' => {
                flush(&mut tok, &mut out);
                out.push(c);
            }
            c if c.is_whitespace() => {
                flush(&mut tok, &mut out);
                out.push(c);
            }
            c => tok.push(c),
        }
    }
    flush(&mut tok, &mut out);
    out
}

pub fn as_list(v: &Value) -> Option<Vec<&Value>> {
    v.list_iter().map(|it| it.collect())
}

pub fn head_sym(v: &Value) -> Option<&str> {
    v.list_iter().and_then(|mut it| it.next()).and_then(|h| h.as_symbol())
}

/// `@f`: Lean's explicit application — every binder of f takes a written argument.
pub fn strip_at(x: &str) -> &str {
    x.strip_prefix('@').unwrap_or(x)
}

impl Loader {
    pub fn new(root: &Path, facts: HashMap<Name, Fact>) -> Loader {
        let mut t = Tables { facts, ..Tables::default() };
        for n in t.facts.keys() {
            t.k_last.entry(last_component(n).to_string()).or_default().push(n.clone());
        }
        // Init's inductives that are E types wait for their first citation
        for (n, f) in &t.facts {
            if let Fact::Ind { view: facts::View::E(np, ctors), .. } = f {
                t.init_views.insert(n.clone(), (*np, ctors.clone()));
            }
        }
        Loader { root: root.to_path_buf(), t, mods: HashMap::new(), stack: Vec::new(), unlinked: Vec::new(), roots: Vec::new() }
    }

    /// A file named on the command line: under the root, spelled from the same base.
    pub fn load_root_file(&mut self, file: &Path) -> Result<(), LoadError> {
        let rel = match file.strip_prefix(&self.root) {
            Ok(r) => r,
            Err(_) => return err(format!("{} is outside the root {}", file.display(), self.root.display())),
        };
        let parts = norm_parts(split_path(&rel.to_string_lossy())).ok_or_else(|| LoadError("bad path".into()))?;
        let mp = self.load_module(&parts)?;
        self.roots.push(mp);
        Ok(())
    }

    /// A root-relative file as a module, once: the module's path.
    fn load_module(&mut self, rel: &[String]) -> Result<Name, LoadError> {
        match module_of(rel)? {
            ModPath::Dir => {
                // a directory module is its view (dir form first, then flat)
                let mut dir_form = rel.to_vec();
                dir_form.push("mod.req".into());
                dir_form.push("mod.req.shard".into());
                let mut flat = rel.to_vec();
                flat.push("mod.req.shard".into());
                let view = if self.path_of(&dir_form).is_file() { dir_form } else { flat };
                let mp = rel.join(".");
                self.load_file_module(&view, &mp, true)?;
                Ok(mp)
            }
            ModPath::View(mp) => {
                self.load_file_module(rel, &mp, true)?;
                Ok(mp)
            }
            ModPath::File(mp) => {
                self.load_file_module(rel, &mp, false)?;
                Ok(mp)
            }
        }
    }

    fn path_of(&self, rel: &[String]) -> PathBuf {
        let mut p = self.root.clone();
        for c in rel {
            p.push(c);
        }
        p
    }

    /// The prefix a file's declarations take: its own module, or the sealed
    /// directory's for a file directly inside a directory that has a view.
    fn sealed_ident(&self, rel: &[String], mp: &str, view: bool) -> Name {
        if view || rel.len() < 2 {
            return mp.to_string();
        }
        let dir = &rel[..rel.len() - 1];
        let mut a = dir.to_vec();
        a.push("mod.req.shard".into());
        let mut b = dir.to_vec();
        b.push("mod.req".into());
        b.push("mod.req.shard".into());
        if self.path_of(&a).is_file() || self.path_of(&b).is_file() { dir.join(".") } else { mp.to_string() }
    }

    /// A file's forms, its records expanded.
    fn read_forms(&self, rel: &[String]) -> Result<Vec<Value>, LoadError> {
        let path = self.path_of(rel);
        let src = std::fs::read_to_string(&path).map_err(|e| LoadError(format!("reading {}: {}", path.display(), e)))?;
        record::parse_all(&prelex(&src))
            .and_then(record::expand)
            .map_err(|e| LoadError(format!("{}: {}", path.display(), e)))
    }

    fn load_file_module(&mut self, rel: &[String], mp: &str, view: bool) -> Result<(), LoadError> {
        if self.mods.contains_key(mp) {
            return Ok(());
        }
        if self.stack.iter().any(|m| m == mp) {
            return err(format!("import cycle through {mp}"));
        }
        let forms = self.read_forms(rel)?;
        let ident = self.sealed_ident(rel, mp, view);
        let sc = Scope { ident, tag: mp.to_string(), opened: Vec::new(), vis: vec![mp.to_string()] };
        self.stack.push(mp.to_string());
        let r = self.load_forms(rel, sc, &forms);
        self.stack.pop();
        let sc = r?;
        self.mods.insert(mp.to_string(), ModInfo { closure: sc.vis.clone() });
        if view {
            // DIR/mod.req.shard, or the dir form DIR/mod.req/mod.req.shard
            let up = if rel.len() >= 2 && rel[rel.len() - 2] == "mod.req" { 2 } else { 1 };
            self.unlinked.push(Unlinked { dir: rel[..rel.len() - up].to_vec(), scope: sc });
        }
        Ok(())
    }

    /// Every view loaded is linked: its implementation `DIR/BASE.shard` is read where the
    /// view's forms end — the view's module, its scope — and each `fn` and `type` of the
    /// directory that a `sig` of the view names takes the sig's place (docs/LANGUAGE.md
    /// "Checking an implementation": the merge of the fork's programs). The loader links a
    /// directory it is given as a root, and checks it there; this reader links every view it
    /// loaded, and checks nothing. An implementation may bring further views.
    pub fn link_views(&mut self) -> Result<(), LoadError> {
        while !self.unlinked.is_empty() {
            let u = self.unlinked.remove(0);
            let Some(base) = u.dir.last() else { continue };
            let mut rel = u.dir.clone();
            rel.push(format!("{base}.shard"));
            if !self.path_of(&rel).is_file() {
                self.t.unread.push((u.scope.tag.clone(), Unread("missing_implementation", rel.join("/"))));
                continue;
            }
            let forms = self.read_forms(&rel)?;
            self.load_forms(&rel, u.scope, &forms)?;
        }
        Ok(())
    }

    /// A file's forms read from the scope given: a module's from its own, empty; an
    /// implementation's from its view's. The scope after the last form.
    fn load_forms(&mut self, rel: &[String], mut sc: Scope, forms: &[Value]) -> Result<Scope, LoadError> {
        use std::rc::Rc;
        let mp = sc.tag.clone();
        // every `type` of the file opens its constructors for the whole file
        for f in forms {
            if head_sym(f) == Some("type") {
                if let Some((n, ctors)) = read::type_head_ctors(f) {
                    sc.opened.push(Open::Some(sc.qualify(&n), ctors));
                }
            }
        }
        // the file's heads, before any body: a body cites what its file declares later
        for f in forms {
            read::pre_register(&mut self.t, &sc, f);
        }
        // the directives where they stand (an import loads the module and makes its closure
        // visible, a use opens a prefix for the rest of the file); each other form with the
        // scope it is read in
        let mut scoped: Vec<(Rc<Scope>, &Value)> = Vec::new();
        let mut cur: Option<Rc<Scope>> = None;
        for f in forms {
            match head_sym(f) {
                Some("import") => {
                    let items = as_list(f).unwrap_or_default();
                    if items.get(1).and_then(|v| v.as_symbol()) == Some("Init") {
                        return err(format!("{mp}: (import Init): every file sees Init, and no line says so — delete the line"));
                    } else if let Some(p) = items.get(1).and_then(|v| v.as_str()) {
                        let mut parts: Vec<String> = rel[..rel.len() - 1].to_vec();
                        parts.extend(split_path(p));
                        let parts = norm_parts(parts).ok_or_else(|| LoadError(format!("{mp}: import {p} climbs above the root")))?;
                        let dep = self.load_module(&parts)?;
                        let m = self.mods.get(&dep).ok_or_else(|| LoadError(format!("{mp}: import {p} did not load")))?;
                        for v in &m.closure {
                            if !sc.vis.contains(v) {
                                sc.vis.push(v.clone());
                            }
                        }
                    } else {
                        return err(format!("{mp}: (import \"FILE\")"));
                    }
                    cur = None;
                }
                Some("use") => {
                    let items = as_list(f).unwrap_or_default();
                    let Some(p) = items.get(1).and_then(|v| v.as_symbol()) else {
                        return err(format!("{mp}: (use PREFIX NAME…)"));
                    };
                    let ms: Vec<String> = items[2..].iter().filter_map(|v| v.as_symbol()).map(|s| s.to_string()).collect();
                    let p = drop_init(p);
                    sc.opened.push(if ms.is_empty() { Open::All(p) } else { Open::Some(p, ms) });
                    cur = None;
                }
                Some("trusts") => {}
                _ => {
                    let s = match &cur {
                        Some(s) => s.clone(),
                        None => {
                            let s = Rc::new(sc.clone());
                            cur = Some(s.clone());
                            s
                        }
                    };
                    scoped.push((s, f));
                }
            }
        }
        // the declarations of the logic and the types, then the realize heads (their
        // targets resolve through the directives), then every signature, then the bodies:
        // a body is read with every signature of its file in the table
        for (s, f) in &scoped {
            read::declare(&mut self.t, s, f);
        }
        for (s, f) in &scoped {
            if matches!(head_sym(f), Some("fn" | "extern" | "type" | "sig" | "realize")) {
                self.t.scan_init(s, f);
            }
        }
        for (s, f) in &scoped {
            read::pre_realize(&mut self.t, s, f);
        }
        for (s, f) in &scoped {
            read::signature(&mut self.t, s, f);
        }
        for (s, f) in &scoped {
            read::body(&mut self.t, s, f);
        }
        Ok(sc)
    }
}

/// A closure loaded: the root files under `root`, read by the facts given
/// for Init, every view linked.
pub fn load(root: &Path, facts_file: Option<&Path>, files: &[PathBuf]) -> Result<Loader, LoadError> {
    let facts = match facts_file {
        Some(p) => {
            let src = std::fs::read_to_string(p).map_err(|e| LoadError(format!("reading {}: {}", p.display(), e)))?;
            facts::parse(&src).map_err(LoadError)?
        }
        None => HashMap::new(),
    };
    let mut l = Loader::new(root, facts);
    for f in files {
        l.load_root_file(f)?;
    }
    l.link_views()?;
    Ok(l)
}
