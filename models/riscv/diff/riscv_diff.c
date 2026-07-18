/* models/riscv/diff/riscv_diff.c — engine side of the RISC-V engine differential
 * (see riscv_diff_run.shard for the plan format). The "engine" is a RISC-V
 * core emulated by qemu-user: each RVMOD's bytes are mapped into an
 * executable page and CALLED; each RVCASE/RVMEMCASE replays the model's
 * vector and compares the core's result (and memory) against the model's.
 * Any mismatch is a FAIL; exit code = number of failing lines (0 = agreement).
 *
 * Dev-side only — this exercises the "the core conforms to the model" trust
 * leaf; nothing here is in-logic. A model None (trap) is scored as agreement
 * exactly when the core faults (SIGSEGV/SIGILL/SIGBUS) — the trap leg. Unlike
 * x86 there is no native silicon leg on this box, so qemu-user plays V8's role
 * from the wasm arc (docs/RISCV.md §7.6).
 *
 * Freestanding: no libc/gcc/binutils for RISC-V exists on this box, so this is
 * ONE source compiled for both widths (clang --target=riscv{32,64}, -nostdlib,
 * rust-lld) with its own _start, raw Linux syscalls (asm-generic numbers,
 * shared by rv32/rv64), a naked-asm SysV trampoline, and a hand-written
 * setjmp/longjmp for the SIGSEGV trap leg. The only bytes ever mapped
 * executable are the shard MODEL's own emitted output for the closed set of
 * pieces in riscv_diff_run.shard — deterministic, never external input.
 *
 * Usage: riscv_diff <plan.txt> <width>     width = "32" | "64"
 *   processes only the plan lines tagged with <width> (a module's bytes are
 *   byte-identical across widths for everything encoded, but the SEMANTICS
 *   differ per core, so each qemu runs its own width). */

typedef unsigned long ulong;         /* native word: 32-bit rv32, 64-bit rv64 */
typedef unsigned long long u64;      /* 64-bit on BOTH widths (wire values) */
typedef unsigned char u8;

/* ---- raw syscalls (asm-generic numbers; identical on rv32 and rv64) ---- */
static long sysc(long n, long a0, long a1, long a2, long a3, long a4, long a5) {
  register long a7 __asm__("a7") = n;
  register long r0 __asm__("a0") = a0;
  register long r1 __asm__("a1") = a1;
  register long r2 __asm__("a2") = a2;
  register long r3 __asm__("a3") = a3;
  register long r4 __asm__("a4") = a4;
  register long r5 __asm__("a5") = a5;
  __asm__ volatile("ecall"
                   : "+r"(r0)
                   : "r"(a7), "r"(r1), "r"(r2), "r"(r3), "r"(r4), "r"(r5)
                   : "memory");
  return r0;
}
#define SYS_openat 56
#define SYS_read 63
#define SYS_write 64
#define SYS_exit 93
#define SYS_rt_sigaction 134
#define SYS_mmap 222
#define AT_FDCWD -100

static void wr(const char *s, long n) { sysc(SYS_write, 1, (long)s, n, 0, 0, 0); }
static long slen(const char *s) { long n = 0; while (s[n]) n++; return n; }
static void wrs(const char *s) { wr(s, slen(s)); }

/* ---- freestanding libcalls the compiler may still emit (aggregate init,
 * struct copy) even under -fno-builtin; provide them so the link resolves. ---- */
void *memset(void *d, int c, unsigned long n) { u8 *p = d; for (unsigned long i = 0; i < n; i++) p[i] = (u8)c; return d; }
void *memcpy(void *d, const void *s, unsigned long n) { u8 *a = d; const u8 *b = s; for (unsigned long i = 0; i < n; i++) a[i] = b[i]; return d; }
unsigned long strlen(const char *s) { unsigned long n = 0; while (s[n]) n++; return n; }

/* ---- tiny mem/str helpers (no libc) ---- */
static void zero(u8 *p, long n) { for (long i = 0; i < n; i++) p[i] = 0; }
static int seq(const char *a, const char *b) {
  while (*a && *b) { if (*a != *b) return 0; a++; b++; }
  return *a == *b;
}
static void cpy(char *d, const char *s, long max) {
  long i = 0; for (; s[i] && i < max - 1; i++) d[i] = s[i]; d[i] = 0;
}

/* 16-char little-endian hex -> u64 (byte 0 = least significant) */
static int hexv(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  return -1;
}
static u64 parse_le_u64(const char *h) {
  u64 v = 0;
  for (int i = 0; i < 8; i++) {
    int hi = hexv(h[2 * i]), lo = hexv(h[2 * i + 1]);
    if (hi < 0 || lo < 0) break;
    v |= (u64)((hi << 4) | lo) << (8 * i);
  }
  return v;
}
static void fmt_le_u64(u64 v, char *out) {
  const char *d = "0123456789abcdef";
  for (int i = 0; i < 8; i++) {
    u8 b = (u8)((v >> (8 * i)) & 0xFF);
    out[2 * i] = d[b >> 4];
    out[2 * i + 1] = d[b & 0xF];
  }
  out[16] = 0;
}
static long atoi_(const char *s) { long n = 0; while (*s >= '0' && *s <= '9') { n = n * 10 + (*s - '0'); s++; } return n; }

/* ---- the SysV trampoline: load a0..a7 from an 8-word array, jalr, return a0.
 * A well-behaved C function: it saves/restores s0..s11 (and ra) so the emitted
 * code is free to clobber every caller/callee-saved GPR. sp/gp/tp are never
 * model registers, so the core preserves them. ---- */
extern ulong call_code(void *code, const ulong *args);
__asm__(
    ".text\n.globl call_code\ncall_code:\n"
#if __riscv_xlen == 64
    "  addi sp, sp, -112\n"
    "  sd ra, 104(sp)\n"
    "  sd s0,0(sp)\n  sd s1,8(sp)\n  sd s2,16(sp)\n  sd s3,24(sp)\n"
    "  sd s4,32(sp)\n  sd s5,40(sp)\n  sd s6,48(sp)\n  sd s7,56(sp)\n"
    "  sd s8,64(sp)\n  sd s9,72(sp)\n  sd s10,80(sp)\n  sd s11,88(sp)\n"
    "  mv t0, a0\n  mv t1, a1\n"
    "  ld a0,0(t1)\n  ld a1,8(t1)\n  ld a2,16(t1)\n  ld a3,24(t1)\n"
    "  ld a4,32(t1)\n  ld a5,40(t1)\n  ld a6,48(t1)\n  ld a7,56(t1)\n"
    "  jalr ra, 0(t0)\n"
    "  ld ra, 104(sp)\n"
    "  ld s0,0(sp)\n  ld s1,8(sp)\n  ld s2,16(sp)\n  ld s3,24(sp)\n"
    "  ld s4,32(sp)\n  ld s5,40(sp)\n  ld s6,48(sp)\n  ld s7,56(sp)\n"
    "  ld s8,64(sp)\n  ld s9,72(sp)\n  ld s10,80(sp)\n  ld s11,88(sp)\n"
    "  addi sp, sp, 112\n  ret\n"
#else
    "  addi sp, sp, -64\n"
    "  sw ra, 52(sp)\n"
    "  sw s0,0(sp)\n  sw s1,4(sp)\n  sw s2,8(sp)\n  sw s3,12(sp)\n"
    "  sw s4,16(sp)\n  sw s5,20(sp)\n  sw s6,24(sp)\n  sw s7,28(sp)\n"
    "  sw s8,32(sp)\n  sw s9,36(sp)\n  sw s10,40(sp)\n  sw s11,44(sp)\n"
    "  mv t0, a0\n  mv t1, a1\n"
    "  lw a0,0(t1)\n  lw a1,4(t1)\n  lw a2,8(t1)\n  lw a3,12(t1)\n"
    "  lw a4,16(t1)\n  lw a5,20(t1)\n  lw a6,24(t1)\n  lw a7,28(t1)\n"
    "  jalr ra, 0(t0)\n"
    "  lw ra, 52(sp)\n"
    "  lw s0,0(sp)\n  lw s1,4(sp)\n  lw s2,8(sp)\n  lw s3,12(sp)\n"
    "  lw s4,16(sp)\n  lw s5,20(sp)\n  lw s6,24(sp)\n  lw s7,28(sp)\n"
    "  lw s8,32(sp)\n  lw s9,36(sp)\n  lw s10,40(sp)\n  lw s11,44(sp)\n"
    "  addi sp, sp, 64\n  ret\n"
#endif
);

/* ---- setjmp/longjmp (ra, sp, s0..s11) for the SIGSEGV trap leg ---- */
typedef ulong jbuf[14];
extern int my_setjmp(jbuf);
extern void my_longjmp(jbuf, int);
__asm__(
    ".text\n.globl my_setjmp\nmy_setjmp:\n"
#if __riscv_xlen == 64
    "  sd ra,0(a0)\n  sd sp,8(a0)\n"
    "  sd s0,16(a0)\n  sd s1,24(a0)\n  sd s2,32(a0)\n  sd s3,40(a0)\n"
    "  sd s4,48(a0)\n  sd s5,56(a0)\n  sd s6,64(a0)\n  sd s7,72(a0)\n"
    "  sd s8,80(a0)\n  sd s9,88(a0)\n  sd s10,96(a0)\n  sd s11,104(a0)\n"
    "  li a0,0\n  ret\n"
    ".globl my_longjmp\nmy_longjmp:\n"
    "  ld ra,0(a0)\n  ld sp,8(a0)\n"
    "  ld s0,16(a0)\n  ld s1,24(a0)\n  ld s2,32(a0)\n  ld s3,40(a0)\n"
    "  ld s4,48(a0)\n  ld s5,56(a0)\n  ld s6,64(a0)\n  ld s7,72(a0)\n"
    "  ld s8,80(a0)\n  ld s9,88(a0)\n  ld s10,96(a0)\n  ld s11,104(a0)\n"
    "  mv a0,a1\n  bnez a0,1f\n  li a0,1\n1:\n  ret\n"
#else
    "  sw ra,0(a0)\n  sw sp,4(a0)\n"
    "  sw s0,8(a0)\n  sw s1,12(a0)\n  sw s2,16(a0)\n  sw s3,20(a0)\n"
    "  sw s4,24(a0)\n  sw s5,28(a0)\n  sw s6,32(a0)\n  sw s7,36(a0)\n"
    "  sw s8,40(a0)\n  sw s9,44(a0)\n  sw s10,48(a0)\n  sw s11,52(a0)\n"
    "  li a0,0\n  ret\n"
    ".globl my_longjmp\nmy_longjmp:\n"
    "  lw ra,0(a0)\n  lw sp,4(a0)\n"
    "  lw s0,8(a0)\n  lw s1,12(a0)\n  lw s2,16(a0)\n  lw s3,20(a0)\n"
    "  lw s4,24(a0)\n  lw s5,28(a0)\n  lw s6,32(a0)\n  lw s7,36(a0)\n"
    "  lw s8,40(a0)\n  lw s9,44(a0)\n  lw s10,48(a0)\n  lw s11,52(a0)\n"
    "  mv a0,a1\n  bnez a0,1f\n  li a0,1\n1:\n  ret\n"
#endif
);

static jbuf fault_env;
static volatile int faulted;
static void on_fault(int s) { (void)s; faulted = 1; my_longjmp(fault_env, 1); }
struct ksig { void (*h)(int); ulong flags; ulong mask; };
#define SA_NODEFER 0x40000000

/* ---- module table ---- */
#define DATA_BASE 0x40000000UL
#define DATA_SIZE 0x10000UL
#define MAXMOD 128
struct mod { char name[32]; void *code; };
static struct mod mods[MAXMOD];
static int nmods;
static void *find_mod(const char *name) {
  for (int i = 0; i < nmods; i++)
    if (seq(mods[i].name, name)) return mods[i].code;
  return 0;
}

static int okc, failc;
static void report(int good, const char *line, const char *detail) {
  if (good) { okc++; return; }
  failc++;
  wrs("FAIL "); wrs(line); wrs("  ["); wrs(detail); wrs("]\n");
}

/* split a NUL-terminated line into space tokens (in place); returns count. */
static int split(char *p, char **toks, int max) {
  int n = 0;
  while (*p && n < max) {
    while (*p == ' ') p++;
    if (!*p) break;
    toks[n++] = p;
    while (*p && *p != ' ') p++;
    if (*p) *p++ = 0;
  }
  return n;
}

static char filebuf[1 << 20];
static u8 *data_page;

static void run_case(char **t, int nt, int memc, const char *line) {
  /* layout: RVCASE  name width argc a0 a1 a2 a3 -> exp
   *         RVMEMCASE name width argc a0 a1 a2 a3 READ addr len -> exp outhex */
  const char *name = t[1];
  void *code = find_mod(name);
  if (!code) { report(0, line, "module unavailable"); return; }
  ulong args[8] = {0, 0, 0, 0, 0, 0, 0, 0};
  args[0] = (ulong)parse_le_u64(t[4]);
  args[1] = (ulong)parse_le_u64(t[5]);
  args[2] = (ulong)parse_le_u64(t[6]);
  args[3] = (ulong)parse_le_u64(t[7]);
  const char *exp, *outhex = 0;
  ulong addr = 0, len = 0;
  if (memc) {
    addr = (ulong)parse_le_u64(t[9]);
    len = (ulong)parse_le_u64(t[10]);
    exp = t[12];
    outhex = t[13];
    zero(data_page, DATA_SIZE);
  } else {
    exp = t[9];
  }
  faulted = 0;
  if (my_setjmp(fault_env) == 0) {
    u64 got = (u64)(ulong)call_code(code, args);
    char gothex[17];
    fmt_le_u64(got, gothex);
    int good = seq(gothex, exp);
    if (memc && good) {
      char rb[64];
      int p = 0;
      const char *dch = "0123456789abcdef";
      for (ulong i = 0; i < len && p < 60; i++) {
        u8 b = ((u8 *)addr)[i];
        rb[p++] = dch[b >> 4];
        rb[p++] = dch[b & 0xF];
      }
      rb[p] = 0;
      good = seq(rb, outhex);
      report(good, line, good ? gothex : rb);
    } else {
      report(good, line, gothex);
    }
  } else {
    report(seq(exp, "None"), line, "core faulted");
  }
}

int real_main(ulong *stack) {
  long argc = (long)stack[0];
  char **argv = (char **)(stack + 1);
  if (argc < 3) { wrs("usage: riscv_diff plan.txt width\n"); sysc(SYS_exit, 2, 0, 0, 0, 0, 0); }
  const char *path = argv[1];
  const char *mywidth = argv[2];

  /* data page at the model's absolute base (MAP_FIXED so model addr == ptr) */
  data_page = (u8 *)sysc(SYS_mmap, DATA_BASE, DATA_SIZE, 3 /*RW*/,
                         0x32 /*PRIVATE|ANON|FIXED*/, -1, 0);
  if ((ulong)data_page != DATA_BASE) { wrs("mmap data page FAILED\n"); sysc(SYS_exit, 2, 0, 0, 0, 0, 0); }

  /* SIGSEGV/SIGILL/SIGBUS -> longjmp back (SA_NODEFER so repeated faults work) */
  struct ksig sa; sa.h = on_fault; sa.flags = SA_NODEFER; sa.mask = 0;
  sysc(SYS_rt_sigaction, 11 /*SIGSEGV*/, (long)&sa, 0, 8, 0, 0);
  sysc(SYS_rt_sigaction, 4 /*SIGILL*/, (long)&sa, 0, 8, 0, 0);
  sysc(SYS_rt_sigaction, 7 /*SIGBUS*/, (long)&sa, 0, 8, 0, 0);

  /* slurp the plan */
  long fd = sysc(SYS_openat, AT_FDCWD, (long)path, 0, 0, 0, 0);
  if (fd < 0) { wrs("open plan FAILED\n"); sysc(SYS_exit, 2, 0, 0, 0, 0, 0); }
  long total = 0;
  for (;;) {
    long r = sysc(SYS_read, fd, (long)(filebuf + total), (long)(sizeof filebuf - 1 - total), 0, 0, 0);
    if (r <= 0) break;
    total += r;
    if (total >= (long)sizeof filebuf - 1) break;
  }
  filebuf[total] = 0;

  /* process line by line */
  char *p = filebuf;
  while (*p) {
    char *nl = p;
    while (*nl && *nl != '\n') nl++;
    int end = (*nl == '\n');
    *nl = 0;
    if (*p) {
      char *toks[16];
      char orig[512];
      cpy(orig, p, 512);           /* split() mangles p; keep a clean copy */
      int nt = split(p, toks, 16);
      if (nt >= 3 && seq(toks[2], mywidth)) {
        if (seq(toks[0], "RVMOD")) {
          /* toks: RVMOD name width hexcode */
          const char *hex = toks[3];
          long nbytes = slen(hex) / 2;
          void *page = (void *)sysc(SYS_mmap, 0, 4096, 7 /*RWX*/,
                                    0x22 /*PRIVATE|ANON*/, -1, 0);
          if ((long)page < 0 && (long)page > -4096) { wrs("mmap code FAILED\n"); sysc(SYS_exit, 2, 0, 0, 0, 0, 0); }
          u8 *cp = (u8 *)page;
          for (long i = 0; i < nbytes; i++) {
            int hi = hexv(hex[2 * i]), lo = hexv(hex[2 * i + 1]);
            cp[i] = (u8)((hi << 4) | lo);
          }
          __asm__ volatile("fence.i" ::: "memory");
          if (nmods < MAXMOD) { cpy(mods[nmods].name, toks[1], 32); mods[nmods].code = page; nmods++; }
        } else if (seq(toks[0], "RVCASE") && nt >= 10) {
          run_case(toks, nt, 0, orig);
        } else if (seq(toks[0], "RVMEMCASE") && nt >= 14) {
          run_case(toks, nt, 1, orig);
        }
      }
    }
    if (!end) break;
    p = nl + 1;
  }

  wrs("riscv "); wrs(mywidth); wrs(" differential: ");
  char nb[24]; int q = 0;
  { int v = okc; char tmp[16]; int k = 0; if (v == 0) tmp[k++] = '0'; while (v) { tmp[k++] = '0' + v % 10; v /= 10; } while (k) nb[q++] = tmp[--k]; }
  nb[q] = 0; wrs(nb); wrs(" agree, ");
  q = 0; { int v = failc; char tmp[16]; int k = 0; if (v == 0) tmp[k++] = '0'; while (v) { tmp[k++] = '0' + v % 10; v /= 10; } while (k) nb[q++] = tmp[--k]; }
  nb[q] = 0; wrs(nb); wrs(" disagree\n");
  sysc(SYS_exit, failc, 0, 0, 0, 0, 0);
  return failc;
}

__asm__(
    ".text\n.globl _start\n_start:\n"
    "  mv a0, sp\n"
    "  call real_main\n"
    "  li a7, 93\n  ecall\n");
