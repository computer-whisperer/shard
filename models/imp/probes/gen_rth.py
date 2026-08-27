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

# ---------------- the predicate extractions ----------------
# One claim per clause of each andb-chain predicate: peel with andb_r,
# land with andb_l (the fe_ctx idiom).  Consumers cite these instead of
# re-peeling inline.

def _andb_chain(clauses):
    t = clauses[-1]
    for g in reversed(clauses[:-1]):
        t = f"(andb {g} {t})"
    return t


def _extract_claims(pred, binders, apply_, clauses, names):
    out = []
    for i, (gi, nm) in enumerate(zip(clauses, names)):
        out.append(f"(claim {nm}")
        out.append(f"  (goal ({binders})")
        out.append(f"    ((= {apply_} True))")
        out.append(f"    (= {gi} True))")
        out.append("  (chain")
        chain0 = _andb_chain(clauses)
        out.append(f"    (have h0 (= {chain0} True)")
        out.append("      (steps")
        out.append("        ((rewrite (premise 0) rl rhs true ())")
        out.append(f"         (unfold {pred} rhs)")
        out.append("         (reduce rhs))")
        out.append("        refl))")
        prev = "h0"
        # peel clauses 0..i-1
        for j in range(i):
            tail = _andb_chain(clauses[j + 1 :])
            out.append(f"    (have h{j+1} (= {tail} True)")
            out.append("      (rewrite-with")
            out.append("        (lemma andb_r)")
            out.append("        lr")
            out.append("        lhs")
            out.append(f"        ((inst a {clauses[j]}))")
            out.append(f"        ((steps ((rewrite (premise {prev}) lr lhs true ())) refl))")
            out.append("        refl))")
            prev = f"h{j+1}"
        if i < len(clauses) - 1:
            tail_after = _andb_chain(clauses[i + 1 :])
            out.append(f"    (have hx (= {gi} True)")
            out.append("      (rewrite-with")
            out.append("        (lemma andb_l)")
            out.append("        lr")
            out.append("        lhs")
            out.append(f"        ((inst b {tail_after}))")
            out.append(f"        ((steps ((rewrite (premise {prev}) lr lhs true ())) refl))")
            out.append("        refl))")
            prev = "hx"
        out.append(f"    (steps ((rewrite (premise {prev}) lr lhs true ())) refl)))")
        out.append("")
    return out


def extract_pred_block():
    out = []
    out.append(";; cells_ok, clause by clause")
    out.extend(_extract_claims(
        "cells_ok",
        "(c HCell) (rest (List HCell))",
        "(cells_ok (Cons c rest))",
        ["(slots_ok (hslots_of c) (haddrs rest))",
         "(int_eq (ilen (hslots_of c)) (harity_of c))",
         "(le 1 (hcount_of c))",
         "(lt (hcount_of c) 4294967296)",
         "(le 0 (htag_of c))",
         "(lt (htag_of c) 65536)",
         "(le 0 (harity_of c))",
         "(lt (harity_of c) 65536)",
         "(cells_ok rest)"],
        ["co_slots", "co_len", "co_cnt1", "co_cnthi", "co_tag0", "co_taghi",
         "co_ar0", "co_arhi", "co_tl"]))
    out.append(";; counts_ok")
    out.extend(_extract_claims(
        "counts_ok",
        "(all (List HCell)) (c HCell) (rest (List HCell)) (r (List Int)) (raw (Option HRaw))",
        "(counts_ok all (Cons c rest) r raw)",
        ["(if (le 2147483648 (hcount_of c)) True\n       (int_eq\n         (hcount_of c)\n         (+\n           (count_in r (haddr_of c))\n           (+ (inbound_cells all (haddr_of c)) (raw_refs raw (haddr_of c))))))",
         "(counts_ok all rest r raw)"],
        ["cnt_hd", "cnt_tl"]))
    out.append(";; p_cov")
    out.extend(_extract_claims(
        "p_cov",
        "(all (List HCell)) (c HCell) (rest (List HCell)) (fr (List Int))",
        "(p_cov all (Cons c rest) fr)",
        ["(if\n         (if (le 2147483648 (hcount_of c)) False\n         (lt (inbound_cells all (haddr_of c)) (hcount_of c)))\n         (memb fr (haddr_of c))\n         True)",
         "(p_cov all rest fr)"],
        ["pc_hd", "pc_tl"]))
    out.append(";; e_brk")
    out.extend(_extract_claims(
        "e_brk",
        "(a Int) (n Int) (rest (List (Pair Int Int))) (lo Int) (top Int)",
        "(e_brk (Cons (Pair a n) rest) lo top)",
        ["(le lo a)",
         "(le (+ a (+ 8 (* 8 n))) top)",
         "(int_eq (mod a 8) 0)",
         "(le 0 n)",
         "(e_brk rest lo top)"],
        ["eb_lo", "eb_hi", "eb_al", "eb_n0", "eb_tl"]))
    out.append(";; e_disj1 / e_disj")
    out.extend(_extract_claims(
        "e_disj1",
        "(a Int) (n Int) (b Int) (k Int) (rest (List (Pair Int Int)))",
        "(e_disj1 a n (Cons (Pair b k) rest))",
        ["(e_disj2 a n b k)", "(e_disj1 a n rest)"],
        ["ed1_hd", "ed1_tl"]))
    out.extend(_extract_claims(
        "e_disj",
        "(a Int) (n Int) (rest (List (Pair Int Int)))",
        "(e_disj (Cons (Pair a n) rest))",
        ["(e_disj1 a n rest)", "(e_disj rest)"],
        ["ed_hd", "ed_tl"]))
    out.append(";; roots_ok / slots_ok / all_in")
    out.extend(_extract_claims(
        "roots_ok",
        "(x Int) (t (List Int)) (as (List Int))",
        "(roots_ok (Cons x t) as)",
        ["(memb as x)", "(roots_ok t as)"],
        ["ro_hd", "ro_tl"]))
    out.extend(_extract_claims(
        "slots_ok",
        "(w Int) (rest (List Int)) (older (List Int))",
        "(slots_ok (Cons w rest) older)",
        ["(slot_ok w older)", "(slots_ok rest older)"],
        ["so_hd", "so_tl"]))
    out.extend(_extract_claims(
        "aligned8",
        "(x Int) (t (List Int))",
        "(aligned8 (Cons x t))",
        ["(int_eq (mod x 8) 0)", "(aligned8 t)"],
        ["al_hd", "al_tl"]))
    out.extend(_extract_claims(
        "all_in",
        "(x Int) (t (List Int)) (ys (List Int))",
        "(all_in (Cons x t) ys)",
        ["(memb ys x)", "(all_in t ys)"],
        ["ai_hd", "ai_tl"]))
    out.append(";; anodup / msub (the release toolkit's carriers)")
    out.extend(_extract_claims(
        "anodup",
        "(x Int) (t (List Int))",
        "(anodup (Cons x t))",
        ["(int_eq (count_in t x) 0)", "(anodup t)"],
        ["an_hd", "an_tl"]))
    out.extend(_extract_claims(
        "msub",
        "(x Int) (t (List Int)) (ys (List Int))",
        "(msub (Cons x t) ys)",
        ["(le (+ 1 (count_in t x)) (count_in ys x))", "(msub t ys)"],
        ["ms_hd", "ms_tl"]))
    out.extend(_extract_claims(
        "eall",
        "(a Int) (n Int) (t (List (Pair Int Int))) (ys (List (Pair Int Int)))",
        "(eall (Cons (Pair a n) t) ys)",
        ["(emem ys a n)", "(eall t ys)"],
        ["ea_hd", "ea_tl"]))
    out.extend(_extract_claims(
        "wexact",
        "(all (List HCell)) (c HCell) (rest (List HCell)) (r (List Int)) (wl (List HCell)) (ws (List Int))",
        "(wexact all (Cons c rest) r wl ws)",
        ["(if (le 2147483648 (hcount_of c)) True\n       (int_eq\n         (hcount_of c)\n         (+\n           (count_in r (haddr_of c))\n           (+\n             (inbound_cells all (haddr_of c))\n             (+\n               (inbound_cells wl (haddr_of c))\n               (count_in ws (haddr_of c)))))))",
         "(wexact all rest r wl ws)"],
        ["we_hd", "we_tl"]))
    out.extend(_extract_claims(
        "imm_at",
        "(c HCell) (rest (List HCell)) (w Int)",
        "(imm_at (Cons c rest) w)",
        ["(if (int_eq (haddr_of c) w) (le 2147483648 (hcount_of c)) True)",
         "(imm_at rest w)"],
        ["ia_hd", "ia_tl"]))
    out.extend(_extract_claims(
        "nimm_at",
        "(c HCell) (rest (List HCell)) (w Int)",
        "(nimm_at (Cons c rest) w)",
        ["(if (int_eq (haddr_of c) w)\n       (andb (lt 1 (hcount_of c)) (lt (hcount_of c) 2147483648))\n       True)",
         "(nimm_at rest w)"],
        ["na_hd", "na_tl"]))
    out.extend(_extract_claims(
        "winv",
        "(cs0 (List HCell)) (es0 (List (Pair Int Int))) (r (List Int)) (cells (List HCell)) (fls (List (List Int))) (wl (List HCell)) (ws (List Int))",
        "(winv cs0 es0 r cells fls wl ws)",
        ["(wexact cells cells r wl ws)",
         "(cells_ok cells)",
         "(roots_ok r (haddrs cells))",
         "(eall (wext cells fls wl) es0)",
         "(msub (wa cells fls wl) (e_addrs es0))",
         "(hsubq cells cs0)",
         "(aligned8 (haddrs cells))"],
        ["wi_ex", "wi_co", "wi_ro", "wi_ea", "wi_ms", "wi_sq", "wi_al"]))
    out.append(";; hinv, clause by clause")
    out.extend(_extract_claims(
        "hinv",
        "(rb Int) (g GHeap) (r (List Int))",
        "(hinv rb g r)",
        ["(le 0 rb)",
         "(le (+ rb 152) (glo_of g))",
         "(le (glo_of g) (gtop_of g))",
         "(le (gtop_of g) (gend_of g))",
         "(le (gend_of g) 4294967296)",
         "(int_eq (mod (glo_of g) 8) 0)",
         "(int_eq (mod (gtop_of g) 8) 0)",
         "(e_disj (ext_all g))",
         "(e_brk (ext_all g) (glo_of g) (gtop_of g))",
         "(cells_ok (gcells_of g))",
         "(counts_ok (gcells_of g) (gcells_of g) r (graw_of g))",
         "(roots_ok r (haddrs (gcells_of g)))",
         "(raw_ok (graw_of g) (gcells_of g))"],
        ["hi_rb0", "hi_hdr", "hi_lot", "hi_te", "hi_e32", "hi_alo", "hi_atop",
         "hi_disj", "hi_brk", "hi_cells", "hi_cnts", "hi_roots", "hi_raw"]))
    out.append(";; the representation conjuncts (the frame toolkit's peels)")
    out.extend(_extract_claims(
        "hslots_rep",
        "(m Mem) (a Int) (w Int) (rest (List Int))",
        "(hslots_rep m a (Cons w rest))",
        ["(int_eq (load_le (iw8) m a) w)", "(hslots_rep m (+ a 8) rest)"],
        ["hs_hd", "hs_tl"]))
    out.extend(_extract_claims(
        "hcell_rep",
        "(m Mem) (c HCell)",
        "(hcell_rep m c)",
        ["(int_eq (load_le (iw8) m (haddr_of c)) (hword c))",
         "(hslots_rep m (+ (haddr_of c) 8) (hslots_of c))"],
        ["hc_hdr", "hc_slots"]))
    out.extend(_extract_claims(
        "hcells_rep",
        "(m Mem) (c HCell) (rest (List HCell))",
        "(hcells_rep m (Cons c rest))",
        ["(hcell_rep m c)", "(hcells_rep m rest)"],
        ["hcs_hd", "hcs_tl"]))
    out.extend(_extract_claims(
        "hchain_rep",
        "(m Mem) (a Int) (rest (List Int))",
        "(hchain_rep m (Cons a rest))",
        ["(int_eq (load_le (iw8) m a) (hhead rest))", "(hchain_rep m rest)"],
        ["hch_hd", "hch_tl"]))
    out.extend(_extract_claims(
        "hfree_rep",
        "(m Mem) (rb Int) (k Int) (fl (List Int)) (rest (List (List Int)))",
        "(hfree_rep m rb k (Cons fl rest))",
        ["(int_eq (load_le (iw8) m (+ rb (+ 24 (* 8 k)))) (hhead fl))",
         "(hchain_rep m fl)",
         "(hfree_rep m rb (+ k 1) rest)"],
        ["hf_hd", "hf_chain", "hf_tl"]))
    out.extend(_extract_claims(
        "hraw_rep",
        "(m Mem) (rc HRaw)",
        "(hraw_rep m (Some rc))",
        ["(int_eq (load_le (iw8) m (raddr_of rc)) (rword rc))",
         "(hslots_rep m (+ (raddr_of rc) 8) (rfill_of rc))"],
        ["hrw_hdr", "hrw_slots"]))
    out.extend(_extract_claims(
        "raw_ok",
        "(rc HRaw) (cs (List HCell))",
        "(raw_ok (Some rc) cs)",
        ["(slots_ok (rfill_of rc) (haddrs cs))",
         "(le (ilen (rfill_of rc)) (rarity_of rc))",
         "(le 0 (rtag_of rc))",
         "(lt (rtag_of rc) 65536)",
         "(le 0 (rarity_of rc))",
         "(lt (rarity_of rc) 65536)"],
        ["rk_slots", "rk_len", "rk_tag0", "rk_taghi", "rk_ar0", "rk_arhi"]))
    out.append(";; heap_rep, clause by clause")
    out.extend(_extract_claims(
        "heap_rep",
        "(m Mem) (rb Int) (g GHeap)",
        "(heap_rep m rb g)",
        ["(int_eq (load_le (iw8) m rb) (gtop_of g))",
         "(int_eq (load_le (iw8) m (+ rb 8)) (gend_of g))",
         "(int_eq (load_le (iw8) m (+ rb 16)) 0)",
         "(int_eq (fllen (gfree_of g)) 16)",
         "(hfree_rep m rb 0 (gfree_of g))",
         "(hcells_rep m (gcells_of g))",
         "(hraw_rep m (graw_of g))"],
        ["hrx_top", "hrx_end", "hrx_dep", "hrx_fl16", "hrx_free", "hrx_cells", "hrx_raw"]))
    return "\n".join(out)


BANNER_EXTRACT = ";; --- THE PREDICATE EXTRACTIONS (generated by gen_rth.py extract — REGENERATE, never hand-patch) ---"
BLOCKS.append(("extract", BANNER_EXTRACT, extract_pred_block))




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
