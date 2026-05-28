;; Highlights for the v2 narrow language.
;; Built on top of tree-sitter-scheme's sexp grammar.
;; See docs/LANGUAGE.md for the language definition.
;;
;; Ordering: more specific patterns FIRST. Tree-sitter / Zed captures
;; later patterns override earlier ones when they overlap, so we want
;; the catch-all "Capitalized → type" rule LAST so the explicit
;; keyword / primitive / Bool matches still win on the symbols they
;; care about.

;; -----------------------------------------------------------------
;; Lexical: comments, numbers, brackets.
;; -----------------------------------------------------------------

(comment) @comment

(number) @number

["(" ")"] @punctuation.bracket

;; -----------------------------------------------------------------
;; Top-level / structural keywords.
;; -----------------------------------------------------------------

;; fn, type, extern, match, let, if, quote — always written as the
;; head of a list. The (#match?) predicate is used rather than (#eq?)
;; so all of them are covered in one rule.
((symbol) @keyword
  (#match? @keyword "^(fn|type|extern|match|let|if|quote)$"))

;; -----------------------------------------------------------------
;; Native primitives (from src/prim.rs).
;; -----------------------------------------------------------------

((symbol) @function.builtin
  (#match? @function.builtin "^([-+*/]|mod|band|bor|bxor|bshl|bshr|int_eq|sym_eq|lt|le|gen_fresh)$"))

;; -----------------------------------------------------------------
;; Bool / wildcard.
;; -----------------------------------------------------------------

;; True and False are the hardcoded Bool ctors (see prim.rs + step's
;; If arm). Highlight them as constants so they pop visually.
((symbol) @constant.builtin
  (#match? @constant.builtin "^(True|False)$"))

;; `_` is the conventional ignored binding in patterns and let-args.
((symbol) @variable.special
  (#match? @variable.special "^_$"))

;; -----------------------------------------------------------------
;; Heuristic: identifiers starting with an uppercase letter are
;; treated as types or constructors. This catches all of Expr, Pat,
;; Ctor, Cons, Nil, PVar, Some, None, Goal, Sequent, Theory, …
;; without needing to know the schema. Lowercase identifiers fall
;; through to plain symbol coloring (= local binders or fn names).
;; -----------------------------------------------------------------

((symbol) @type
  (#match? @type "^[A-Z][A-Za-z0-9_]*$"))
