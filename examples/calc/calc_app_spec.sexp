;;; calc_app_spec — spec-correctness of the stateful calculator (slice B).
;;;
;;; The interactive app's `step` is built on the IMPLEMENTATION `run`. Here
;;; we define `step_spec`, the same update against the high-level `spec_run`,
;;; and prove they agree on every state and input line:
;;;
;;;   ∀ s line.  (step s line) = (step_spec s line)
;;;
;;; This is the stateful sequel to the headline `run = spec_run`: the
;;; observable behaviour of the running application — the next state AND the
;;; Action it emits — is spec-correct, derived in one rewrite from that
;;; capstone. `step` and `step_spec` differ ONLY in their scrutinee (run vs
;;; spec_run); unfold both, rewrite the scrutinee by run_eq_spec, and the
;;; two match expressions are syntactically identical.

(import "calc_app.sexp")    ; step, show_ascii, CalcState/Action/Step, run
(import "calc_equiv.sexp")  ; run_eq_spec (the capstone) + spec_run

;; The spec-side update: identical to `step` but reading `spec_run`.
(fn step_spec ((s CalcState) (line (List Int))) (Step CalcState Action)
  (match (spec_run line)
    (None       (Step s                   (Print (Cons 63 Nil))))
    ((Some n)   (Step (CalcState (Some n)) (Print (show_ascii n))))))

;; step = step_spec, for ALL prior states and ALL input lines.
(claim step_eq_spec
  (Goal
    (list (Param 's    (ty CalcState))
          (Param 'line (ty List Int)))
    (list)
    (Equation
      (Call 'step      (list (FVar 's) (FVar 'line)))
      (Call 'step_spec (list (FVar 's) (FVar 'line)))))
  (Steps (list (Unfold 'step      Lhs)
               (Unfold 'step_spec Rhs)
               (Rewrite (Lemma 'run_eq_spec) Lr Lhs True (list)))
    Refl))
