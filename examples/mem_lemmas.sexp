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
;; read_swap_i: ∀ m i j. (int_eq i j) = False ⊢
;;   (read (swap m i j) i) = (read m j).
;;
;; Observing a swap at the i end returns what was at j — when i ≠ j.
;; Two-step framing: the OUTER write (at j) is skipped by the
;; disequality (read_write_neq, premise discharged from the goal's
;; i≠j), exposing the INNER write (at i) which read_write_eq collapses.
;; First use of RewriteWith here: cite the CONDITIONAL read_write_neq,
;; discharge its (int_eq i j)=False premise with the goal's own premise,
;; then continue.
;; ---------------------------------------------------------------------------

(claim read_swap_i
  (Goal
    (list (Param 'm (ty Map Int))
          (Param 'i (ty Int))
          (Param 'j (ty Int)))
    (list
      (Equation (Call 'int_eq (list (FVar 'i) (FVar 'j))) (Ctor 'False (list))))
    (Equation
      (Call 'read (list (Call 'swap (list (FVar 'm) (FVar 'i) (FVar 'j)))
                        (FVar 'i)))
      (Call 'read (list (FVar 'm) (FVar 'j)))))
  (Steps
    (list (Unfold 'swap Lhs))   ; (read (write (write m i (read m j)) j (read m i)) i)
    ;; Skip the outer write (at j): read_write_neq, premise (int_eq i j)=False.
    (RewriteWith (Lemma 'read_write_neq) Lr Lhs
      (list)
      (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
      ;; Now (read (write m i (read m j)) i): inner write at i is the hit.
      (Steps (list (Rewrite (Lemma 'read_write_eq) Lr Lhs True (list))) Refl))))

;; ---------------------------------------------------------------------------
;; read_swap_other: ∀ m i j p. (int_eq p j) = False, (int_eq p i) = False ⊢
;;   (read (swap m i j) p) = (read m p).
;;
;; A swap of i and j is invisible to a read at any OTHER position p.
;; Both writes are skipped, each by a read_write_neq whose premise is
;; discharged from the matching goal premise (nested RewriteWith). This
;; is the framing the loop invariant leans on most — the bulk of memory
;; is untouched by each swap.
;; ---------------------------------------------------------------------------

(claim read_swap_other
  (Goal
    (list (Param 'm (ty Map Int))
          (Param 'i (ty Int))
          (Param 'j (ty Int))
          (Param 'p (ty Int)))
    (list
      (Equation (Call 'int_eq (list (FVar 'p) (FVar 'j))) (Ctor 'False (list)))
      (Equation (Call 'int_eq (list (FVar 'p) (FVar 'i))) (Ctor 'False (list))))
    (Equation
      (Call 'read (list (Call 'swap (list (FVar 'm) (FVar 'i) (FVar 'j)))
                        (FVar 'p)))
      (Call 'read (list (FVar 'm) (FVar 'p)))))
  (Steps
    (list (Unfold 'swap Lhs))   ; (read (write (write m i (read m j)) j (read m i)) p)
    ;; Skip outer write (at j): premise (int_eq p j)=False is Premise 0.
    (RewriteWith (Lemma 'read_write_neq) Lr Lhs
      (list)
      (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
      ;; Now (read (write m i (read m j)) p): skip inner write (at i),
      ;; premise (int_eq p i)=False is Premise 1.
      (RewriteWith (Lemma 'read_write_neq) Lr Lhs
        (list)
        (list (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl))
        Refl))))                ; (read m p) = (read m p)

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
;; WHY IT'S NOT A CLAIM YET: the proof is an induction maintaining a
;; loop invariant ("after k swaps, position p holds the cell originally
;; at its mirror image"), discharged with the swap framing lemmas.
;; Progress against the gaps surfaced when standing it up:
;;   1. [DONE, slice 36] read_swap at all three position classes
;;      (read_swap_j / read_swap_i / read_swap_other) — the full per-
;;      position swap framing, via RewriteWith + read_write_neq with
;;      disequality-premise discharge.
;;   2. [BACKEND READY, slice 35] the loop guard / invariant arithmetic
;;      is INEQUALITY reasoning (i < j, p between bounds, i + j = n - 1).
;;      The `ord` backend now decides `lt`/`le` tautologies; conditional
;;      bounds come in as premises. The invariant induction (below) is
;;      where these get consumed — the remaining work.
;;   3. The list↔memory bridge (load/dump) carries a plumbing tax
;;      (length_nat / half_nat / int_of_nat conversions) — TRANSFER's
;;      "representation alignment is death by a thousand lemmas". The
;;      open design question: how much of it the ord backend erases.
;;
;; NEXT: state and prove the loop-invariant lemma (induction on the Nat
;; counter k), the inductive heart that read_swap_* + ord + rev feed
;; into. That is the dragon TRANSFER names; it gets its own slice.
;; ---------------------------------------------------------------------------
