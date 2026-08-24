#!/usr/bin/env python3
"""gen_rth.py -- the generated blocks of rth_kit.shard (COVERAGE.md C2b).

The in-tree generator for the runtime-theorem kit's repetitive proofs
(the gen_fra/gen_fra4 contract: banner-marked blocks, `splice`
regenerates them in place, NEVER hand-patch a block; always shardfmt
the probe after a splice).

Blocks:
  init  -- rth_init_run: the engine leg of rt_init's law (19 straight
           stores at a literal fuel tower over symbolic memory; the
           rth_step1 recipe per statement).

Usage: python3 models/imp/probes/gen_rth.py [init | splice]
"""

PATH = "models/imp/probes/rth_kit.shard"

# ---------------- the shared recipe ----------------
# Claim premises, in slot order (the arith certs below index them):
#   p0 (le mlo rb)            p1 (le (+ rb 152) msz)
#   p2 (le 0 rb)              p3 (le (+ rb 152) 4294967296)
#   p4 (le 0 lo)              p5 (lt lo 18446744073709551616)
#   p6 (le 0 hi)              p7 (lt hi 18446744073709551616)
#   p8 (lt d dmax)
# Slot tables: [0] = G (negated goal), then p0..p8, then the haves in
# order of introduction.  Every arith cert is zero except G and the one
# carrying row.

NPREM = 9


def cert(width, hot=None, g=1):
    """A flat Farkas cert: G at slot 0, one hot row, zeros elsewhere."""
    row = [0] * width
    row[0] = g
    if hot is not None:
        row[hot[0]] = hot[1]
    return "(list " + " ".join(str(x) for x in row) + ")"


def addr(k):
    return "rb" if k == 0 else f"(+ rb {k})"


# engine order: rb, rb+8, rb+16, then the zero heads rb+144 down to rb+24
ADDRS = [0, 8, 16] + [24 + 8 * (k - 1) for k in range(16, 0, -1)]


def init_block():
    out = []
    w = []
    w.append(";; rt_init's engine leg: under the window, band and depth premises the")
    w.append(";; call RETURNS 0 and leaves exactly m_init's stores.  Fuel: 21 = the")
    w.append(";; ipcall peel + the 20 ipstmts levels (the last level's S serves the")
    w.append(";; final statement and the Nil arm both).")
    w.append("(claim rth_init_run")
    w.append("  (goal")
    w.append("    ((fs (List IpFn))")
    w.append("     (mlo Int)")
    w.append("     (msz Int)")
    w.append("     (dmax Int)")
    w.append("     (d Int)")
    w.append("     (rb Int)")
    w.append("     (lo Int)")
    w.append("     (hi Int)")
    w.append("     (m Mem)")
    w.append("     (fuel Nat))")
    w.append("    ((= (le mlo rb) True)")
    w.append("     (= (le (+ rb 152) msz) True)")
    w.append("     (= (le 0 rb) True)")
    w.append("     (= (le (+ rb 152) 4294967296) True)")
    w.append("     (= (le 0 lo) True)")
    w.append("     (= (lt lo 18446744073709551616) True)")
    w.append("     (= (le 0 hi) True)")
    w.append("     (= (lt hi 18446744073709551616) True)")
    w.append("     (= (lt d dmax) True))")
    w.append("    (=")
    w.append("      (ipcall")
    w.append("        (nS 21 (ntl 21 fuel))")
    w.append("        (rt_app (rt_fns rb) fs)")
    w.append("        mlo")
    w.append("        msz")
    w.append("        dmax")
    w.append("        d")
    w.append("        0")
    w.append("        (Cons lo (Cons hi Nil))")
    w.append("        m)")
    w.append("      (Some (IpRv 0 (m_init m rb lo hi)))))")
    w.append("  (chain")
    out.extend(w)

    haves = []  # names in slot order

    def slot_width():
        return 1 + NPREM + len(haves)

    def have_arith(name, goal, hot):
        # hot = (slot index, coefficient) BEFORE this have lands
        out.append(f"    (have {name} {goal}")
        out.append(f"      (by arith {cert(slot_width(), hot)}))")
        haves.append(name)

    def have_band32(name, k):
        # (band A 4294967295) = A via bandu32_id; discharges: 0 <= A (p2), A < 2^32 (p3)
        a = addr(k)
        out.append(f"    (have {name} (= (band {a} 4294967295) {a})")
        out.append("      (rewrite-with")
        out.append("        (lemma bandu32_id)")
        out.append("        lr")
        out.append("        lhs")
        out.append("        ()")
        out.append(f"        ((by arith {cert(slot_width(), (3, 1))})")
        out.append(f"         (by arith {cert(slot_width(), (4, 1))}))")
        out.append("        refl))")
        haves.append(name)

    def have_mod64(name, v, plo, phi):
        # (mod v 2^64) = v via modu64_id; discharges are direct premise rewrites
        out.append(f"    (have {name} (= (mod {v} 18446744073709551616) {v})")
        out.append("      (rewrite-with")
        out.append("        (lemma modu64_id)")
        out.append("        lr")
        out.append("        lhs")
        out.append("        ()")
        out.append(f"        ((steps ((rewrite (premise {plo}) lr lhs true ())) refl)")
        out.append(f"         (steps ((rewrite (premise {phi}) lr lhs true ())) refl))")
        out.append("        refl))")
        haves.append(name)

    # the U64 entry-band collapses for the two arguments
    have_mod64("hmlo", "lo", 4, 5)
    have_mod64("hmhi", "hi", 6, 7)

    # per statement: the band collapse and the two window guards
    for j, k in enumerate(ADDRS):
        have_band32(f"hb{j}", k)
        # (le mlo A): G + p0 (rb - mlo >= 0)
        have_arith(f"hl{j}", f"(= (le mlo {addr(k)}) True)", (1, 1))
        # (le (+ A 8) msz): G + p1 (msz - (rb+152) >= 0)
        have_arith(f"hh{j}", f"(= (le (+ {addr(k)} 8) msz) True)", (2, 1))

    # the master steps
    out.append("    (steps")
    out.append("      ((compute lhs (stop iw8))")
    out.append("       (rewrite (premise 8) lr lhs true ())")
    out.append("       (reduce lhs)")
    out.append("       (compute lhs (stop iw8))")
    for j in range(len(ADDRS)):
        out.append(f"       (rewrite (premise hb{j}) lr lhs true ())")
        out.append(f"       (rewrite (premise hl{j}) lr lhs true ())")
        out.append("       (reduce lhs)")
        out.append(f"       (rewrite (premise hh{j}) lr lhs true ())")
        out.append("       (reduce lhs)")
        out.append("       (compute lhs (stop iw8))")
    out.append("       (rewrite (premise hmlo) lr lhs true ())")
    out.append("       (rewrite (premise hmhi) lr lhs true ())")
    out.append("       (compute rhs (stop iw8)))")
    out.append("      refl)))")
    return "\n".join(out)


BANNER_INIT = ";; --- THE INIT ENGINE LEG rth_init_run (generated by gen_rth.py init — REGENERATE, never hand-patch) ---"

BLOCKS = [("init", BANNER_INIT, init_block)]


# ---------------- the minit read lemmas + hr_init ----------------

B1 = "(store_le (iw8) m rb lo)"
B2 = f"(store_le (iw8) {B1} (+ rb 8) hi)"
B3 = f"(store_le (iw8) {B2} (+ rb 16) 0)"


def LD(mem, a):
    return f"(load_le (iw8) {mem} {a})"


def minit_block():
    out = []

    def rd(name, prems, target, val, haves):
        out.append(f"(claim {name}")
        out.append("  (goal ((m Mem) (rb Int) (lo Int) (hi Int))")
        out.append("    (" + (" ".join(prems) if prems else "") + ")")
        out.append(f"    (= {LD('(m_init m rb lo hi)', target)} {val}))")
        out.append("  (chain")
        out.extend(haves)
        names = [h.split()[1] for h in haves if h.strip().startswith("(have")]
        out.append("    (steps")
        out.append("      ((unfold m_init lhs)")
        for n in names:
            out.append(f"       (rewrite (premise {n}) lr lhs true ())")
        out.append("       )")
        out.append("      refl)))")
        out.append("")

    def hv(name, lhs_mem, tgt, rhs, lemma, discharges):
        h = [f"    (have {name} (= {LD(lhs_mem, tgt)} {rhs})"]
        h.append(f"      (rewrite-with (lemma {lemma}) lr lhs ()")
        h.append("        (" + " ".join(discharges) + ")")
        h.append("        refl))")
        return h

    zh = f"(m_zh {B3} rb 16)"

    # read [rb] = lo (band premises on lo)
    haves = []
    haves += hv("hz", zh.replace(f"(m_zh {B3} rb 16)", f"(m_zh {B3} rb 16)"), "rb", LD(B3, "rb"), "m_zh_below", ["(by arith (list 1 0 0))"])
    haves += hv("h16", B3, "rb", LD(B2, "rb"), "ldw_below", ["(by arith (list 1 0 0 0))"])
    haves += hv("h8", B2, "rb", LD(B1, "rb"), "ldw_below", ["(by arith (list 1 0 0 0 0))"])
    haves += hv("h0", B1, "rb", "lo", "ls8w",
                ["(steps ((rewrite (premise 0) lr lhs true ())) refl)",
                 "(steps ((rewrite (premise 1) lr lhs true ())) refl)"])
    rd("m_init_rd_top",
       ["(= (le 0 lo) True)", "(= (lt lo 18446744073709551616) True)"],
       "rb", "lo", haves)

    # read [rb+8] = hi (band premises on hi)
    haves = []
    haves += hv("hz", zh, "(+ rb 8)", LD(B3, "(+ rb 8)"), "m_zh_below", ["(by arith (list 1 0 0))"])
    haves += hv("h16", B3, "(+ rb 8)", LD(B2, "(+ rb 8)"), "ldw_below", ["(by arith (list 1 0 0 0))"])
    haves += hv("h8", B2, "(+ rb 8)", "hi", "ls8w",
                ["(steps ((rewrite (premise 0) lr lhs true ())) refl)",
                 "(steps ((rewrite (premise 1) lr lhs true ())) refl)"])
    rd("m_init_rd_end",
       ["(= (le 0 hi) True)", "(= (lt hi 18446744073709551616) True)"],
       "(+ rb 8)", "hi", haves)

    # read [rb+16] = 0 (no premises; ground ls8w discharges)
    haves = []
    haves += hv("hz", zh, "(+ rb 16)", LD(B3, "(+ rb 16)"), "m_zh_below", ["(by arith (list 1))"])
    haves += hv("h16", B3, "(+ rb 16)", "0", "ls8w",
                ["(steps ((compute lhs)) refl)", "(steps ((compute lhs)) refl)"])
    rd("m_init_rd_dep", [], "(+ rb 16)", "0", haves)

    # the sixteen heads
    for K in range(16):
        A = 24 + 8 * K
        haves = []
        haves.append(f"    (have hq (= (+ rb {A}) (+ rb (+ 24 (* 8 {K}))))")
        haves.append("      (steps ((compute rhs)) refl))")
        haves.append(f"    (have hh (= {LD(zh, f'(+ rb (+ 24 (* 8 {K})))')} 0)")
        haves.append("      (rewrite-with (lemma m_zh_head) lr lhs ()")
        haves.append("        ((steps ((compute lhs)) refl) (steps ((compute lhs)) refl))")
        haves.append("        refl))")
        rd(f"m_init_rd_h{K}", [], f"(+ rb {A})", "0", haves)

    # hr_init: the bytes of m_init mean the empty ghost
    out.append(";; the init bytes mean the empty ghost")
    out.append("(claim hr_init")
    out.append("  (goal ((m Mem) (rb Int) (lo Int) (hi Int))")
    out.append("    ((= (le 0 lo) True)")
    out.append("     (= (lt lo 18446744073709551616) True)")
    out.append("     (= (le 0 hi) True)")
    out.append("     (= (lt hi 18446744073709551616) True))")
    out.append("    (= (heap_rep (m_init m rb lo hi) rb (gh_init lo hi)) True))")
    out.append("  (chain")
    nh = 0
    out.append(f"    (have hqlo (= (int_eq lo lo) True)")
    w = 1 + 4 + nh
    out.append(f"      (by arith (list (list {' '.join(['1'] + ['0']*(w-1))}) (list {' '.join(['1'] + ['0']*(w-1))}))))")
    nh += 1
    out.append(f"    (have hqhi (= (int_eq hi hi) True)")
    w = 1 + 4 + nh
    out.append(f"      (by arith (list (list {' '.join(['1'] + ['0']*(w-1))}) (list {' '.join(['1'] + ['0']*(w-1))}))))")
    nh += 1
    MI = "(m_init m rb lo hi)"
    out.append(f"    (have hrt (= {LD(MI, 'rb')} lo)")
    out.append("      (rewrite-with (lemma m_init_rd_top) lr lhs ()")
    out.append("        ((steps ((rewrite (premise 0) lr lhs true ())) refl)")
    out.append("         (steps ((rewrite (premise 1) lr lhs true ())) refl))")
    out.append("        refl))")
    out.append(f"    (have hre (= {LD(MI, '(+ rb 8)')} hi)")
    out.append("      (rewrite-with (lemma m_init_rd_end) lr lhs ()")
    out.append("        ((steps ((rewrite (premise 2) lr lhs true ())) refl)")
    out.append("         (steps ((rewrite (premise 3) lr lhs true ())) refl))")
    out.append("        refl))")
    out.append(f"    (have hrd (= {LD(MI, '(+ rb 16)')} 0)")
    out.append("      (rewrite-with (lemma m_init_rd_dep) lr lhs () () refl))")
    for K in range(16):
        A = 24 + 8 * K
        out.append(f"    (have hh{K} (= {LD(MI, f'(+ rb {A})')} 0)")
        out.append(f"      (rewrite-with (lemma m_init_rd_h{K}) lr lhs () () refl))")
    out.append("    (steps")
    out.append("      ((compute lhs (stop iw8 m_init))")
    out.append("       (rewrite (lemma gend_of_def) lr lhs true ())")
    out.append("       (rewrite (lemma gfree_of_def) lr lhs true ())")
    out.append("       (rewrite (lemma gcells_of_def) lr lhs true ())")
    out.append("       (rewrite (lemma graw_of_def) lr lhs true ())")
    out.append("       (rewrite (premise hrt) lr lhs true ())")
    out.append("       (rewrite (premise hqlo) lr lhs true ())")
    out.append("       (reduce lhs)")
    out.append("       (compute lhs (stop iw8 m_init))")
    out.append("       (rewrite (premise hre) lr lhs true ())")
    out.append("       (rewrite (premise hqhi) lr lhs true ())")
    out.append("       (reduce lhs)")
    out.append("       (compute lhs (stop iw8 m_init))")
    out.append("       (rewrite (premise hrd) lr lhs true ())")
    out.append("       (compute lhs (stop iw8 m_init))")
    for K in range(16):
        out.append(f"       (rewrite (premise hh{K}) lr lhs true ())")
        out.append("       (compute lhs (stop iw8 m_init))")
    out.append("       )")
    out.append("      refl)))")
    return "\n".join(out)


BANNER_MINIT = ";; --- THE INIT READS + hr_init (generated by gen_rth.py minit — REGENERATE, never hand-patch) ---"
BLOCKS.append(("minit", BANNER_MINIT, minit_block))



def splice(path=PATH):
    s = open(path).read()
    for key, banner, fn in BLOCKS:
        new = banner + "\n\n" + fn().rstrip("\n") + "\n"
        i = s.find(banner)
        if i < 0:
            s = s.rstrip("\n") + "\n\n" + new
            continue
        tail = s[i:]
        ends = [tail.find(b, 1) for _, b, _ in BLOCKS]
        ends.append(tail.find("\n;; ====", 1))
        # a block also ends at the NEXT section header (hand or generated) —
        # the gen_fra bea6925 lesson: never swallow a hand section
        ends.append(tail.find("\n;; --- ", len(banner)))
        ends = [e for e in ends if e > 0]
        j = i + (min(ends) if ends else len(tail))
        s = s[:i] + new + ("\n" + s[j:].lstrip("\n") if j < len(s) else "")
    open(path, "w").write(s)


if __name__ == "__main__":
    import sys

    if len(sys.argv) > 1 and sys.argv[1] == "splice":
        splice()
        print("spliced")
    else:
        key = sys.argv[1] if len(sys.argv) > 1 else "init"
        for k, banner, fn in BLOCKS:
            if k == key:
                print(banner)
                print()
                print(fn())
