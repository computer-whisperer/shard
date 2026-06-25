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
    /// The verbatim source text of the definition form.
    pub src: String,
    /// Resolved callee fn indices within this project (deduped, excludes self).
    pub calls: Vec<usize>,
    /// Reverse edges: fn indices that call this one (the "called by" set).
    /// Populated after `calls`; project-wide in-degree is `callers.len()`.
    pub callers: Vec<usize>,
    /// True if this fn's name appears in a claim/fulfills/requirement form
    /// anywhere in the project — i.e. it is reasoned ABOUT even if nothing
    /// calls it. In a proof corpus most "uncalled" fns are proof subjects,
    /// not dead code, so this keeps them out of the orphan set.
    pub proof_refd: bool,
}

impl FnDef {
    /// Source line count of the definition form — a cheap complexity proxy.
    pub fn src_lines(&self) -> usize {
        self.src.lines().count().max(1)
    }
    /// Total connectivity (callees + callers): how much of a hub this fn is.
    pub fn degree(&self) -> usize {
        self.calls.len() + self.callers.len()
    }
    /// A dead-code / cut candidate: a real fn (not a bodyless `sig`) that
    /// nothing in the project calls, isn't reasoned about in a proof, and isn't
    /// a program entry point. Heuristic — the model's call resolution is
    /// short-name + same-file-first, so a cross-file caller resolving to a
    /// homonym can hide a true caller. A "look here", not an authority (verify
    /// with grep before cutting).
    pub fn is_orphan(&self) -> bool {
        self.callers.is_empty() && !self.proof_refd && !self.is_sig && self.name != "main"
    }
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
    /// Import strings resolved to file indices (deduped; unresolved/external
    /// imports are dropped). This file *depends on* each target.
    pub import_targets: Vec<usize>,
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
    /// Short names referenced anywhere in a claim/fulfills/requirement form
    /// (the proof "uses" set — a fn here is reasoned about, not dead).
    pub proof_refs: BTreeSet<String>,
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
                import_targets: Vec::new(),
                fns: Vec::new(),
                types: Vec::new(),
                claims: Vec::new(),
                parse_error: None,
            };
            let src = std::fs::read_to_string(&path)?;
            match sexpr::parse_top_spanned(&src) {
                Ok(forms) => extract_file(&mut project, &mut file, forms),
                Err(e) => file.parse_error = Some(e.to_string()),
            }
            project.files.push(file);
        }

        project.build_name_index();
        project.resolve_calls();
        project.resolve_imports();
        // Mark fns reasoned about in proofs so they don't read as dead code.
        for f in &mut project.fns {
            f.proof_refd = project.proof_refs.contains(&f.name);
        }
        Ok(project)
    }

    /// Resolve each file's raw import strings to file indices (the import
    /// dependency graph). Relative `.shard` paths resolve against the importing
    /// file's directory; a bare module name `m` tries `m/mod.req.shard`,
    /// `m.shard`, then `m/mod.shard`. Unresolved (external) imports are dropped.
    fn resolve_imports(&mut self) {
        let by_rel: HashMap<String, usize> = self
            .files
            .iter()
            .enumerate()
            .map(|(i, f)| (f.rel.clone(), i))
            .collect();
        for i in 0..self.files.len() {
            let importer = self.files[i].rel.clone();
            let imports = self.files[i].imports.clone();
            let mut targets: Vec<usize> = imports
                .iter()
                .filter_map(|imp| resolve_import(&importer, imp, &by_rel))
                .filter(|&t| t != i)
                .collect();
            targets.sort_unstable();
            targets.dedup();
            self.files[i].import_targets = targets;
        }
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
        // Reverse edges: for each resolved call i -> t, record i as a caller of t.
        for i in 0..self.fns.len() {
            for t in self.fns[i].calls.clone() {
                self.fns[t].callers.push(i);
            }
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

fn extract_file(project: &mut Project, file: &mut ShardFile, forms: Vec<(Sexpr, String)>) {
    let file_idx = project.files.len();
    for (form, src) in forms {
        match form.head() {
            Some("import") => {
                if let Sexpr::List(items) = &form
                    && let Some(Sexpr::Str(s)) = items.get(1)
                {
                    file.imports.push(s.clone());
                }
            }
            Some("fn") => {
                if let Some(def) = parse_fn(&form, file_idx, false, src) {
                    let idx = project.fns.len();
                    file.fns.push(idx);
                    project.fns.push(def);
                }
            }
            Some("sig") => {
                // (sig fn NAME PARAMS RET) — a bodyless signature.
                if let Sexpr::List(items) = &form
                    && items.get(1).and_then(|s| s.as_sym()) == Some("fn")
                    && let Some(def) = parse_fn_from(&items[1..], file_idx, true, src)
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
                // Every symbol the proof form mentions is a "use": the fns it
                // reasons about (goal terms) and cites (lemma/premise names).
                collect_refs(&form, &BTreeSet::new(), &mut project.proof_refs);
            }
            _ => {}
        }
    }
}

/// Parse a `(fn NAME PARAMS RET BODY...)` form.
fn parse_fn(form: &Sexpr, file: usize, is_sig: bool, src: String) -> Option<FnDef> {
    let items = form.as_list()?;
    parse_fn_from(items, file, is_sig, src)
}

/// `items` starts at the `fn` head: `[fn, NAME, PARAMS, RET, BODY...]`.
fn parse_fn_from(items: &[Sexpr], file: usize, is_sig: bool, src: String) -> Option<FnDef> {
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
        src,
        calls: Vec::new(),
        callers: Vec::new(),
        proof_refd: false,
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

/// Resolve one import string (relative to `importer`'s directory) to a file
/// index via the rel-path index, trying module-name fallbacks for bare names.
fn resolve_import(importer: &str, import: &str, by_rel: &HashMap<String, usize>) -> Option<usize> {
    let base = importer.rsplit_once('/').map(|(d, _)| d).unwrap_or("");
    let candidates: Vec<String> = if import.ends_with(".shard") {
        vec![normalize_rel(base, import)]
    } else {
        vec![
            normalize_rel(base, &format!("{import}/mod.req.shard")),
            normalize_rel(base, &format!("{import}.shard")),
            normalize_rel(base, &format!("{import}/mod.shard")),
        ]
    };
    candidates.iter().find_map(|c| by_rel.get(c).copied())
}

/// Join `base` (a dir) and `rel` (which may contain `.`/`..`) into a normalized
/// `/`-separated path, resolving `..` against the accumulated components.
fn normalize_rel(base: &str, rel: &str) -> String {
    let mut stack: Vec<&str> = Vec::new();
    for comp in base.split('/').chain(rel.split('/')) {
        match comp {
            "" | "." => {}
            ".." => {
                stack.pop();
            }
            c => stack.push(c),
        }
    }
    stack.join("/")
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
        } else if path.extension().and_then(|e| e.to_str()) == Some("shard")
            // Skip generated artifacts: .shard.low.shard (lowered) carries
            // duplicate fn defs that pollute the call graph and orphan set.
            && !name.ends_with(".low.shard")
        {
            out.push(path);
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{normalize_rel, FnDef};

    fn fnd(name: &str, callers: Vec<usize>, proof_refd: bool, is_sig: bool) -> FnDef {
        FnDef {
            name: name.into(),
            params: vec![],
            ret: String::new(),
            body: vec![],
            file: 0,
            is_sig,
            src: String::new(),
            calls: vec![],
            callers,
            proof_refd,
        }
    }

    #[test]
    fn orphan_excludes_proven_sigs_and_entry() {
        assert!(fnd("dead", vec![], false, false).is_orphan()); // nothing uses it
        assert!(!fnd("hub", vec![1], false, false).is_orphan()); // has a caller
        assert!(!fnd("lemma_subject", vec![], true, false).is_orphan()); // proven about
        assert!(!fnd("iface", vec![], false, true).is_orphan()); // bodyless sig
        assert!(!fnd("main", vec![], false, false).is_orphan()); // entry point
    }

    #[test]
    fn normalize_resolves_dotdot() {
        assert_eq!(normalize_rel("kernel", "stdlib.shard"), "kernel/stdlib.shard");
        assert_eq!(
            normalize_rel("examples/modules_demo/bump", "../../../kernel/stdlib.shard"),
            "kernel/stdlib.shard"
        );
        assert_eq!(normalize_rel("", "foo.shard"), "foo.shard");
        assert_eq!(normalize_rel("a/b", "./c.shard"), "a/b/c.shard");
    }
}
