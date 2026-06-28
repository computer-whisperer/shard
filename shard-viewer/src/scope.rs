//! Scope: the viewer's **subject selection** — the single source of truth for
//! *what set of fns the canvas is currently about*.
//!
//! It replaces the old ad-hoc `(selected_file, selected_fn)` pair: every view
//! projects the scope down to what it needs. The single-file views (Methods,
//! Board) ask for [`Scope::focus_file`]; the Map view asks for the full
//! [`Scope::fns`] / [`Scope::files`] sets. A separate *focus* cursor (which fn
//! is highlighted / shown in the detail panel) stays orthogonal to the scope —
//! you can have a `File` scope with one fn focused inside it.
//!
//! The variants form a flexibility ladder, from one fn up to the whole project;
//! they're all defined now even though the early views only exercise `Fn` /
//! `File`, so the Map view and the richer sidebar picker can grow into them
//! without reshaping the type.

use crate::model::Project;
use std::collections::BTreeSet;

/// What the canvas is about. See the module docs.
#[derive(Clone, Debug, PartialEq, Eq, Default)]
pub enum Scope {
    /// Nothing selected yet (fresh window, or a cleared selection).
    #[default]
    None,
    /// One fn — the Flow view's natural subject.
    Fn(usize),
    /// Every fn defined in one file — feeds Methods / Board.
    File(usize),
    /// Every fn under a directory subtree, named by its `rel`-path prefix with
    /// no trailing slash (e.g. `"kernel"`; `""` is the project root dir).
    Dir(String),
    /// The call neighborhood around a fn: transitive callees `down` levels deep
    /// and callers `up` levels deep (`root` itself always included).
    CallTree { root: usize, up: u8, down: u8 },
    /// Every fn in the project.
    Project,
}

impl Scope {
    /// The single file this scope is anchored on, if it resolves to one — what
    /// the single-file views (Methods, Board) draw and the toolbar titles.
    /// `Dir` / `Project` span many files and have no single anchor (`None`).
    pub fn focus_file(&self, project: &Project) -> Option<usize> {
        match self {
            Scope::File(i) => Some(*i),
            Scope::Fn(i) | Scope::CallTree { root: i, .. } => Some(project.fns.get(*i)?.file),
            Scope::Dir(_) | Scope::Project | Scope::None => None,
        }
    }

    /// The fns in scope, as project indices in ascending (stable) order. This is
    /// the set the Map view renders; an empty scope yields an empty set.
    pub fn fns(&self, project: &Project) -> Vec<usize> {
        match self {
            Scope::None => Vec::new(),
            Scope::Fn(i) => project.fns.get(*i).map(|_| vec![*i]).unwrap_or_default(),
            Scope::File(i) => project.files.get(*i).map(|f| f.fns.clone()).unwrap_or_default(),
            Scope::Dir(prefix) => (0..project.fns.len())
                .filter(|&fi| dir_contains(prefix, &project.files[project.fns[fi].file].rel))
                .collect(),
            Scope::CallTree { root, up, down } => call_tree(project, *root, *up, *down),
            Scope::Project => (0..project.fns.len()).collect(),
        }
    }

    /// The files this scope spans, ascending — the dir/file boxes the Map view
    /// groups its fn cards under. Derived from [`Self::fns`] so the two agree.
    pub fn files(&self, project: &Project) -> Vec<usize> {
        match self {
            Scope::File(i) => vec![*i],
            _ => {
                let set: BTreeSet<usize> =
                    self.fns(project).iter().map(|&fi| project.fns[fi].file).collect();
                set.into_iter().collect()
            }
        }
    }

    /// A short human label for the toolbar (e.g. `kernel/reader.shard`, or
    /// `mc_walk · callers+callees`).
    pub fn label(&self, project: &Project) -> String {
        match self {
            Scope::None => "—".to_string(),
            Scope::Fn(i) => project.fns.get(*i).map_or("?".into(), |f| f.name.clone()),
            Scope::File(i) => project.files.get(*i).map_or("?".into(), |f| f.rel.clone()),
            Scope::Dir(p) if p.is_empty() => "(root)/".to_string(),
            Scope::Dir(p) => format!("{p}/"),
            Scope::CallTree { root, up, down } => {
                let name = project.fns.get(*root).map_or("?", |f| f.name.as_str());
                format!("{name} · {up}↑ {down}↓")
            }
            Scope::Project => format!("all · {} files", project.files.len()),
        }
    }
}

/// True when `rel` (a file's project-relative path) sits under directory
/// `prefix`. `prefix` is the dir with no trailing slash; `""` matches every
/// file (the project root). Matches on path components, so `ker` does *not*
/// match `kernel/…`.
fn dir_contains(prefix: &str, rel: &str) -> bool {
    if prefix.is_empty() {
        return true;
    }
    rel.strip_prefix(prefix).is_some_and(|rest| rest.starts_with('/'))
}

/// BFS the call graph out from `root`: callees up to `down` hops, callers up to
/// `up` hops. Returns the reached fns (including `root`) ascending.
fn call_tree(project: &Project, root: usize, up: u8, down: u8) -> Vec<usize> {
    if root >= project.fns.len() {
        return Vec::new();
    }
    let mut seen = BTreeSet::from([root]);
    expand(project, root, down, &mut seen, |f| &f.calls);
    expand(project, root, up, &mut seen, |f| &f.callers);
    seen.into_iter().collect()
}

/// Frontier expansion shared by the callee/caller directions: walk `hops` steps
/// from `root` along `edges`, adding every reached fn to `seen`.
fn expand(
    project: &Project,
    root: usize,
    hops: u8,
    seen: &mut BTreeSet<usize>,
    edges: impl Fn(&crate::model::FnDef) -> &Vec<usize>,
) {
    let mut frontier = vec![root];
    for _ in 0..hops {
        let mut next = Vec::new();
        for fi in frontier.drain(..) {
            for &t in edges(&project.fns[fi]) {
                if seen.insert(t) {
                    next.push(t);
                }
            }
        }
        if next.is_empty() {
            break;
        }
        frontier = next;
    }
}

#[cfg(test)]
mod tests {
    use super::dir_contains;

    #[test]
    fn dir_contains_matches_components_not_substrings() {
        assert!(dir_contains("", "anything/at/all.shard"));
        assert!(dir_contains("kernel", "kernel/reader.shard"));
        assert!(dir_contains("kernel", "kernel/sub/deep.shard"));
        assert!(!dir_contains("kernel", "kernel.shard")); // file, not under the dir
        assert!(!dir_contains("ker", "kernel/reader.shard")); // not a component
        assert!(!dir_contains("kernel", "std/list.shard"));
    }
}
