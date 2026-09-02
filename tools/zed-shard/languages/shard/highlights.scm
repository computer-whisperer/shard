;; Highlights for shard (object language + proof DSL + module/artifact
;; forms), over tree-sitter-scheme's s-expression grammar.
;;
;; Vocabulary source: the kernel readers — kernel/reader.shard (top-level
;; and object forms, loader sugars, bin/lib clauses), kernel/proof_reader.shard
;; (proofs, steps, eqrefs), kernel/types.shard (the native prim table).
;; docs/LANGUAGE.md is the prose spec.
;;
;; ORDERING: Zed applies the LAST pattern that captures a node, so this
;; file goes generic → specific. Catch-alls (plain symbol, list head,
;; capitalized = type) come first; keyword / builtin / definition-name
;; rules follow and override them.

;; -----------------------------------------------------------------
;; Lexical layer.
;; -----------------------------------------------------------------

(comment) @comment

;; `;;;` = file-header / section docstrings (LANGUAGE.md §1).
((comment) @comment.doc
  (#match? @comment.doc "^;;;"))

(number) @number

;; import paths, `"x+y"` codepoint-list snippets, I/O text.
(string) @string
(escape_sequence) @string.escape

["(" ")"] @punctuation.bracket

;; -----------------------------------------------------------------
;; Catch-alls.
;; -----------------------------------------------------------------

(symbol) @variable

;; A list head is a call (user fn, ctor, or form); the rules below
;; refine the ones that are keywords / builtins.
(list
  .
  (symbol) @function)

;; Capitalized identifier = type or constructor (the kernel's convention:
;; all variables are lowercase). Covers Int, Bool, List, Cons, Nil, Some,
;; None, Z, S, Expr, … in every position.
((symbol) @type
  (#match? @type "^[A-Z][A-Za-z0-9_]*$"))

;; -----------------------------------------------------------------
;; Constants and small enum-like operands.
;; -----------------------------------------------------------------

;; True/False = the kernel's Bool ctors; true/false = the rewrite
;; occurrence flag (`(rewrite EQREF DIR SIDE ALL …)`).
((symbol) @boolean
  (#match? @boolean "^(True|False|true|false)$"))

;; sequent sides, rewrite directions, the one `by` theory, axiom kinds.
((symbol) @constant
  (#match? @constant "^(lhs|rhs|both|lr|rl|arith|operational|bridging)$"))

;; `_` = ignored binder / counted hole; `ih`, `ih1`, … = the auto-named
;; induction hypotheses.
((symbol) @variable.special
  (#match? @variable.special "^(_|ih[0-9]*)$"))

;; `(= L R)` heads every equation; `::` heads a `use` path.
((list
   .
   (symbol) @operator)
  (#match? @operator "^(=|::)$"))

;; -----------------------------------------------------------------
;; Native primitives (kernel/types.shard tc_builtin_sig) + the literal
;; sugars that read like calls (`list`, `refine_val`, `refine_try`).
;; -----------------------------------------------------------------

((list
   .
   (symbol) @function.builtin)
  (#match? @function.builtin "^([-+*/]|mod|tmod|ediv|band|bor|bxor|bshl|bshr|int_eq|lt|le|sym_eq|gen_fresh|sym_of_chars|chars_of_sym|list|refine_val|refine_try)$"))

;; -----------------------------------------------------------------
;; Object language + module system + artifacts (reader.shard).
;;   definitional: type fn extern sig record
;;   expressions:  match let if quote
;;   modules:      import use use-module
;;   artifacts:    bin lib app cli returns
;;   sugars:       make with refine S^ inline chain measure
;; -----------------------------------------------------------------

((list
   .
   (symbol) @keyword)
  (#match? @keyword "^(type|fn|extern|sig|record|match|let|if|quote|import|use|use-module|bin|lib|app|cli|returns|make|with|refine|S\\^|inline|chain|measure)$"))

;; `(sig fn NAME …)` / `(sig type NAME …)` — the second word is a keyword too.
((list
   .
   (symbol) @_sig
   .
   (symbol) @keyword)
  (#eq? @_sig "sig")
  (#match? @keyword "^(fn|type)$"))

;; `(use (:: …) as ALIAS)`
((list
   .
   (symbol) @_use
   .
   (list)
   .
   (symbol) @keyword)
  (#eq? @_use "use")
  (#eq? @keyword "as"))

;; -----------------------------------------------------------------
;; Proof language (proof_reader.shard).
;;   declarations: claim axiom requirement fulfills proof-for goal
;;   proofs:       refl steps induct case-on case wf-induct subterm-induct
;;                 below refine-fact have fin-split div-facts inject
;;                 rewrite-with exact-conv absurd by admit auto
;; -----------------------------------------------------------------

((list
   .
   (symbol) @keyword)
  (#match? @keyword "^(claim|axiom|requirement|fulfills|proof-for|goal|refl|steps|induct|case-on|case|wf-induct|subterm-induct|below|refine-fact|have|fin-split|div-facts|inject|rewrite-with|exact-conv|absurd|by|admit|auto)$"))

;; The bare leaf spellings: `refl`, `admit`, `auto` as a whole proof.
((symbol) @keyword
  (#match? @keyword "^(refl|admit|auto)$"))

;; Steps (inside `steps`) and equation references (what a rewrite cites).
((list
   .
   (symbol) @function.builtin)
  (#match? @function.builtin "^(reduce|simp|compute|unfold|rewrite|change|inspect|hyp|premise|lemma)$"))

;; -----------------------------------------------------------------
;; Clause heads — the sub-forms that qualify a declaration or step:
;;   bin/lib:  entry externs trusts requires exports accepts
;;   axiom:    kind      record: ctor      fn: measure's (struct X)
;;   steps:    stop at inst     certs: rows
;; -----------------------------------------------------------------

((list
   .
   (symbol) @attribute)
  (#match? @attribute "^(entry|externs|trusts|requires|exports|accepts|kind|ctor|struct|stop|at|inst|rows)$"))

;; -----------------------------------------------------------------
;; Names being defined or cited.
;; -----------------------------------------------------------------

;; (fn NAME …) (extern NAME …) (claim NAME …) (bin NAME …) … — the name
;; right after the head. Capitalized names (type/record) keep @type.
((list
   .
   (symbol) @_def
   .
   (symbol) @function)
  (#match? @_def "^(fn|extern|claim|axiom|requirement|fulfills|proof-for|returns|bin|lib|app|cli)$")
  (#match? @function "^[^A-Z]"))

;; (sig fn NAME …)
((list
   .
   (symbol) @_sig
   .
   (symbol) @_fn
   .
   (symbol) @function)
  (#eq? @_sig "sig")
  (#eq? @_fn "fn"))

;; Cited fn / theorem names — one name: (lemma NAME) (unfold NAME SIDE)
;; (inline NAME) (entry NAME); a name list: (stop NAME …) (externs NAME …)
;; (exports NAME …) (trusts NAME …) (requires NAME …).
((list
   .
   (symbol) @_cite
   .
   (symbol) @function)
  (#match? @_cite "^(lemma|unfold|inline|entry)$"))

((list
   .
   (symbol) @_cites
   (symbol) @function)
  (#match? @_cites "^(stop|externs|exports|trusts|requires)$"))

;; Keyed cert rows: (rows (KEY MULT) …) — KEY is `goal` or a have's name.
((list
   .
   (symbol) @_rows
   (list
     .
     (symbol) @label))
  (#eq? @_rows "rows"))

;; Record fields: (record NAME (ctor C)? (FIELD TYPE)…), (make NAME (FIELD V)…),
;; (with E (FIELD V)…).
((list
   .
   (symbol) @_rec
   .
   (_)
   (list
     .
     (symbol) @property))
  (#match? @_rec "^(record|make|with)$")
  (#not-match? @property "^ctor$"))

;; Named hypotheses / cut facts: (have NAME EQ …) (premise NAME) (hyp NAME)
;; (inject EQREF (NAME…) …). Positional `(premise 0)` stays a number.
((list
   .
   (symbol) @_h
   .
   (symbol) @label)
  (#match? @_h "^(have|premise|hyp)$")
  (#not-match? @label "^(ih[0-9]*|_)$"))

;; Binders: fn / extern / sig-fn parameters, goal binders, let bindings,
;; (inst NAME TERM), the field names of an induct/case-on `case`.
((list
   .
   (symbol) @_fn
   .
   (symbol)
   .
   (list
     (list
       .
       (symbol) @variable.parameter)))
  (#match? @_fn "^(fn|extern)$"))

((list
   .
   (symbol) @_sig
   .
   (symbol)
   .
   (symbol)
   .
   (list
     (list
       .
       (symbol) @variable.parameter)))
  (#eq? @_sig "sig"))

((list
   .
   (symbol) @_form
   .
   (list
     (list
       .
       (symbol) @variable.parameter)))
  (#match? @_form "^(goal|let)$"))

((list
   .
   (symbol) @_inst
   .
   (symbol) @variable.parameter)
  (#eq? @_inst "inst"))

((list
   .
   (symbol) @_case
   .
   (symbol)
   .
   (list
     (symbol) @variable.parameter))
  (#eq? @_case "case"))

;; -----------------------------------------------------------------
;; Quoted symbols: 'foo = (quote foo) = SymLit.
;; -----------------------------------------------------------------

(abbreviation
  "'" @string.special.symbol
  (symbol) @string.special.symbol)
