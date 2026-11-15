//! The table of Init's constants (`load.shard --facts`; the format is
//! `v3/kernel/facts.shard`'s header): what the plain reading asks K about a
//! constant of Init, read here in K's place.

use std::collections::HashMap;

use lexpr::Value;

use super::{ETy, Name, as_list};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// runtime data (for a type's parameter: a type argument the E type keeps)
    Keep,
    /// a type, or an argument the E type drops
    Drop,
    Proof,
}

/// One binder of a head's declared signature: written at a call or not, and what it is.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Flag {
    pub explicit: bool,
    pub role: Role,
}

#[derive(Clone, Debug)]
pub enum View {
    /// an E type: its type parameters' count, its constructors' runtime field types
    E(usize, Vec<(Name, Vec<ETy>)>),
    /// a row of the registry: represented by its one runtime field
    Row,
    /// Nat, Int: the integer
    Int,
    None,
}

/// A projection function: the structure, the field, which of its lambdas a call writes.
#[derive(Clone, Debug)]
pub struct Proj {
    pub structure: Name,
    pub index: usize,
    pub lams: Vec<bool>,
}

#[derive(Clone, Debug)]
pub enum Fact {
    Ind { prop: bool, params: Option<Vec<Flag>>, ctors: Vec<Name>, view: View },
    Ctor { ind: Name, binders: Option<Vec<Flag>>, fields: Option<Vec<bool>> },
    Def { prop: bool, binders: Option<Vec<Flag>>, proj: Option<Proj> },
    /// a theorem, an axiom, an opaque constant, anything else: no program
    Other { prop: bool },
}

impl Fact {
    pub fn prop(&self) -> bool {
        match self {
            Fact::Ind { prop, .. } | Fact::Def { prop, .. } | Fact::Other { prop } => *prop,
            Fact::Ctor { .. } => false,
        }
    }
}

pub fn parse(src: &str) -> Result<HashMap<Name, Fact>, String> {
    let mut out = HashMap::new();
    for v in super::record::parse_all(src).map_err(|e| format!("the facts: {e}"))? {
        let (n, f) = fact(&v).ok_or_else(|| format!("the facts: not a fact: {v}"))?;
        out.insert(n, f);
    }
    Ok(out)
}

fn sym(v: &Value) -> Option<&str> {
    v.as_symbol()
}

fn fact(v: &Value) -> Option<(Name, Fact)> {
    let items = as_list(v)?;
    let kind = sym(items.first()?)?;
    let name = sym(items.get(1)?)?.to_string();
    let f = match kind {
        "ind" => Fact::Ind {
            prop: prop(items.get(2)?)?,
            params: flags(items.get(3)?)?,
            ctors: names(items.get(4)?)?,
            view: view(items.get(5)?)?,
        },
        "ctor" => Fact::Ctor {
            ind: sym(items.get(2)?)?.to_string(),
            binders: flags(items.get(3)?)?,
            fields: bools(items.get(4)?)?,
        },
        "def" => Fact::Def { prop: prop(items.get(2)?)?, binders: flags(items.get(3)?)?, proj: proj(items.get(4)?)? },
        "thm" | "axiom" | "opaque" | "other" => Fact::Other { prop: prop(items.get(2)?)? },
        _ => return None,
    };
    Some((name, f))
}

fn prop(v: &Value) -> Option<bool> {
    match sym(v)? {
        "p" => Some(true),
        "-" => Some(false),
        _ => None,
    }
}

/// `-`, or `((E R)…)`
fn flags(v: &Value) -> Option<Option<Vec<Flag>>> {
    if sym(v) == Some("-") {
        return Some(None);
    }
    let mut out = Vec::new();
    for b in list_or_empty(v)? {
        let b = as_list(b)?;
        let explicit = match sym(b.first()?)? {
            "x" => true,
            "i" => false,
            _ => return None,
        };
        let role = match sym(b.get(1)?)? {
            "k" => Role::Keep,
            "d" => Role::Drop,
            "p" => Role::Proof,
            _ => return None,
        };
        out.push(Flag { explicit, role });
    }
    Some(Some(out))
}

/// `()` reads as the empty list (lexpr's Null) or as Nil.
fn list_or_empty(v: &Value) -> Option<Vec<&Value>> {
    if v.is_null() { Some(Vec::new()) } else { as_list(v) }
}

fn names(v: &Value) -> Option<Vec<Name>> {
    list_or_empty(v)?.into_iter().map(|x| sym(x).map(|s| s.to_string())).collect()
}

fn bools(v: &Value) -> Option<Option<Vec<bool>>> {
    if sym(v) == Some("-") {
        return Some(None);
    }
    let mut out = Vec::new();
    for b in list_or_empty(v)? {
        out.push(match sym(b)? {
            "t" => true,
            "f" => false,
            _ => return None,
        });
    }
    Some(Some(out))
}

fn view(v: &Value) -> Option<View> {
    match sym(v) {
        Some("row") => return Some(View::Row),
        Some("int") => return Some(View::Int),
        Some("none") => return Some(View::None),
        Some(_) => return None,
        None => {}
    }
    let items = as_list(v)?;
    if sym(items.first()?)? != "e" {
        return None;
    }
    let np = items.get(1)?.as_u64()? as usize;
    let mut ctors = Vec::new();
    for c in &items[2..] {
        let c = as_list(c)?;
        let name = sym(c.first()?)?.to_string();
        let mut fts = Vec::new();
        for t in &c[1..] {
            fts.push(ety(t)?);
        }
        ctors.push((name, fts));
    }
    Some(View::E(np, ctors))
}

/// a type parameter by position (a numeral), NAME, or (NAME T…)
fn ety(v: &Value) -> Option<ETy> {
    if let Some(i) = v.as_u64() {
        return Some(ETy::Var(i as usize));
    }
    if let Some(s) = sym(v) {
        return Some(ETy::Con(s.to_string(), Vec::new()));
    }
    let items = as_list(v)?;
    let head = sym(items.first()?)?.to_string();
    let mut args = Vec::new();
    for a in &items[1..] {
        args.push(ety(a)?);
    }
    Some(ETy::Con(head, args))
}

/// `-`, or `(STRUCT INDEX E…)`
fn proj(v: &Value) -> Option<Option<Proj>> {
    if sym(v) == Some("-") {
        return Some(None);
    }
    let items = as_list(v)?;
    let structure = sym(items.first()?)?.to_string();
    let index = items.get(1)?.as_u64()? as usize;
    let mut lams = Vec::new();
    for e in &items[2..] {
        lams.push(match sym(e)? {
            "x" => true,
            "i" => false,
            _ => return None,
        });
    }
    Some(Some(Proj { structure, index, lams }))
}
