;;; snake_app — the IO shell over the proven game core, in proper MVU form.
;;;
;;;   view   = render     (snake_view.sexp)  — what the player sees; the
;;;                                            OBSERVABLE we prove about.
;;;   update = step_app                       — parse a key, advance the core.
;;;
;;; The driver shows view(state) after init and after each tick, so the
;;; update's Action carries only EFFECTS (Exit on quit, else Nop). Run with:
;;;
;;;   check app --no-bootstrap examples/snake_game/snake_app.sexp            (play)
;;;   check app --no-bootstrap --script <moves> examples/snake_game/snake_app.sexp
;;;
;;; One input line = one tick. Keys: w/a/s/d steer; q quits; anything else
;;; (incl. an empty line) advances in the current heading. (--no-bootstrap
;;; runs the native engine — the bootstrapped reducer is correct but far too
;;; slow to render a board each tick.)

(import "snake_view.sexp")   ; render (the view) + step + GameState/Heading

;; ---- app runtime types (the shape `check app` interprets) ------------------
(type Action (Print (List Int)) (Exit Int) (Nop))
(type (Step S A) (Step S A))

;; ---- input ----------------------------------------------------------------
(fn parse_heading ((line (List Int)) (cur Heading)) Heading
  (match line
    (Nil cur)
    ((Cons c rest)
      (if (int_eq c 119) Up                  ; 'w'
        (if (int_eq c 115) Down              ; 's'
          (if (int_eq c 97) Left             ; 'a'
            (if (int_eq c 100) Right         ; 'd'
              cur)))))))

(fn is_quit ((line (List Int))) Bool
  (match line (Nil False) ((Cons c rest) (int_eq c 113))))   ; 'q'

;; ---- update: advance the core; display is the view's job -------------------
(fn step_app ((s GameState) (line (List Int))) (Step GameState Action)
  (if (is_quit line)
      (Step s (Exit 0))
      (Step (step s (parse_heading line (dir_of s))) (Nop))))

;; ---- entrypoint ------------------------------------------------------------
(app
  (state  GameState)
  (init   (GS (Cons (Pos 8 10) (Cons (Pos 7 10) (Cons (Pos 6 10) Nil)))
              Right (Pos 10 10) 0 Alive 7))
  (view   render)
  (update step_app))
