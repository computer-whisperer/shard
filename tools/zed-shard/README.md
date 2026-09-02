# Zed extension: Shard

Editor support for the shard language — object language, proof DSL,
module and artifact forms — for every `*.shard` file. The prose spec
is `docs/LANGUAGE.md`; the vocabulary this extension highlights is
taken from the kernel readers (`kernel/reader.shard`,
`kernel/proof_reader.shard`, `kernel/types.shard`), which are the
authority on what the language accepts.

## Install as a dev extension

1. Open Zed.
2. Command palette (`Ctrl-Shift-P` / `Cmd-Shift-P`) →
   **`zed: install dev extension`**.
3. Pick this directory (`tools/zed-shard/`).

Zed fetches the tree-sitter-scheme grammar pinned in `extension.toml`,
compiles it, and loads `languages/shard/`. After editing the query
files run **`zed: rebuild dev extension`** (or reinstall).

If you had the old `narrow` dev extension installed from
`tools/zed-narrow/`, uninstall it (**`zed: extensions`**, find
"Narrow") and install this one — the directory and extension id both
changed.

## What you get

**Highlighting** (`highlights.scm`), keyed on list-head symbols:

- Object-language forms: `type fn extern sig record match let if quote`,
  the module forms `import use use-module`, the artifact forms
  `bin lib app cli returns`, and the loader sugars
  `make with refine S^ inline chain measure`.
- Proof declarations and proof forms: `claim axiom requirement fulfills
  proof-for goal`, `refl steps induct case-on case wf-induct
  subterm-induct below refine-fact have fin-split div-facts inject
  rewrite-with exact-conv absurd by admit auto`.
- Steps and citations as builtins: `reduce simp compute unfold rewrite
  change inspect`, `hyp premise lemma`; native primitives
  (`+ - * / mod tmod ediv band bor bxor bshl bshr int_eq lt le sym_eq
  gen_fresh sym_of_chars chars_of_sym`) and the `list` /
  `refine_val` / `refine_try` sugars likewise.
- Clause heads as attributes: `entry externs trusts requires exports
  accepts kind ctor struct stop at inst rows`.
- Definition names (`(fn NAME`, `(claim NAME`, `(bin NAME`, …) and
  cited names (`(lemma NAME)`, `(unfold NAME …)`, `(stop NAME …)`) as
  functions; named hypotheses (`(have NAME …)`, `(premise NAME)`,
  `(hyp NAME)`, cert-row keys) as labels; fn/goal/let binders and
  `case` field names as parameters; record fields as properties.
- `True`/`False` and the rewrite flags `true`/`false` as booleans;
  `lhs rhs both lr rl arith operational bridging` as constants; `_`
  and the auto-named `ih`/`ih1`/… as special variables; `'sym` as a
  symbol literal; `;;;` lines as doc comments.
- Capitalized identifiers as types (the kernel's convention: every
  variable is lowercase, so this also covers constructors).

**Outline** (`outline.scm`): every `fn`, `extern`, `sig`, `type`,
`record`, `claim`, `axiom`, `requirement`, `fulfills`, `proof-for`,
`returns`, `bin`, `lib`, `import`, `use`, `use-module` by name — the
outline panel and `outline: toggle` / project symbol search work over
proof files.

**Editing**: 2-space indentation inside any list (`indents.scm`),
bracket matching and rainbow-bracket exclusion of strings
(`brackets.scm`), no autoclose inside strings/comments
(`overrides.scm`), `-`/`_`/`^` as word characters so `case-on` and
`S^` select as one word, and text objects (`textobjects.scm`): the
enclosing top-level form for vim-mode `af`, comment blocks for `gc`.

## Optional: shardfmt on save

`tools/shardfmt` is a gate-not-printer (`shardfmt --check FILE`) with
a rewriting mode (`shardfmt FILE`). Zed runs external formatters from
user settings, not from an extension, so if you want format-on-save add
something like this to your `settings.json` once the `shardfmt` binary
is on your `PATH`:

```json
"languages": {
  "Shard": {
    "formatter": { "external": { "command": "shardfmt", "arguments": ["{buffer_path}"] } },
    "format_on_save": "on"
  }
}
```

(`shardfmt` rewrites the file in place rather than printing to stdout,
so pair this with Zed's autosave or run it as a task instead.)

## Caveats

- **`.shard` is a generic suffix.** The language applies to every
  `.shard` file Zed opens, not only this repo's.
- **The grammar is tree-sitter-scheme.** It parses generic
  s-expressions; the whole hand-written corpus (636 files) parses with
  no error nodes, but arity mistakes and unknown forms are not flagged —
  `bin/check` is the syntax authority. Scheme-only lexemes (`#t`,
  `#\a`, `[ ]`, `{ }`, `#| |#`) never appear in shard source, so their
  rules are dormant.
- **Capitalized = type is a heuristic.** Types and constructors share
  one color; a capitalized `use … as ALIAS` gets it too.
- **Head-keyed rules.** A keyword is only colored in head position, so
  a binder named `case` or `list` stays a plain variable — but a
  lowercase user fn *called* `stop` or `at` would be colored as a
  clause head.

## Iterating

The query files are plain tree-sitter queries and can be checked
without Zed: with the `tree-sitter` CLI available, copy a `.shard`
file to `x.scm`, `cd grammars/scheme` (after Zed has fetched it) and
run `tree-sitter query ../../languages/shard/highlights.scm x.scm`.
Inside Zed, **`debug: open syntax tree view`** shows the node under the
cursor; note that when several patterns capture one node, Zed applies
the **last** one in the file, so generic rules go first.
