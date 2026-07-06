/* rt.h — the C runtime under codegen'd RS-shard (see tools/lower/DESIGN.md).
 *
 * Generated out.c #defines the core ctor tags (RT_TAG_True, RT_TAG_False,
 * RT_TAG_Nil, RT_TAG_Cons, RT_TAG_None, RT_TAG_Some, RT_TAG_Pair,
 * RT_TAG_World) BEFORE including this header.
 *
 * Representation: value_t is one machine word.
 *   odd  = small int, (i << 1) | 1, i in i63.
 *   even = pointer to cell { tag, arity, fields... }.
 * Ints outside i63 are BIGNUM cells (tag RT_TAG_BIG_, see the bignum
 * section) — arithmetic is exact like the interpreted engines; the old
 * "overflow traps" posture survives only as the explicit guards (shift
 * counts, RT_BIG_MAXLIMB), still loud never wrong.
 * Symbols are cells with tag RT_TAG_SYM_ (runtime-reserved, not a ctor
 * tag) whose single field is the intern id (small int). Equality = id.
 *
 * Allocator: bump pointer over a big MAP_NORESERVE region, backed by a
 * CONSERVATIVE mark-sweep collector (rt_gc). GC runs only inside rt_alloc,
 * which is a function call, so the C ABI guarantees every value_t live
 * across it is on the C stack or in a callee-saved register — setjmp +
 * a word scan of [sp..stack_base] captures all roots. Marking is sound
 * against false roots because a per-16B-slot START BITMAP (set on every
 * fresh bump allocation) validates that a candidate points at a real cell
 * head before any deref or mark; the mark itself is the top bit of the
 * 32-bit tag, cleared by the sweep so the hot rt_tag path is untouched
 * between collections. Reclaimed cells go to size-segregated free lists
 * (non-moving — conservative roots can't be relocated). This is a dev-loop
 * engine only; the Rust interpreter is the soundness authority and the
 * byte-identical corpus cross-check catches any divergence.
 */
#ifndef RT_H
#define RT_H
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <dirent.h>
#include <sys/stat.h>
#include <sys/mman.h>
#include <sys/resource.h>

typedef intptr_t value_t;

#define RT_TAG_SYM_ 0x7fffffffu
#define RT_MARK_ 0x80000000u  /* GC mark = top bit of tag (free; tags < SYM_) */

static void rt_trap(const char *msg) {
  fprintf(stderr, "rt trap: %s\n", msg);
  exit(3);
}

/* ---- ints -------------------------------------------------------------- */
#define RT_INT_MAX ((int64_t)0x3fffffffffffffffLL)
#define RT_INT_MIN ((int64_t)-0x4000000000000000LL)

static inline int rt_is_int(value_t v) { return v & 1; }
static inline int64_t rt_untag(value_t v) { return (int64_t)v >> 1; }
/* i outside i63 promotes to a bignum cell (see the bignum section below) —
   the arithmetic prims are exact like the interpreted engines. */
static value_t rt_big_from_i64(int64_t i);
static inline value_t rt_mk_small(int64_t i) {  /* caller guarantees i63 */
  return (value_t)((uint64_t)i << 1 | 1);
}
static inline value_t rt_tag_int(int64_t i) {
  if (i > RT_INT_MAX || i < RT_INT_MIN) return rt_big_from_i64(i);
  return (value_t)((uint64_t)i << 1 | 1);
}

/* ---- heap -------------------------------------------------------------- */
typedef struct { uint32_t tag; uint32_t arity; value_t fields[]; } rt_cell;

static char *rt_heap, *rt_heap_base, *rt_heap_end;
static inline rt_cell *rt_cellof(value_t v) { return (rt_cell *)v; }
static inline size_t rt_cellsz(uint32_t arity) {
  return (sizeof(rt_cell) + arity * sizeof(value_t) + 15) & ~15ul;
}
static inline uint32_t rt_tag(value_t v) {
  return rt_is_int(v) ? 0xfffffffeu : rt_cellof(v)->tag;
}
static inline value_t rt_field(value_t v, uint32_t i) {
  return rt_cellof(v)->fields[i];
}

/* ---- conservative mark-sweep GC ---------------------------------------- */
/* All cells are 16-aligned, so size classes index by (size >> 4). Arity is
   small; 64 classes covers cell sizes up to 1 KiB. */
#define RT_NCLASS 64
static rt_cell *rt_free[RT_NCLASS];   /* size-segregated free lists */
static uint8_t *rt_startmap;          /* 1 bit per 16B slot: is a cell head */
static char *rt_stack_base;           /* high end of the C stack (set in init) */
static int rt_gc_on = 0;              /* enabled at the end of rt_init */
static char *rt_gc_trigger;           /* next GC when bump passes this */
/* ADAPTIVE spacing: collect after max(live, MIN) fresh bump bytes, capped at
   MAX. Spacing proportional to the live set keeps GC cost amortized-constant
   per allocated byte AND holds the heap extent (and so RSS + sweep length +
   fresh-page faults) at a small multiple of live. The old FIXED 2 GiB
   spacing grew the extent by 2 GiB per cycle regardless of a tiny live set —
   measured ~30% of wall time in fresh-page faults (1.07M faults/6s) and
   32 GB RSS on a nested-eval run (tools/bench). MAX keeps the old
   parallel-jobs bound: peak resident ~= live + MAX per process (JOBS-parallel
   gate_sweep/run_corpus without OOM). */
#define RT_GC_MIN_SPACING (64ul << 20)
#define RT_GC_MAX_SPACING (2ul << 30)
static size_t rt_live_bytes = 0;      /* live bytes found by the last sweep */

static inline size_t rt_slot(char *p) { return (size_t)(p - rt_heap_base) >> 4; }
static inline void rt_setstart(char *p) {
  size_t s = rt_slot(p); rt_startmap[s >> 3] |= (uint8_t)(1u << (s & 7));
}
static inline int rt_isstart(char *p) {
  size_t s = rt_slot(p); return (rt_startmap[s >> 3] >> (s & 7)) & 1;
}

/* mark stack: cells marked but not yet scanned. Grows by doubling. */
static rt_cell **rt_mstk; static size_t rt_mtop, rt_mcap;
static void rt_mgrow(void) {
  rt_mcap = rt_mcap ? rt_mcap * 2 : (1ul << 20);
  rt_mstk = (rt_cell **)realloc(rt_mstk, rt_mcap * sizeof(rt_cell *));
  if (!rt_mstk) rt_trap("gc: mark stack realloc failed");
}
/* if v is an unmarked valid cell head, mark it and return it, else 0. */
static inline rt_cell *rt_try_mark(value_t v) {
  if (v & 1) return 0;                 /* small int */
  char *p = (char *)v;
  if (p < rt_heap_base || p >= rt_heap) return 0;   /* not in live heap */
  if (((uintptr_t)p) & 15) return 0;   /* cells are 16-aligned */
  if (!rt_isstart(p)) return 0;        /* not a real cell head (false root) */
  rt_cell *c = (rt_cell *)p;
  if (c->tag & RT_MARK_) return 0;     /* already marked */
  c->tag |= RT_MARK_;
  return c;
}
static inline void rt_mpush(value_t v) {
  rt_cell *c = rt_try_mark(v);
  if (c) { if (rt_mtop == rt_mcap) rt_mgrow(); rt_mstk[rt_mtop++] = c; }
}
/* scan a marked cell, tail-iterating the LAST field so a long Cons spine
   costs O(1) mark-stack depth rather than O(length). */
static void rt_scan(rt_cell *c) {
  for (;;) {
    uint32_t n = c->arity;
    if (n == 0) return;
    for (uint32_t i = 0; i + 1 < n; i++) rt_mpush(c->fields[i]);
    rt_cell *nc = rt_try_mark(c->fields[n - 1]);
    if (!nc) return;
    c = nc;
  }
}
static void rt_scan_range(char *lo, char *hi) {
  lo = (char *)(((uintptr_t)lo + 7) & ~7ul);   /* word-align */
  for (char *p = lo; p + sizeof(value_t) <= hi; p += sizeof(value_t))
    rt_mpush(*(value_t *)p);
}
/* preallocated nullary cells, filled in rt_init from generated tag macros */
static value_t rt_true_, rt_false_, rt_nil_, rt_none_, rt_unit_world;
static void rt_gc(void) {
  for (int i = 0; i < RT_NCLASS; i++) rt_free[i] = 0;
  rt_mtop = 0;
  /* roots: the preallocated singletons, callee-saved registers, the stack */
  rt_mpush(rt_true_); rt_mpush(rt_false_); rt_mpush(rt_nil_);
  rt_mpush(rt_none_); rt_mpush(rt_unit_world);
  /* The codegen emits file-scope `static value_t sl_N` globals for interned
     string/symbol literals (e.g. extern names) — roots not on any stack. Scan
     the whole data+bss segment conservatively. Safe: rt_free[] was just
     cleared (so the free lists can't keep dead cells alive), and rt.h's other
     globals point to non-cell memory (mmap/malloc), which the start-bitmap
     check in rt_try_mark rejects. */
  { extern char __data_start[], _end[];
    rt_scan_range(__data_start, _end); }
  /* Spill ALL callee-saved registers onto this frame's stack so the scan
     below catches roots the running computation keeps in registers. This is
     the GC idiom (cf. Boehm's GC_with_callee_saves_pushed) — more reliable
     than setjmp, whose jmp_buf mangles rbp/rsp (glibc PTR_MANGLE) and would
     hide a register-resident root. The spilled words sit in rt_gc's frame,
     which is between sp (taken after the spill) and rt_stack_base. */
  __builtin_unwind_init();
  /* Read the ACTUAL stack pointer after the spill: everything live is at an
     address >= rsp, so [rsp, base] is guaranteed to cover the spilled
     registers (a frame local like &probe might be placed above them). */
  char *sp;
#if defined(__x86_64__)
  __asm__ volatile("mov %%rsp, %0" : "=r"(sp));
#else
  volatile char probe; sp = (char *)&probe;
#endif
  rt_scan_range(sp, rt_stack_base);
  while (rt_mtop) rt_scan(rt_mstk[--rt_mtop]);
  /* sweep: linear walk; live -> clear mark, dead -> size-class free list */
  size_t live = 0;
  for (char *p = rt_heap_base; p < rt_heap; ) {
    rt_cell *c = (rt_cell *)p;
    size_t sz = rt_cellsz(c->arity);
    if (c->tag & RT_MARK_) { c->tag &= ~RT_MARK_; live += sz; }
    else { unsigned cl = (unsigned)(sz >> 4);
           c->fields[0] = (value_t)rt_free[cl]; rt_free[cl] = c; }
    p += sz;
  }
  rt_live_bytes = live;
}

static value_t rt_alloc(uint32_t tag, uint32_t arity) {
  size_t sz = rt_cellsz(arity);
  unsigned cl = (unsigned)(sz >> 4);
  for (;;) {
    rt_cell *c = rt_free[cl];
    if (c) { rt_free[cl] = (rt_cell *)c->fields[0];
             c->tag = tag; c->arity = arity; return (value_t)c; }
    if (rt_gc_on && rt_heap + sz > rt_gc_trigger) {
      rt_gc();
      size_t spacing = rt_live_bytes;
      if (spacing < RT_GC_MIN_SPACING) spacing = RT_GC_MIN_SPACING;
      if (spacing > RT_GC_MAX_SPACING) spacing = RT_GC_MAX_SPACING;
      rt_gc_trigger = rt_heap + spacing;
      if (rt_gc_trigger > rt_heap_end) rt_gc_trigger = rt_heap_end;
      if (rt_free[cl]) continue;       /* sweep produced a cell of this class */
    }
    if (rt_heap + sz > rt_heap_end) rt_trap("heap exhausted");
    rt_cell *nc = (rt_cell *)rt_heap; rt_heap += sz; rt_setstart((char *)nc);
    nc->tag = tag; nc->arity = arity; return (value_t)nc;
  }
}

static inline int rt_truthy(value_t v) { return rt_tag(v) == RT_TAG_True; }
static inline value_t rt_bool(int b) { return b ? rt_true_ : rt_false_; }

static inline value_t rt_cons(value_t h, value_t t) {
  value_t c = rt_alloc(RT_TAG_Cons, 2);
  rt_cellof(c)->fields[0] = h; rt_cellof(c)->fields[1] = t;
  return c;
}

/* ---- symbols: intern table --------------------------------------------- */
typedef struct { char *s; uint32_t len; } rt_symrec;
static rt_symrec *rt_syms; static uint32_t rt_nsyms, rt_symcap;

static uint32_t rt_intern(const char *s, uint32_t len) {
  for (uint32_t i = 0; i < rt_nsyms; i++)
    if (rt_syms[i].len == len && !memcmp(rt_syms[i].s, s, len)) return i;
  if (rt_nsyms == rt_symcap) {
    rt_symcap = rt_symcap ? rt_symcap * 2 : 4096;
    rt_syms = realloc(rt_syms, rt_symcap * sizeof(rt_symrec));
  }
  rt_syms[rt_nsyms].s = malloc(len + 1);
  memcpy(rt_syms[rt_nsyms].s, s, len); rt_syms[rt_nsyms].s[len] = 0;
  rt_syms[rt_nsyms].len = len;
  return rt_nsyms++;
}
static value_t rt_sym(uint32_t id) {
  value_t c = rt_alloc(RT_TAG_SYM_, 1);
  rt_cellof(c)->fields[0] = rt_tag_int((int64_t)id);
  return c;
}
static inline uint32_t rt_symid(value_t v) { return (uint32_t)rt_untag(rt_field(v, 0)); }

/* ---- bignum: out-of-i63 ints as heap cells ------------------------------- */
/* Cell layout: tag RT_TAG_BIG_ (runtime-reserved, like SYM_), field 0 = sign
 * (tagged -1/+1), fields 1..arity-1 = 32-bit magnitude limbs little-endian,
 * each stored as a TAGGED SMALL INT so the conservative GC needs no changes.
 * CANONICAL: top limb nonzero, magnitude outside i63 (in-range values demote
 * to small ints in rt_big_make) — so tagged-word equality stays valid for
 * smalls and small-vs-big is never equal. Slow paths only: every prim keeps
 * its small-int fast path and falls through on overflow or a big operand.
 * Semantics authority = prim.rs (num-bigint): truncating `/`+`tmod`,
 * euclidean `mod`+`ediv`, two's-complement bitwise, floor shifts. Division
 * is bit-serial (simple over fast; division is rare in cert checking). */
#define RT_TAG_BIG_ 0x7ffffffeu
#define RT_BIG_MAXLIMB 120  /* ~2^3840; stays inside the GC size classes */

static inline int rt_is_big(value_t v) {
  return !(v & 1) && rt_cellof(v)->tag == RT_TAG_BIG_;
}
static inline int rt_big_sign(value_t v) { return (int)rt_untag(rt_field(v, 0)); }
static inline uint32_t rt_big_n(value_t v) { return rt_cellof(v)->arity - 1; }
static inline uint32_t rt_big_limb(value_t v, uint32_t i) {
  return (uint32_t)rt_untag(rt_field(v, i + 1));
}

/* magnitude helpers: little-endian uint32 limb arrays */
static uint32_t rt_mag_norm(const uint32_t *a, uint32_t n) {
  while (n && !a[n - 1]) n--;
  return n;
}
static int rt_mag_cmp(const uint32_t *a, uint32_t na,
                      const uint32_t *b, uint32_t nb) {
  if (na != nb) return na < nb ? -1 : 1;
  for (uint32_t i = na; i > 0; i--)
    if (a[i - 1] != b[i - 1]) return a[i - 1] < b[i - 1] ? -1 : 1;
  return 0;
}
/* r = a + b (r cap max(na,nb)+1) */
static uint32_t rt_mag_add(const uint32_t *a, uint32_t na,
                           const uint32_t *b, uint32_t nb, uint32_t *r) {
  if (na < nb) { const uint32_t *tp = a; a = b; b = tp;
                 uint32_t tn = na; na = nb; nb = tn; }
  uint64_t carry = 0;
  for (uint32_t i = 0; i < na; i++) {
    uint64_t s = (uint64_t)a[i] + (i < nb ? b[i] : 0) + carry;
    r[i] = (uint32_t)s; carry = s >> 32;
  }
  if (carry) { r[na] = (uint32_t)carry; return na + 1; }
  return rt_mag_norm(r, na);
}
/* r = a - b, requires a >= b; in-place safe (r may alias a) */
static uint32_t rt_mag_sub(const uint32_t *a, uint32_t na,
                           const uint32_t *b, uint32_t nb, uint32_t *r) {
  int64_t borrow = 0;
  for (uint32_t i = 0; i < na; i++) {
    int64_t d = (int64_t)a[i] - (int64_t)(i < nb ? b[i] : 0) - borrow;
    if (d < 0) { d += (int64_t)1 << 32; borrow = 1; } else borrow = 0;
    r[i] = (uint32_t)d;
  }
  return rt_mag_norm(r, na);
}
/* r = a * b (r cap na+nb, distinct from a and b) */
static uint32_t rt_mag_mul(const uint32_t *a, uint32_t na,
                           const uint32_t *b, uint32_t nb, uint32_t *r) {
  memset(r, 0, (size_t)(na + nb) * 4);
  for (uint32_t i = 0; i < na; i++) {
    uint64_t ai = a[i], carry = 0;
    if (!ai) continue;
    for (uint32_t j = 0; j < nb; j++) {
      uint64_t s = ai * b[j] + r[i + j] + carry;
      r[i + j] = (uint32_t)s; carry = s >> 32;
    }
    r[i + nb] = (uint32_t)carry;  /* slot untouched until iteration i */
  }
  return rt_mag_norm(r, na + nb);
}
/* r = a << k, k in 0..63 (r cap na + k/32 + 2, distinct from a) */
static uint32_t rt_mag_shl(const uint32_t *a, uint32_t na, uint32_t k,
                           uint32_t *r) {
  uint32_t lw = k >> 5, lb = k & 31, n = na + lw + (lb ? 1 : 0);
  memset(r, 0, (size_t)n * 4);
  for (uint32_t i = 0; i < na; i++) {
    uint64_t v = (uint64_t)a[i] << lb;
    r[i + lw] |= (uint32_t)v;
    if (lb) r[i + lw + 1] |= (uint32_t)(v >> 32);
  }
  return rt_mag_norm(r, n);
}
/* r = a >> k; *dropped = 1 iff any shifted-out bit was set (floor fix-up) */
static uint32_t rt_mag_shr(const uint32_t *a, uint32_t na, uint32_t k,
                           uint32_t *r, int *dropped) {
  uint32_t lw = k >> 5, lb = k & 31;
  *dropped = 0;
  for (uint32_t i = 0; i < lw && i < na; i++) if (a[i]) *dropped = 1;
  if (lb && lw < na && (a[lw] & ((1u << lb) - 1))) *dropped = 1;
  if (lw >= na) return 0;
  uint32_t n = na - lw;
  for (uint32_t i = 0; i < n; i++) {
    uint64_t hi = (i + lw + 1 < na) ? a[i + lw + 1] : 0;
    r[i] = lb ? (uint32_t)(((uint64_t)a[i + lw] >> lb) | (hi << (32 - lb)))
              : a[i + lw];
  }
  return rt_mag_norm(r, n);
}
/* bit-serial divmod: q = a/b, rem = a%b on magnitudes; b nonzero.
   q cap na (zeroed here), rem cap nb+1 (zeroed here). */
static void rt_mag_divmod(const uint32_t *a, uint32_t na,
                          const uint32_t *b, uint32_t nb,
                          uint32_t *q, uint32_t *nq,
                          uint32_t *rem, uint32_t *nrem) {
  memset(q, 0, (size_t)na * 4);
  memset(rem, 0, (size_t)(nb + 1) * 4);
  uint32_t rn = 0;
  for (uint32_t bit = na * 32; bit > 0; bit--) {
    uint32_t i = bit - 1;
    uint32_t carry = (a[i >> 5] >> (i & 31)) & 1;  /* rem = rem<<1 | bit */
    for (uint32_t j = 0; j <= nb; j++) {
      uint32_t nc = rem[j] >> 31;
      rem[j] = (rem[j] << 1) | carry;
      carry = nc;
    }
    rn = rt_mag_norm(rem, nb + 1);
    if (rt_mag_cmp(rem, rn, b, nb) >= 0) {
      rn = rt_mag_sub(rem, rn, b, nb, rem);
      q[i >> 5] |= 1u << (i & 31);
    }
  }
  *nq = rt_mag_norm(q, na);
  *nrem = rn;
}

/* build the canonical value; demotes to a small int when it fits */
static value_t rt_big_make(int sign, const uint32_t *limbs, uint32_t n) {
  n = rt_mag_norm(limbs, n);
  if (!n || !sign) return rt_mk_small(0);
  if (n <= 2) {
    uint64_t m = (uint64_t)limbs[0] | (n > 1 ? (uint64_t)limbs[1] << 32 : 0);
    if (sign > 0 && m <= (uint64_t)RT_INT_MAX) return rt_mk_small((int64_t)m);
    if (sign < 0 && m <= (uint64_t)RT_INT_MAX + 1)
      return rt_mk_small(m == (uint64_t)RT_INT_MAX + 1 ? RT_INT_MIN
                                                       : -(int64_t)m);
  }
  if (n > RT_BIG_MAXLIMB) rt_trap("bignum too large (exceeds 2^3840)");
  value_t c = rt_alloc(RT_TAG_BIG_, n + 1);
  rt_cellof(c)->fields[0] = rt_mk_small(sign);
  for (uint32_t i = 0; i < n; i++)
    rt_cellof(c)->fields[i + 1] = rt_mk_small((int64_t)limbs[i]);
  return c;
}
static value_t rt_big_from_i64(int64_t x) {
  uint64_t m = x < 0 ? -(uint64_t)x : (uint64_t)x;
  uint32_t limbs[2] = { (uint32_t)m, (uint32_t)(m >> 32) };
  return rt_big_make(x < 0 ? -1 : 1, limbs, 2);
}

/* copy any int value's limbs into a fresh malloc'd array (>= 1 slot) */
static uint32_t *rt_limbs_alloc(value_t v, uint32_t *n, int *sign) {
  if (v & 1) {
    int64_t x = rt_untag(v);
    uint32_t *p = (uint32_t *)malloc(2 * 4);
    if (!x) { *sign = 0; *n = 0; return p; }
    uint64_t m = x < 0 ? -(uint64_t)x : (uint64_t)x;
    *sign = x < 0 ? -1 : 1;
    p[0] = (uint32_t)m; p[1] = (uint32_t)(m >> 32);
    *n = p[1] ? 2 : 1;
    return p;
  }
  uint32_t nn = rt_big_n(v);
  uint32_t *p = (uint32_t *)malloc((size_t)nn * 4);
  for (uint32_t i = 0; i < nn; i++) p[i] = rt_big_limb(v, i);
  *sign = rt_big_sign(v);
  *n = nn;
  return p;
}

/* a + b, or a - b when negb (slow path; at least one operand big or the
   small fast path overflowed) */
static value_t rt_iadd2(value_t a, value_t b, int negb) {
  int sa, sb; uint32_t na, nb;
  uint32_t *A = rt_limbs_alloc(a, &na, &sa);
  uint32_t *B = rt_limbs_alloc(b, &nb, &sb);
  if (negb) sb = -sb;
  uint32_t *R = (uint32_t *)malloc(((size_t)(na > nb ? na : nb) + 1) * 4);
  int rs; uint32_t rn;
  if (!sa)      { memcpy(R, B, (size_t)nb * 4); rn = nb; rs = sb; }
  else if (!sb) { memcpy(R, A, (size_t)na * 4); rn = na; rs = sa; }
  else if (sa == sb) { rn = rt_mag_add(A, na, B, nb, R); rs = sa; }
  else {
    int c = rt_mag_cmp(A, na, B, nb);
    if (!c)      { rn = 0; rs = 0; }
    else if (c > 0) { rn = rt_mag_sub(A, na, B, nb, R); rs = sa; }
    else            { rn = rt_mag_sub(B, nb, A, na, R); rs = sb; }
  }
  value_t out = rt_big_make(rs, R, rn);
  free(A); free(B); free(R);
  return out;
}
static value_t rt_imul2(value_t a, value_t b) {
  int sa, sb; uint32_t na, nb;
  uint32_t *A = rt_limbs_alloc(a, &na, &sa);
  uint32_t *B = rt_limbs_alloc(b, &nb, &sb);
  if (!sa || !sb) { free(A); free(B); return rt_mk_small(0); }
  uint32_t *R = (uint32_t *)malloc((size_t)(na + nb) * 4);
  uint32_t rn = rt_mag_mul(A, na, B, nb, R);
  value_t out = rt_big_make(sa == sb ? 1 : -1, R, rn);
  free(A); free(B); free(R);
  return out;
}
/* op: 0 = `/` (trunc), 1 = tmod, 2 = mod (euclid), 3 = ediv */
static value_t rt_idiv2(int op, value_t a, value_t b) {
  int sa, sb; uint32_t na, nb;
  uint32_t *A = rt_limbs_alloc(a, &na, &sa);
  uint32_t *B = rt_limbs_alloc(b, &nb, &sb);
  if (!sb) rt_trap("div/mod by zero (stuck on the interpreted engines)");
  uint32_t *Q = (uint32_t *)malloc(((size_t)na + 1) * 4);
  uint32_t *REM = (uint32_t *)malloc(((size_t)nb + 1) * 4);
  uint32_t nq = 0, nrem = 0;
  if (sa) rt_mag_divmod(A, na, B, nb, Q, &nq, REM, &nrem);
  else { memset(Q, 0, ((size_t)na + 1) * 4); memset(REM, 0, ((size_t)nb + 1) * 4); }
  value_t out;
  switch (op) {
    case 0:  /* trunc quotient: sign = sa*sb */
      out = rt_big_make(sa == sb ? 1 : -1, Q, nq);
      break;
    case 1:  /* trunc remainder: sign follows the dividend */
      out = rt_big_make(sa, REM, nrem);
      break;
    case 2:  /* euclidean remainder: in [0, |b|) */
      if (sa >= 0 || !nrem) out = rt_big_make(1, REM, nrem);
      else { nrem = rt_mag_sub(B, nb, REM, nrem, REM);
             out = rt_big_make(1, REM, nrem); }
      break;
    default: /* ediv: trunc quotient, minus-one-ward fix when a<0 with rest */
      if (sa < 0 && nrem) {
        uint32_t one = 1;
        nq = rt_mag_add(Q, nq, &one, 1, Q);  /* Q cap na+1, in-place ok */
        out = rt_big_make(-sb, Q, nq);
      } else out = rt_big_make(sa == sb ? 1 : -1, Q, nq);
      break;
  }
  free(A); free(B); free(Q); free(REM);
  return out;
}
/* three-way compare (slow path; at least one big). Canonical form makes
   sign/size decide before any limb walk. */
static int rt_icmp2(value_t a, value_t b) {
  int64_t xa, xb;
  int sa = (a & 1) ? ((xa = rt_untag(a)) > 0 ? 1 : xa < 0 ? -1 : 0)
                   : rt_big_sign(a);
  int sb = (b & 1) ? ((xb = rt_untag(b)) > 0 ? 1 : xb < 0 ? -1 : 0)
                   : rt_big_sign(b);
  if (sa != sb) return sa < sb ? -1 : 1;
  if (!sa) return 0;
  int abig = !(a & 1), bbig = !(b & 1);
  if (abig != bbig) return (abig ? 1 : -1) * sa;  /* big magnitude > small */
  if (!abig) { int64_t x = rt_untag(a), y = rt_untag(b);
               return x < y ? -1 : x > y ? 1 : 0; }
  uint32_t na = rt_big_n(a), nb = rt_big_n(b);
  if (na != nb) return (na < nb ? -1 : 1) * sa;
  for (uint32_t i = na; i > 0; i--) {
    uint32_t la = rt_big_limb(a, i - 1), lb = rt_big_limb(b, i - 1);
    if (la != lb) return (la < lb ? -1 : 1) * sa;
  }
  return 0;
}
/* two's-complement bitwise (num-bigint semantics); op: 0 & 1 | 2 ^ */
static value_t rt_ibit2(int op, value_t a, value_t b) {
  int sa, sb; uint32_t na, nb;
  uint32_t *A = rt_limbs_alloc(a, &na, &sa);
  uint32_t *B = rt_limbs_alloc(b, &nb, &sb);
  uint32_t len = (na > nb ? na : nb) + 1;
  uint32_t *TA = (uint32_t *)malloc((size_t)len * 4);
  uint32_t *TB = (uint32_t *)malloc((size_t)len * 4);
  /* materialize two's complement over len limbs (sign-extended) */
  for (uint32_t i = 0; i < len; i++) TA[i] = i < na ? A[i] : 0;
  for (uint32_t i = 0; i < len; i++) TB[i] = i < nb ? B[i] : 0;
  if (sa < 0) { uint64_t c = 1;
    for (uint32_t i = 0; i < len; i++) { uint64_t s = (uint64_t)(~TA[i]) + c;
      TA[i] = (uint32_t)s; c = s >> 32; } }
  if (sb < 0) { uint64_t c = 1;
    for (uint32_t i = 0; i < len; i++) { uint64_t s = (uint64_t)(~TB[i]) + c;
      TB[i] = (uint32_t)s; c = s >> 32; } }
  for (uint32_t i = 0; i < len; i++)
    TA[i] = op == 0 ? (TA[i] & TB[i]) : op == 1 ? (TA[i] | TB[i])
                                                : (TA[i] ^ TB[i]);
  value_t out;
  if (TA[len - 1] & 0x80000000u) {   /* negative: back to sign-magnitude */
    uint64_t c = 1;
    for (uint32_t i = 0; i < len; i++) { uint64_t s = (uint64_t)(~TA[i]) + c;
      TA[i] = (uint32_t)s; c = s >> 32; }
    out = rt_big_make(-1, TA, len);
  } else out = rt_big_make(1, TA, len);
  free(A); free(B); free(TA); free(TB);
  return out;
}
static value_t rt_ishl2(value_t a, uint32_t k) {
  int sa; uint32_t na;
  uint32_t *A = rt_limbs_alloc(a, &na, &sa);
  uint32_t *R = (uint32_t *)malloc(((size_t)na + (k >> 5) + 2) * 4);
  uint32_t rn = rt_mag_shl(A, na, k, R);
  value_t out = rt_big_make(sa, R, rn);
  free(A); free(R);
  return out;
}
static value_t rt_ishr2(value_t a, uint32_t k) {
  int sa, dropped; uint32_t na;
  uint32_t *A = rt_limbs_alloc(a, &na, &sa);
  uint32_t *R = (uint32_t *)malloc(((size_t)na + 1) * 4);
  uint32_t rn = rt_mag_shr(A, na, k, R, &dropped);
  if (sa < 0 && dropped) {           /* floor: -(|a|>>k) - 1 */
    uint32_t one = 1;
    rn = rt_mag_add(R, rn, &one, 1, R);
  }
  value_t out = rt_big_make(sa, R, rn);
  free(A); free(R);
  return out;
}
/* decimal literal -> value (codegen emits rt_big_dec("...") for out-of-i63
   literals; also the unit-test entry) */
static value_t rt_big_dec(const char *s) {
  int sign = 1;
  if (*s == '-') { sign = -1; s++; }
  uint32_t cap = (uint32_t)(strlen(s) / 9 + 2), n = 0;
  uint32_t *L = (uint32_t *)malloc((size_t)cap * 4);
  for (; *s; s++) {
    uint32_t carry = (uint32_t)(*s - '0');
    for (uint32_t i = 0; i < n; i++) {
      uint64_t v = (uint64_t)L[i] * 10 + carry;
      L[i] = (uint32_t)v; carry = (uint32_t)(v >> 32);
    }
    if (carry) L[n++] = carry;
  }
  value_t r = rt_big_make(sign, L, n);
  free(L);
  return r;
}
/* euclidean low byte of any int (the extern wire's mod-256 masking) */
static inline int rt_byte_of(value_t v) {
  if (v & 1) return (int)(rt_untag(v) & 0xFF);
  int b = (int)(rt_big_limb(v, 0) & 0xFF);
  return rt_big_sign(v) < 0 ? (256 - b) & 0xFF : b;
}

/* ---- prims (specs = kernel reduce.shard / host prim.rs) ------------------ */
/* Every prim: small-int fast path first (both operands tagged odd), big
 * slow path on overflow or a big operand. rt_tag_int itself promotes
 * int64-but-not-i63 results, so fast paths only pre-check int64 overflow. */
static inline value_t rt_add(value_t a, value_t b) {
  if (a & b & 1) {
    int64_t r;
    if (!__builtin_add_overflow(rt_untag(a), rt_untag(b), &r))
      return rt_tag_int(r);
  }
  return rt_iadd2(a, b, 0);
}
static inline value_t rt_sub(value_t a, value_t b) {
  if (a & b & 1) {
    int64_t r;
    if (!__builtin_sub_overflow(rt_untag(a), rt_untag(b), &r))
      return rt_tag_int(r);
  }
  return rt_iadd2(a, b, 1);
}
static inline value_t rt_mul(value_t a, value_t b) {
  if (a & b & 1) {
    int64_t r;
    if (!__builtin_mul_overflow(rt_untag(a), rt_untag(b), &r))
      return rt_tag_int(r);
  }
  return rt_imul2(a, b);
}
/* `/` truncates toward zero; `mod` is the EUCLIDEAN remainder (always >= 0);
 * `tmod` is the truncating remainder; `ediv` the Euclidean division.
 * Small/small never overflows int64 (operands are i63), so the fast paths
 * are exact as-is. */
static inline value_t rt_div(value_t a, value_t b) {
  if (a & b & 1) {
    int64_t d = rt_untag(b);
    if (d == 0) rt_trap("div by zero (stuck on the interpreted engines)");
    return rt_tag_int(rt_untag(a) / d);  /* RT_INT_MIN / -1 = 2^62 promotes */
  }
  return rt_idiv2(0, a, b);
}
static inline value_t rt_mod(value_t a, value_t b) {
  if (a & b & 1) {
    int64_t n = rt_untag(a), d = rt_untag(b);
    if (d == 0) rt_trap("mod by zero (stuck on the interpreted engines)");
    int64_t r = n % d;
    if (r < 0) r += (d < 0 ? -d : d);
    return rt_mk_small(r);
  }
  return rt_idiv2(2, a, b);
}
static inline value_t rt_tmod(value_t a, value_t b) {
  if (a & b & 1) {
    int64_t d = rt_untag(b);
    if (d == 0) rt_trap("tmod by zero");
    return rt_mk_small(rt_untag(a) % d);
  }
  return rt_idiv2(1, a, b);
}
static inline value_t rt_ediv(value_t a, value_t b) {
  if (a & b & 1) {
    int64_t n = rt_untag(a), d = rt_untag(b);
    if (d == 0) rt_trap("ediv by zero");
    int64_t q = n / d, r = n % d;
    if (r < 0) q -= (d > 0 ? 1 : -1);
    return rt_tag_int(q);  /* RT_INT_MIN / -1 = 2^62 promotes */
  }
  return rt_idiv2(3, a, b);
}
static inline value_t rt_lt(value_t a, value_t b) {
  if (a & b & 1) return rt_bool(rt_untag(a) < rt_untag(b));
  return rt_bool(rt_icmp2(a, b) < 0);
}
static inline value_t rt_le(value_t a, value_t b) {
  if (a & b & 1) return rt_bool(rt_untag(a) <= rt_untag(b));
  return rt_bool(rt_icmp2(a, b) <= 0);
}
static inline value_t rt_int_eq(value_t a, value_t b) {
  if (a & b & 1) return rt_bool(a == b);  /* canonical tagged words */
  return rt_bool(rt_icmp2(a, b) == 0);
}
static inline value_t rt_sym_eq(value_t a, value_t b) { return rt_bool(rt_symid(a) == rt_symid(b)); }
/* small & | ^ stay in i63 (two's complement is closed under them) */
static inline value_t rt_band(value_t a, value_t b) {
  if (a & b & 1) return rt_mk_small(rt_untag(a) & rt_untag(b));
  return rt_ibit2(0, a, b);
}
static inline value_t rt_bor(value_t a, value_t b) {
  if (a & b & 1) return rt_mk_small(rt_untag(a) | rt_untag(b));
  return rt_ibit2(1, a, b);
}
static inline value_t rt_bxor(value_t a, value_t b) {
  if (a & b & 1) return rt_mk_small(rt_untag(a) ^ rt_untag(b));
  return rt_ibit2(2, a, b);
}
/* shift counts stay guarded to 0..63 (the interpreted engines' stuck rule;
 * a big count is out of range by canonicality) */
static inline value_t rt_bshl(value_t a, value_t b) {
  if (!(b & 1)) rt_trap("bshl shift out of range (stuck on engines)");
  int64_t s = rt_untag(b);
  if (s < 0 || s >= 64) rt_trap("bshl shift out of range (stuck on engines)");
  if (a & 1) {
    int64_t r = rt_untag(a);
    if (s < 2 || (r <= (RT_INT_MAX >> s) && r >= (RT_INT_MIN >> s)))
      /* shift via unsigned: defined for negative r, same bits */
      return rt_tag_int((int64_t)((uint64_t)r << s));
  }
  return rt_ishl2(a, (uint32_t)s);
}
static inline value_t rt_bshr(value_t a, value_t b) {
  if (!(b & 1)) rt_trap("bshr shift out of range (stuck on engines)");
  int64_t s = rt_untag(b);
  if (s < 0 || s >= 64) rt_trap("bshr shift out of range (stuck on engines)");
  if (a & 1) return rt_mk_small(rt_untag(a) >> s);  /* arithmetic = floor */
  return rt_ishr2(a, (uint32_t)s);
}
static value_t rt_word_stub(void) { rt_trap("word ops unimplemented in v0 codegen"); return 0; }

static int64_t rt_fresh_ctr = 0;
static value_t rt_gen_fresh(void) {
  char buf[32];
  int n = snprintf(buf, sizeof buf, "g__%lld", (long long)rt_fresh_ctr++);
  return rt_sym(rt_intern(buf, (uint32_t)n));
}

static value_t rt_chars_of_sym(value_t s) {
  rt_symrec *r = &rt_syms[rt_symid(s)];
  value_t acc = rt_nil_;
  for (int64_t i = (int64_t)r->len - 1; i >= 0; i--)
    acc = rt_cons(rt_tag_int((unsigned char)r->s[i]), acc);
  return acc;
}
static value_t rt_sym_of_chars(value_t l) {
  char buf[4096]; uint32_t n = 0;
  for (value_t p = l; rt_tag(p) == RT_TAG_Cons; p = rt_field(p, 1)) {
    if (n >= sizeof buf) rt_trap("sym_of_chars: name too long");
    buf[n++] = (char)rt_untag(rt_field(p, 0));
  }
  return rt_sym(rt_intern(buf, n));
}

/* ---- externs (World = preallocated unit cell, threaded for order) -------- */
static int rt_argc; static char **rt_argv;

/* The extern wire speaks RAW BYTES (issue #2 Phase 3): every (List Int)
   payload is a byte sequence, one element per byte (masked mod 256 on the
   way out, mirroring bytes_of_list). No UTF-8 codec at this boundary —
   reads are binary-safe, writes emit the list verbatim. Matches the Rust
   engine (eval.rs byte_list/list_bytes). */
static value_t rt_str_list(const char *s) {
  /* argv -> its raw bytes */
  size_t len = strlen(s); const unsigned char *us = (const unsigned char *)s;
  value_t acc = rt_nil_;
  for (size_t k = len; k > 0; k--) acc = rt_cons(rt_tag_int(us[k-1]), acc);
  return acc;
}
static char *rt_cstr(value_t l, size_t *lenp) {
  size_t n = 0;
  for (value_t p = l; rt_tag(p) == RT_TAG_Cons; p = rt_field(p, 1)) n++;
  char *buf = malloc(n + 1); size_t i = 0;
  for (value_t p = l; rt_tag(p) == RT_TAG_Cons; p = rt_field(p, 1))
    buf[i++] = (char)rt_byte_of(rt_field(p, 0));
  buf[i] = 0; if (lenp) *lenp = i;
  return buf;
}
static value_t rt_pair(value_t a, value_t b) {
  value_t c = rt_alloc(RT_TAG_Pair, 2);
  rt_cellof(c)->fields[0] = a; rt_cellof(c)->fields[1] = b;
  return c;
}
static value_t rt_some(value_t a) {
  value_t c = rt_alloc(RT_TAG_Some, 1);
  rt_cellof(c)->fields[0] = a;
  return c;
}

static value_t rt_get_args(value_t w) {
  value_t acc = rt_nil_;
  for (int i = rt_argc - 1; i >= 1; i--) acc = rt_cons(rt_str_list(rt_argv[i]), acc);
  return rt_pair(acc, w);
}
static value_t rt_read_file(value_t path, value_t w) {
  char *p = rt_cstr(path, 0);
  FILE *f = fopen(p, "rb"); free(p);
  if (!f) return rt_pair(rt_none_, w);
  fseek(f, 0, SEEK_END); long sz = ftell(f); fseek(f, 0, SEEK_SET);
  unsigned char *buf = malloc((size_t)sz);
  if (sz > 0 && fread(buf, 1, (size_t)sz, f) != (size_t)sz) { fclose(f); free(buf); return rt_pair(rt_none_, w); }
  fclose(f);
  /* the raw bytes, binary-safe — None only on an I/O error above */
  value_t acc = rt_nil_;
  for (long k = sz; k > 0; k--) acc = rt_cons(rt_tag_int(buf[k-1]), acc);
  free(buf);
  return rt_pair(rt_some(acc), w);
}
/* directory entries as (Some (List (Pair is_dir name))), or None if the path
 * is not a readable directory. Mirrors eval.rs read_dir: each entry is a
 * (Pair Bool (List Int)) of the is-directory flag and the basename bytes,
 * sorted by name so the listing is deterministic. */
static int rt_name_cmp(const void *a, const void *b) {
  return strcmp(*(const char *const *)a, *(const char *const *)b);
}
static value_t rt_read_dir(value_t path, value_t w) {
  char *p = rt_cstr(path, 0);
  DIR *d = opendir(p);
  if (!d) { free(p); return rt_pair(rt_none_, w); }
  /* collect names, then sort for a deterministic listing */
  size_t cap = 16, n = 0;
  char **names = malloc(cap * sizeof(char *));
  struct dirent *de;
  while ((de = readdir(d))) {
    if (!strcmp(de->d_name, ".") || !strcmp(de->d_name, "..")) continue;
    if (n == cap) { cap *= 2; names = realloc(names, cap * sizeof(char *)); }
    names[n++] = strdup(de->d_name);
  }
  closedir(d);
  qsort(names, n, sizeof(char *), rt_name_cmp);
  value_t acc = rt_nil_;
  for (size_t k = n; k > 0; k--) {
    const char *nm = names[k - 1];
    /* is_dir via stat on the joined path */
    size_t pl = strlen(p), nl = strlen(nm);
    char *full = malloc(pl + nl + 2);
    memcpy(full, p, pl); full[pl] = '/'; memcpy(full + pl + 1, nm, nl); full[pl + nl + 1] = 0;
    struct stat st;
    int is_dir = stat(full, &st) == 0 && S_ISDIR(st.st_mode);
    free(full);
    value_t namelist = rt_str_list(nm);
    acc = rt_cons(rt_pair(rt_bool(is_dir), namelist), acc);
    free((void *)nm);
  }
  free(names); free(p);
  return rt_pair(rt_some(acc), w);
}
static value_t rt_write(value_t s, value_t w) {
  size_t n; char *p = rt_cstr(s, &n);
  fwrite(p, 1, n, stdout); fflush(stdout); free(p);
  return w;
}
static value_t rt_write_line(value_t s, value_t w) {
  rt_write(s, w); fputc('\n', stdout); fflush(stdout);
  return w;
}
static value_t rt_write_file(value_t path, value_t bytes, value_t w) {
  char *p = rt_cstr(path, 0);
  size_t n; char *b = rt_cstr(bytes, &n);
  FILE *f = fopen(p, "wb");
  int ok = f && fwrite(b, 1, n, f) == n;
  if (f) fclose(f);
  free(p); free(b);
  return rt_pair(rt_bool(ok), w);
}
static value_t rt_exit(value_t code, value_t w) {
  (void)w; fflush(stdout); exit((int)rt_untag(code));
}
static value_t rt_read_key(value_t w) {
  int c = getchar();
  return rt_pair(c == EOF ? rt_none_ : rt_some(rt_tag_int(c)), w);
}

static value_t rt_no_match(const char *fn) {
  fprintf(stderr, "rt trap: inexhaustive match in %s\n", fn);
  exit(3);
}

/* ---- init ---------------------------------------------------------------- */
/* Called from main as `rt_init(argc, argv)`, so this frame sits just below
   main's — &argc is a safe high boundary for the conservative stack scan
   (all later computation frames are at lower addresses). */
static void rt_init(int argc, char **argv) {
  rt_argc = argc; rt_argv = argv;
  rt_stack_base = (char *)&argc;
  struct rlimit rl = { 2ul << 30, 2ul << 30 };
  setrlimit(RLIMIT_STACK, &rl);  /* best effort; deep non-self recursion */
  size_t cap = 64ul << 30;       /* 64 GB reserve, NORESERVE — pay as used */
  rt_heap = mmap(0, cap, PROT_READ | PROT_WRITE,
                 MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE, -1, 0);
  if (rt_heap == MAP_FAILED) rt_trap("mmap failed");
  rt_heap_base = rt_heap;
  rt_heap_end = rt_heap + cap;
  /* one start-bit per 16B slot, NORESERVE (touched pages only) */
  rt_startmap = mmap(0, (cap >> 4) >> 3, PROT_READ | PROT_WRITE,
                     MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE, -1, 0);
  if (rt_startmap == MAP_FAILED) rt_trap("startmap mmap failed");
  rt_gc_trigger = rt_heap + RT_GC_MIN_SPACING;
  rt_true_  = rt_alloc(RT_TAG_True, 0);
  rt_false_ = rt_alloc(RT_TAG_False, 0);
  rt_nil_   = rt_alloc(RT_TAG_Nil, 0);
  rt_none_  = rt_alloc(RT_TAG_None, 0);
  rt_unit_world = rt_alloc(RT_TAG_World, 0);
  rt_gc_on = 1;   /* singletons allocated; collector live from here */
}
#endif
