;;; std/mem — M3 linear memory = (Map Int): read/write/swap/rev_loop/
;;; load/dump/rdump, plus the full framing -> mirror -> bridge theory
;;; culminating in mem_reverses (in-place reverse = rev, universal n).
;;; The M3 capstone, as a reusable topic.

(import "nat.sexp")

(import "order.sexp")

(import "list.sexp")

(import "map.sexp")

(import "arith.sexp")

;; ======================= object machinery =========================
;;; M3 — linear memory model, as the degenerate seed of finite maps.
;;;
;;; TRANSFER.md names the pilot's single `Mem` array as "the degenerate
;;; seed" of the maps mandate. Here Mem IS a map: `(Map Int)` — Int
;;; addresses (the map keys) to Int cell values. Writes ARE inserts
;;; (reusing map_lib's `insert`); the array-framing lemma
;;; `read(write(m,a,v),b)` cased on `a=b` is exactly slice 33's
;;; insert/lookup story, re-proven at the total-read level below.
;;;
;;; Design note surfaced while standing this up: `read` is TOTAL
;;; (returns a default 0 for unwritten cells) and recurses over the map
;;; DIRECTLY, rather than wrapping map_lib's Option-returning `lookup`.
;;; Reason: a `(match (lookup …) …)` form buries the `int_eq` test
;;; inside a Match SCRUTINEE, and the kernel's rewriter does not descend
;;; into match scrutinees (check.sexp) — so it can't reach the `int_eq`
;;; to discharge reflexivity. A total `read` reduces to a top-level
;;; `if`, which the rewriter handles. (A memory cell read is total
;;; anyway, so this is also the more honest model.) Whether to teach the
;;; rewriter to descend scrutinees is the slice-34 reassess question.
;;;
;;; Modules merged at load time (see mem_lemmas.sexp): nat_lib (Nat),
;;; map_lib (Map type + insert). Cross-file refs resolve in the merged
;;; Module.

;; read: total cell read. Recurses over the map; default 0 if absent.
;; Structurally recursive on the map.
(fn read ((m (Map Int)) (a Int)) Int
  (match m
    (MEmpty 0)
    ((MCons k2 v rest)
      (if (int_eq a k2) v (read rest a)))))

;; write: store v at address a. A write IS an insert (prepend).
(fn write ((m (Map Int)) (a Int) (v Int)) (Map Int)
  (insert a v m))

;; swap: exchange the cells at addresses i and j. Reads both first,
;; then writes them back crossed. (Reads are taken against the ORIGINAL
;; m, so order of the two writes doesn't matter for i ≠ j.)
(fn swap ((m (Map Int)) (i Int) (j Int)) (Map Int)
  (write (write m i (read m j)) j (read m i)))

;; rev_loop: swap ends inward while i < j. The Nat counter k carries
;; STRUCTURAL termination (the address pair i,j are Ints with no
;; structural decrease); the `(lt i j)` GUARD carries CORRECTNESS —
;; real work stops when the pointers cross, independent of k. Running
;; with k = floor(n/2) reverses n cells; the guard makes a too-large k
;; harmless (it just stops early). The guard is also what makes the
;; loop-invariant proof clean: the recursive branch supplies `i < j`
;; as a case hypothesis, so "position p < i is untouched" needs only
;; p < i (no int_of_nat k bound arithmetic).
(fn rev_loop ((m (Map Int)) (i Int) (j Int) (k Nat)) (Map Int)
  (match k
    (Z m)
    ((S k2)
      (if (lt i j)
          (rev_loop (swap m i j) (+ i 1) (- j 1) k2)
          m))))

;; ---------------------------------------------------------------------------
;; Bridge functions — list ↔ linear memory. Used only to STATE the
;; capstone (see mem_lemmas.sexp); their lemmas are next-slice work.
;; ---------------------------------------------------------------------------

;; length as a Nat (structural on the list).
(fn length_nat ((xs (List Int))) Nat
  (match xs
    (Nil Z)
    ((Cons _ t) (S (length_nat t)))))

;; load: write xs into m at base, base+1, … (structural on xs).
(fn load ((xs (List Int)) (base Int) (m (Map Int))) (Map Int)
  (match xs
    (Nil m)
    ((Cons h t) (load t (+ base 1) (write m base h)))))

;; dump: read k cells starting at base, as a list (structural on k).
(fn dump ((base Int) (k Nat) (m (Map Int))) (List Int)
  (match k
    (Z Nil)
    ((S k2) (Cons (read m base) (dump (+ base 1) k2 m)))))

;; rdump: read k cells DOWNWARD from top (top, top-1, …), as a list.
;; The reverse-order companion to dump: rev (dump base k m) = rdump
;; (base+k-1) k m. Used to bridge the mirror's high→low read order to
;; the forward dump in the capstone (structural on k).
(fn rdump ((top Int) (k Nat) (m (Map Int))) (List Int)
  (match k
    (Z Nil)
    ((S k2) (Cons (read m top) (rdump (- top 1) k2 m)))))

;; ======================= theory ===================================

;; int_eq_refl ((int_eq k k)=True) is imported from std/map; lt_succ_from_lt
;; and lt_implies_neq from std/order — cited below, no longer re-proven here.

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

;; precond_shrink: the Nat/Int bridge for the mirror induction. Shaped
;; to discharge the mirror IH's completion-bound premise EXACTLY: when
;; the IH is applied at the shrunk segment (i+1, j-1, k2), its bound
;; premise comes out with the literal segment arithmetic
;; ((j-1)-(i+1))+1, NOT pre-normalized — so the conclusion's LHS must be
;; that same tree (lemma application matches by AST, not by polynomial).
;;   P (Q0): (j-i) <= 2*int_of_nat(S n)          [outer completion bound]
;;   L (Q1): int_of_nat(S n) = 1 + int_of_nat(n) [the definitional link]
;;   ⊢ (j-1)-(i+1) <= 2*int_of_nat(n)            [IH's completion bound]
;; farkas can't unfold int_of_nat in a premise, so the successor link L
;; is supplied as a premise (discharged by Simp at the cite site, where
;; (S n) is a constructor) and consumed here as an equality constraint.
;; cert: G=1 on ¬goal, +1 on P (inequality), -2 on L (equality, any sign).
;; (The polynomial is still j-i-1 <= 2N; only the tree shape changed.)
(claim precond_shrink
  (Goal
    (list (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'n (ty Nat)))
    (list
      (Equation
        (Call 'le (list (Call '- (list (FVar 'j) (FVar 'i)))
                        (Call '* (list (IntLit 2)
                                       (Call 'int_of_nat (list (Ctor 'S (list (FVar 'n)))))))))
        (Ctor 'True (list)))
      (Equation
        (Call 'int_of_nat (list (Ctor 'S (list (FVar 'n)))))
        (Call '+ (list (IntLit 1) (Call 'int_of_nat (list (FVar 'n)))))))
    (Equation
      (Call 'le (list (Call '- (list (Call '- (list (FVar 'j) (IntLit 1)))
                                     (Call '+ (list (FVar 'i) (IntLit 1)))))
                      (Call '* (list (IntLit 2) (Call 'int_of_nat (list (FVar 'n)))))))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1 -2))))

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

;; ===========================================================================
;; rev_loop_mirror — THE substantive M3 case. After running the loop,
;; every position p in [i,j] holds the cell originally at its mirror
;; image i+j-p (the center i+j is recursion-invariant). Proven by
;; induction on the counter k, with a completion bound P2 guaranteeing
;; the loop runs far enough.
;;
;;   ∀ m i j p k.
;;     [ i <= p,  p <= j,  (j-i) <= 2*int_of_nat k ]
;;     ⊢ (read (rev_loop m i j k) p) = (read m (i+j-p))
;;
;; The bound (j-i) <= 2k is exactly "k is at least ceil((j-i)/2) swaps" —
;; tight for BOTH parities of the segment length (at i=0,j=n-1,k=half n
;; it is n-1 <= 2*floor(n/2), true for even and odd n alike).
;;
;; The proof's spine:
;;   Z   : the bound forces (j-i) <= 0 while i<=p<=j forces (j-i)>=0, so
;;         i=j=p and the index collapses (mirror_idx_z, link
;;         int_of_nat Z = 0 discharged by Simp).
;;   S k2: CaseOn the guard (lt i j):
;;     False: pointers crossed ⟹ i=j=p, loop=identity, index collapses
;;            (mirror_idx_cross).
;;     True : i<j. Split p three ways via CaseOn (lt i p), (lt p j):
;;       p=i (lt i p False): the low end. Loop leaves it below the
;;            shrunk range (untouched_below), the first swap put m[j]
;;            there (read_swap_i), and i+j-p = j (idx_lo).
;;       p=j (lt p j False): the high end, mirror of the above
;;            (untouched_above / read_swap_j / idx_hi).
;;       i<p<j: the interior. The IH at (i+1,j-1,k2) carries it (its
;;            bound premise via precond_shrink), the inner index
;;            (i+1)+(j-1)-p normalizes to i+j-p (idx_inner_simp), and
;;            that interior cell is untouched by the outer swap
;;            (read_swap_other, ends discharged by inner_neq_*).
;;
;; The farkas/lia helpers below are each a single linear fact; they were
;; verified in isolation before the induction was assembled.
;; ===========================================================================

;; --- index collapses (plain term-equalities, two-sided farkas) -------------

;; Z base: int_of_nat Z = 0 turns the bound (j-i) <= 2*int_of_nat Z into
;; j <= i; with i<=p<=j that squeezes i=j=p, so the mirror index i+j-p
;; collapses to p. Two-sided; -2 on the (any-sign) link constraint.
(claim mirror_idx_z
  (Goal
    (list (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list
      (Equation (Call 'le (list (FVar 'i) (FVar 'p))) (Ctor 'True (list)))
      (Equation (Call 'le (list (FVar 'p) (FVar 'j))) (Ctor 'True (list)))
      (Equation
        (Call 'le (list (Call '- (list (FVar 'j) (FVar 'i)))
                        (Call '* (list (IntLit 2)
                                       (Call 'int_of_nat (list (Ctor 'Z (list))))))))
        (Ctor 'True (list)))
      (Equation (Call 'int_of_nat (list (Ctor 'Z (list)))) (IntLit 0)))
    (Equation
      (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j))) (FVar 'p)))
      (FVar 'p)))
  (ByTheory 'farkas (Cert 'farkas (list (list 1 2 0 1 -2) (list 1 0 2 1 -2)))))

;; guard-false: i<=p, p<=j, and (lt i j)=False (i>=j) squeeze p=i=j, so
;; the mirror index collapses to p. Same polynomial as idx_collapse, but
;; the third premise is the guard's (lt i j)=False (not (le j i)).
(claim mirror_idx_cross
  (Goal
    (list (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list
      (Equation (Call 'le (list (FVar 'i) (FVar 'p))) (Ctor 'True (list)))
      (Equation (Call 'le (list (FVar 'p) (FVar 'j))) (Ctor 'True (list)))
      (Equation (Call 'lt (list (FVar 'i) (FVar 'j))) (Ctor 'False (list))))
    (Equation
      (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j))) (FVar 'p)))
      (FVar 'p)))
  (ByTheory 'farkas (Cert 'farkas (list (list 1 2 0 1) (list 1 0 2 1)))))

;; low end (p=i, given i<=p and ¬(i<p)): the mirror index i+j-p = j.
(claim idx_lo
  (Goal
    (list (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list
      (Equation (Call 'le (list (FVar 'i) (FVar 'p))) (Ctor 'True (list)))
      (Equation (Call 'lt (list (FVar 'i) (FVar 'p))) (Ctor 'False (list))))
    (Equation
      (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j))) (FVar 'p)))
      (FVar 'j)))
  (ByTheory 'farkas (Cert 'farkas (list (list 1 1 0) (list 1 0 1)))))

;; high end (p=j, given p<=j and ¬(p<j)): the mirror index i+j-p = i.
(claim idx_hi
  (Goal
    (list (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list
      (Equation (Call 'le (list (FVar 'p) (FVar 'j))) (Ctor 'True (list)))
      (Equation (Call 'lt (list (FVar 'p) (FVar 'j))) (Ctor 'False (list))))
    (Equation
      (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j))) (FVar 'p)))
      (FVar 'i)))
  (ByTheory 'farkas (Cert 'farkas (list (list 1 0 1) (list 1 1 0)))))

;; --- variable identifications (p collapses to an endpoint) -----------------

;; p=i from i<=p and ¬(i<p). Cited with both binders Inst-pinned so the
;; pattern is the literal FVar p (rewrites p → i at a chosen site).
(claim eq_lo
  (Goal
    (list (Param 'i (ty Int)) (Param 'p (ty Int)))
    (list
      (Equation (Call 'le (list (FVar 'i) (FVar 'p))) (Ctor 'True (list)))
      (Equation (Call 'lt (list (FVar 'i) (FVar 'p))) (Ctor 'False (list))))
    (Equation (FVar 'p) (FVar 'i)))
  (ByTheory 'farkas (Cert 'farkas (list (list 1 0 1) (list 1 1 0)))))

;; p=j from p<=j and ¬(p<j).
(claim eq_hi
  (Goal
    (list (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list
      (Equation (Call 'le (list (FVar 'p) (FVar 'j))) (Ctor 'True (list)))
      (Equation (Call 'lt (list (FVar 'p) (FVar 'j))) (Ctor 'False (list))))
    (Equation (FVar 'p) (FVar 'j)))
  (ByTheory 'farkas (Cert 'farkas (list (list 1 1 0) (list 1 0 1)))))

;; --- order facts feeding the untouched lemmas / the IH ---------------------

;; p<i+1 from i<=p and ¬(i<p) (p=i ⟹ p below the shrunk range i+1).
(claim lt_succ_lo
  (Goal
    (list (Param 'i (ty Int)) (Param 'p (ty Int)))
    (list
      (Equation (Call 'le (list (FVar 'i) (FVar 'p))) (Ctor 'True (list)))
      (Equation (Call 'lt (list (FVar 'i) (FVar 'p))) (Ctor 'False (list))))
    (Equation
      (Call 'lt (list (FVar 'p) (Call '+ (list (FVar 'i) (IntLit 1)))))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 0 1))))

;; j-1<p from p<=j and ¬(p<j) (p=j ⟹ p above the shrunk range j-1).
(claim lt_pred_hi
  (Goal
    (list (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list
      (Equation (Call 'le (list (FVar 'p) (FVar 'j))) (Ctor 'True (list)))
      (Equation (Call 'lt (list (FVar 'p) (FVar 'j))) (Ctor 'False (list))))
    (Equation
      (Call 'lt (list (Call '- (list (FVar 'j) (IntLit 1))) (FVar 'p)))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 0 1))))

;; --- interior cell is not an endpoint (read_swap_other's premises) ---------

;; i+j-p ≠ j from i<p (i.e. i-p ≠ 0). Goal mult +1 (G any sign for diseq).
(claim inner_neq_hi
  (Goal
    (list (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'i) (FVar 'p))) (Ctor 'True (list))))
    (Equation
      (Call 'int_eq (list (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j))) (FVar 'p)))
                          (FVar 'j)))
      (Ctor 'False (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1))))

;; i+j-p ≠ i from p<j (i.e. j-p ≠ 0). Goal mult -1 (equality-negation).
(claim inner_neq_lo
  (Goal
    (list (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'p (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'p) (FVar 'j))) (Ctor 'True (list))))
    (Equation
      (Call 'int_eq (list (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j))) (FVar 'p)))
                          (FVar 'i)))
      (Ctor 'False (list))))
  (ByTheory 'farkas (Cert 'farkas (list -1 1))))

;; ---------------------------------------------------------------------------
;; The induction. (Hyp indices: each CaseOn prepends its hypothesis at 0,
;; the IH is appended last — so inside the deepest case the order is
;; (lt p j), (lt i p), (lt i j), IH. Premises P0/P1/P2 = i<=p / p<=j /
;; completion-bound throughout.)
;; ---------------------------------------------------------------------------

(claim rev_loop_mirror
  (Goal
    (list (Param 'm (ty Map Int))
          (Param 'i (ty Int))
          (Param 'j (ty Int))
          (Param 'p (ty Int))
          (Param 'k (ty Nat)))
    (list
      (Equation (Call 'le (list (FVar 'i) (FVar 'p))) (Ctor 'True (list)))
      (Equation (Call 'le (list (FVar 'p) (FVar 'j))) (Ctor 'True (list)))
      (Equation
        (Call 'le (list (Call '- (list (FVar 'j) (FVar 'i)))
                        (Call '* (list (IntLit 2) (Call 'int_of_nat (list (FVar 'k)))))))
        (Ctor 'True (list))))
    (Equation
      (Call 'read (list (Call 'rev_loop (list (FVar 'm) (FVar 'i) (FVar 'j) (FVar 'k)))
                        (FVar 'p)))
      (Call 'read (list (FVar 'm)
                        (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j))) (FVar 'p)))))))
  (Induct 'k
    (list
      ;; -------------------------------------------------------------------
      ;; Z: rev_loop = identity; the premises are contradictory, so the
      ;; mirror index collapses to p (mirror_idx_z, link by Simp).
      ;; -------------------------------------------------------------------
      (Case 'Z
        (Steps (list (Simp Lhs))                  ; (read m p) = (read m (i+j-p))
          (RewriteWith (Lemma 'mirror_idx_z) Lr Rhs (list)
            (list
              (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)  ; i<=p
              (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl)  ; p<=j
              (Steps (list (Rewrite (Premise 2) Lr Lhs True (list))) Refl)  ; bound
              (Steps (list (Simp Lhs)) Refl))                               ; int_of_nat Z = 0
            Refl)))
      ;; -------------------------------------------------------------------
      ;; S k2.
      ;; -------------------------------------------------------------------
      (Case 'S
        (Steps (list (Simp Lhs))   ; (read (if (lt i j) (rev_loop (swap m i j)(i+1)(j-1) k2) m) p)
          (CaseOn (Call 'lt (list (FVar 'i) (FVar 'j))) 'Bool
            (list
              ;; guard False: i>=j, loop is identity, index collapses.
              (Case 'False
                (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))   ; (lt i j) → False
                             (Simp Lhs))                            ; → (read m p)
                  (RewriteWith (Lemma 'mirror_idx_cross) Lr Rhs (list)
                    (list
                      (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)  ; i<=p
                      (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl)  ; p<=j
                      (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))     ; (lt i j)=False
                    Refl)))
              ;; guard True: i<j (Hyp 0). Drop into the loop body, then
              ;; split p three ways.
              (Case 'True
                (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))   ; (lt i j) → True
                             (Simp Lhs))                            ; → (read (rev_loop (swap m i j)(i+1)(j-1) k2) p)
                  ;; Now hyps: 0=(lt i j)=True, 1=IH.
                  (CaseOn (Call 'lt (list (FVar 'i) (FVar 'p))) 'Bool
                    (list
                      ;; ---- p = i (low end). hyps: 0=(lt i p)F,1=(lt i j)T,2=IH
                      (Case 'False
                        ;; RHS: i+j-p → j.
                        (RewriteWith (Lemma 'idx_lo) Lr Rhs (list)
                          (list
                            (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)  ; i<=p
                            (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))      ; (lt i p)=False
                          ;; LHS: p is below the shrunk range — untouched.
                          (RewriteWith (Lemma 'rev_loop_untouched_below) Lr Lhs (list)
                            (list
                              (RewriteWith (Lemma 'lt_succ_lo) Lr Lhs (list)
                                (list
                                  (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)
                                  (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))
                                Refl))
                            ;; (read (swap m i j) p): rewrite p → i, then read_swap_i.
                            (RewriteWith (Lemma 'eq_lo) Lr Lhs
                              (list (Inst 'p (FVar 'p)) (Inst 'i (FVar 'i)))
                              (list
                                (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)
                                (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))
                              (RewriteWith (Lemma 'read_swap_i) Lr Lhs (list)
                                (list
                                  (RewriteWith (Lemma 'lt_implies_neq) Lr Lhs (list)
                                    (list
                                      (Steps (list (Rewrite (Hyp 1) Lr Lhs True (list))) Refl))  ; (lt i j)=True
                                    Refl))
                                Refl)))))
                      ;; ---- i < p. split on (lt p j).
                      (Case 'True
                        (CaseOn (Call 'lt (list (FVar 'p) (FVar 'j))) 'Bool
                          (list
                            ;; ---- p = j (high end). hyps: 0=(lt p j)F,1=(lt i p)T,2=(lt i j)T,3=IH
                            (Case 'False
                              ;; RHS: i+j-p → i.
                              (RewriteWith (Lemma 'idx_hi) Lr Rhs (list)
                                (list
                                  (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl)  ; p<=j
                                  (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))      ; (lt p j)=False
                                ;; LHS: p above the shrunk range — untouched.
                                (RewriteWith (Lemma 'rev_loop_untouched_above) Lr Lhs (list)
                                  (list
                                    (RewriteWith (Lemma 'lt_pred_hi) Lr Lhs (list)
                                      (list
                                        (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl)
                                        (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))
                                      Refl))
                                  ;; (read (swap m i j) p): rewrite p → j, then read_swap_j.
                                  (RewriteWith (Lemma 'eq_hi) Lr Lhs
                                    (list (Inst 'p (FVar 'p)) (Inst 'j (FVar 'j)))
                                    (list
                                      (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl)
                                      (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))
                                    (RewriteWith (Lemma 'read_swap_j) Lr Lhs (list)
                                      (list)
                                      Refl)))))
                            ;; ---- i < p < j (interior). hyps: 0=(lt p j)T,1=(lt i p)T,2=(lt i j)T,3=IH
                            (Case 'True
                              ;; Apply the IH at (swap m i j, i+1, j-1, k2).
                              (RewriteWith (Hyp 3) Lr Lhs (list)
                                (list
                                  ;; IH prem0: i+1 <= p   (from i<p)
                                  (RewriteWith (Lemma 'le_succ_from_lt) Lr Lhs (list)
                                    (list
                                      (Steps (list (Rewrite (Hyp 1) Lr Lhs True (list))) Refl))
                                    Refl)
                                  ;; IH prem1: p <= j-1   (from p<j)
                                  (RewriteWith (Lemma 'le_pred_from_lt) Lr Lhs (list)
                                    (list
                                      (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))
                                    Refl)
                                  ;; IH prem2: completion bound at k2 (precond_shrink).
                                  (RewriteWith (Lemma 'precond_shrink) Lr Lhs (list)
                                    (list
                                      (Steps (list (Rewrite (Premise 2) Lr Lhs True (list))) Refl)  ; outer bound
                                      (Steps (list (Simp Lhs)) Refl))                               ; int_of_nat(S k2) link
                                    Refl))
                                ;; LHS now: (read (swap m i j) ((i+1)+(j-1)-p)). Normalize index.
                                (RewriteWith (Lemma 'idx_inner_simp) Lr Lhs (list)
                                  (list)
                                  ;; (read (swap m i j) (i+j-p)): interior cell, untouched by the swap.
                                  (RewriteWith (Lemma 'read_swap_other) Lr Lhs (list)
                                    (list
                                      (RewriteWith (Lemma 'inner_neq_hi) Lr Lhs (list)
                                        (list
                                          (Steps (list (Rewrite (Hyp 1) Lr Lhs True (list))) Refl))  ; (lt i p)=True
                                        Refl)
                                      (RewriteWith (Lemma 'inner_neq_lo) Lr Lhs (list)
                                        (list
                                          (Steps (list (Rewrite (Hyp 0) Lr Lhs True (list))) Refl))  ; (lt p j)=True
                                        Refl))
                                    Refl))))))))))))))))))

;; read_load_below: a cell strictly below the load's base is untouched by
;; the load (load only writes base, base+1, …). Induction on xs: the Cons
;; case writes `base`, recurses at base+1 (IH, premise q<base+1 from q<base
;; via lt_succ_from_lt), then steps back over the base write (read_write_neq,
;; q≠base from lt_implies_neq).
(claim read_load_below
  (Goal
    (list (Param 'xs (ty List Int))
          (Param 'base (ty Int))
          (Param 'm (ty Map Int))
          (Param 'q (ty Int)))
    (list (Equation (Call 'lt (list (FVar 'q) (FVar 'base))) (Ctor 'True (list))))
    (Equation
      (Call 'read (list (Call 'load (list (FVar 'xs) (FVar 'base) (FVar 'm))) (FVar 'q)))
      (Call 'read (list (FVar 'm) (FVar 'q)))))
  (Induct 'xs
    (list
      (Case 'Nil (Steps (list (Simp Lhs)) Refl))
      (Case 'Cons
        (Steps (list (Simp Lhs))   ; read (load t (base+1) (write m base h)) q
          ;; IH at (base+1, write m base h); premise q < base+1.
          (RewriteWith (Hyp 0) Lr Lhs (list)
            (list
              (RewriteWith (Lemma 'lt_succ_from_lt) Lr Lhs (list)
                (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
                Refl))
            ;; (read (write m base h) q): q ≠ base, step over the write.
            (RewriteWith (Lemma 'read_write_neq) Lr Lhs (list)
              (list
                (RewriteWith (Lemma 'lt_implies_neq) Lr Lhs (list)
                  (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl))
                  Refl))
              Refl)))))))

;; dump_load_id: the round trip. dump base (length xs) (load xs base m) =
;; xs — reading back the n cells you wrote gives the list. Induction on
;; xs: the Cons case Simp-fires length/load/dump one step, the IH handles
;; the tail dump at base+1, and the head cell is recovered by stepping the
;; load past base (read_load_below, base<base+1) then read_write_eq.
(claim dump_load_id
  (Goal
    (list (Param 'xs (ty List Int))
          (Param 'base (ty Int))
          (Param 'm (ty Map Int)))
    (list)
    (Equation
      (Call 'dump (list (FVar 'base)
                        (Call 'length_nat (list (FVar 'xs)))
                        (Call 'load (list (FVar 'xs) (FVar 'base) (FVar 'm)))))
      (FVar 'xs)))
  (Induct 'xs
    (list
      (Case 'Nil (Steps (list (Simp Lhs)) Refl))
      (Case 'Cons
        (Steps (list (Simp Lhs))   ; Cons (read M base) (dump (base+1) (length t) M),  M = load t (base+1)(write m base h)
          ;; tail: IH at (base+1, write m base h).
          (RewriteWith (Hyp 0) Lr Lhs (list)
            (list)
            ;; head: read M base = read (write m base h) base (load base+1 > base) = h.
            (RewriteWith (Lemma 'read_load_below) Lr Lhs (list)
              (list
                (RewriteWith (Lemma 'lt_self_succ) Lr Lhs (list) (list) Refl))
              (RewriteWith (Lemma 'read_write_eq) Lr Lhs (list)
                (list)
                Refl))))))))

;; rdump_snoc: peel the LAST cell of an rdump. rdump top (S c) A =
;; (rdump top c A) ++ [read A (top - c)]. Induction on c; the Z case
;; needs (top-0)=top, the S case reconciles (top-1)-c' with top-(1+c').
(claim rdump_snoc
  (Goal
    (list (Param 'top (ty Int)) (Param 'c (ty Nat)) (Param 'A (ty Map Int)))
    (list)
    (Equation
      (Call 'rdump (list (FVar 'top) (Ctor 'S (list (FVar 'c))) (FVar 'A)))
      (Call 'append
        (list (Call 'rdump (list (FVar 'top) (FVar 'c) (FVar 'A)))
              (Ctor 'Cons
                (list (Call 'read (list (FVar 'A)
                                        (Call '- (list (FVar 'top)
                                                       (Call 'int_of_nat (list (FVar 'c)))))))
                      (Ctor 'Nil (list))))))))
  (Induct 'c
    (list
      (Case 'Z
        (Steps (list (Simp Both))
          (RewriteWith (Lemma 'sub_zero) Lr Rhs (list) (list) Refl)))
      (Case 'S
        ;; Simp fully unfolds both sides (S c' is a concrete ctor). We
        ;; reconcile the RHS back to the LHS's normal form: fix the snoc
        ;; index shape (sub_sub_one Rl), fold append→rdump via the IH
        ;; (Rl), then re-Simp the RHS to the same NF as the LHS.
        (Steps (list (Simp Both)
                     (Rewrite (Lemma 'sub_sub_one) Rl Rhs True (list))
                     (Rewrite (Hyp 0) Rl Rhs False (list))
                     (Simp Rhs))
          Refl)))))

;; rev_dump_rdump: the front/back flip. rev (dump base cnt A) =
;; rdump (base+cnt-1) cnt A — reading forward then reversing equals
;; reading backward from the top. Induction on cnt: the S case Simps the
;; LHS (rev unfolds to append … [read A base]), applies the IH to the
;; inner rev, then on the RHS unfolds int_of_nat(S c), reassociates the
;; top index (reassoc_succ), snocs via rdump_snoc, and cancels the tail
;; index (idx_cancel) so both sides are append (rdump … c A) [read A base].
(claim rev_dump_rdump
  (Goal
    (list (Param 'base (ty Int)) (Param 'cnt (ty Nat)) (Param 'A (ty Map Int)))
    (list)
    (Equation
      (Call 'rev (list (Call 'dump (list (FVar 'base) (FVar 'cnt) (FVar 'A)))))
      (Call 'rdump (list (Call '- (list (Call '+ (list (FVar 'base)
                                                       (Call 'int_of_nat (list (FVar 'cnt)))))
                                        (IntLit 1)))
                         (FVar 'cnt) (FVar 'A)))))
  (Induct 'cnt
    (list
      (Case 'Z (Steps (list (Simp Both)) Refl))
      (Case 'S
        (Steps (list (Simp Lhs)                                   ; append (rev (dump (base+1) c A)) [read A base]
                     (Rewrite (Hyp 0) Lr Lhs False (list))        ; inner rev → rdump IDX1 c A
                     (Unfold 'int_of_nat Rhs)                     ; expose int_of_nat(S c)
                     (Reduce Rhs)                                 ; → (+ 1 (int_of_nat c))
                     (Rewrite (Lemma 'reassoc_succ) Lr Rhs True (list))   ; top idx → IDX1
                     (Rewrite (Lemma 'rdump_snoc) Lr Rhs False (list))    ; rdump IDX1 (S c) A → append … [read A (IDX1 - c)]
                     (Rewrite (Lemma 'idx_cancel) Lr Rhs True (list)))    ; (IDX1 - c) → base
          Refl)))))

;; base <= j, from base+(S c) <= j+1 (the load/dump range bound), the
;; successor link, and int_of_nat c >= 0. c is a pivot (premises only),
;; so it is Inst-pinned at the cite site.
(claim base_le_j
  (Goal
    (list (Param 'base (ty Int)) (Param 'j (ty Int)) (Param 'c (ty Nat)))
    (list
      (Equation
        (Call 'le (list (Call '+ (list (FVar 'base)
                                       (Call 'int_of_nat (list (Ctor 'S (list (FVar 'c)))))))
                        (Call '+ (list (FVar 'j) (IntLit 1)))))
        (Ctor 'True (list)))
      (Equation (Call 'int_of_nat (list (Ctor 'S (list (FVar 'c)))))
                (Call '+ (list (IntLit 1) (Call 'int_of_nat (list (FVar 'c))))))
      (Equation (Call 'le (list (IntLit 0) (Call 'int_of_nat (list (FVar 'c)))))
                (Ctor 'True (list))))
    (Equation (Call 'le (list (FVar 'base) (FVar 'j))) (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1 1 1))))

;; the dump-range bound steps to the IH: base+(S c) <= j+1 (+ link)
;; ⊢ (base+1)+c <= j+1.  Same polynomial, shifted shape (cf. precond_shrink).
(claim bound_step
  (Goal
    (list (Param 'base (ty Int)) (Param 'j (ty Int)) (Param 'c (ty Nat)))
    (list
      (Equation
        (Call 'le (list (Call '+ (list (FVar 'base)
                                       (Call 'int_of_nat (list (Ctor 'S (list (FVar 'c)))))))
                        (Call '+ (list (FVar 'j) (IntLit 1)))))
        (Ctor 'True (list)))
      (Equation (Call 'int_of_nat (list (Ctor 'S (list (FVar 'c)))))
                (Call '+ (list (IntLit 1) (Call 'int_of_nat (list (FVar 'c)))))))
    (Equation
      (Call 'le (list (Call '+ (list (Call '+ (list (FVar 'base) (IntLit 1)))
                                     (Call 'int_of_nat (list (FVar 'c)))))
                      (Call '+ (list (FVar 'j) (IntLit 1)))))
      (Ctor 'True (list))))
  (ByTheory 'farkas (Cert 'farkas (list 1 1 1))))

;; dump_R_rdump_step: ONE peel of the mirror traversal, with the counter
;; `c` a NAMED param (so base_le_j's pivot c binds via Inst — after an
;; Induct the field is an anonymous fresh symbol we can't name, hence the
;; factoring). The tail equation is taken as a premise (Premise 3); the
;; outer induction supplies it from its IH.
;;   head: read (rev_loop M i j k) base → read M (i+j-base) (rev_loop_mirror,
;;         base<=j via base_le_j from the range bound + link + nonneg);
;;   tail: the premise rewrites dump (base+1) c R → rdump (i+j-(base+1)) c M;
;;   then idx_pred reconciles (i+j-(base+1)) with ((i+j-base)-1).
(claim dump_R_rdump_step
  (Goal
    (list (Param 'M (ty Map Int))
          (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'k (ty Nat))
          (Param 'base (ty Int)) (Param 'c (ty Nat)))
    (list
      (Equation (Call 'le (list (FVar 'i) (FVar 'base))) (Ctor 'True (list)))
      (Equation
        (Call 'le (list (Call '+ (list (FVar 'base)
                                       (Call 'int_of_nat (list (Ctor 'S (list (FVar 'c)))))))
                        (Call '+ (list (FVar 'j) (IntLit 1)))))
        (Ctor 'True (list)))
      (Equation
        (Call 'le (list (Call '- (list (FVar 'j) (FVar 'i)))
                        (Call '* (list (IntLit 2) (Call 'int_of_nat (list (FVar 'k)))))))
        (Ctor 'True (list)))
      (Equation
        (Call 'dump (list (Call '+ (list (FVar 'base) (IntLit 1))) (FVar 'c)
                          (Call 'rev_loop (list (FVar 'M) (FVar 'i) (FVar 'j) (FVar 'k)))))
        (Call 'rdump (list (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j)))
                                          (Call '+ (list (FVar 'base) (IntLit 1)))))
                           (FVar 'c) (FVar 'M)))))
    (Equation
      (Call 'dump (list (FVar 'base) (Ctor 'S (list (FVar 'c)))
                        (Call 'rev_loop (list (FVar 'M) (FVar 'i) (FVar 'j) (FVar 'k)))))
      (Call 'rdump (list (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j))) (FVar 'base)))
                         (Ctor 'S (list (FVar 'c))) (FVar 'M)))))
  (Steps (list (Simp Both))
    ;; head: read (rev_loop M i j k) base → read M (i+j-base).
    (RewriteWith (Lemma 'rev_loop_mirror) Lr Lhs (list)
      (list
        (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)   ; i<=base
        (RewriteWith (Lemma 'base_le_j) Lr Lhs (list (Inst 'c (FVar 'c)))
          (list
            (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl)  ; range bound
            (Steps (list (Simp Lhs)) Refl)                                ; link
            (RewriteWith (Lemma 'int_of_nat_nonneg) Lr Lhs (list) (list) Refl))  ; 0<=int_of_nat c
          Refl)
        (Steps (list (Rewrite (Premise 2) Lr Lhs True (list))) Refl))   ; loop bound
      ;; tail: dump (base+1) c R → rdump (i+j-(base+1)) c M  (Premise 3).
      (RewriteWith (Premise 3) Lr Lhs (list)
        (list)
        ;; reconcile rdump index (i+j-(base+1)) → ((i+j-base)-1).
        (RewriteWith (Lemma 'idx_pred) Lr Lhs (list) (list) Refl)))))

;; dump_R_rdump: dumping cnt cells of the reversed memory (rev_loop M i j k)
;; forward from base = reading the ORIGINAL memory M backward from i+j-base.
;; Induction on cnt; the S case delegates the peel to dump_R_rdump_step,
;; supplying its tail premise from the IH (lower bound via le_succ_r, range
;; bound via bound_step, loop bound carried).
(claim dump_R_rdump
  (Goal
    (list (Param 'M (ty Map Int))
          (Param 'i (ty Int)) (Param 'j (ty Int)) (Param 'k (ty Nat))
          (Param 'base (ty Int)) (Param 'cnt (ty Nat)))
    (list
      (Equation (Call 'le (list (FVar 'i) (FVar 'base))) (Ctor 'True (list)))
      (Equation
        (Call 'le (list (Call '+ (list (FVar 'base) (Call 'int_of_nat (list (FVar 'cnt)))))
                        (Call '+ (list (FVar 'j) (IntLit 1)))))
        (Ctor 'True (list)))
      (Equation
        (Call 'le (list (Call '- (list (FVar 'j) (FVar 'i)))
                        (Call '* (list (IntLit 2) (Call 'int_of_nat (list (FVar 'k)))))))
        (Ctor 'True (list))))
    (Equation
      (Call 'dump (list (FVar 'base) (FVar 'cnt)
                        (Call 'rev_loop (list (FVar 'M) (FVar 'i) (FVar 'j) (FVar 'k)))))
      (Call 'rdump (list (Call '- (list (Call '+ (list (FVar 'i) (FVar 'j))) (FVar 'base)))
                         (FVar 'cnt) (FVar 'M)))))
  (Induct 'cnt
    (list
      (Case 'Z (Steps (list (Simp Both)) Refl))
      (Case 'S
        (RewriteWith (Lemma 'dump_R_rdump_step) Lr Lhs (list)
          (list
            (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)   ; i<=base
            (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl)   ; range bound at S c
            (Steps (list (Rewrite (Premise 2) Lr Lhs True (list))) Refl)   ; loop bound
            ;; tail premise: dump (base+1) c R = rdump (i+j-(base+1)) c M, via the IH.
            (RewriteWith (Hyp 0) Lr Lhs (list)
              (list
                (RewriteWith (Lemma 'le_succ_r) Lr Lhs (list)
                  (list (Steps (list (Rewrite (Premise 0) Lr Lhs True (list))) Refl)) Refl)
                (RewriteWith (Lemma 'bound_step) Lr Lhs (list)
                  (list
                    (Steps (list (Rewrite (Premise 1) Lr Lhs True (list))) Refl)
                    (Steps (list (Simp Lhs)) Refl)) Refl)
                (Steps (list (Rewrite (Premise 2) Lr Lhs True (list))) Refl))
              Refl))
          Refl)))))

;; ===========================================================================
;; CAPSTONE — mem_reverses (PROVEN). The M3 data-refinement dragon:
;;
;;   ∀ xs : (List Int).
;;     (dump 0 (length_nat xs)
;;           (rev_loop (load xs 0 MEmpty)
;;                     0
;;                     (- (int_of_nat (length_nat xs)) 1)
;;                     (half_nat (length_nat xs))))
;;     = (rev xs)
;;
;; "Load xs into fresh memory at 0…n-1, run the in-place two-pointer swap
;; loop floor(n/2) times, dump the n cells back out — you get rev xs",
;; for UNIVERSAL n. list ↔ linear memory, the imperative array-reverse
;; refined against the functional spec, all decided not assumed.
;;
;; The proof is the four-lemma chain, no new reasoning — just composition:
;;   dump 0 n R
;;     = rdump (0+(n-1)-0) n M0           [dump_R_rdump: the mirror, with
;;                                          premises 0<=0, range bound (ord),
;;                                          loop bound (sub_zero + half_bound)]
;;     = rdump ((0+n)-1) n M0             [cap_idx: lia index reconcile]
;;     = rev (dump 0 n M0)               [rev_dump_rdump (flip), reversed]
;;     = rev xs                          [dump_load_id: the load round trip]
;; where n = length_nat xs, M0 = load xs 0 MEmpty, R = rev_loop M0 0 (n-1)
;; (half n). Cites list_lib's `rev`.
;; ===========================================================================

(claim mem_reverses
  (Goal
    (list (Param 'xs (ty List Int)))
    (list)
    (Equation
      (Call 'dump
        (list (IntLit 0)
              (Call 'length_nat (list (FVar 'xs)))
              (Call 'rev_loop
                (list (Call 'load (list (FVar 'xs) (IntLit 0) (Ctor 'MEmpty (list))))
                      (IntLit 0)
                      (Call '- (list (Call 'int_of_nat
                                           (list (Call 'length_nat (list (FVar 'xs)))))
                                     (IntLit 1)))
                      (Call 'half_nat (list (Call 'length_nat (list (FVar 'xs)))))))))
      (Call 'rev (list (FVar 'xs)))))
  ;; dump 0 n R → rdump (0+(n-1)-0) n M0.
  (RewriteWith (Lemma 'dump_R_rdump) Lr Lhs (list)
    (list
      (Steps (list (Simp Lhs)) Refl)                                 ; 0 <= 0
      (ByTheory 'ord (Cert 'ord (list)))                             ; 0+n <= (n-1)+1
      (Steps (list (Rewrite (Lemma 'sub_zero) Lr Lhs True (list)))   ; (n-1)-0 → n-1
        (RewriteWith (Lemma 'half_bound) Lr Lhs (list) (list) Refl)));  n-1 <= 2*half n
    ;; rdump (0+(n-1)-0) … → reconcile index → flip back to rev(dump) → round trip.
    (Steps (list (Rewrite (Lemma 'cap_idx) Lr Lhs True (list)))      ; (0+(n-1))-0 → (0+n)-1
      (RewriteWith (Lemma 'rev_dump_rdump) Rl Lhs (list) (list)      ; rdump ((0+n)-1)… → rev(dump 0 n M0)
        (RewriteWith (Lemma 'dump_load_id) Lr Lhs (list) (list)      ; dump 0 n (load xs 0 ∅) → xs
          Refl)))))
