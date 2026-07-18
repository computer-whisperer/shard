/* models/x86/diff/x86_diff.c — engine side of the x86_64 silicon differential
 * (see x86_diff_run.shard for the plan format). The "engine" is the CPU:
 * each XMOD's bytes are mapped into an executable page and CALLED on real
 * hardware; each XCASE/XMEMCASE replays the model's vector and compares the
 * hardware result (and memory) against the model's expectation. Any
 * mismatch is a FAIL; exit code = number of failing lines (0 = agreement).
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

#define DATA_BASE 0x40000000UL
#define DATA_SIZE 0x10000UL
#define MAXMOD 64

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

struct mod { char name[32]; void *code; };
static struct mod mods[MAXMOD];
static int nmods;

static void *find_mod(const char *name) {
  for (int i = 0; i < nmods; i++)
    if (!strcmp(mods[i].name, name)) return mods[i].code;
  return NULL;
}

static int ok = 0, fail = 0;
static void report(int good, const char *line, const char *detail) {
  if (good) ok++;
  else { fail++; printf("FAIL %s  [%s]\n", line, detail); }
}

int main(int argc, char **argv) {
  if (argc < 2) { fprintf(stderr, "usage: %s plan.txt\n", argv[0]); return 2; }
  FILE *f = fopen(argv[1], "r");
  if (!f) { perror("fopen"); return 2; }

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
    }
  }
  fclose(f);
  printf("x86 silicon differential: %d agree, %d disagree\n", ok, fail);
  return fail;
}
