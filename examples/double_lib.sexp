;;; A tiny user module: a single fn `double` that returns 2x.
;;; Used by examples/double_claims.sexp via (use-module …).

(fn double ((x Int)) Int (+ x x))
