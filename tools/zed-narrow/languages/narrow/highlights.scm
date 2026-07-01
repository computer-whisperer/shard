;; Highlights for the v2 shard language (object + proof DSL).
;; Built on top of tree-sitter-scheme's sexp grammar.
;; See docs/LANGUAGE.md for both languages: §1–9 the object language,
;; §10 the proof language (a *distinct* language sharing the file).
;;
;; Ordering: more specific patterns FIRST. Tree-sitter / Zed captures
;; later patterns override earlier ones when they overlap, so we want
;; the catch-all "Capitalized → type" rule LAST so the explicit
;; keyword / primitive / Bool matches still win on the symbols they
;; care about. All the keyword rules below are lowercase, so they never
;; collide with that uppercase catch-all.

;; -----------------------------------------------------------------
;; Lexical: comments, numbers, strings, brackets.
;; -----------------------------------------------------------------

(comment) @comment

(number) @number

;; String literals: import paths `(import "list")` and the proof
;; language's `"x+y"` codepoint-list snippets (§10.8).
(string) @string

["(" ")"] @punctuation.bracket

;; -----------------------------------------------------------------
;; Object-language structural keywords (§2–4) + the module system
;; forms layered on top (`import`, `sig` for opaque interfaces).
;; -----------------------------------------------------------------

;; All written as the head of a list. The (#match?) predicate covers
;; them in one rule.
((symbol) @keyword
  (#match? @keyword "^(fn|type|extern|match|let|if|quote|import|sig)$"))

;; -----------------------------------------------------------------
;; Native primitives (from rust_bootstrap/src/prim.rs).
;; -----------------------------------------------------------------

((symbol) @function.builtin
  (#match? @function.builtin "^([-+*/]|mod|band|bor|bxor|bshl|bshr|int_eq|sym_eq|lt|le|gen_fresh|chars_of_sym|sym_of_chars)$"))

;; -----------------------------------------------------------------
;; Proof language — top-level declarations (§10.1) and goal form
;; (§10.2). claim / axiom / requirement / fulfills drive a proof file.
;; -----------------------------------------------------------------

((symbol) @keyword
  (#match? @keyword "^(claim|axiom|requirement|fulfills|goal)$"))

;; -----------------------------------------------------------------
;; Proof language — proof / tactic combinators (§10.3–10.5).
;; The structural "verbs" of a proof: how a goal is decomposed and
;; how a sequent is rewritten. Coloured as keywords (control flow).
;; -----------------------------------------------------------------

((symbol) @keyword
  (#match? @keyword "^(refl|steps|induct|induct2|case-on|case|wf-induct|rewrite-with|absurd|by)$"))

;; -----------------------------------------------------------------
;; Proof language — tactic steps + equation-reference builders
;; (§10.5–10.6). The operations applied inside `steps` and the things
;; a rewrite/absurd cites. Coloured as builtin functions.
;; -----------------------------------------------------------------

((symbol) @function.builtin
  (#match? @function.builtin "^(reduce|simp|compute|unfold|rewrite|hyp|premise|lemma|inst)$"))

;; -----------------------------------------------------------------
;; Proof language — `by` theory names (§10.7): the registered decision
;; procedures. Coloured as builtin constants.
;; -----------------------------------------------------------------

((symbol) @constant.builtin
  (#match? @constant.builtin "^(arith)$"))

;; -----------------------------------------------------------------
;; Bool / wildcard.
;; -----------------------------------------------------------------

;; True/False are the hardcoded Bool ctors (prim.rs + step's If arm).
;; lowercase true/false appear in proofs as the rewrite ALL flag
;; (`(rewrite EQREF DIR SIDE ALL …)`, §10.5). Both as constants.
((symbol) @constant.builtin
  (#match? @constant.builtin "^(True|False|true|false)$"))

;; Sequent sides (`lhs`/`rhs`/`both`) and rewrite directions
;; (`lr`/`rl`) — small enum-like operands of the tactic steps.
((symbol) @constant
  (#match? @constant "^(lhs|rhs|both|lr|rl)$"))

;; `_` is the conventional ignored binding in patterns and let-args.
((symbol) @variable.special
  (#match? @variable.special "^_$"))

;; -----------------------------------------------------------------
;; Equation operator: `(= L R)` is the head of every goal/premise
;; equation (§10.2).
;; -----------------------------------------------------------------

((symbol) @operator
  (#match? @operator "^=$"))

;; -----------------------------------------------------------------
;; Heuristic: identifiers starting with an uppercase letter are
;; treated as types or constructors. This catches all of Expr, Pat,
;; Ctor, Cons, Nil, PVar, Some, None, Goal, Sequent, Theory, Map,
;; MEmpty, … without needing to know the schema. Lowercase identifiers
;; fall through to plain symbol coloring (= local binders or fn names).
;; -----------------------------------------------------------------

((symbol) @type
  (#match? @type "^[A-Z][A-Za-z0-9_]*$"))
