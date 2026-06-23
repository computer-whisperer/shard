#!/usr/bin/env python3
# Throwaway generator for std/word: emits the 8-type family from the verified
# u8 (unsigned) / i8 (signed) templates. Output is the source of truth.

UNSIGNED = [("U8","u8",8),("U16","u16",16),("U32","u32",32)]
SIGNED   = [("I8","i8",8),("I16","i16",16),("I32","i32",32)]

def M(w): return 1 << w
def HALF(w): return 1 << (w-1)

# ----------------------------- interface (mod.req) -----------------------------

def req_unsigned(T,n,w):
    m=M(w); mm1=m-1
    return f"""
;; ---- {n}: unsigned {w}-bit -------------------------------------------------
(sig type {T})
(sig fn {n} ((nn Int)) {T})
(sig fn {n}_val ((x {T})) Int)
(sig fn {n}_add ((a {T}) (b {T})) {T})
(sig fn {n}_sub ((a {T}) (b {T})) {T})
(sig fn {n}_mul ((a {T}) (b {T})) {T})
(sig fn {n}_and ((a {T}) (b {T})) {T})
(sig fn {n}_or ((a {T}) (b {T})) {T})
(sig fn {n}_xor ((a {T}) (b {T})) {T})
(sig fn {n}_not ((x {T})) {T})
(sig fn {n}_eq ((a {T}) (b {T})) Bool)
(sig fn {n}_lt ((a {T}) (b {T})) Bool)
(sig fn {n}_le ((a {T}) (b {T})) Bool)
(sig fn {n}_shl ((x {T}) (k Int)) {T})
(sig fn {n}_shr ((x {T}) (k Int)) {T})

(requirement {n}_val_of (goal ((nn Int)) () (= ({n}_val ({n} nn)) (mod nn {m}))))
(requirement {n}_made_lo (goal ((nn Int)) () (= (le 0 ({n}_val ({n} nn))) True)))
(requirement {n}_made_hi (goal ((nn Int)) () (= (lt ({n}_val ({n} nn)) {m}) True)))
(requirement {n}_add_val (goal ((a {T}) (b {T})) () (= ({n}_val ({n}_add a b)) (mod (+ ({n}_val a) ({n}_val b)) {m}))))
(requirement {n}_sub_val (goal ((a {T}) (b {T})) () (= ({n}_val ({n}_sub a b)) (mod (- ({n}_val a) ({n}_val b)) {m}))))
(requirement {n}_mul_val (goal ((a {T}) (b {T})) () (= ({n}_val ({n}_mul a b)) (mod (* ({n}_val a) ({n}_val b)) {m}))))
(requirement {n}_and_val (goal ((a {T}) (b {T})) () (= ({n}_val ({n}_and a b)) (mod (band ({n}_val a) ({n}_val b)) {m}))))
(requirement {n}_or_val (goal ((a {T}) (b {T})) () (= ({n}_val ({n}_or a b)) (mod (bor ({n}_val a) ({n}_val b)) {m}))))
(requirement {n}_xor_val (goal ((a {T}) (b {T})) () (= ({n}_val ({n}_xor a b)) (mod (bxor ({n}_val a) ({n}_val b)) {m}))))
(requirement {n}_not_val (goal ((x {T})) () (= ({n}_val ({n}_not x)) (mod (- {mm1} ({n}_val x)) {m}))))
(requirement {n}_eq_val (goal ((a {T}) (b {T})) () (= ({n}_eq a b) (int_eq ({n}_val a) ({n}_val b)))))
(requirement {n}_lt_val (goal ((a {T}) (b {T})) () (= ({n}_lt a b) (lt ({n}_val a) ({n}_val b)))))
(requirement {n}_le_val (goal ((a {T}) (b {T})) () (= ({n}_le a b) (le ({n}_val a) ({n}_val b)))))
(requirement {n}_shl_val (goal ((x {T}) (k Int)) () (= ({n}_val ({n}_shl x k)) (mod (bshl ({n}_val x) k) {m}))))
(requirement {n}_shr_val (goal ((x {T}) (k Int)) () (= ({n}_val ({n}_shr x k)) (mod (bshr ({n}_val x) k) {m}))))
"""

def req_signed(T,n,w):
    m=M(w); h=HALF(w)
    return f"""
;; ---- {n}: signed {w}-bit (stores the signed value in [-{h},{h})) ----------
(sig type {T})
(sig fn {n} ((nn Int)) {T})
(sig fn {n}_val ((x {T})) Int)
(sig fn {n}_wrap ((nn Int)) Int)
(sig fn {n}_add ((a {T}) (b {T})) {T})
(sig fn {n}_sub ((a {T}) (b {T})) {T})
(sig fn {n}_mul ((a {T}) (b {T})) {T})
(sig fn {n}_eq ((a {T}) (b {T})) Bool)
(sig fn {n}_slt ((a {T}) (b {T})) Bool)
(sig fn {n}_sle ((a {T}) (b {T})) Bool)
(sig fn {n}_shl ((x {T}) (k Int)) {T})
(sig fn {n}_sshr ((x {T}) (k Int)) {T})

(requirement {n}_val_of (goal ((nn Int)) () (= ({n}_val ({n} nn)) ({n}_wrap nn))))
(requirement {n}_wrap_lo (goal ((nn Int)) () (= (le -{h} ({n}_wrap nn)) True)))
(requirement {n}_wrap_hi (goal ((nn Int)) () (= (lt ({n}_wrap nn) {h}) True)))
(requirement {n}_add_val (goal ((a {T}) (b {T})) () (= ({n}_val ({n}_add a b)) ({n}_wrap (+ ({n}_val a) ({n}_val b))))))
(requirement {n}_sub_val (goal ((a {T}) (b {T})) () (= ({n}_val ({n}_sub a b)) ({n}_wrap (- ({n}_val a) ({n}_val b))))))
(requirement {n}_mul_val (goal ((a {T}) (b {T})) () (= ({n}_val ({n}_mul a b)) ({n}_wrap (* ({n}_val a) ({n}_val b))))))
(requirement {n}_eq_val (goal ((a {T}) (b {T})) () (= ({n}_eq a b) (int_eq ({n}_val a) ({n}_val b)))))
(requirement {n}_slt_val (goal ((a {T}) (b {T})) () (= ({n}_slt a b) (lt ({n}_val a) ({n}_val b)))))
(requirement {n}_sle_val (goal ((a {T}) (b {T})) () (= ({n}_sle a b) (le ({n}_val a) ({n}_val b)))))
(requirement {n}_shl_val (goal ((x {T}) (k Int)) () (= ({n}_val ({n}_shl x k)) ({n}_wrap (bshl ({n}_val x) k)))))
(requirement {n}_sshr_val (goal ((x {T}) (k Int)) () (= ({n}_val ({n}_sshr x k)) ({n}_wrap (ediv ({n}_val x) (bshl 1 k))))))
"""

# ----------------------------- impl (word.shard) ------------------------------

def impl_unsigned(T,n,w):
    m=M(w); mm1=m-1; c=f"mk_{n}"
    return f"""
;; ---- {n}: unsigned {w}-bit -------------------------------------------------
(type {T} ({c} Int))
(fn {n} ((nn Int)) {T} ({c} (mod nn {m})))
(fn {n}_val ((x {T})) Int (match x (({c} r) r)))
(fn {n}_add ((a {T}) (b {T})) {T} ({n} (+ ({n}_val a) ({n}_val b))))
(fn {n}_sub ((a {T}) (b {T})) {T} ({n} (- ({n}_val a) ({n}_val b))))
(fn {n}_mul ((a {T}) (b {T})) {T} ({n} (* ({n}_val a) ({n}_val b))))
(fn {n}_and ((a {T}) (b {T})) {T} ({n} (band ({n}_val a) ({n}_val b))))
(fn {n}_or ((a {T}) (b {T})) {T} ({n} (bor ({n}_val a) ({n}_val b))))
(fn {n}_xor ((a {T}) (b {T})) {T} ({n} (bxor ({n}_val a) ({n}_val b))))
(fn {n}_not ((x {T})) {T} ({n} (- {mm1} ({n}_val x))))
(fn {n}_eq ((a {T}) (b {T})) Bool (int_eq ({n}_val a) ({n}_val b)))
(fn {n}_lt ((a {T}) (b {T})) Bool (lt ({n}_val a) ({n}_val b)))
(fn {n}_le ((a {T}) (b {T})) Bool (le ({n}_val a) ({n}_val b)))
(fn {n}_shl ((x {T}) (k Int)) {T} ({n} (bshl ({n}_val x) k)))
(fn {n}_shr ((x {T}) (k Int)) {T} ({n} (bshr ({n}_val x) k)))

(fulfills {n}_val_of (steps ((unfold {n} lhs) (unfold {n}_val lhs) (simp lhs)) refl))
(fulfills {n}_made_lo
  (steps ((rewrite (lemma {n}_val_of) lr lhs true ()))
    (rewrite-with (lemma mod_lo) lr lhs ((inst n nn) (inst d {m})) ((steps ((compute both)) refl)) refl)))
(fulfills {n}_made_hi
  (steps ((rewrite (lemma {n}_val_of) lr lhs true ()))
    (rewrite-with (lemma mod_hi) lr lhs ((inst n nn) (inst d {m})) ((steps ((compute both)) refl)) refl)))
(fulfills {n}_add_val (steps ((unfold {n}_add lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_sub_val (steps ((unfold {n}_sub lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_mul_val (steps ((unfold {n}_mul lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_and_val (steps ((unfold {n}_and lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_or_val (steps ((unfold {n}_or lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_xor_val (steps ((unfold {n}_xor lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_not_val (steps ((unfold {n}_not lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_eq_val (steps ((unfold {n}_eq lhs)) refl))
(fulfills {n}_lt_val (steps ((unfold {n}_lt lhs)) refl))
(fulfills {n}_le_val (steps ((unfold {n}_le lhs)) refl))
(fulfills {n}_shl_val (steps ((unfold {n}_shl lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_shr_val (steps ((unfold {n}_shr lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
"""

def impl_signed(T,n,w):
    m=M(w); h=HALF(w); c=f"mk_{n}"
    return f"""
;; ---- {n}: signed {w}-bit (stores the signed value in [-{h},{h})) ----------
(type {T} ({c} Int))
(fn {n}_wrap ((nn Int)) Int
  (if (lt (mod nn {m}) {h}) (mod nn {m}) (- (mod nn {m}) {m})))
(fn {n} ((nn Int)) {T} ({c} ({n}_wrap nn)))
(fn {n}_val ((x {T})) Int (match x (({c} v) v)))
(fn {n}_add ((a {T}) (b {T})) {T} ({n} (+ ({n}_val a) ({n}_val b))))
(fn {n}_sub ((a {T}) (b {T})) {T} ({n} (- ({n}_val a) ({n}_val b))))
(fn {n}_mul ((a {T}) (b {T})) {T} ({n} (* ({n}_val a) ({n}_val b))))
(fn {n}_eq ((a {T}) (b {T})) Bool (int_eq ({n}_val a) ({n}_val b)))
(fn {n}_slt ((a {T}) (b {T})) Bool (lt ({n}_val a) ({n}_val b)))
(fn {n}_sle ((a {T}) (b {T})) Bool (le ({n}_val a) ({n}_val b)))
(fn {n}_shl ((x {T}) (k Int)) {T} ({n} (bshl ({n}_val x) k)))
(fn {n}_sshr ((x {T}) (k Int)) {T} ({n} (ediv ({n}_val x) (bshl 1 k))))

(fulfills {n}_val_of (steps ((unfold {n} lhs) (unfold {n}_val lhs) (simp lhs)) refl))
(fulfills {n}_wrap_lo
  (steps ((unfold {n}_wrap lhs))
    (case-on (lt (mod nn {m}) {h}) Bool
      ((case True
         (steps ((rewrite (hyp 0) lr lhs true ()) (simp lhs))
           (have (= (le 0 (mod nn {m})) True)
             (rewrite-with (lemma mod_lo) lr lhs ((inst n nn) (inst d {m})) ((steps ((compute both)) refl)) refl)
             (by farkas (list 1 1)))))
       (case False
         (steps ((rewrite (hyp 0) lr lhs true ()) (simp lhs))
           (have (= (lt (mod nn {m}) {h}) False)
             (steps ((rewrite (hyp 0) lr lhs true ())) refl)
             (by farkas (list 1 1)))))))))
(fulfills {n}_wrap_hi
  (steps ((unfold {n}_wrap lhs))
    (case-on (lt (mod nn {m}) {h}) Bool
      ((case True
         (steps ((rewrite (hyp 0) lr lhs true ()) (simp lhs) (rewrite (hyp 0) lr lhs true ())) refl))
       (case False
         (steps ((rewrite (hyp 0) lr lhs true ()) (simp lhs))
           (have (= (lt (mod nn {m}) {m}) True)
             (rewrite-with (lemma mod_hi) lr lhs ((inst n nn) (inst d {m})) ((steps ((compute both)) refl)) refl)
             (by farkas (list 1 1)))))))))
(fulfills {n}_add_val (steps ((unfold {n}_add lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_sub_val (steps ((unfold {n}_sub lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_mul_val (steps ((unfold {n}_mul lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_eq_val (steps ((unfold {n}_eq lhs)) refl))
(fulfills {n}_slt_val (steps ((unfold {n}_slt lhs)) refl))
(fulfills {n}_sle_val (steps ((unfold {n}_sle lhs)) refl))
(fulfills {n}_shl_val (steps ((unfold {n}_shl lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
(fulfills {n}_sshr_val (steps ((unfold {n}_sshr lhs) (rewrite (lemma {n}_val_of) lr lhs true ())) refl))
"""

REQ_HEADER = """;;; std/word/mod.req.shard — the PUBLIC INTERFACE of the `word` module (its ".h").
;;;
;;; Fixed-width modular integers as OPAQUE std constructions (replacing the
;;; revoked kernel (Word W S) former — trusted-core contraction, issue #15).
;;; Each type's underlying Int stores its own LOGICAL value: uN in [0,2^W),
;;; iN the signed value in [-2^(W-1), 2^(W-1)). Consumers build words only
;;; through the makers and observe through *_val, reasoning via the law family.
;;;
;;; WIDTHS: u8/u16/u32 + i8/i16/i32. u64/i64 are DEFERRED — the compiled
;;; dev engine uses i63 native ints and traps on the 2^64 literal (the same
;;; i63 debt that kept the kernel former's real consumers <= 32-bit). They are
;;; sound under the Rust authority engine; add them when the engine gets BigInt.
;;;
;;; GENERATED from the u8 (unsigned) / i8 (signed) templates — see the slice-5
;;; commit and the (regenerable) /tmp/wordgen.py. Range facts are COMPOSITIONAL
;;; (over constructed values); a structural forall-inhabitant range needs the
;;; refinement type (designed follow-up). Bitwise/div on signed types and the
;;; unwrapped bitwise image laws are deferred (issue #15).

(import "../../kernel/stdlib.shard")  ; Int / Bool / True for the goals
"""

IMPL_HEADER = """;;; std/word/word.shard — the `word` module IMPLEMENTATION (private).
;;;
;;; Full types (private ctors mk_uN/mk_iN), the bodies behind the opaque sig
;;; fns, and the fulfills proofs. Every op projects to Int, applies the Int op,
;;; re-wraps — so the range invariant holds BY CONSTRUCTION and each law reduces
;;; to a fact about euclidean `mod` (std/div). GENERATED — see /tmp/wordgen.py.

(import "../../kernel/stdlib.shard")
(import "../div")  ; mod_lo / mod_hi — euclidean remainder range
(import "mod.req.shard")  ; our interface — the opaque types + sig fns + reqs
(use (:: std div *))  ; the mod range axioms for citation
(use (:: std word *))  ; impl-proof: rebind bodies + bring lemma names into scope
"""

PINS = """
;; ---- ground pins: the ops COMPUTE the wrapping semantics (an op-semantics
;; change can never slip through silently) -----------------------------------
(claim u8_wrap_pin (goal () () (= (u8_val (u8 300)) 44)) (steps ((compute lhs)) refl))
(claim u8_xor_pin (goal () () (= (u8_val (u8_xor (u8 12) (u8 10))) 6)) (steps ((compute lhs)) refl))
(claim u8_shl_pin (goal () () (= (u8_val (u8_shl (u8 1) 9)) 0)) (steps ((compute lhs)) refl))
(claim u32_wrap_pin (goal () () (= (u32_val (u32 4294967296)) 0)) (steps ((compute lhs)) refl))
(claim i8_wrap_pin (goal () () (= (i8_val (i8 200)) -56)) (steps ((compute lhs)) refl))
(claim i8_neg_pin (goal () () (= (i8_val (i8 -1)) -1)) (steps ((compute lhs)) refl))
(claim i8_add_ovf_pin (goal () () (= (i8_val (i8_add (i8 100) (i8 100))) -56)) (steps ((compute lhs)) refl))
(claim i32_neg_pin (goal () () (= (i32_val (i32 -1)) -1)) (steps ((compute lhs)) refl))
"""

req = [REQ_HEADER]
impl = [IMPL_HEADER]
for T,n,w in UNSIGNED:
    req.append(req_unsigned(T,n,w)); impl.append(impl_unsigned(T,n,w))
for T,n,w in SIGNED:
    req.append(req_signed(T,n,w)); impl.append(impl_signed(T,n,w))
impl.append(PINS)

open("std/word/mod.req.shard","w").write("".join(req))
open("std/word/word.shard","w").write("".join(impl))
print("generated std/word/{mod.req,word}.shard")
