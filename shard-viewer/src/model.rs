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

/// Which proof-layer form a [`ClaimDef`] came from. Together these are the
/// project's *statements* — the layer that reasons about the fns.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ClaimKind {
    /// `(claim NAME (goal …) PROOF)` — a self-contained proven lemma.
    Claim,
    /// `(axiom NAME (kind …) (goal …))` — assumed, not proven: a trust root.
    Axiom,
    /// `(requirement NAME (goal …))` — a declared obligation, proof elsewhere.
    Requirement,
    /// `(fulfills NAME PROOF)` — the proof discharging requirement `NAME`.
    Fulfills,
}

/// A proof-layer form: claim, axiom, requirement, or fulfills. The proof
/// analog of [`FnDef`] — a named node with resolved edges into both layers.
#[derive(Debug, Clone)]
pub struct ClaimDef {
    pub name: String,
    pub kind: ClaimKind,
    pub file: usize,
    /// The whole parsed form (kept like a fn's body — a future proof-card
    /// renderer draws structure from it).
    pub form: Sexpr,
    /// The verbatim source text of the form.
    pub src: String,
    /// The goal statement, prettied (a fulfills inherits its requirement's).
    pub goal: String,
    /// Resolved proof-layer citations: claims/axioms whose names this form's
    /// proof mentions (for a fulfills, that includes its requirement). Same
    /// shallow same-file-first resolution as fn calls.
    pub cites: Vec<usize>,
    /// Resolved fn indices the *statement* (the `(goal …)` subform, or the
    /// whole form when there is none) reasons about.
    pub about: Vec<usize>,
    /// Requirement only: a `(fulfills NAME …)` exists somewhere. An
    /// unfulfilled requirement is an open obligation — views draw it loud.
    pub fulfilled: bool,
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

/// Per-file line tally by shard complexity category. A direct port of the
/// column-0-head-atom state machine in `tools/loc/loc.shard`: canonical
/// (shardfmt) layout starts every top-level form with `(` at column 0 and
/// indents its body, so a line's category is the category of the form it sits
/// in — no full parse needed.
#[derive(Debug, Clone, Default)]
pub struct Counts {
    pub impl_: u32,    // fn/type/import/use/sig/extern/bin — implementation
    pub measure: u32,  // inline (measure …) totality obligations
    pub proof: u32,    // (claim …) proof blocks
    pub reqproof: u32, // (fulfills …) requirement-fulfillment proofs
    pub req: u32,      // (requirement …)/(axiom …) interface declarations
    pub comment: u32,  // ; lines
    pub blank: u32,    // whitespace-only
    pub sidecar: u32,  // whole .auto.shard files (machine-generated proof fill)
}

impl Counts {
    /// Substantive (non comment/blank) lines.
    pub fn code(&self) -> u32 {
        self.impl_ + self.measure + self.proof + self.reqproof + self.req + self.sidecar
    }
    /// Every counted line.
    pub fn total(&self) -> u32 {
        self.code() + self.comment + self.blank
    }
    /// Proof-burden lines: totality obligations + claim/fulfills proofs +
    /// machine-generated sidecar fill.
    pub fn proof_lines(&self) -> u32 {
        self.measure + self.proof + self.reqproof + self.sidecar
    }
    /// Non-proof code: implementation + interface declarations.
    pub fn impl_lines(&self) -> u32 {
        self.impl_ + self.req
    }
    /// Proof share of substantive code, in `[0,1]` — the heat axis. `None` when
    /// the file has no substantive code (pure comments/blank) so it reads
    /// neutral rather than as either extreme.
    pub fn proof_share(&self) -> Option<f32> {
        let denom = self.impl_lines() + self.proof_lines();
        if denom == 0 {
            None
        } else {
            Some(self.proof_lines() as f32 / denom as f32)
        }
    }
}

/// The head atom of a column-0 form: bytes after `(` up to the first delimiter.
fn head_atom(line: &str) -> &str {
    let after = &line[1..]; // line starts with '(' (one ASCII byte)
    let end = after
        .find([' ', '\t', '(', ')'])
        .unwrap_or(after.len());
    &after[..end]
}

/// Map a top-level form's head atom to its category code (0 = implementation).
fn head_code(atom: &str) -> u8 {
    match atom {
        "claim" => 2,
        "fulfills" => 3,
        "requirement" | "axiom" => 4,
        "proof-for" => 7,
        _ => 0,
    }
}

/// Classify `src` into per-category line counts. `forced` (`Some(7)` for
/// `.auto.shard` sidecars) forces every substantive line to that category;
/// blanks and comments still split out.
pub fn classify_source(src: &str, forced: Option<u8>) -> Counts {
    let mut c = Counts::default();
    let mut cur: u8 = 0; // running category of the enclosing top-level form
    for line in src.lines() {
        let trimmed = line.trim_start();
        let code: u8 = if trimmed.is_empty() {
            6
        } else if trimmed.starts_with(';') {
            5
        } else if let Some(f) = forced {
            f
        } else if line.starts_with('(') {
            // New top-level form: its head atom sets the running category.
            cur = head_code(head_atom(line));
            cur
        } else if trimmed.starts_with("(measure") {
            1 // inline totality obligation; the form's category is unchanged
        } else {
            cur // body line inherits its form's category
        };
        match code {
            0 => c.impl_ += 1,
            1 => c.measure += 1,
            2 => c.proof += 1,
            3 => c.reqproof += 1,
            4 => c.req += 1,
            5 => c.comment += 1,
            6 => c.blank += 1,
            7 => c.sidecar += 1,
            _ => {}
        }
    }
    c
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
    pub claims: Vec<usize>, // indices into Project::claims
    /// Line tally by complexity category (the heat-map source).
    pub counts: Counts,
    pub parse_error: Option<String>,
}

#[derive(Debug, Default)]
pub struct Project {
    pub root: PathBuf,
    pub files: Vec<ShardFile>,
    pub fns: Vec<FnDef>,
    /// Every proof-layer form in the project (see [`ClaimDef`]).
    pub claims: Vec<ClaimDef>,
    /// fn short-name -> indices (homonyms across files are common in shard).
    pub by_name: HashMap<String, Vec<usize>>,
    /// Citable claim name -> indices. Fulfills forms are excluded: their name
    /// *refers to* a requirement, it doesn't declare a citable statement.
    pub claims_by_name: HashMap<String, Vec<usize>>,
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
                counts: Counts::default(),
                parse_error: None,
            };
            let src = std::fs::read_to_string(&path)?;
            // Category tally is independent of the parse, so it stands even for
            // files the structural reader chokes on.
            let forced = if file.rel.ends_with(".auto.shard") { Some(7) } else { None };
            file.counts = classify_source(&src, forced);
            match sexpr::parse_top_spanned(&src) {
                Ok(forms) => extract_file(&mut project, &mut file, forms),
                Err(e) => file.parse_error = Some(e.to_string()),
            }
            project.files.push(file);
        }

        project.build_name_index();
        project.resolve_calls();
        project.resolve_claims();
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
        for (i, c) in self.claims.iter().enumerate() {
            if c.kind != ClaimKind::Fulfills {
                self.claims_by_name.entry(c.name.clone()).or_default().push(i);
            }
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

    /// Resolve the proof layer's edges, the claim analog of [`Self::resolve_calls`]:
    /// `cites` from every name the form mentions that is a citable claim, `about`
    /// from the fn names its statement mentions — both same-file-first. Then mark
    /// each requirement some fulfills discharges.
    fn resolve_claims(&mut self) {
        for i in 0..self.claims.len() {
            let binders = goal_binders(&self.claims[i].form);
            let mut refs = BTreeSet::new();
            collect_refs(&self.claims[i].form, &binders, &mut refs);
            if self.claims[i].kind != ClaimKind::Fulfills {
                refs.remove(&self.claims[i].name); // its own head name
            }
            // The statement's subject fns come from the (goal …) subform alone —
            // proof-body mentions (tactic keywords, premise handles, farkas
            // coefficients) would drown the signal. A fulfills has no goal of its
            // own, so its whole form speaks (unfold targets ARE its subjects).
            let stmt_refs = match goal_form(&self.claims[i].form) {
                Some(g) => {
                    let mut s = BTreeSet::new();
                    collect_refs(g, &binders, &mut s);
                    s
                }
                None => refs.clone(),
            };
            let file = self.claims[i].file;
            let mut cites = BTreeSet::new();
            for r in &refs {
                if let Some(targets) = self.claims_by_name.get(r) {
                    let local: Vec<usize> = targets
                        .iter()
                        .copied()
                        .filter(|&t| self.claims[t].file == file)
                        .collect();
                    let chosen = if local.is_empty() { targets } else { &local };
                    cites.extend(chosen.iter().copied().filter(|&t| t != i));
                }
            }
            let mut about = BTreeSet::new();
            for r in &stmt_refs {
                if let Some(targets) = self.by_name.get(r) {
                    let local: Vec<usize> = targets
                        .iter()
                        .copied()
                        .filter(|&t| self.fns[t].file == file)
                        .collect();
                    let chosen = if local.is_empty() { targets } else { &local };
                    about.extend(chosen.iter().copied());
                }
            }
            self.claims[i].cites = cites.into_iter().collect();
            self.claims[i].about = about.into_iter().collect();
        }
        // A fulfills discharges every requirement it cites by name (normally
        // exactly one) and inherits its goal statement for display.
        for i in 0..self.claims.len() {
            if self.claims[i].kind != ClaimKind::Fulfills {
                continue;
            }
            for t in self.claims[i].cites.clone() {
                if self.claims[t].kind == ClaimKind::Requirement {
                    self.claims[t].fulfilled = true;
                    if self.claims[i].goal.is_empty() {
                        self.claims[i].goal = self.claims[t].goal.clone();
                    }
                }
            }
        }
    }
}

/// The `(goal BINDERS PREMISES GOAL)` subform of a proof-layer form.
fn goal_form(form: &Sexpr) -> Option<&Sexpr> {
    form.as_list()?.iter().find(|it| it.head() == Some("goal"))
}

/// The bound variable names of the form's goal — the claim analog of fn
/// params for ref collection (locals, not project names).
fn goal_binders(form: &Sexpr) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    if let Some(g) = goal_form(form)
        && let Some(gi) = g.as_list()
        && let Some(Sexpr::List(binders)) = gi.get(1)
    {
        for b in binders {
            if let Some(pair) = b.as_list()
                && let Some(Sexpr::Sym(n)) = pair.first()
            {
                out.insert(n.clone());
            }
        }
    }
    out
}

/// The goal statement (the final term of the `(goal …)` subform), prettied.
fn goal_text(form: &Sexpr) -> Option<String> {
    Some(pretty(goal_form(form)?.as_list()?.last()?))
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
            Some(head @ ("claim" | "requirement" | "fulfills" | "axiom")) => {
                // Every symbol the proof form mentions is a "use": the fns it
                // reasons about (goal terms) and cites (lemma/premise names).
                collect_refs(&form, &BTreeSet::new(), &mut project.proof_refs);
                if let Sexpr::List(items) = &form
                    && let Some(name) = items.get(1).and_then(|s| s.as_sym())
                {
                    let kind = match head {
                        "axiom" => ClaimKind::Axiom,
                        "requirement" => ClaimKind::Requirement,
                        "fulfills" => ClaimKind::Fulfills,
                        _ => ClaimKind::Claim,
                    };
                    file.claims.push(project.claims.len());
                    project.claims.push(ClaimDef {
                        name: name.to_string(),
                        kind,
                        file: file_idx,
                        goal: goal_text(&form).unwrap_or_default(),
                        form,
                        src,
                        cites: Vec::new(),
                        about: Vec::new(),
                        fulfilled: false,
                    });
                }
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
    use super::{classify_source, normalize_rel, FnDef};

    #[test]
    fn classify_categories_and_heat() {
        let src = "\
;; a comment
(fn f ((x Int)) Int
  (measure (struct x))
  (+ x 1))

(claim thing
  (by lia))
(requirement r Bool)
";
        let c = classify_source(src, None);
        assert_eq!(c.comment, 1);
        assert_eq!(c.blank, 1);
        // (fn… header + body line = impl; the (measure…) line splits out.
        assert_eq!(c.impl_, 2);
        assert_eq!(c.measure, 1);
        // (claim header + its body line.
        assert_eq!(c.proof, 2);
        assert_eq!(c.req, 1);
        // impl_lines = impl_ + req = 3; proof_lines = measure + proof = 3.
        assert_eq!(c.proof_share(), Some(0.5));
    }

    #[test]
    fn classify_forced_sidecar() {
        let src = "; note\n(fulfills x (by auto))\nbody line\n";
        let c = classify_source(src, Some(7));
        assert_eq!(c.comment, 1);
        assert_eq!(c.sidecar, 2); // both substantive lines forced, comment split
        assert_eq!(c.proof, 0);
    }

    use super::Counts;
    #[test]
    fn pure_comment_file_reads_neutral() {
        let c = Counts { comment: 5, blank: 2, ..Counts::default() };
        assert_eq!(c.proof_share(), None);
    }

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

    /// A tiny two-file project on disk (temp dir), exercising the whole
    /// proof-layer resolution: kinds, citations, subjects, cross-file
    /// requirement fulfillment, and the pending (unfulfilled) case.
    #[test]
    fn proof_layer_resolution() {
        use super::{ClaimKind, Project};
        let dir = std::env::temp_dir().join(format!("shard_viewer_claims_{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        std::fs::write(
            dir.join("iface.shard"),
            "(requirement round_trips (goal ((n Int)) () (= (dec (inc n)) n)))\n\
             (requirement never_done (goal ((n Int)) () (= n n)))\n",
        )
        .unwrap();
        std::fs::write(
            dir.join("impl.shard"),
            "(fn inc ((n Int)) Int (+ n 1))\n\
             (fn dec ((n Int)) Int (- n 1))\n\
             (axiom ground (kind operational) (goal ((n Int)) () (= (inc n) (+ n 1))))\n\
             (claim inc_pos (goal ((n Int)) ((= (le 0 n) True)) (= (le 0 (inc n)) True))\n\
               (steps ((rewrite (lemma ground) lr lhs true ())) refl))\n\
             (fulfills round_trips\n\
               (steps ((unfold inc lhs) (unfold dec lhs) (rewrite (lemma inc_pos) lr lhs true ())) refl))\n",
        )
        .unwrap();
        let p = Project::load(&dir).unwrap();
        std::fs::remove_dir_all(&dir).unwrap();

        let by_name = |n: &str| {
            p.claims.iter().position(|c| c.name == n).unwrap_or_else(|| panic!("claim {n}"))
        };
        let (rt, nd, gr, ip) =
            (by_name("round_trips"), by_name("never_done"), by_name("ground"), by_name("inc_pos"));
        // The fulfills shares the requirement's name; find it by kind.
        let ff = p
            .claims
            .iter()
            .position(|c| c.kind == ClaimKind::Fulfills)
            .expect("the fulfills form");

        assert_eq!(p.claims[rt].kind, ClaimKind::Requirement);
        assert_eq!(p.claims[gr].kind, ClaimKind::Axiom);
        assert_eq!(p.claims[ip].kind, ClaimKind::Claim);
        // Cross-file fulfillment: the fulfills cites its requirement (plus the
        // lemma its proof rewrites with) and discharges it; the other
        // requirement stays open.
        assert!(p.claims[ff].cites.contains(&rt));
        assert!(p.claims[ff].cites.contains(&ip));
        assert!(p.claims[rt].fulfilled);
        assert!(!p.claims[nd].fulfilled);
        // The fulfills inherits the requirement's goal for display.
        assert_eq!(p.claims[ff].goal, p.claims[rt].goal);
        assert_eq!(p.claims[rt].goal, "(= (dec (inc n)) n)");
        // Subjects come from the statement: the claim is about inc, not about
        // its binder n; the citation of ground lives in cites, not about.
        let inc = p.by_name["inc"][0];
        assert!(p.claims[ip].about.contains(&inc));
        assert!(p.claims[ip].cites.contains(&gr));
        assert!(!p.claims[ip].cites.contains(&ip));
        // The requirement's goal names both fns even from the other file.
        let dec = p.by_name["dec"][0];
        assert!(p.claims[rt].about.contains(&inc) && p.claims[rt].about.contains(&dec));
        // Per-file claim rosters point back at the same defs.
        let iface = p.files.iter().position(|f| f.rel == "iface.shard").unwrap();
        assert_eq!(p.files[iface].claims.len(), 2);
    }
}
