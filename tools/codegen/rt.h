/* rt.h — the C runtime under codegen'd RS-shard (see tools/lower/DESIGN.md).
 *
 * Generated out.c #defines the core ctor tags (RT_TAG_True, RT_TAG_False,
 * RT_TAG_Nil, RT_TAG_Cons, RT_TAG_None, RT_TAG_Some, RT_TAG_Pair,
 * RT_TAG_World) BEFORE including this header.
 *
 * Representation: value_t is one machine word.
 *   odd  = small int, (i << 1) | 1, i in i63 — overflow TRAPS (the
 *          interpreted engines are BigInt-exact; a trap means "rerun on
 *          the direct engine", never a wrong answer).
 *   even = pointer to cell { tag, arity, fields... }.
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
static inline value_t rt_tag_int(int64_t i) {
  if (i > RT_INT_MAX || i < RT_INT_MIN) rt_trap("i63 overflow");
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
#define RT_GC_SPACING (2ul << 30)     /* bump this much between collections.
   2 GiB (was 8): the collector reclaims, so peak resident ~= live + this per
   process — keeping it small lets gate_sweep/run_corpus run JOBS-parallel
   without OOM (8 GiB x N processes was the killer). */

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
  for (char *p = rt_heap_base; p < rt_heap; ) {
    rt_cell *c = (rt_cell *)p;
    size_t sz = rt_cellsz(c->arity);
    if (c->tag & RT_MARK_) c->tag &= ~RT_MARK_;
    else { unsigned cl = (unsigned)(sz >> 4);
           c->fields[0] = (value_t)rt_free[cl]; rt_free[cl] = c; }
    p += sz;
  }
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
      rt_gc_trigger = rt_heap + RT_GC_SPACING;
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

/* ---- prims (specs = kernel reduce.shard / host prim.rs) ------------------ */
static inline value_t rt_add(value_t a, value_t b) {
  int64_t r;
  if (__builtin_add_overflow(rt_untag(a), rt_untag(b), &r)) rt_trap("add overflow");
  return rt_tag_int(r);
}
static inline value_t rt_sub(value_t a, value_t b) {
  int64_t r;
  if (__builtin_sub_overflow(rt_untag(a), rt_untag(b), &r)) rt_trap("sub overflow");
  return rt_tag_int(r);
}
static inline value_t rt_mul(value_t a, value_t b) {
  int64_t r;
  if (__builtin_mul_overflow(rt_untag(a), rt_untag(b), &r)) rt_trap("mul overflow");
  return rt_tag_int(r);
}
/* `/` truncates toward zero; `mod` is the EUCLIDEAN remainder (always >= 0);
 * `tmod` is the truncating remainder; `ediv` the Euclidean division. */
static inline value_t rt_div(value_t a, value_t b) {
  int64_t d = rt_untag(b);
  if (d == 0) rt_trap("div by zero (stuck on the interpreted engines)");
  return rt_tag_int(rt_untag(a) / d);
}
static inline value_t rt_mod(value_t a, value_t b) {
  int64_t n = rt_untag(a), d = rt_untag(b);
  if (d == 0) rt_trap("mod by zero (stuck on the interpreted engines)");
  int64_t r = n % d;
  if (r < 0) r += (d < 0 ? -d : d);
  return rt_tag_int(r);
}
static inline value_t rt_tmod(value_t a, value_t b) {
  int64_t d = rt_untag(b);
  if (d == 0) rt_trap("tmod by zero");
  return rt_tag_int(rt_untag(a) % d);
}
static inline value_t rt_ediv(value_t a, value_t b) {
  int64_t n = rt_untag(a), d = rt_untag(b);
  if (d == 0) rt_trap("ediv by zero");
  int64_t q = n / d, r = n % d;
  if (r < 0) q -= (d > 0 ? 1 : -1);
  return rt_tag_int(q);
}
static inline value_t rt_lt(value_t a, value_t b) { return rt_bool(rt_untag(a) < rt_untag(b)); }
static inline value_t rt_le(value_t a, value_t b) { return rt_bool(rt_untag(a) <= rt_untag(b)); }
static inline value_t rt_int_eq(value_t a, value_t b) { return rt_bool(rt_untag(a) == rt_untag(b)); }
static inline value_t rt_sym_eq(value_t a, value_t b) { return rt_bool(rt_symid(a) == rt_symid(b)); }
static inline value_t rt_band(value_t a, value_t b) { return rt_tag_int(rt_untag(a) & rt_untag(b)); }
static inline value_t rt_bor(value_t a, value_t b) { return rt_tag_int(rt_untag(a) | rt_untag(b)); }
static inline value_t rt_bxor(value_t a, value_t b) { return rt_tag_int(rt_untag(a) ^ rt_untag(b)); }
static inline value_t rt_bshl(value_t a, value_t b) {
  int64_t s = rt_untag(b);
  if (s < 0 || s >= 64) rt_trap("bshl shift out of range (stuck on engines)");
  int64_t r = rt_untag(a);
  if (s >= 2 && (r > (RT_INT_MAX >> s) || r < (RT_INT_MIN >> s))) rt_trap("bshl overflow");
  return rt_tag_int(r << s);
}
static inline value_t rt_bshr(value_t a, value_t b) {
  int64_t s = rt_untag(b);
  if (s < 0 || s >= 64) rt_trap("bshr shift out of range (stuck on engines)");
  return rt_tag_int(rt_untag(a) >> s);
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
    buf[i++] = (char)(rt_untag(rt_field(p, 0)) & 0xFF);
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
  rt_gc_trigger = rt_heap + RT_GC_SPACING;
  rt_true_  = rt_alloc(RT_TAG_True, 0);
  rt_false_ = rt_alloc(RT_TAG_False, 0);
  rt_nil_   = rt_alloc(RT_TAG_Nil, 0);
  rt_none_  = rt_alloc(RT_TAG_None, 0);
  rt_unit_world = rt_alloc(RT_TAG_World, 0);
  rt_gc_on = 1;   /* singletons allocated; collector live from here */
}
#endif
