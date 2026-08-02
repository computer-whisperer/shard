/* models/x86/diff/x86_diff.c — engine side of the x86_64 silicon differential
 * (see x86_diff_run.shard for the plan format). The "engine" is the CPU:
 * each XMOD's bytes are mapped into an executable page and CALLED on real
 * hardware; each XCASE/XMEMCASE/XVCASE replays the model's vector and compares
 * the hardware result (registers, memory, and for XVCASE the whole XMM file)
 * against the model's expectation. Any mismatch is a FAIL; exit code = number
 * of failing lines (0 = agreement).
 *
 * Dev-side only — this exercises the "the CPU conforms to the model" trust
 * leaf; nothing here is in-logic. A model None (trap) is scored as agreement
 * exactly when the hardware faults (SIGSEGV/SIGILL/SIGFPE) — the trap leg.
 *
 * This is the standard compiler-conformance / JIT differential-testing
 * technique (cf. LLVM's JIT tests, QEMU TCG, V8's test suite): the only bytes
 * ever mapped executable are the shard MODEL's own emitted output for the
 * closed set of arithmetic/loop pieces in x86_diff_run.shard — deterministic,
 * compiler-generated, never external, network, or otherwise untrusted input.
 * The executable page's contents come from one source (the encoder under
 * test) and are thrown away after each run.
 *
 * Data memory: the model uses REAL absolute addresses (DATA_BASE); we
 * MAP_FIXED a page there so model-address == silicon-pointer. */
#define _GNU_SOURCE
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>
#include <setjmp.h>
#include <signal.h>
#include <sys/mman.h>
#include <cpuid.h>

#define DATA_BASE 0x40000000UL
#define DATA_SIZE 0x10000UL
#define MAXMOD 512

static uint8_t *data_page;

static sigjmp_buf fault_env;
static volatile sig_atomic_t faulted;
static void on_fault(int sig) { (void)sig; faulted = 1; siglongjmp(fault_env, 1); }

/* SysV trampoline: establish the model's entry state (args in
 * rdi/rsi/rdx/rcx, rax zeroed — a piece that never writes rax returns 0,
 * matching xregs0) and call the code. Non-arg scratch registers are
 * clobbered; a real emitter materializes its result so their entry values
 * never matter. Case kinds with fewer meaningful args pass 0 for the rest
 * (an extra register argument is ABI-harmless). */
static uint64_t call_code(void *code, uint64_t a0, uint64_t a1, uint64_t a2,
                          uint64_t a3) {
  uint64_t r;
  __asm__ volatile("xor %%eax, %%eax\n\t call *%[cd]"
                   : "=a"(r), "+c"(a3)
                   : [cd] "r"(code), "D"(a0), "S"(a1), "d"(a2)
                   : "r8", "r9", "r10", "r11", "cc", "memory");
  return r;
}

/* --- the XMM leg (docs/STREAM.md §8, B5/E1: the vector tier) ----------------
 *
 * A vector row must establish the FULL architectural XMM file before the call
 * and read all sixteen registers back after, so the comparison catches a wrong
 * REX.R (an instruction that writes xmm9 where the model wrote xmm1) and not
 * only a wrong value. C cannot express that: the compiler owns the xmm
 * registers around any call it generates, so the load, the call and the store
 * must be ONE assembly sequence. xv_trampoline is that sequence.
 *
 * The model's register file is all fifteen GPRs, so the callee may clobber
 * every one of them; only rsp survives (the encoder owns the control stack, so
 * rsp is never a model register). The xin/xout pointers therefore ride on the
 * STACK across the call, not in a register. Callee-saved registers are spilled
 * because the model code treats them as scratch.
 *
 *   uint64_t xv_trampoline(void *code, uint64_t a0, uint64_t a1, uint64_t a2,
 *                          const void *xin, void *xout);
 *
 * xin/xout are 256 bytes: sixteen registers, each in movdqu byte order (lane 0
 * first, little-endian within a lane) — exactly what the model's plan side
 * writes and reads. */
extern uint64_t xv_trampoline(void *code, uint64_t a0, uint64_t a1, uint64_t a2,
                              const void *xin, void *xout);

__asm__(".text\n"
        ".globl xv_trampoline\n"
        ".type  xv_trampoline,@function\n"
        "xv_trampoline:\n"
        "  push %rbp\n"
        "  push %rbx\n"
        "  push %r12\n"
        "  push %r13\n"
        "  push %r14\n"
        "  push %r15\n"
        "  push %r9\n"  /* xout -> 16(%rsp) after the sub below */
        "  push %rdi\n" /* code -> 8(%rsp) */
        "  sub  $8, %rsp\n" /* rsp %% 16 == 0 at the call (SysV) */
        "  movdqu   0(%r8), %xmm0\n"
        "  movdqu  16(%r8), %xmm1\n"
        "  movdqu  32(%r8), %xmm2\n"
        "  movdqu  48(%r8), %xmm3\n"
        "  movdqu  64(%r8), %xmm4\n"
        "  movdqu  80(%r8), %xmm5\n"
        "  movdqu  96(%r8), %xmm6\n"
        "  movdqu 112(%r8), %xmm7\n"
        "  movdqu 128(%r8), %xmm8\n"
        "  movdqu 144(%r8), %xmm9\n"
        "  movdqu 160(%r8), %xmm10\n"
        "  movdqu 176(%r8), %xmm11\n"
        "  movdqu 192(%r8), %xmm12\n"
        "  movdqu 208(%r8), %xmm13\n"
        "  movdqu 224(%r8), %xmm14\n"
        "  movdqu 240(%r8), %xmm15\n"
        "  mov  %rsi, %rdi\n" /* the model's arg registers: rdi/rsi/rdx */
        "  mov  %rdx, %rsi\n"
        "  mov  %rcx, %rdx\n"
        "  xor  %ecx, %ecx\n"
        "  xor  %eax, %eax\n" /* a piece that never writes rax returns 0 */
        "  call *8(%rsp)\n"
        "  mov  16(%rsp), %rcx\n"
        "  movdqu %xmm0,    0(%rcx)\n"
        "  movdqu %xmm1,   16(%rcx)\n"
        "  movdqu %xmm2,   32(%rcx)\n"
        "  movdqu %xmm3,   48(%rcx)\n"
        "  movdqu %xmm4,   64(%rcx)\n"
        "  movdqu %xmm5,   80(%rcx)\n"
        "  movdqu %xmm6,   96(%rcx)\n"
        "  movdqu %xmm7,  112(%rcx)\n"
        "  movdqu %xmm8,  128(%rcx)\n"
        "  movdqu %xmm9,  144(%rcx)\n"
        "  movdqu %xmm10, 160(%rcx)\n"
        "  movdqu %xmm11, 176(%rcx)\n"
        "  movdqu %xmm12, 192(%rcx)\n"
        "  movdqu %xmm13, 208(%rcx)\n"
        "  movdqu %xmm14, 224(%rcx)\n"
        "  movdqu %xmm15, 240(%rcx)\n"
        "  add  $8, %rsp\n"
        "  pop  %rdi\n"
        "  pop  %rcx\n"
        "  pop  %r15\n"
        "  pop  %r14\n"
        "  pop  %r13\n"
        "  pop  %r12\n"
        "  pop  %rbx\n"
        "  pop  %rbp\n"
        "  ret\n"
        ".size xv_trampoline,.-xv_trampoline\n");

static uint8_t xin_buf[256] __attribute__((aligned(16)));
static uint8_t xout_buf[256] __attribute__((aligned(16)));

/* even-length lowercase hex -> bytes; 0 = malformed or too long */
static int hex_bytes(const char *h, uint8_t *out, size_t maxn, size_t *n) {
  size_t l = strlen(h);
  if (l % 2 || l / 2 > maxn) return 0;
  for (size_t i = 0; i < l / 2; i++) {
    unsigned b;
    if (sscanf(h + 2 * i, "%2x", &b) != 1) return 0;
    out[i] = (uint8_t)b;
  }
  *n = l / 2;
  return 1;
}

/* name the FIRST disagreeing xmm register — the useful half of a 512-char
 * mismatch (both files are 32 hex chars per register, xmm0 first) */
static void xmm_diff(const char *got, const char *want, char *out, size_t n) {
  for (int r = 0; r < 16; r++)
    if (memcmp(got + 32 * r, want + 32 * r, 32)) {
      snprintf(out, n, "xmm%d got %.32s want %.32s", r, got + 32 * r,
               want + 32 * r);
      return;
    }
  snprintf(out, n, "xmm file agrees");
}

/* 16-char little-endian hex -> u64 (byte 0 = least significant) */
static uint64_t parse_le_u64(const char *h) {
  uint64_t v = 0;
  for (int i = 0; i < 8; i++) {
    unsigned b;
    sscanf(h + 2 * i, "%2x", &b);
    v |= (uint64_t)b << (8 * i);
  }
  return v;
}
static void fmt_le_u64(uint64_t v, char *out) {
  for (int i = 0; i < 8; i++)
    sprintf(out + 2 * i, "%02x", (unsigned)((v >> (8 * i)) & 0xFF));
}

struct mod { char name[32]; void *code; int has_sha; };
static struct mod mods[MAXMOD];
static int nmods;

static void *find_mod(const char *name) {
  for (int i = 0; i < nmods; i++)
    if (!strcmp(mods[i].name, name)) return mods[i].code;
  return NULL;
}

static int mod_has_sha(const char *name) {
  for (int i = 0; i < nmods; i++)
    if (!strcmp(mods[i].name, name)) return mods[i].has_sha;
  return 0;
}

/* does the byte buffer contain a SHA-NI instruction? The three species are
 * NP 0F 38 CB/CC/CD /r (sha256rnds2/msg1/msg2 — no mandatory prefix byte).
 * A false positive (the pattern inside an immediate) only skips a row
 * conservatively; the modules here are deterministic compiler output. */
static int bytes_have_sha(const uint8_t *b, size_t n) {
  for (size_t i = 0; i + 2 < n; i++)
    if (b[i] == 0x0F && b[i + 1] == 0x38 &&
        (b[i + 2] == 0xCB || b[i + 2] == 0xCC || b[i + 2] == 0xCD))
      return 1;
  return 0;
}

/* SHA-NI on this CPU (CPUID leaf 7 subleaf 0, EBX bit 29)? Rows whose bytes
 * include a SHA instruction SKIP (not FAIL) when it's absent — CI runners
 * without the extension SIGILL on them, which is a machine limitation, not a
 * model disagreement. X86_DIFF_FORCE_NO_SHA=1 forces the skip path so it is
 * testable on SHA-capable silicon. */
static int sha_ok;

static int ok = 0, fail = 0, vrow = 0, skipped = 0;
static void report(int good, const char *line, const char *detail) {
  if (good) ok++;
  else { fail++; printf("FAIL %s  [%s]\n", line, detail); }
}

int main(int argc, char **argv) {
  if (argc < 2) { fprintf(stderr, "usage: %s plan.txt\n", argv[0]); return 2; }
  FILE *f = fopen(argv[1], "r");
  if (!f) { perror("fopen"); return 2; }

  unsigned ceax, cebx, cecx, cedx;
  sha_ok = __get_cpuid_count(7, 0, &ceax, &cebx, &cecx, &cedx) &&
           ((cebx >> 29) & 1);
  const char *force = getenv("X86_DIFF_FORCE_NO_SHA");
  if (force && !strcmp(force, "1")) sha_ok = 0;

  /* the data page at the model's absolute base */
  data_page = mmap((void *)DATA_BASE, DATA_SIZE, PROT_READ | PROT_WRITE,
                   MAP_PRIVATE | MAP_ANONYMOUS | MAP_FIXED, -1, 0);
  if (data_page == MAP_FAILED || (uintptr_t)data_page != DATA_BASE) {
    perror("mmap data page at DATA_BASE");
    return 2;
  }

  struct sigaction sa = {0};
  sa.sa_handler = on_fault;
  sigemptyset(&sa.sa_mask);
  sigaction(SIGSEGV, &sa, NULL);
  sigaction(SIGILL, &sa, NULL);
  sigaction(SIGFPE, &sa, NULL);
  sigaction(SIGBUS, &sa, NULL);

  char line[65536];
  while (fgets(line, sizeof line, f)) {
    line[strcspn(line, "\n")] = 0;
    if (!strncmp(line, "XMOD ", 5)) {
      char name[32], hex[32768];
      if (sscanf(line + 5, "%31s %32767s", name, hex) != 2) continue;
      size_t n = strlen(hex) / 2;
      uint8_t *page = mmap(NULL, 4096, PROT_READ | PROT_WRITE,
                           MAP_PRIVATE | MAP_ANONYMOUS, -1, 0);
      if (page == MAP_FAILED) { perror("mmap code"); return 2; }
      for (size_t i = 0; i < n; i++) {
        unsigned b;
        sscanf(hex + 2 * i, "%2x", &b);
        page[i] = (uint8_t)b;
      }
      if (mprotect(page, 4096, PROT_READ | PROT_EXEC)) { perror("mprotect"); return 2; }
      if (nmods < MAXMOD) {
        strncpy(mods[nmods].name, name, 31);
        mods[nmods].code = page;
        mods[nmods].has_sha = bytes_have_sha(page, n);
        nmods++;
      }
    } else if (!strncmp(line, "XCASE ", 6)) {
      char name[32], a0[24], a1[24], a2[24], a3[24], exp[24];
      int ac;
      if (sscanf(line + 6, "%31s %d %23s %23s %23s %23s -> %23s",
                 name, &ac, a0, a1, a2, a3, exp) != 7) {
        report(0, line, "unparseable");
        continue;
      }
      void *code = find_mod(name);
      if (!code) { report(0, line, "module unavailable"); continue; }
      uint64_t got;
      char gothex[17];
      faulted = 0;
      if (sigsetjmp(fault_env, 1) == 0) {
        got = call_code(code, parse_le_u64(a0), parse_le_u64(a1),
                        parse_le_u64(a2), parse_le_u64(a3));
        fmt_le_u64(got, gothex);
        report(!strcmp(gothex, exp), line, gothex);
      } else {
        report(!strcmp(exp, "None"), line, "hardware faulted");
      }
    } else if (!strncmp(line, "XMEMCASE ", 9)) {
      char name[32], a0[24], a1[24], a2[24], addr[24], len[24], exp[24], outhex[4096];
      int ac;
      if (sscanf(line + 9,
                 "%31s %d %23s %23s %23s READ %23s %23s -> %23s %4095s",
                 name, &ac, a0, a1, a2, addr, len, exp, outhex) != 9) {
        report(0, line, "unparseable");
        continue;
      }
      void *code = find_mod(name);
      if (!code) { report(0, line, "module unavailable"); continue; }
      memset(data_page, 0, DATA_SIZE);
      uint64_t rd = parse_le_u64(addr), ln = parse_le_u64(len);
      uint64_t got;
      char gothex[17];
      faulted = 0;
      if (sigsetjmp(fault_env, 1) == 0) {
        got = call_code(code, parse_le_u64(a0), parse_le_u64(a1),
                        parse_le_u64(a2), 0);
        fmt_le_u64(got, gothex);
        /* read back ln bytes at rd (an absolute address inside data_page) */
        char rb[4096];
        int p = 0;
        for (uint64_t i = 0; i < ln && p < 4000; i++)
          p += sprintf(rb + p, "%02x", ((uint8_t *)(uintptr_t)rd)[i]);
        if (ln == 0) strcpy(rb, "");
        char detail[8300];
        snprintf(detail, sizeof detail, "%s %s", gothex, rb);
        report(!strcmp(gothex, exp) && !strcmp(rb, outhex), line, detail);
      } else {
        report(!strcmp(exp, "None"), line, "hardware faulted");
      }
    } else if (!strncmp(line, "XSEEDCASE ", 10)) {
      char name[32], a0[24], a1[24], a2[24], seedaddr[24], seedhex[4096];
      char addr[24], len[24], exp[24], outhex[4096];
      int ac;
      if (sscanf(line + 10,
                 "%31s %d %23s %23s %23s SEED %23s %4095s READ %23s %23s -> %23s %4095s",
                 name, &ac, a0, a1, a2, seedaddr, seedhex, addr, len, exp, outhex) != 11) {
        report(0, line, "unparseable");
        continue;
      }
      void *code = find_mod(name);
      if (!code) { report(0, line, "module unavailable"); continue; }
      memset(data_page, 0, DATA_SIZE);
      /* seed the data page BEFORE the call ("-" = empty seed, zero bytes) */
      uint64_t sd = parse_le_u64(seedaddr);
      size_t seedlen = strcmp(seedhex, "-") ? strlen(seedhex) / 2 : 0;
      if (sd < DATA_BASE || sd + seedlen > DATA_BASE + DATA_SIZE) {
        report(0, line, "seed out of range");
        continue;
      }
      for (size_t i = 0; i < seedlen; i++) {
        unsigned b;
        sscanf(seedhex + 2 * i, "%2x", &b);
        ((uint8_t *)(uintptr_t)sd)[i] = (uint8_t)b;
      }
      uint64_t rd = parse_le_u64(addr), ln = parse_le_u64(len);
      uint64_t got;
      char gothex[17];
      faulted = 0;
      if (sigsetjmp(fault_env, 1) == 0) {
        got = call_code(code, parse_le_u64(a0), parse_le_u64(a1),
                        parse_le_u64(a2), 0);
        fmt_le_u64(got, gothex);
        /* read back ln bytes at rd (an absolute address inside data_page) */
        char rb[4096];
        int p = 0;
        for (uint64_t i = 0; i < ln && p < 4000; i++)
          p += sprintf(rb + p, "%02x", ((uint8_t *)(uintptr_t)rd)[i]);
        if (ln == 0) strcpy(rb, "");
        char detail[8300];
        snprintf(detail, sizeof detail, "%s %s", gothex, rb);
        report(!strcmp(gothex, exp) && !strcmp(rb, outhex), line, detail);
      } else {
        report(!strcmp(exp, "None"), line, "hardware faulted");
      }
    } else if (!strncmp(line, "XVCASE ", 7)) {
      /* the vector row (docs/STREAM.md §8): GPR args + the whole XMM file in,
       * the whole XMM file (plus rax and an optional memory window) out. The
       * model's expectation comes from the EXTENDED tier (xvrun_regs) — the
       * scalar evaluator traps on every vector species by design. */
      char name[32], a0[24], a1[24], a2[24], xinhex[544], seedaddr[24];
      char seedhex[4096], addr[24], len[24], exp[24], xexp[544], outhex[4096];
      int ac;
      if (sscanf(line + 7,
                 "%31s %d %23s %23s %23s XIN %543s SEED %23s %4095s READ %23s "
                 "%23s -> %23s %543s %4095s",
                 name, &ac, a0, a1, a2, xinhex, seedaddr, seedhex, addr, len,
                 exp, xexp, outhex) != 13) {
        report(0, line, "unparseable");
        continue;
      }
      vrow++;
      char tag[96];
      snprintf(tag, sizeof tag, "XVCASE %s (row %d)", name, vrow);
      if (!sha_ok && mod_has_sha(name)) {
        skipped++;
        printf("SKIP %s [no sha_ni on this cpu]\n", tag);
        continue;
      }
      void *code = find_mod(name);
      if (!code) { report(0, tag, "module unavailable"); continue; }
      memset(data_page, 0, DATA_SIZE);
      if (strcmp(seedhex, "-")) {
        uint8_t sb[4096];
        size_t sl;
        uint64_t sd = parse_le_u64(seedaddr);
        if (!hex_bytes(seedhex, sb, sizeof sb, &sl)) {
          report(0, tag, "bad seed hex");
          continue;
        }
        if (sd < DATA_BASE || sd + sl > DATA_BASE + DATA_SIZE) {
          report(0, tag, "seed out of range");
          continue;
        }
        memcpy((void *)(uintptr_t)sd, sb, sl);
      }
      size_t xn;
      if (!hex_bytes(xinhex, xin_buf, sizeof xin_buf, &xn) || xn != 256) {
        report(0, tag, "XIN must be 16 registers (512 hex chars)");
        continue;
      }
      memset(xout_buf, 0, sizeof xout_buf);
      uint64_t rd = parse_le_u64(addr), ln = parse_le_u64(len);
      char gothex[17], xgot[513];
      faulted = 0;
      if (sigsetjmp(fault_env, 1) == 0) {
        uint64_t got = xv_trampoline(code, parse_le_u64(a0), parse_le_u64(a1),
                                     parse_le_u64(a2), xin_buf, xout_buf);
        fmt_le_u64(got, gothex);
        for (int i = 0; i < 256; i++)
          sprintf(xgot + 2 * i, "%02x", (unsigned)xout_buf[i]);
        char rb[4096];
        int p = 0;
        for (uint64_t i = 0; i < ln && p < 4000; i++)
          p += sprintf(rb + p, "%02x", ((uint8_t *)(uintptr_t)rd)[i]);
        if (ln == 0) strcpy(rb, "-");
        int good = !strcmp(gothex, exp) && !strcmp(xgot, xexp) &&
                   !strcmp(rb, outhex);
        char detail[10240];
        if (good) snprintf(detail, sizeof detail, "agree");
        else {
          char xd[160];
          xmm_diff(xgot, xexp, xd, sizeof xd);
          snprintf(detail, sizeof detail,
                   "rax %s want %s | %s | mem %s want %s | XIN %s", gothex, exp,
                   xd, rb, outhex, xinhex);
        }
        report(good, tag, detail);
      } else {
        report(!strcmp(exp, "None"), tag, "hardware faulted");
      }
    }
  }
  fclose(f);
  if (skipped)
    printf("x86 silicon differential: %d agree, %d disagree, %d skipped (no sha_ni)\n",
           ok, fail, skipped);
  else
    printf("x86 silicon differential: %d agree, %d disagree\n", ok, fail);
  return fail;
}
