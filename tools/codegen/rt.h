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
 * Bump allocator over a big MAP_NORESERVE region; NEVER frees (batch
 * processes; the box has 125 GB).
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

static char *rt_heap, *rt_heap_end;
static inline rt_cell *rt_cellof(value_t v) { return (rt_cell *)v; }
static inline value_t rt_alloc(uint32_t tag, uint32_t arity) {
  size_t sz = (sizeof(rt_cell) + arity * sizeof(value_t) + 15) & ~15ul;
  if (rt_heap + sz > rt_heap_end) rt_trap("heap exhausted");
  rt_cell *c = (rt_cell *)rt_heap;
  rt_heap += sz;
  c->tag = tag; c->arity = arity;
  return (value_t)c;
}
static inline uint32_t rt_tag(value_t v) {
  return rt_is_int(v) ? 0xfffffffeu : rt_cellof(v)->tag;
}
static inline value_t rt_field(value_t v, uint32_t i) {
  return rt_cellof(v)->fields[i];
}

/* preallocated nullary cells, filled in rt_init from generated tag macros */
static value_t rt_true_, rt_false_, rt_nil_, rt_none_, rt_unit_world;

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

/* Strings are (List Int) of Unicode CODEPOINTS, not bytes — the contract set
   by the Rust engine (eval.rs str_bytes/decode_str): read decodes UTF-8,
   write re-encodes. Truncating to bytes here mangles any non-ASCII char. */
static int rt_utf8_decode(const unsigned char *s, size_t n, size_t *i, uint32_t *cp) {
  /* strict decoder, rejects overlong/surrogate/out-of-range; 0 = malformed */
  unsigned char b = s[*i]; uint32_t c; int len;
  if (b < 0x80) { c = b; len = 1; }
  else if ((b & 0xE0) == 0xC0) { c = b & 0x1F; len = 2; }
  else if ((b & 0xF0) == 0xE0) { c = b & 0x0F; len = 3; }
  else if ((b & 0xF8) == 0xF0) { c = b & 0x07; len = 4; }
  else return 0;
  if (*i + (size_t)len > n) return 0;
  for (int k = 1; k < len; k++) {
    unsigned char cb = s[*i + (size_t)k];
    if ((cb & 0xC0) != 0x80) return 0;
    c = (c << 6) | (uint32_t)(cb & 0x3F);
  }
  if (len == 2 && c < 0x80) return 0;
  if (len == 3 && c < 0x800) return 0;
  if (len == 4 && c < 0x10000) return 0;
  if (c >= 0xD800 && c <= 0xDFFF) return 0;
  if (c > 0x10FFFF) return 0;
  *i += (size_t)len; *cp = c; return 1;
}
static size_t rt_utf8_encode(uint32_t c, char *out) {
  if (c < 0x80) { out[0] = (char)c; return 1; }
  if (c < 0x800) {
    out[0] = (char)(0xC0 | (c >> 6)); out[1] = (char)(0x80 | (c & 0x3F)); return 2;
  }
  if (c < 0x10000) {
    if (c >= 0xD800 && c <= 0xDFFF) return 0;
    out[0] = (char)(0xE0 | (c >> 12)); out[1] = (char)(0x80 | ((c >> 6) & 0x3F));
    out[2] = (char)(0x80 | (c & 0x3F)); return 3;
  }
  if (c <= 0x10FFFF) {
    out[0] = (char)(0xF0 | (c >> 18)); out[1] = (char)(0x80 | ((c >> 12) & 0x3F));
    out[2] = (char)(0x80 | ((c >> 6) & 0x3F)); out[3] = (char)(0x80 | (c & 0x3F)); return 4;
  }
  return 0; /* invalid codepoint: skipped, like decode_str's from_u32 filter */
}
static value_t rt_str_list(const char *s) {
  /* argv -> codepoints; a malformed byte passes through raw (argv is ASCII
     in practice; the Rust engine would have refused the process args). */
  size_t len = strlen(s); const unsigned char *us = (const unsigned char *)s;
  /* collect codepoints forward, then build the list backward */
  uint32_t *cps = malloc(len * sizeof(uint32_t)); size_t ncp = 0, i = 0;
  while (i < len) {
    uint32_t cp;
    if (rt_utf8_decode(us, len, &i, &cp)) cps[ncp++] = cp;
    else cps[ncp++] = us[i++];
  }
  value_t acc = rt_nil_;
  for (size_t k = ncp; k > 0; k--) acc = rt_cons(rt_tag_int(cps[k-1]), acc);
  free(cps);
  return acc;
}
static char *rt_cstr(value_t l, size_t *lenp) {
  size_t n = 0;
  for (value_t p = l; rt_tag(p) == RT_TAG_Cons; p = rt_field(p, 1)) n++;
  char *buf = malloc(4 * n + 1); size_t i = 0;
  for (value_t p = l; rt_tag(p) == RT_TAG_Cons; p = rt_field(p, 1))
    i += rt_utf8_encode((uint32_t)rt_untag(rt_field(p, 0)), buf + i);
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
  char *buf = malloc((size_t)sz);
  if (sz > 0 && fread(buf, 1, (size_t)sz, f) != (size_t)sz) { fclose(f); free(buf); return rt_pair(rt_none_, w); }
  fclose(f);
  /* decode UTF-8 -> codepoints; malformed input is None, mirroring the Rust
     engine's read_to_string (which errors on invalid UTF-8). */
  uint32_t *cps = malloc((size_t)sz * sizeof(uint32_t)); size_t ncp = 0, i = 0;
  while (i < (size_t)sz) {
    uint32_t cp;
    if (!rt_utf8_decode((const unsigned char *)buf, (size_t)sz, &i, &cp)) {
      free(buf); free(cps); return rt_pair(rt_none_, w);
    }
    cps[ncp++] = cp;
  }
  free(buf);
  value_t acc = rt_nil_;
  for (size_t k = ncp; k > 0; k--) acc = rt_cons(rt_tag_int(cps[k-1]), acc);
  free(cps);
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
static void rt_init(int argc, char **argv) {
  rt_argc = argc; rt_argv = argv;
  struct rlimit rl = { 2ul << 30, 2ul << 30 };
  setrlimit(RLIMIT_STACK, &rl);  /* best effort; deep non-self recursion */
  size_t cap = 64ul << 30;       /* 64 GB reserve, NORESERVE — pay as used */
  rt_heap = mmap(0, cap, PROT_READ | PROT_WRITE,
                 MAP_PRIVATE | MAP_ANONYMOUS | MAP_NORESERVE, -1, 0);
  if (rt_heap == MAP_FAILED) rt_trap("mmap failed");
  rt_heap_end = rt_heap + cap;
  rt_true_  = rt_alloc(RT_TAG_True, 0);
  rt_false_ = rt_alloc(RT_TAG_False, 0);
  rt_nil_   = rt_alloc(RT_TAG_Nil, 0);
  rt_none_  = rt_alloc(RT_TAG_None, 0);
  rt_unit_world = rt_alloc(RT_TAG_World, 0);
}
#endif
