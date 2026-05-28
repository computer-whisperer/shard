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
;; Arithmetic helpers for the loop invariant, proven by the farkas
;; entailment backend (each is a linear consequence of its premises).
;; Re-proven here (self-containment) — same shapes as farkas_basics.
;; ---------------------------------------------------------------------------

;; (lt p i)=True ⊢ (lt p (+ i 1))=True.   (feed the IH at the shrunk segment)
(claim lt_succ_from_lt
  (Goal
    (list (Param 'p (ty Int)) (Param 'i (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'p) (FVar 'i))) (Ctor 'True (list))))
    (Equation
      (Call 'lt (list (FVar 'p) (Call '+ (list (FVar 'i) (IntLit 1)))))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

;; (lt a b)=True ⊢ (int_eq a b)=False.   (a strict bound gives a ≠ b)
(claim lt_implies_neq
  (Goal
    (list (Param 'a (ty Int)) (Param 'b (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'a) (FVar 'b))) (Ctor 'True (list))))
    (Equation
      (Call 'int_eq (list (FVar 'a) (FVar 'b)))
      (Ctor 'False (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

;; (lt p i)=True, (lt i j)=True ⊢ (int_eq p j)=False.   (p < i < j ⟹ p ≠ j)
(claim lt_trans_to_neq
  (Goal
    (list (Param 'p (ty Int)) (Param 'i (ty Int)) (Param 'j (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'p) (FVar 'i))) (Ctor 'True (list)))
          (Equation (Call 'lt (list (FVar 'i) (FVar 'j))) (Ctor 'True (list))))
    (Equation
      (Call 'int_eq (list (FVar 'p) (FVar 'j)))
      (Ctor 'False (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1 1))))

;; idx_collapse: (le i p)=True, (le p j)=True, (le j i)=True ⊢
;;   ((i + j) - p) = p.   (a squeezed index collapses — the center case)
;; A PLAIN term-equality proven by two-sided farkas; used to rewrite the
;; mirror read index when i = j = p. Cert: two multiplier lists.
(claim idx_collapse
  (Goal
    (list (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list (Equation (Call 'le (list (FVar 'i) (FVar 'p))) (Ctor 'True (list)))
          (Equation (Call 'le (list (FVar 'p) (FVar 'j))) (Ctor 'True (list)))
          (Equation (Call 'le (list (FVar 'j) (FVar 'i))) (Ctor 'True (list))))
    (Equation
      (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j))) (FVar 'p)))
      (FVar 'p)))
  (ByTheory 'farkas (Cert 'farkas (list (list 1 2 0 1) (list 1 0 2 1)))))

;; ---------------------------------------------------------------------------
;; read_swap_below: ∀ m i j p. (lt p i)=True, (lt i j)=True ⊢
;;   (read (swap m i j) p) = (read m p).
;;
;; The swap-framing lemma in ORDER form (what the loop invariant has):
;; a position below the swap range is untouched. Bridges the order
;; premises (p<i, i<j) to read_swap_other's disequality premises by
;; citing the neq helpers: p≠i from (lt p i), p≠j from (lt p i)+(lt i j).
;; ---------------------------------------------------------------------------

(claim read_swap_below
  (Goal
    (list (Param 'm (ty Map Int))
          (Param 'i (ty Int))
          (Param 'j (ty Int))
          (Param 'p (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'p) (FVar 'i))) (Ctor 'True (list)))
          (Equation (Call 'lt (list (FVar 'i) (FVar 'j))) (Ctor 'True (list))))
    (Equation
      (Call 'read (list (Call 'swap (list (FVar 'm) (FVar 'i) (FVar 'j)))
                        (FVar 'p)))
      (Call 'read (list (FVar 'm) (FVar 'p)))))
  ;; Cite read_swap_other (premises p≠j, p≠i); discharge each from the
  ;; order premises via the neq helpers.
  (RewriteWith (Lemma 'read_swap_other) Lr Lhs (list)
    (list
      ;; premise 0 of read_swap_other: (int_eq p j) = False.
      ;; lt_trans_to_neq's middle var `i` appears only in its premises,
      ;; not its conclusion (int_eq p j) — so pin it with an Inst.
      (RewriteWith (Lemma 'lt_trans_to_neq) Lr Lhs (list (Inst 'i (FVar 'i)))
        (list
          (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)   ; (lt p i)=True
          (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl))  ; (lt i j)=True
        Refl)
      ;; premise 1 of read_swap_other: (int_eq p i) = False
      (RewriteWith (Lemma 'lt_implies_neq) Lr Lhs (list)
        (list
          (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))  ; (lt p i)=True
        Refl))
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
;; rev_loop_untouched_below (THE loop invariant, below-the-range case):
;;   ∀ m i j p k. (lt p i) = True ⊢ (read (rev_loop m i j k) p) = (read m p).
;;
;; A cell below the swap range is unchanged by the whole loop. Induction
;; on the counter k:
;;   Z: the loop is the identity (Simp).
;;   S: unfold one step to (if (lt i j) <recurse> m), CASE-SPLIT on the
;;      guard (lt i j):
;;        False: pointers crossed, loop returns m — done.
;;        True:  i < j is now a hypothesis. Apply the IH at the shrunk
;;               segment (swap m i j, i+1, j-1) — its premise p < i+1 is
;;               lt_succ_from_lt of the goal premise p < i — leaving
;;               (read (swap m i j) p), which read_swap_below collapses
;;               to (read m p) using p < i (premise) and i < j (the
;;               case hypothesis).
;; This is the structural-induction-plus-discharge proof the whole
;; framing + ord + farkas stack was built to enable.
;; ---------------------------------------------------------------------------

(claim rev_loop_untouched_below
  (Goal
    (list (Param 'm (ty Map Int))
          (Param 'i (ty Int))
          (Param 'j (ty Int))
          (Param 'p (ty Int))
          (Param 'k (ty Nat)))
    (list (Equation (Call 'lt (list (FVar 'p) (FVar 'i))) (Ctor 'True (list))))
    (Equation
      (Call 'read (list (Call 'rev_loop (list (FVar 'm) (FVar 'i) (FVar 'j) (FVar 'k)))
                        (FVar 'p)))
      (Call 'read (list (FVar 'm) (FVar 'p)))))
  (Induct 'k
    (list
      (Case 'Z
        (Steps (list (Simp Lhs)) Refl))
      (Case 'S
        ;; Lhs: (read (rev_loop m i j (S k2)) p). Unfold one loop step.
        (Steps (list (Simp Lhs))     ; → (read (if (lt i j) (rev_loop (swap m i j) (i+1) (j-1) k2) m) p)
          (CaseOn (Call 'lt (list (FVar 'i) (FVar 'j))) 'Bool
            (list
              ;; guard False — pointers crossed, loop is the identity.
              (Case 'False
                (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))   ; (lt i j) → False
                             (Simp Lhs))                            ; if False → m → (read m p)
                       Refl))
              ;; guard True — i < j is Hyp 0; IH is Hyp 1.
              (Case 'True
                (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))   ; (lt i j) → True
                             (Simp Lhs))                            ; if True → (read (rev_loop (swap m i j) (i+1) (j-1) k2) p)
                  ;; Apply the IH at the shrunk segment; its premise is p < i+1.
                  (RewriteWith (Hyp 1) Lr Lhs (list)
                    (list
                      (RewriteWith (Lemma 'lt_succ_from_lt) Lr Lhs (list)
                        (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
                        Refl))
                    ;; Now (read (swap m i j) p): peel the swap (p below the range).
                    (RewriteWith (Lemma 'read_swap_below) Lr Lhs (list)
                      (list
                        (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)   ; (lt p i)=True
                        (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))      ; (lt i j)=True
                      Refl)))))))))))

;; ---------------------------------------------------------------------------
;; Above-the-range case. Flipped-orientation arithmetic helpers (the
;; bounds now read (lt j p) — p above j — so the disequalities come out
;; with p on the other side from the below case).
;; ---------------------------------------------------------------------------

;; (lt j p)=True ⊢ (lt (- j 1) p)=True.   (feed the IH at the shrunk j-1)
(claim lt_pred_from_lt
  (Goal
    (list (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'j) (FVar 'p))) (Ctor 'True (list))))
    (Equation
      (Call 'lt (list (Call '- (list (FVar 'j) (IntLit 1))) (FVar 'p)))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

;; (lt a b)=True ⊢ (int_eq b a)=False.   (flipped: ≠ with the larger first)
;; The negated goal is the equality b=a; cancelling it against the
;; premise needs goal multiplier -1 — allowed because the goal is a
;; disequality (equality negation), not an inequality.
(claim lt_implies_neq_flip
  (Goal
    (list (Param 'a (ty Int)) (Param 'b (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'a) (FVar 'b))) (Ctor 'True (list))))
    (Equation
      (Call 'int_eq (list (FVar 'b) (FVar 'a)))
      (Ctor 'False (list))))
  (ByTheory 'farkas (Cert 'farkas (list -1 1))))

;; (lt i j)=True, (lt j p)=True ⊢ (int_eq p i)=False.   (i < j < p ⟹ p ≠ i)
(claim lt_trans_to_neq_flip
  (Goal
    (list (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'i) (FVar 'j))) (Ctor 'True (list)))
          (Equation (Call 'lt (list (FVar 'j) (FVar 'p))) (Ctor 'True (list))))
    (Equation
      (Call 'int_eq (list (FVar 'p) (FVar 'i)))
      (Ctor 'False (list))))
  (ByTheory 'farkas (Cert 'farkas (list -1 1 1))))

;; read_swap_above: ∀ m i j p. (lt i j)=True, (lt j p)=True ⊢
;;   (read (swap m i j) p) = (read m p).   (p above the swap range)
(claim read_swap_above
  (Goal
    (list (Param 'm (ty Map Int))
          (Param 'i (ty Int))
          (Param 'j (ty Int))
          (Param 'p (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'i) (FVar 'j))) (Ctor 'True (list)))
          (Equation (Call 'lt (list (FVar 'j) (FVar 'p))) (Ctor 'True (list))))
    (Equation
      (Call 'read (list (Call 'swap (list (FVar 'm) (FVar 'i) (FVar 'j)))
                        (FVar 'p)))
      (Call 'read (list (FVar 'm) (FVar 'p)))))
  (RewriteWith (Lemma 'read_swap_other) Lr Lhs (list)
    (list
      ;; premise 0: (int_eq p j) = False — from (lt j p) [Premise 1], flipped.
      (RewriteWith (Lemma 'lt_implies_neq_flip) Lr Lhs (list)
        (list (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl))
        Refl)
      ;; premise 1: (int_eq p i) = False — from (lt i j),(lt j p), pin j.
      (RewriteWith (Lemma 'lt_trans_to_neq_flip) Lr Lhs (list (Inst 'j (FVar 'j)))
        (list
          (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)   ; (lt i j)=True
          (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl))  ; (lt j p)=True
        Refl))
    Refl))

;; ---------------------------------------------------------------------------
;; rev_loop_untouched_above: ∀ m i j p k. (lt j p) = True ⊢
;;   (read (rev_loop m i j k) p) = (read m p).
;;
;; Mirror of rev_loop_untouched_below: a cell above the swap range is
;; unchanged. Same induction; the IH precondition at the shrunk segment
;; is p > j-1 (lt_pred_from_lt of p > j), and the swap is peeled by
;; read_swap_above.
;; ---------------------------------------------------------------------------

(claim rev_loop_untouched_above
  (Goal
    (list (Param 'm (ty Map Int))
          (Param 'i (ty Int))
          (Param 'j (ty Int))
          (Param 'p (ty Int))
          (Param 'k (ty Nat)))
    (list (Equation (Call 'lt (list (FVar 'j) (FVar 'p))) (Ctor 'True (list))))
    (Equation
      (Call 'read (list (Call 'rev_loop (list (FVar 'm) (FVar 'i) (FVar 'j) (FVar 'k)))
                        (FVar 'p)))
      (Call 'read (list (FVar 'm) (FVar 'p)))))
  (Induct 'k
    (list
      (Case 'Z
        (Steps (list (Simp Lhs)) Refl))
      (Case 'S
        (Steps (list (Simp Lhs))
          (CaseOn (Call 'lt (list (FVar 'i) (FVar 'j))) 'Bool
            (list
              (Case 'False
                (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))
                             (Simp Lhs))
                       Refl))
              (Case 'True
                (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))
                             (Simp Lhs))
                  ;; IH at the shrunk segment; precondition p > j-1.
                  (RewriteWith (Hyp 1) Lr Lhs (list)
                    (list
                      (RewriteWith (Lemma 'lt_pred_from_lt) Lr Lhs (list)
                        (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
                        Refl))
                    ;; (read (swap m i j) p): peel (p above the range).
                    (RewriteWith (Lemma 'read_swap_above) Lr Lhs (list)
                      (list
                        (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl)        ; (lt i j)=True
                        (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))   ; (lt j p)=True
                      Refl)))))))))))

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
;; PROGRESS (slice 39): the loop-invariant induction WORKS — see
;; rev_loop_untouched_below above (the below-the-range case), proven by
;; induction on k with the guarded loop, the IH at the shrunk segment,
;; and read_swap_below. This validates the whole approach end to end.
;; REMAINING for the capstone:
;;   - rev_loop_untouched_above (symmetric; p > j).
;;   - rev_loop_mirror: a swapped position p ends up holding
;;     (read m (i+j-p)) — the center i+j is recursion-invariant. This is
;;     the substantive case (both swap ends + the middle).
;;   - the list↔memory bridge: dump∘load = id, and dump-of-mirror = rev,
;;     then instantiate at i=0, j=n-1, k=half(n) and discharge the
;;     bound arithmetic via farkas.
;; ---------------------------------------------------------------------------
