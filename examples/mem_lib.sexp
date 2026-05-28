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

;; Nat → Int (structural on the Nat).
(fn int_of_nat ((n Nat)) Int
  (match n
    (Z 0)
    ((S k) (+ 1 (int_of_nat k)))))

;; floor(n/2) as a Nat (structural, two S's at a time).
(fn half_nat ((n Nat)) Nat
  (match n
    (Z Z)
    ((S k)
      (match k
        (Z Z)
        ((S k2) (S (half_nat k2)))))))

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
