;;; M3 — linear-memory reverse: foundation lemmas + the capstone
;;; statement. Slice 34 stands up the model and the array framing; the
;;; loop-invariant induction (the capstone proof) is the next slice.
;;;
;;; Layering:
;;;   - read_write_eq / read_write_neq : the array framing
;;;     `read(write(m,a,v),b)` cased on a=b. THE foundation — these
;;;     unfold all the way to the `int_eq` test and discharge it via
;;;     eqdec reflexivity (read_write_eq) or a disequality premise
;;;     (read_write_neq). This is slice 33's insert/lookup story
;;;     re-proven at the total-read level.
;;;   - read_swap_j : how a swap is observed at one end. Proven purely
;;;     by COMPOSING the framing lemmas — never unfolds to MCons. The
;;;     demonstration that the framing layer is reusable.
;;;   - rev_loop_zero : the loop base case (zero swaps = identity).
;;;
;;; The capstone (`mem_reverses`, stated at the bottom) is NOT yet a
;;; claim — it needs the loop-invariant induction. See the reassess
;;; note there.

(use-module "nat_lib.sexp")
(use-module "map_lib.sexp")
(use-module "mem_lib.sexp")

;; ---------------------------------------------------------------------------
;; int_eq_refl — reflexivity of int_eq on a variable, via eqdec.
;; (Re-proven here for self-containment; the same one-liner appears in
;; map_lemmas.sexp. A shared "claims library" mechanism would let us
;; cite it across files — there isn't one yet, see REVISIT.)
;; ---------------------------------------------------------------------------

(claim int_eq_refl
  (Goal
    (list (Param 'k (ty Int)))
    (list)
    (Equation
      (Call 'int_eq (list (FVar 'k) (FVar 'k)))
      (Ctor 'True (list))))
  (ByTheory 'eqdec (Cert 'eqdec (list))))

;; ---------------------------------------------------------------------------
;; read_write_eq: ∀ m a v. (read (write m a v) a) = v.
;;
;; Read back the value you just wrote. Peel write→insert→MCons, fire
;; read's match to expose (if (int_eq a a) v …), discharge int_eq a a
;; via int_eq_refl, fire the if.
;; ---------------------------------------------------------------------------

(claim read_write_eq
  (Goal
    (list (Param 'm (ty Map Int))
          (Param 'a (ty Int))
          (Param 'v (ty Int)))
    (list)
    (Equation
      (Call 'read (list (Call 'write (list (FVar 'm) (FVar 'a) (FVar 'v)))
                        (FVar 'a)))
      (FVar 'v)))
  (Steps
    (list (Unfold 'write Lhs)                              ; (read (insert a v m) a)
          (Unfold 'insert Lhs)                             ; (read (MCons a v m) a)
          (Simp Lhs)                                       ; (if (int_eq a a) v (read m a))
          (Rewrite (Lemma 'int_eq_refl) Lr Lhs True (list)); (if True v …)
          (Simp Lhs))                                      ; v
    Refl))

;; ---------------------------------------------------------------------------
;; read_write_neq: ∀ m a b v. (int_eq b a) = False ⊢
;;   (read (write m a v) b) = (read m b).
;;
;; A write at a different address is invisible to a read at b. The
;; disequality is a premise (consumed, not proven); collapse the if's
;; else-branch with it. Note the RHS (read m b) is left as a stuck Call
;; (m is a variable), and the LHS Simps to the same stuck Call — Refl
;; matches them directly.
;; ---------------------------------------------------------------------------

(claim read_write_neq
  (Goal
    (list (Param 'm (ty Map Int))
          (Param 'a (ty Int))
          (Param 'b (ty Int))
          (Param 'v (ty Int)))
    (list
      (Equation (Call 'int_eq (list (FVar 'b) (FVar 'a))) (Ctor 'False (list))))
    (Equation
      (Call 'read (list (Call 'write (list (FVar 'm) (FVar 'a) (FVar 'v)))
                        (FVar 'b)))
      (Call 'read (list (FVar 'm) (FVar 'b)))))
  (Steps
    (list (Unfold 'write Lhs)                          ; (read (insert a v m) b)
          (Unfold 'insert Lhs)                         ; (read (MCons a v m) b)
          (Simp Lhs)                                   ; (if (int_eq b a) v (read m b))
          (Rewrite (Premise 0) Lr Lhs True (list))     ; (if False v (read m b))
          (Simp Lhs))                                  ; (read m b)
    Refl))

;; ---------------------------------------------------------------------------
;; read_swap_j: ∀ m i j. (read (swap m i j) j) = (read m i).
;;
;; Observing a swap at the j end returns what was at i. Proven PURELY by
;; composing the framing layer: swap's outer write is at j with value
;; (read m i), so read_write_eq fires directly. No unfold-to-MCons, no
;; premise. This is the payoff of the framing lemmas — the swap reasons
;; abstractly over read/write.
;; ---------------------------------------------------------------------------

(claim read_swap_j
  (Goal
    (list (Param 'm (ty Map Int))
          (Param 'i (ty Int))
          (Param 'j (ty Int)))
    (list)
    (Equation
      (Call 'read (list (Call 'swap (list (FVar 'm) (FVar 'i) (FVar 'j)))
                        (FVar 'j)))
      (Call 'read (list (FVar 'm) (FVar 'i)))))
  (Steps
    (list (Unfold 'swap Lhs)   ; (read (write (write m i (read m j)) j (read m i)) j)
          (Rewrite (Lemma 'read_write_eq) Lr Lhs True (list)))  ; (read m i)
    Refl))

;; ---------------------------------------------------------------------------
;; rev_loop_zero: ∀ m i j. (rev_loop m i j Z) = m.
;;
;; The loop base case: zero swaps leave memory unchanged. Trivial by
;; Simp (the Z arm returns m). The skeleton the capstone induction
;; bottoms out on.
;; ---------------------------------------------------------------------------

(claim rev_loop_zero
  (Goal
    (list (Param 'm (ty Map Int))
          (Param 'i (ty Int))
          (Param 'j (ty Int)))
    (list)
    (Equation
      (Call 'rev_loop (list (FVar 'm) (FVar 'i) (FVar 'j) (Ctor 'Z (list))))
      (FVar 'm)))
  (Steps (list (Simp Lhs)) Refl))

;; ---------------------------------------------------------------------------
;; CAPSTONE (stated, not yet proven) — mem_reverses:
;;
;;   ∀ xs : (List Int).
;;     (dump 0 (length_nat xs)
;;           (rev_loop (load xs 0 MEmpty)
;;                     0
;;                     (- (int_of_nat (length_nat xs)) 1)
;;                     (half_nat (length_nat xs))))
;;     = (rev xs)
;;
;; "Load xs into fresh memory at 0…n-1, run the in-place swap loop
;; floor(n/2) times, dump the n cells back out — you get rev xs." This
;; is the M3 data-refinement: list ↔ linear memory, proven for
;; universal n. Cites list_lib's `rev`.
;;
;; WHY IT'S NOT A CLAIM YET (the slice-34 reassess point): the proof is
;; an induction maintaining a loop invariant ("after k swaps, position p
;; holds the cell originally at its mirror image"), discharged with the
;; swap framing lemmas. Standing it up surfaced the concrete gaps to
;; decide on next:
;;   1. read_swap at the OTHER positions (read_swap_i, read_swap_other)
;;      needs the CONDITIONAL framing lemma read_write_neq cited via
;;      RewriteWith with disequality-premise discharge — straightforward
;;      but not yet written.
;;   2. The loop guard / invariant arithmetic is INEQUALITY reasoning
;;      (i < j, p between bounds, i + j = n - 1). Our LIA backend decides
;;      EQUALITIES only; the moment the invariant needs `lt`/`le`, we'll
;;      want an order/inequality decision procedure (a sibling backend,
;;      or an LIA extension). This is the predicted "next infrastructure"
;;      from the M3 discussion, now confirmed concretely.
;;   3. The list↔memory bridge (load/dump) carries a plumbing tax
;;      (length_nat / half_nat / int_of_nat conversions) — TRANSFER's
;;      "representation alignment is death by a thousand lemmas". Worth
;;      deciding whether to lean on the order backend (1,2) to erase it.
;; ---------------------------------------------------------------------------
