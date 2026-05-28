# Zed extension: Narrow language

Syntax highlighting for the v2 proving-bootstrap narrow object
language, as documented in `docs/LANGUAGE.md`. Targets the
`.sexp` extension.

## Install as a dev extension

1. Open Zed.
2. Open the command palette (`Ctrl-Shift-P` / `Cmd-Shift-P`).
3. Run **`zed: install dev extension`**.
4. Pick this directory (`tools/zed-narrow/`).

Zed will fetch the tree-sitter-scheme grammar pinned in
`extension.toml` and load the language config + highlight queries
from `languages/narrow/`.

## What gets highlighted

- `fn`, `type`, `extern`, `match`, `let`, `if`, `quote` — as keywords.
- `+ - * / mod band bor bxor bshl bshr int_eq sym_eq lt le gen_fresh`
  — as built-in functions.
- `True` / `False` — as constants (matches the kernel's hardcoded
  Bool ctor names).
- `_` — as a special variable (wildcard / ignored binding).
- Identifiers beginning with an uppercase letter — as types
  (heuristic; covers ctor names like `Cons`, `Nil`, `PVar`, type
  names like `Expr`, `Goal`).
- Lowercase identifiers fall through to default symbol coloring
  (locals, fn names).

## Caveats

- **`.sexp` is a generic extension.** This extension will apply to
  every `.sexp` file open in Zed, not just files in this project.
  If that's a problem, change `path_suffixes` in
  `languages/narrow/config.toml` to something project-specific
  (e.g. `.narrow.sexp`) and rename the kernel files to match.

- **Uppercase = type is a heuristic.** Free variables that happen to
  start with a capital letter will also get the type color. The
  kernel's convention (all FVars are lowercase) keeps this honest in
  practice.

- **The grammar is tree-sitter-scheme.** It parses generic
  s-expressions, not our language specifically. So things like
  arity mismatches or unknown forms are NOT flagged — Zed will
  happily highlight them. The Rust loader is the real syntax
  authority.

- **Tree-sitter-scheme commit is pinned.** If the upstream grammar
  moves and you need a fresher commit, update the `commit = …`
  line in `extension.toml`.

## Iterating

If a particular file doesn't look right, the quickest path to
diagnose is:

1. Open the file in Zed.
2. Open the command palette and run
   **`debug: open syntax tree view`** (or similar — Zed's command
   name has changed across versions).
3. Click on the token that's mis-highlighted. The tree view shows
   the actual node type from tree-sitter-scheme.
4. Edit `languages/narrow/highlights.scm` to target that node type
   correctly, then run **`zed: rebuild dev extension`** (or restart
   Zed if that command isn't present).
