;; Text objects. Zed's textobjects queries accept only the fixed capture
;; names (no `@_helper` captures for predicates), so the "function" object
;; is the enclosing TOP-LEVEL form — a fn, type, claim, bin, … — and
;; comments group as usual.

((comment)+ @comment.around) @comment.inside

(program
  (list) @function.around)
