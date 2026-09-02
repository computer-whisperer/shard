;; Outline / symbol navigation: every top-level definition and proof
;; declaration by name.

(comment) @annotation

;; (fn NAME …) (extern NAME …) (type NAME …) (record NAME …)
;; (claim NAME …) (axiom NAME …) (requirement NAME …) (fulfills NAME …)
;; (proof-for NAME …) (returns NAME …) (bin NAME …) (lib NAME …) (app …) (cli …)
((list
   .
   (symbol) @context
   .
   (symbol) @name)
  (#match? @context "^(fn|extern|type|record|claim|axiom|requirement|fulfills|proof-for|returns|bin|lib|app|cli)$")) @item

;; (type (NAME T…) …) — parameterized type
((list
   .
   (symbol) @context
   .
   (list
     .
     (symbol) @name))
  (#eq? @context "type")) @item

;; (sig fn NAME …) / (sig type NAME …)
((list
   .
   (symbol) @context
   .
   (symbol) @context.extra
   .
   (symbol) @name)
  (#eq? @context "sig")) @item

;; (use (:: a b c *)) / (use-module "file") / (import "file")
((list
   .
   (symbol) @context
   .
   (list) @name)
  (#eq? @context "use")) @item

((list
   .
   (symbol) @context
   .
   (string) @name)
  (#match? @context "^(import|use-module)$")) @item
