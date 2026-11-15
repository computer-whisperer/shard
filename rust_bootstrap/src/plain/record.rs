//! Records, and the host parser's entry (v3/kernel/record.shard is the V3
//! reader's twin; `v3/kernel/test/plain_test.sh` compares the two over a
//! closure with records).
//!
//! `(record NAME (ctor CTOR)? (FIELD TYPE)+)` — NAME may be `(NAME P…)` —
//! expands before any form of its file is read into the positional type
//! `(type NAME (CTOR TYPE…))` (CTOR defaults to MkNAME), an accessor
//! `NAME.FIELD` and an updater `NAME.with_FIELD` per field, in the record's
//! namespace; `(make NAME (FIELD V)…)` — every field exactly once — and
//! `(with NAME E (FIELD V)…)` — chained updaters, a later entry outermost —
//! are rewritten in every subterm, nested values first, against the file's
//! own records.

use lexpr::Value;
use lexpr::parse::Parser;

fn as_symbol(v: &Value) -> Result<&str, String> {
    v.as_symbol().ok_or_else(|| format!("bad shape: expected symbol, got {v}"))
}

fn as_list(v: &Value) -> Result<Vec<&Value>, String> {
    v.list_iter().map(|it| it.collect::<Vec<_>>()).ok_or_else(|| format!("bad shape: expected list, got {v}"))
}

struct RecDef {
    name: String,
    params: Vec<String>,
    ctor: String,
    fields: Vec<(String, Value)>,
}
fn sym(s: &str) -> Value {
    Value::symbol(s)
}
fn vlist(items: Vec<Value>) -> Value {
    Value::list(items)
}
fn is_head(v: &Value, h: &str) -> bool {
    match v.list_iter() {
        Some(mut it) => it.next().and_then(|x| x.as_symbol()) == Some(h),
        None => false,
    }
}
fn parse_record(v: &Value) -> Result<RecDef, String> {
    let parts = as_list(v)?;
    let bad = |m: &str| format!("bad shape: record: {}", m);
    if parts.len() < 3 {
        return Err(bad("(record NAME (ctor CTOR)? (FIELD TYPE)…)"));
    }
    let (name, params) = if let Some(n) = parts[1].as_symbol() {
        (n.to_string(), Vec::new())
    } else {
        let hd = as_list(parts[1])?;
        let n = as_symbol(hd[0])?.to_string();
        let mut ps = Vec::new();
        for p in &hd[1..] {
            ps.push(as_symbol(p)?.to_string());
        }
        (n, ps)
    };
    let mut entries: Vec<&Value> = parts[2..].to_vec();
    let mut ctor = format!("Mk{}", name);
    if let Some(first) = entries.first() {
        let e = as_list(first)?;
        if e.len() == 2 && e[0].as_symbol() == Some("ctor") {
            ctor = as_symbol(e[1])?.to_string();
            entries.remove(0);
        }
    }
    let mut fields: Vec<(String, Value)> = Vec::new();
    for e in entries {
        let e = as_list(e)?;
        if e.len() != 2 {
            return Err(bad(&format!("{}: a field is (FIELD TYPE)", name)));
        }
        let f = as_symbol(e[0])?;
        if f == "ctor" {
            return Err(bad(&format!("{}: ctor is not a field name", name)));
        }
        if fields.iter().any(|(g, _)| g == f) {
            return Err(bad(&format!("{}: a field twice", name)));
        }
        fields.push((f.to_string(), e[1].clone()));
    }
    if fields.is_empty() {
        return Err(bad(&format!("{}: at least one field", name)));
    }
    Ok(RecDef { name, params, ctor, fields })
}
fn head_form(rc: &RecDef) -> Value {
    if rc.params.is_empty() {
        sym(&rc.name)
    } else {
        let mut items = vec![sym(&rc.name)];
        items.extend(rc.params.iter().map(|p| sym(p)));
        vlist(items)
    }
}
fn tparam_binders(rc: &RecDef) -> Vec<Value> {
    rc.params.iter().map(|p| vlist(vec![sym(p), sym("Type")])).collect()
}
/// the pattern binds the fields it uses as `fld_FIELD` (a field name itself
/// would shadow the accessor the record's namespace opens) and wildcards the rest
fn pattern(rc: &RecDef, f: &str, all: bool) -> Value {
    let mut items = vec![sym(&rc.ctor)];
    items.extend(rc.fields.iter().map(|(g, _)| if all || g == f { sym(&format!("fld_{}", g)) } else { sym("_") }));
    vlist(items)
}
fn generate(rc: &RecDef) -> Vec<Value> {
    let mut out = Vec::new();
    let mut ctor_form = vec![sym(&rc.ctor)];
    ctor_form.extend(rc.fields.iter().map(|(_, t)| t.clone()));
    out.push(vlist(vec![sym("type"), head_form(rc), vlist(ctor_form)]));
    for (f, t) in &rc.fields {
        // (fn NAME.FIELD ((P Type)… (r NAME)) TYPE (match r ((CTOR f…) FIELD)))
        let mut binders = tparam_binders(rc);
        binders.push(vlist(vec![sym("r"), head_form(rc)]));
        out.push(vlist(vec![
            sym("fn"),
            sym(&format!("{}.{}", rc.name, f)),
            vlist(binders),
            t.clone(),
            vlist(vec![sym("match"), sym("r"), vlist(vec![pattern(rc, f, false), sym(&format!("fld_{}", f))])]),
        ]));
        // (fn NAME.with_FIELD ((P Type)… (new_FIELD TYPE) (r NAME)) NAME (match r ((CTOR f…) (CTOR … new_FIELD …))))
        let nv = format!("new_{}", f);
        let mut binders = tparam_binders(rc);
        binders.push(vlist(vec![sym(&nv), t.clone()]));
        binders.push(vlist(vec![sym("r"), head_form(rc)]));
        let mut build = vec![sym(&rc.ctor)];
        build.extend(rc.fields.iter().map(|(g, _)| if g == f { sym(&nv) } else { sym(&format!("fld_{}", g)) }));
        out.push(vlist(vec![
            sym("fn"),
            sym(&format!("{}.with_{}", rc.name, f)),
            vlist(binders),
            head_form(rc),
            vlist(vec![sym("match"), sym("r"), vlist(vec![pattern(rc, f, true), vlist(build)])]),
        ]));
    }
    out
}
fn rewrite(recs: &[RecDef], v: &Value) -> Result<Value, String> {
    let items: Vec<&Value> = match v.list_iter() {
        Some(it) => it.collect(),
        None => return Ok(v.clone()),
    };
    if items.is_empty() {
        return Ok(v.clone());
    }
    let mut items1: Vec<Value> = Vec::with_capacity(items.len());
    for x in &items {
        items1.push(rewrite(recs, x)?);
    }
    let bad = |m: String| format!("bad shape: {m}");
    match items1[0].as_symbol() {
        Some("make") => {
            let n = items1
                .get(1)
                .and_then(|x| x.as_symbol())
                .ok_or_else(|| bad("(make NAME (FIELD V)…)".into()))?;
            let rc = recs
                .iter()
                .find(|r| r.name == n)
                .ok_or_else(|| bad(format!("make {}: no such record", n)))?;
            let mut given: Vec<(String, Value)> = Vec::new();
            for e in &items1[2..] {
                let e = as_list(e)?;
                if e.len() != 2 {
                    return Err(bad(format!("make {}: an entry is (FIELD V)", n)));
                }
                given.push((as_symbol(e[0])?.to_string(), e[1].clone()));
            }
            if given.len() != rc.fields.len() {
                return Err(bad(format!("make {}: every field exactly once", n)));
            }
            let mut out = vec![sym(&rc.ctor)];
            for (f, _) in &rc.fields {
                let vs: Vec<&Value> = given.iter().filter(|(g, _)| g == f).map(|(_, v)| v).collect();
                if vs.len() != 1 {
                    return Err(bad(format!("make {}: every field exactly once", n)));
                }
                out.push(vs[0].clone());
            }
            Ok(vlist(out))
        }
        Some("with") => {
            if items1.len() < 3 {
                return Err(bad("(with NAME E (FIELD V)…)".into()));
            }
            let n = items1[1]
                .as_symbol()
                .ok_or_else(|| bad("(with NAME E (FIELD V)…)".into()))?;
            let rc = recs
                .iter()
                .find(|r| r.name == n)
                .ok_or_else(|| bad(format!("with {}: no such record", n)))?;
            let mut acc = items1[2].clone();
            for e in &items1[3..] {
                let e = as_list(e)?;
                if e.len() != 2 {
                    return Err(bad("(with NAME E (FIELD V)…)".into()));
                }
                let f = as_symbol(e[0])?;
                if !rc.fields.iter().any(|(g, _)| g == f) {
                    return Err(bad(format!("with {}: no such field {}", n, f)));
                }
                acc = vlist(vec![sym(&format!("{}.with_{}", rc.name, f)), e[1].clone(), acc]);
            }
            Ok(acc)
        }
        _ => Ok(vlist(items1)),
    }
}
/// A file's forms with its records expanded.
pub fn expand(values: Vec<Value>) -> Result<Vec<Value>, String> {
    let mut recs: Vec<RecDef> = Vec::new();
    for v in &values {
        if is_head(v, "record") {
            recs.push(parse_record(v)?);
        }
    }
    if recs.is_empty() {
        return Ok(values);
    }
    let mut out = Vec::with_capacity(values.len());
    for v in &values {
        if is_head(v, "record") {
            out.extend(generate(&parse_record(v)?));
        } else {
            out.push(rewrite(&recs, v)?);
        }
    }
    Ok(out)
}

/// Every top-level form of a source text.
pub fn parse_all(src: &str) -> Result<Vec<Value>, String> {
    let mut parser = Parser::from_str(src);
    let mut out = Vec::new();
    loop {
        match parser.next_value() {
            Ok(Some(v)) => out.push(v),
            Ok(None) => return Ok(out),
            Err(e) => return Err(format!("parse error: {e}")),
        }
    }
}
