//! Structural model of a shard project: files, their fns/types/claims, and the
//! call graph between fns.
//!
//! Extraction is deliberately shallow — we read the paren tree (see `sexpr`),
//! pick out the top-level forms we care about, and resolve calls by matching
//! the symbols a fn body references against the set of fn names in the project.
//! This is a navigator's view, not the kernel's name resolution: it ignores
//! `use`-scoping subtleties and treats a `(:: a b c name)` qualified reference
//! by its short name. Good enough to draw the graph; refine later if needed.

use crate::sexpr::{self, Sexpr};
use std::collections::{BTreeSet, HashMap};
use std::path::{Path, PathBuf};

/// A function (or bodyless `sig fn`) definition.
#[derive(Debug, Clone)]
pub struct FnDef {
    pub name: String,
    pub params: Vec<(String, String)>, // (name, pretty type)
    pub ret: String,
    pub body: Vec<Sexpr>,
    pub file: usize,
    pub is_sig: bool,
    /// Resolved callee fn indices within this project (deduped, excludes self).
    pub calls: Vec<usize>,
}

/// One parsed `.shard` file.
#[derive(Debug, Clone)]
pub struct ShardFile {
    pub path: PathBuf,
    /// Path relative to the project root, e.g. `kernel/reader.shard`.
    pub rel: String,
    /// Dotted namespace derived from the path, e.g. `kernel.reader`.
    pub module: String,
    /// Raw `(import "...")` target strings.
    pub imports: Vec<String>,
    pub fns: Vec<usize>, // indices into Project::fns
    pub types: Vec<String>,
    pub claims: Vec<String>,
    pub parse_error: Option<String>,
}

#[derive(Debug, Default)]
pub struct Project {
    pub root: PathBuf,
    pub files: Vec<ShardFile>,
    pub fns: Vec<FnDef>,
    /// fn short-name -> indices (homonyms across files are common in shard).
    pub by_name: HashMap<String, Vec<usize>>,
}

impl Project {
    /// Load every `.shard` file under `root`, parse it, and resolve the call
    /// graph. I/O / parse errors on a single file are recorded on that file
    /// rather than aborting the whole load.
    pub fn load(root: &Path) -> std::io::Result<Project> {
        let mut paths = Vec::new();
        collect_shard_files(root, &mut paths)?;
        paths.sort();

        let mut project = Project {
            root: root.to_path_buf(),
            ..Default::default()
        };

        for path in paths {
            let rel = path
                .strip_prefix(root)
                .unwrap_or(&path)
                .to_string_lossy()
                .to_string();
            let module = rel
                .trim_end_matches(".shard")
                .replace(['/', '\\'], ".");
            let mut file = ShardFile {
                path: path.clone(),
                rel,
                module,
                imports: Vec::new(),
                fns: Vec::new(),
                types: Vec::new(),
                claims: Vec::new(),
                parse_error: None,
            };
            let src = std::fs::read_to_string(&path)?;
            match sexpr::parse_top(&src) {
                Ok(forms) => extract_file(&mut project, &mut file, forms),
                Err(e) => file.parse_error = Some(e.to_string()),
            }
            project.files.push(file);
        }

        project.build_name_index();
        project.resolve_calls();
        Ok(project)
    }

    fn build_name_index(&mut self) {
        for (i, f) in self.fns.iter().enumerate() {
            self.by_name.entry(f.name.clone()).or_default().push(i);
        }
    }

    fn resolve_calls(&mut self) {
        for i in 0..self.fns.len() {
            let mut refs = BTreeSet::new();
            let self_name = self.fns[i].name.clone();
            let params: BTreeSet<String> =
                self.fns[i].params.iter().map(|(n, _)| n.clone()).collect();
            for form in &self.fns[i].body {
                collect_refs(form, &params, &mut refs);
            }
            let file = self.fns[i].file;
            let mut calls = BTreeSet::new();
            for r in refs {
                if r == self_name {
                    continue; // self-recursion: drawn separately if at all
                }
                if let Some(targets) = self.by_name.get(&r) {
                    // Same-file-first: shard fns are file/module-scoped, so a
                    // referenced name resolves to a same-file definition when
                    // one exists. Only fall back to cross-file matches when the
                    // name isn't defined locally — this keeps short local
                    // helpers (`f`, `nl`, …) from drawing spurious edges to
                    // every homonym in the project.
                    let local: Vec<usize> =
                        targets.iter().copied().filter(|&t| self.fns[t].file == file).collect();
                    let chosen = if local.is_empty() { targets } else { &local };
                    for &t in chosen {
                        if t != i {
                            calls.insert(t);
                        }
                    }
                }
            }
            self.fns[i].calls = calls.into_iter().collect();
        }
    }
}

/// Collect every symbol a body references that *could* name a fn. Skips the
/// fn's own params (locals that shadow). For a `(:: a b … name)` qualified
/// reference, only the short (last) name is taken.
fn collect_refs(e: &Sexpr, params: &BTreeSet<String>, out: &mut BTreeSet<String>) {
    match e {
        Sexpr::Sym(s) => {
            if !params.contains(s) {
                out.insert(s.clone());
            }
        }
        Sexpr::List(items) => {
            if e.head() == Some("::") {
                if let Some(Sexpr::Sym(last)) = items.last()
                    && last != "*"
                {
                    out.insert(last.clone());
                }
                return; // don't descend into the path segments
            }
            // `(quote X)` is object data, not calls — skip its payload.
            if e.head() == Some("quote") {
                return;
            }
            for it in items {
                collect_refs(it, params, out);
            }
        }
        Sexpr::Int(_) | Sexpr::Str(_) => {}
    }
}

fn extract_file(project: &mut Project, file: &mut ShardFile, forms: Vec<Sexpr>) {
    let file_idx = project.files.len();
    for form in forms {
        match form.head() {
            Some("import") => {
                if let Sexpr::List(items) = &form
                    && let Some(Sexpr::Str(s)) = items.get(1)
                {
                    file.imports.push(s.clone());
                }
            }
            Some("fn") => {
                if let Some(def) = parse_fn(&form, file_idx, false) {
                    let idx = project.fns.len();
                    file.fns.push(idx);
                    project.fns.push(def);
                }
            }
            Some("sig") => {
                // (sig fn NAME PARAMS RET) — a bodyless signature.
                if let Sexpr::List(items) = &form
                    && items.get(1).and_then(|s| s.as_sym()) == Some("fn")
                    && let Some(def) = parse_fn_from(&items[1..], file_idx, true)
                {
                    let idx = project.fns.len();
                    file.fns.push(idx);
                    project.fns.push(def);
                }
            }
            Some("type") => {
                if let Sexpr::List(items) = &form
                    && let Some(name) = items.get(1).and_then(|s| s.as_sym())
                {
                    file.types.push(name.to_string());
                }
            }
            Some("claim") | Some("requirement") | Some("fulfills") => {
                if let Sexpr::List(items) = &form
                    && let Some(name) = items.get(1).and_then(|s| s.as_sym())
                {
                    file.claims.push(name.to_string());
                }
            }
            _ => {}
        }
    }
}

/// Parse a `(fn NAME PARAMS RET BODY...)` form.
fn parse_fn(form: &Sexpr, file: usize, is_sig: bool) -> Option<FnDef> {
    let items = form.as_list()?;
    parse_fn_from(items, file, is_sig)
}

/// `items` starts at the `fn` head: `[fn, NAME, PARAMS, RET, BODY...]`.
fn parse_fn_from(items: &[Sexpr], file: usize, is_sig: bool) -> Option<FnDef> {
    let name = items.get(1)?.as_sym()?.to_string();
    let params = parse_params(items.get(2));
    let ret = items.get(3).map(pretty).unwrap_or_default();
    let body = if is_sig {
        Vec::new()
    } else {
        items.get(4..).map(|s| s.to_vec()).unwrap_or_default()
    };
    Some(FnDef {
        name,
        params,
        ret,
        body,
        file,
        is_sig,
        calls: Vec::new(),
    })
}

fn parse_params(e: Option<&Sexpr>) -> Vec<(String, String)> {
    let mut out = Vec::new();
    if let Some(Sexpr::List(items)) = e {
        for it in items {
            if let Sexpr::List(pair) = it
                && let Some(Sexpr::Sym(n)) = pair.first()
            {
                let ty = pair.get(1).map(pretty).unwrap_or_default();
                out.push((n.clone(), ty));
            }
        }
    }
    out
}

/// Render an s-expr back to a compact string (for type display).
pub fn pretty(e: &Sexpr) -> String {
    match e {
        Sexpr::Int(n) => n.to_string(),
        Sexpr::Sym(s) => s.clone(),
        Sexpr::Str(s) => format!("{s:?}"),
        Sexpr::List(items) => {
            let inner: Vec<String> = items.iter().map(pretty).collect();
            format!("({})", inner.join(" "))
        }
    }
}

fn collect_shard_files(dir: &Path, out: &mut Vec<PathBuf>) -> std::io::Result<()> {
    for entry in std::fs::read_dir(dir)? {
        let entry = entry?;
        let path = entry.path();
        let name = entry.file_name();
        let name = name.to_string_lossy();
        if path.is_dir() {
            // Skip build/vcs/vendor dirs that never hold project source.
            if matches!(name.as_ref(), "target" | ".git" | "node_modules") {
                continue;
            }
            collect_shard_files(&path, out)?;
        } else if path.extension().and_then(|e| e.to_str()) == Some("shard") {
            out.push(path);
        }
    }
    Ok(())
}
