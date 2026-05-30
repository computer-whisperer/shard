;;; eval_app.sexp — `eval` as a standalone shard CLI app.
;;;
;;;   check cli tools/reader.sexp examples/cli/eval_app.sexp -- <module.sexp> <expr>
;;;
;;; Pipeline, entirely in shard: GetArgs → ReadFile(module) →
;;; parse_module → parse_expr → compute_expr → show_expr → Write → Exit.
;;; The Rust side only ferries bytes (read the file, write the result);
;;; parsing and evaluation are the self-hosted reader + kernel reducer.
;;;
;;; Needs tools/reader.sexp (parse_module / parse_expr / typedef_ctors /
;;; append_sym) loaded alongside it, plus the kernel's compute_expr.

;; ---- effect protocol (shared shape with the cli driver) -------------------
(type Action
  (GetArgs)
  (ReadFile (List Int))
  (Write    (List Int))
  (Exit     Int))

(type Event
  (Started)
  (Args   (List (List Int)))
  (FileOk (List Int))
  (FileErr)
  (Wrote))

(type (Step S A) (Step S A))

;; ---- rendering: Expr → surface bytes --------------------------------------
(fn append_int ((xs (List Int)) (ys (List Int))) (List Int)
  (match xs (Nil ys) ((Cons h t) (Cons h (append_int t ys)))))

(fn show_nat ((n Int)) (List Int)              ; n >= 0
  (if (lt n 10)
      (Cons (+ 48 n) Nil)
      (append_int (show_nat (/ n 10)) (Cons (+ 48 (mod n 10)) Nil))))

(fn show_int ((n Int)) (List Int)
  (if (lt n 0)
      (Cons 45 (show_nat (- 0 n)))             ; '-' then |n|
      (show_nat n)))

(fn show_expr ((e Expr)) (List Int)
  (match e
    ((IntLit n)  (show_int n))
    ((SymLit s)  (Cons 39 (chars_of_sym s)))   ; 'sym
    ((FVar s)    (chars_of_sym s))
    ((BVar k)    (Cons 36 (show_int k)))        ; $k (not expected in NF)
    ((Ctor name args)
      (match args
        (Nil (chars_of_sym name))
        (_   (Cons 40
               (append_int (chars_of_sym name)
                 (append_int (show_args args) (Cons 41 Nil)))))))
    ((Call name args)                           ; a stuck call, if any
      (Cons 40
        (append_int (chars_of_sym name)
          (append_int (show_args args) (Cons 41 Nil)))))
    ((Match _ _) "<match>")
    ((Let _ _)   "<let>")
    ((If _ _ _)  "<if>")))

(fn show_args ((args (List Expr))) (List Int)   ; " a1 a2 …"
  (match args
    (Nil Nil)
    ((Cons a rest) (Cons 32 (append_int (show_expr a) (show_args rest))))))

;; ---- the eval pipeline ----------------------------------------------------
;; The ambient ctor base: stdlib ctors a user program may use without
;; declaring (parse needs them to tell Ctor from Call).
(fn stdlib_ctors () (List Symbol)
  (list (quote Cons) (quote Nil) (quote Some) (quote None)
        (quote True) (quote False) (quote Pair)))

(fn module_all_ctors ((m Module)) (List Symbol)
  (match m ((Module tds _ _) (append_sym (typedef_ctors tds) (stdlib_ctors)))))

(fn eval_run ((module_src (List Int)) (expr_src (List Int))) (List Int)
  (match (parse_module module_src (stdlib_ctors))
    (None "error: could not parse module\n")
    ((Some m)
      (match (parse_expr expr_src (module_all_ctors m))
        (None "error: could not parse expression\n")
        ((Some e)
          (append_int (show_expr (compute_expr m e)) (Cons 10 Nil)))))))

;; ---- update: the protocol state machine -----------------------------------
;; State carries the expression string between GetArgs and the file read.
(type EvalState
  (ESInit)
  (ESHaveExpr (List Int))
  (ESDone))

(fn evalapp ((s EvalState) (e Event)) (Step EvalState Action)
  (match s
    (ESInit
      (match e
        (Started (Step ESInit (GetArgs)))
        ((Args files)
          (match files
            ((Cons mfile (Cons expr rest)) (Step (ESHaveExpr expr) (ReadFile mfile)))
            (_ (Step ESDone (Write "usage: <module.sexp> <expr>\n")))))
        (_ (Step ESDone (Exit 2)))))
    ((ESHaveExpr expr)
      (match e
        ((FileOk bytes) (Step ESDone (Write (eval_run bytes expr))))
        (FileErr        (Step ESDone (Write "error: cannot read module file\n")))
        (_              (Step ESDone (Exit 2)))))
    (ESDone
      (match e
        (Wrote (Step ESDone (Exit 0)))
        (_     (Step ESDone (Exit 0)))))))

(cli
  (state  EvalState)
  (init   ESInit)
  (update evalapp))
