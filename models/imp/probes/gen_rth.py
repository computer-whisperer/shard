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

Usage: python3 models/imp/probes/gen_rth.py [init | minit | extract | alloc | inc | dec | slots | splice]
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
    out.append(";; the pop leg's fact bundle")
    out.extend(_extract_claims(
        "apop_ok",
        "(g GHeap) (tag Int) (n Int) (p Int) (rest (List Int))",
        "(apop_ok g tag n p rest)",
        ["(le (glo_of g) p)",
         "(le (+ p (+ 8 (* 8 n))) (gtop_of g))",
         "(int_eq (mod p 8) 0)",
         "(int_eq (count_in (haddrs (gcells_of g)) p) 0)",
         "(int_eq (count_in (faddrs (fl_set (gfree_of g) n rest)) p) 0)",
         "(eall (ext_all (gh_alloc_pop g tag n)) (ext_all g))",
         "(anodup (e_addrs (ext_all (gh_alloc_pop g tag n))))",
         "(emem (ext_all g) p n)",
         "(eall (fexts (fl_set (gfree_of g) n rest) 0) (ext_all g))",
         "(eall (hexts (gcells_of g)) (ext_all g))"],
        ["ap_lo", "ap_hi", "ap_al", "ap_hc0", "ap_fc0", "ap_eall", "ap_an",
         "ap_mem", "ap_efree", "ap_ecells"]))
    out.append(";; the raw cell's fact bundle")
    out.extend(_extract_claims(
        "araw_ok",
        "(g GHeap) (p Int) (tag Int) (n Int) (fill (List Int))",
        "(araw_ok g p tag n fill)",
        ["(le (glo_of g) p)",
         "(le (+ p (+ 8 (* 8 n))) (gtop_of g))",
         "(int_eq (mod p 8) 0)",
         "(int_eq (count_in (haddrs (gcells_of g)) p) 0)",
         "(int_eq (count_in (faddrs (gfree_of g)) p) 0)",
         "(emem (ext_all g) p n)",
         "(eall (hexts (gcells_of g)) (ext_all g))",
         "(eall (fexts (gfree_of g) 0) (ext_all g))",
         "(e_disj1 p n (hexts (gcells_of g)))",
         "(e_disj1 p n (fexts (gfree_of g) 0))"],
        ["ar_lo", "ar_hi", "ar_al", "ar_hc0", "ar_fc0", "ar_mem", "ar_ecells", "ar_efree", "ar_d1c", "ar_d1f"]))
    out.append(";; a cell's header bands (C2b-4)")
    out.extend(_extract_claims(
        "cbands",
        "(c HCell)",
        "(cbands c)",
        ["(le 1 (hcount_of c))",
         "(lt (hcount_of c) 4294967296)",
         "(le 0 (htag_of c))",
         "(lt (htag_of c) 65536)",
         "(le 0 (harity_of c))",
         "(lt (harity_of c) 65536)"],
        ["cb_cnt1", "cb_cnthi", "cb_tag0", "cb_taghi", "cb_ar0", "cb_arhi"]))
    out.append(";; the live cell's fact bundle (C2b-4)")
    out.extend(_extract_claims(
        "live_ok",
        "(g GHeap) (v Int)",
        "(live_ok g v)",
        ["(le (glo_of g) v)",
         "(le (+ v (+ 8 (* 8 (harity_of (hfget (gcells_of g) v))))) (gtop_of g))",
         "(int_eq (mod v 8) 0)",
         "(le (count_in (haddrs (gcells_of g)) v) 1)",
         "(e_disj (hexts (gcells_of g)))",
         "(e_disj1 v (harity_of (hfget (gcells_of g) v)) (fexts (gfree_of g) 0))",
         "(e_disj1 v (harity_of (hfget (gcells_of g) v)) (rexts (graw_of g)))",
         "(cbands (hfget (gcells_of g) v))"],
        ["lv_lo", "lv_hi", "lv_al", "lv_cin1", "lv_disj", "lv_d1f", "lv_d1r", "lv_bands"]))
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


# ---------------- the alloc engine legs (C2b-3 part iii) ----------------
# Five legs of rt_alloc at the ipcall grain, one claim each, all the
# rth_init_run recipe: compute to the stuck guard, collapse the band,
# discharge the guards, rewrite the condition's Bool fact, read through
# heap_rep at the stuck loads, compute on.  The have texts are the ones
# the hand-written pop leg landed with; the generator only varies the
# premise set and the statement script per leg.

U64 = "18446744073709551616"
U32M = "4294967295"

def _rw(h):
    return f"(rewrite (premise {h}) lr lhs true ())"

def _cite(lemma, insts, dis):
    ins = " ".join(insts)
    d = "\n".join("         " + x for x in dis)
    return f"(rewrite-with (lemma {lemma}) lr lhs ({ins})\n        (\n{d})\n        refl)"

def _rows(*rs):
    return "(by arith (rows " + " ".join(f"({a} {b})" for a, b in rs) + "))"

def _pr(h):
    return f"(steps ((rewrite (premise {h}) lr lhs true ())) refl)"

def alloc_leg(kind):
    pop = kind == "pop"
    hit = kind in ("pop", "bump_hit", "oom_hit")
    fits = kind in ("bump_hit", "bump_big")
    oom = kind in ("oom_hit", "oom_big")
    name = {"pop": "rth_alloc_pop_run", "bump_hit": "rth_alloc_bump_run",
            "bump_big": "rth_alloc_big_run", "oom_hit": "rth_alloc_oom_run",
            "oom_big": "rth_alloc_oom_big_run"}[kind]
    doc = {"pop": "the POP leg: n < 16 and class n has a head p",
           "bump_hit": "the BUMP leg at n < 16 with class n empty; the cell fits",
           "bump_big": "the BUMP leg at n >= 16 (no free list); the cell fits",
           "oom_hit": "the OOM leg at n < 16 with class n empty; the cell does not fit",
           "oom_big": "the OOM leg at n >= 16; the cell does not fit"}[kind]
    prem = [("hr", "(= (heap_rep m rb g) True)"),
            ("hi", "(= (hinv rb g r) True)"),
            ("raw", "(= (graw_of g) None)")]
    if pop:
        prem.append(("fla", "(= (fl_at (gfree_of g) n) (Cons p rest))"))
    elif hit:
        prem.append(("fla", "(= (fl_at (gfree_of g) n) Nil)"))
    prem.append(("n16", "(= (lt n 16) True)" if hit else "(= (lt n 16) False)"))
    prem += [("n0", "(= (le 0 n) True)"), ("nhi", "(= (lt n 65536) True)"),
             ("t0", "(= (le 0 tag) True)"), ("thi", "(= (lt tag 65536) True)")]
    if fits:
        prem.append(("fit", "(= (le (+ (gtop_of g) (+ 8 (* 8 n))) (gend_of g)) True)"))
    if oom:
        prem.append(("fit", "(= (le (+ (gtop_of g) (+ 8 (* 8 n))) (gend_of g)) False)"))
    prem += [("mlo", "(= (le mlo rb) True)"), ("msz", "(= (le (gend_of g) msz) True)"),
             ("dd", "(= (lt d dmax) True)")]
    P = {k: i for i, (k, _) in enumerate(prem)}
    NP = len(prem)
    binders = ("(fs (List IpFn)) (mlo Int) (msz Int) (dmax Int) (d Int) (rb Int) (g GHeap) "
               "(r (List Int)) (m Mem) (tag Int) (n Int)"
               + (" (p Int) (rest (List Int))" if pop else "") + " (fuel Nat)")
    concl = {"pop": "(Some (IpRv p (m_apop m rb n (hhead rest) p tag)))",
             "bump": "(Some (IpRv (gtop_of g) (m_abump m rb (gtop_of g) n tag)))",
             "oom": "(Some (IpRfailed FOom))"}["pop" if pop else ("bump" if fits else "oom")]
    out = []
    out.append(f";; {doc}")
    out.append(f"(claim {name}")
    out.append("  (goal")
    out.append(f"    ({binders})")
    out.append("    (" + "\n     ".join(t for _, t in prem) + ")")
    out.append("    (=")
    out.append("      (ipcall (nS 12 (ntl 12 fuel)) (rt_app (rt_fns rb) fs) mlo msz dmax d 1 (Cons tag (Cons n Nil)) m)")
    out.append(f"      {concl}))")
    out.append("  (chain")
    haves = []  # names in order (for two-sided cert widths)

    def have(nm, goal, proof):
        out.append(f"    (have {nm} {goal}")
        out.append(f"      {proof})")
        haves.append(nm)

    def ident(nm, goal):
        w = 1 + NP + len(haves)
        z = " ".join(["1"] + ["0"] * (w - 1))
        have(nm, goal, f"(by arith (list (list {z}) (list {z})))")

    def mod64(nm, x, lo, hi):
        have(nm, f"(= (mod {x} {U64}) {x})",
             _cite("modu64_id", [], [lo, hi]))

    def band32(nm, x, lo, hi):
        have(nm, f"(= (band {x} {U32M}) {x})",
             _cite("bandu32_id", [], [lo, hi]))

    # the two-sided identities first (small slot tables)
    if pop:
        ident("hHf", "(= (+ (+ rb 24) (* n 8)) (+ rb (+ 24 (* 8 n))))")
        ident("hA0", "(= (+ rb (+ 24 (* 8 (+ 0 n)))) (+ (+ rb 24) (* n 8)))")
        ident("hpz", "(= (+ p 0) p)")
    if fits:
        ident("hTf", "(= (+ (gtop_of g) (+ 8 (* n 8))) (+ (gtop_of g) (+ 8 (* 8 n))))")
        ident("hpzT", "(= (+ (gtop_of g) 0) (gtop_of g))")
    # the entry bands
    mod64("hmtag", "tag", _pr(P["t0"]), _rows(("goal", 1), (P["thi"], 1)))
    mod64("hmn", "n", _pr(P["n0"]), _rows(("goal", 1), (P["nhi"], 1)))
    # hinv's brackets
    have("h1", "(= (le 0 rb) True)", _cite("hi_rb0", ["(inst g g)", "(inst r r)"], [_pr(P["hi"])]))
    have("h2", "(= (le (+ rb 152) (glo_of g)) True)", _cite("hi_hdr", ["(inst r r)"], [_pr(P["hi"])]))
    have("h3", "(= (le (glo_of g) (gtop_of g)) True)", _cite("hi_lot", ["(inst rb rb)", "(inst r r)"], [_pr(P["hi"])]))
    have("h4", "(= (le (gtop_of g) (gend_of g)) True)", _cite("hi_te", ["(inst rb rb)", "(inst r r)"], [_pr(P["hi"])]))
    have("h5", "(= (le (gend_of g) 4294967296) True)", _cite("hi_e32", ["(inst rb rb)", "(inst r r)"], [_pr(P["hi"])]))
    # 8n
    mod64("hm8n", "(* n 8)", _rows(("goal", 1), (P["n0"], 8)), _rows(("goal", 1), (P["nhi"], 8)))
    # the header word's bands
    mod64("hmt", "(* tag 4294967296)", _rows(("goal", 1), (P["t0"], 4294967296)), _rows(("goal", 1), (P["thi"], 4294967296)))
    mod64("hmn2", "(* n 281474976710656)", _rows(("goal", 1), (P["n0"], 281474976710656)), _rows(("goal", 1), (P["nhi"], 281474976710656)))
    mod64("hms", "(+ (* tag 4294967296) (* n 281474976710656))",
          _rows(("goal", 1), (P["t0"], 4294967296), (P["n0"], 281474976710656)),
          _rows(("goal", 1), (P["thi"], 4294967296), (P["nhi"], 281474976710656)))
    mod64("hm1", "(+ 1 (+ (* tag 4294967296) (* n 281474976710656)))",
          _rows(("goal", 1), (P["t0"], 4294967296), (P["n0"], 281474976710656)),
          _rows(("goal", 1), (P["thi"], 4294967296), (P["nhi"], 281474976710656)))
    have("hhw", "(= (+ 1 (+ (* tag 4294967296) (* n 281474976710656))) (hdr_word tag n))",
         "(steps ((unfold hdr_word rhs)) refl)")
    if hit:
        # the head word of class n
        mod64("hmH", "(+ (+ rb 24) (* n 8))", _rows(("goal", 1), ("h1", 1), (P["n0"], 8)),
              _rows(("goal", 1), ("h2", 1), ("h3", 1), ("h4", 1), ("h5", 1), (P["n16"], 8)))
        band32("hbH", "(+ (+ rb 24) (* n 8))", _rows(("goal", 1), ("h1", 1), (P["n0"], 8)),
               _rows(("goal", 1), ("h2", 1), ("h3", 1), ("h4", 1), ("h5", 1), (P["n16"], 8)))
        have("hgl1", "(= (le mlo (+ (+ rb 24) (* n 8))) True)", _rows(("goal", 1), (P["mlo"], 1), (P["n0"], 8)))
        have("hgh1", "(= (le (+ (+ (+ rb 24) (* n 8)) 8) msz) True)",
             _rows(("goal", 1), ("h2", 1), ("h3", 1), ("h4", 1), (P["msz"], 1), (P["n16"], 8)))
        have("hfrg", "(= (hfree_rep m rb 0 (gfree_of g)) True)", _cite("hrx_free", [], [_pr(P["hr"])]))
        have("f16g", "(= (int_eq (fllen (gfree_of g)) 16) True)", _cite("hrx_fl16", ["(inst m m)", "(inst rb rb)"], [_pr(P["hr"])]))
        have("f16e", "(= (fllen (gfree_of g)) 16)", _cite("int_eq_eq", ["(inst b 16)"], [_pr("f16g")]))
        have("hnf", "(= (lt n (fllen (gfree_of g))) True)",
             "(steps ((rewrite (premise f16e) lr lhs true ()) (rewrite (premise %d) lr lhs true ())) refl)" % P["n16"])
        have("hrd0", "(= (load_le (iw8) m (+ rb (+ 24 (* 8 (+ 0 n))))) (hhead (fl_at (gfree_of g) n)))",
             _cite("hfree_rd", ["(inst fls (gfree_of g))"], [_pr("hfrg"), _pr(P["n0"]), _pr("hnf")]))
        if pop:
            have("hrd1", "(= (load_le (iw8) m (+ (+ rb 24) (* n 8))) p)",
                 "(steps\n        ((rewrite (premise hA0) rl lhs true ())\n         (rewrite (premise hrd0) lr lhs true ())\n         (rewrite (premise %d) lr lhs true ())\n         (unfold hhead lhs)\n         (reduce lhs))\n        refl)" % P["fla"])
        else:
            ident("hA0", "(= (+ rb (+ 24 (* 8 (+ 0 n)))) (+ (+ rb 24) (* n 8)))")
            have("hrd1", "(= (load_le (iw8) m (+ (+ rb 24) (* n 8))) 0)",
                 "(steps\n        ((rewrite (premise hA0) rl lhs true ())\n         (rewrite (premise hrd0) lr lhs true ())\n         (rewrite (premise %d) lr lhs true ())\n         (unfold hhead lhs)\n         (reduce lhs))\n        refl)" % P["fla"])
    if pop:
        have("hap", "(= (apop_ok g tag n p rest) True)",
             _cite("apop_facts", ["(inst rb rb)", "(inst r r)"], [_pr(P["hi"]), _pr(P["raw"]), _pr(P["fla"]), _pr(P["n0"])]))
        have("hlo", "(= (le (glo_of g) p) True)", _cite("ap_lo", ["(inst tag tag)", "(inst n n)", "(inst rest rest)"], [_pr("hap")]))
        have("hhi", "(= (le (+ p (+ 8 (* 8 n))) (gtop_of g)) True)", _cite("ap_hi", ["(inst tag tag)", "(inst rest rest)"], [_pr("hap")]))
        have("hp1", "(= (le 1 p) True)", _rows(("goal", 1), ("h1", 1), ("h2", 1), ("hlo", 1)))
        have("hne", "(= (int_eq p 0) False)", _cite("ieq0_false", [], [_pr("hp1")]))
        mod64("hmp", "p", _rows(("goal", 1), ("h1", 1), ("h2", 1), ("hlo", 1)),
              _rows(("goal", 1), ("hhi", 1), ("h4", 1), ("h5", 1), (P["n0"], 8)))
        band32("hbp", "p", _rows(("goal", 1), ("h1", 1), ("h2", 1), ("hlo", 1)),
               _rows(("goal", 1), ("hhi", 1), ("h4", 1), ("h5", 1), (P["n0"], 8)))
        have("hgl2", "(= (le mlo p) True)", _rows(("goal", 1), (P["mlo"], 1), ("h2", 1), ("hlo", 1)))
        have("hgh2", "(= (le (+ p 8) msz) True)", _rows(("goal", 1), ("hhi", 1), ("h4", 1), (P["msz"], 1), (P["n0"], 8)))
        have("hchn", "(= (hchain_rep m (fl_at (gfree_of g) n)) True)",
             _cite("hfree_chain", ["(inst rb rb)", "(inst k 0)"], [_pr("hfrg"), _pr(P["n0"]), _pr("hnf")]))
        have("hchc", "(= (hchain_rep m (Cons p rest)) True)",
             "(steps ((rewrite (premise %d) rl lhs true ()) (rewrite (premise hchn) lr lhs true ())) refl)" % P["fla"])
        have("hrd2i", "(= (int_eq (load_le (iw8) m p) (hhead rest)) True)", _cite("hch_hd", [], [_pr("hchc")]))
        have("hrd2", "(= (load_le (iw8) m p) (hhead rest))", _cite("int_eq_eq", ["(inst b (hhead rest))"], [_pr("hrd2i")]))
    else:
        # the bump/oom legs: the header block's two reads, the size, the fit test
        band32("hb0", "rb", _rows(("goal", 1), ("h1", 1)), _rows(("goal", 1), ("h2", 1), ("h3", 1), ("h4", 1), ("h5", 1)))
        band32("hb1", "(+ rb 8)", _rows(("goal", 1), ("h1", 1)), _rows(("goal", 1), ("h2", 1), ("h3", 1), ("h4", 1), ("h5", 1)))
        have("hh0", "(= (le (+ rb 8) msz) True)", _rows(("goal", 1), ("h2", 1), ("h3", 1), ("h4", 1), (P["msz"], 1)))
        have("hl1", "(= (le mlo (+ rb 8)) True)", _rows(("goal", 1), (P["mlo"], 1)))
        have("hh1", "(= (le (+ (+ rb 8) 8) msz) True)", _rows(("goal", 1), ("h2", 1), ("h3", 1), ("h4", 1), (P["msz"], 1)))
        have("hrdTi", "(= (int_eq (load_le (iw8) m rb) (gtop_of g)) True)", _cite("hrx_top", [], [_pr(P["hr"])]))
        have("hrdT", "(= (load_le (iw8) m rb) (gtop_of g))", _cite("int_eq_eq", ["(inst b (gtop_of g))"], [_pr("hrdTi")]))
        have("hrdEi", "(= (int_eq (load_le (iw8) m (+ rb 8)) (gend_of g)) True)", _cite("hrx_end", [], [_pr(P["hr"])]))
        have("hrdE", "(= (load_le (iw8) m (+ rb 8)) (gend_of g))", _cite("int_eq_eq", ["(inst b (gend_of g))"], [_pr("hrdEi")]))
        mod64("hm8s", "(+ 8 (* n 8))", _rows(("goal", 1), (P["n0"], 8)), _rows(("goal", 1), (P["nhi"], 8)))
        mod64("hmsum", "(+ (gtop_of g) (+ 8 (* n 8)))",
              _rows(("goal", 1), ("h1", 1), ("h2", 1), ("h3", 1), (P["n0"], 8)),
              _rows(("goal", 1), ("h4", 1), ("h5", 1), (P["nhi"], 8)))
        if fits:
            have("hfitE", "(= (le (+ (gtop_of g) (+ 8 (* n 8))) (gend_of g)) True)", _rows(("goal", 1), (P["fit"], 1)))
            mod64("hmpT", "(gtop_of g)", _rows(("goal", 1), ("h1", 1), ("h2", 1), ("h3", 1)), _rows(("goal", 1), (P["fit"], 1), ("h5", 1), (P["n0"], 8)))
            band32("hbpT", "(gtop_of g)", _rows(("goal", 1), ("h1", 1), ("h2", 1), ("h3", 1)), _rows(("goal", 1), (P["fit"], 1), ("h5", 1), (P["n0"], 8)))
            have("hglT", "(= (le mlo (gtop_of g)) True)", _rows(("goal", 1), (P["mlo"], 1), ("h2", 1), ("h3", 1)))
            have("hghT", "(= (le (+ (gtop_of g) 8) msz) True)", _rows(("goal", 1), (P["fit"], 1), (P["msz"], 1), (P["n0"], 8)))
        else:
            have("hfitE", "(= (le (+ (gtop_of g) (+ 8 (* n 8))) (gend_of g)) False)", _rows(("goal", 1), (P["fit"], 1)))

    # ---- the steps ----
    C = "(compute lhs (stop iw8 hhead gtop_of gend_of))"
    st = [C, _rw(P["dd"]), "(reduce lhs)", C, _rw("hmtag"), _rw("hmn"), _rw(P["n16"]), C]
    if hit:
        st += [_rw("hm8n"), _rw("hmH"), _rw("hbH"), _rw("hgl1"), "(reduce lhs)", _rw("hgh1"), "(reduce lhs)", C, _rw("hrd1")]
        if pop:
            st += [_rw("hne")]
        st += [C]
    if pop:
        st += [_rw("hpz"), _rw("hmp"), _rw("hbp"), _rw("hgl2"), "(reduce lhs)", _rw("hgh2"), "(reduce lhs)", C, _rw("hrd2"),
               _rw("hm8n"), _rw("hmH"), _rw("hbH"), _rw("hgl1"), "(reduce lhs)", _rw("hgh1"), "(reduce lhs)", C,
               _rw("hpz"), _rw("hmp"), _rw("hbp"), _rw("hgl2"), "(reduce lhs)", _rw("hgh2"), "(reduce lhs)", C,
               _rw("hmt"), _rw("hmn2"), _rw("hms"), _rw("hm1"), _rw("hhw"), _rw("hHf"), "(unfold m_apop rhs)"]
    else:
        # the bump branch: [rb], [rb+8], size, the fit test
        st += [_rw("hb0"), _rw(P["mlo"]), "(reduce lhs)", _rw("hh0"), "(reduce lhs)", C, _rw("hrdT"),
               _rw("hb1"), _rw("hl1"), "(reduce lhs)", _rw("hh1"), "(reduce lhs)", C, _rw("hrdE"),
               _rw("hm8n"), _rw("hm8s"), _rw("hmsum"), _rw("hfitE"), C]
        if fits:
            st += [_rw("hb0"), _rw(P["mlo"]), "(reduce lhs)", _rw("hh0"), "(reduce lhs)", C, _rw("hmsum"),
                   _rw("hpzT"), _rw("hmpT"), _rw("hbpT"), _rw("hglT"), "(reduce lhs)", _rw("hghT"), "(reduce lhs)", C,
                   _rw("hmt"), _rw("hmn2"), _rw("hms"), _rw("hm1"), _rw("hhw"), _rw("hTf"), "(unfold m_abump rhs)"]
    out.append("    (steps")
    out.append("      (" + "\n       ".join(st) + ")")
    out.append("      refl)))")
    return "\n".join(out)


def alloc_block():
    return "\n\n".join(alloc_leg(k) for k in ("pop", "bump_hit", "bump_big", "oom_hit", "oom_big"))


BANNER_ALLOC = ";; --- THE ALLOC ENGINE LEGS (generated by gen_rth.py alloc — REGENERATE, never hand-patch) ---"
BLOCKS.append(("alloc", BANNER_ALLOC, alloc_block))





# ---------------- the inc engine legs (C2b-4 part iii) ----------------
# Three legs of rt_inc at the ipcall grain: the immediate (v odd — nothing
# moves), the reference below the band (the one header store), and the
# saturated reference (nothing moves).  The have texts are the ones the
# hand-written ref leg landed with (2026-08-27).

def _lv(nm, goal, lemma, insts=()):
    return (nm, goal, _cite(lemma, list(insts), [_pr("hlv")]))


def inc_leg(kind):
    imm = kind == "imm"
    sat = kind == "sat"
    name = {"imm": "rth_inc_imm_run", "ref": "rth_inc_ref_run", "sat": "rth_inc_sat_run"}[kind]
    doc = {"imm": "the IMMEDIATE leg: v odd, nothing moves",
           "ref": "the REFERENCE leg below the band: the one header store [v] := hword + 1",
           "sat": "the SATURATED leg: the count is in the immortal band, nothing moves"}[kind]
    HW = "(hword (hfget (gcells_of g) v))"
    CNT = "(hcount_of (hfget (gcells_of g) v))"
    AR = "(harity_of (hfget (gcells_of g) v))"
    if imm:
        prem = [("odd", "(= (int_eq (mod v 2) 1) True)"),
                ("v0", "(= (le 0 v) True)"),
                ("v64", "(= (lt v 18446744073709551616) True)"),
                ("dd", "(= (lt d dmax) True)")]
        binders = "(fs (List IpFn)) (mlo Int) (msz Int) (dmax Int) (d Int) (rb Int) (m Mem) (v Int) (fuel Nat)"
        concl = "(Some (IpRv 0 m))"
    else:
        prem = [("hr", "(= (heap_rep m rb g) True)"),
                ("hi", "(= (hinv rb g r) True)"),
                ("mem", "(= (memb (haddrs (gcells_of g)) v) True)"),
                ("cnt", "(= (lt %s 2147483648) True)" % CNT if not sat else "(= (le 2147483648 %s) True)" % CNT),
                ("mlo", "(= (le mlo rb) True)"),
                ("msz", "(= (le (gend_of g) msz) True)"),
                ("dd", "(= (lt d dmax) True)")]
        binders = ("(fs (List IpFn)) (mlo Int) (msz Int) (dmax Int) (d Int) (rb Int) (g GHeap) "
                   "(r (List Int)) (m Mem) (v Int) (fuel Nat)")
        concl = "(Some (IpRv 0 m))" if sat else "(Some (IpRv 0 (store_le (iw8) m v (+ %s 1))))" % HW
    P = {k: i for i, (k, _) in enumerate(prem)}
    NP = len(prem)
    out = []
    out.append(f";; {doc}")
    out.append(f"(claim {name}")
    out.append("  (goal")
    out.append(f"    ({binders})")
    out.append("    (" + "\n     ".join(x for _, x in prem) + ")")
    out.append("    (=")
    out.append("      (ipcall (nS 12 (ntl 12 fuel)) (rt_app (rt_fns rb) fs) mlo msz dmax d 2 (Cons v Nil) m)")
    out.append(f"      {concl}))")
    out.append("  (chain")
    haves = []

    def have(nm, goal, proof):
        out.append(f"    (have {nm} {goal}")
        out.append(f"      {proof})")
        haves.append(nm)

    def ident(nm, goal):
        w = 1 + NP + len(haves)
        z = " ".join(["1"] + ["0"] * (w - 1))
        have(nm, goal, f"(by arith (list (list {z}) (list {z})))")

    if imm:
        have("hm2", "(= (mod v 2) 1)", _cite("int_eq_eq", ["(inst b 1)"], [_pr(P["odd"])]))
        have("hb1", "(= (band v 1) (mod v 2))", _cite("band1_mod2", [], [_pr(P["v0"])]))
        have("hmv", "(= (mod v 18446744073709551616) v)", _cite("modu64_id", [], [_pr(P["v0"]), _pr(P["v64"])]))
    else:
        ident("hpz", "(= (+ v 0) v)")
        have("hlv", "(= (live_ok g v) True)", _cite("live_facts", ["(inst rb rb)", "(inst r r)"], [_pr(P["hi"]), _pr(P["mem"])]))
        have(*_lv("hlo", "(= (le (glo_of g) v) True)", "lv_lo"))
        have(*_lv("hhi", "(= (le (+ v (+ 8 (* 8 %s))) (gtop_of g)) True)" % AR, "lv_hi"))
        have(*_lv("hal", "(= (int_eq (mod v 8) 0) True)", "lv_al", ["(inst g g)"]))
        have(*_lv("hcb", "(= (cbands (hfget (gcells_of g) v)) True)", "lv_bands"))
        have("hn0", "(= (le 0 %s) True)" % AR, _cite("cb_ar0", [], [_pr("hcb")]))
        if not sat:
            have("hthi", "(= (lt (htag_of (hfget (gcells_of g) v)) 65536) True)", _cite("cb_taghi", [], [_pr("hcb")]))
            have("hahi", "(= (lt %s 65536) True)" % AR, _cite("cb_arhi", [], [_pr("hcb")]))
        have("h1", "(= (le 0 rb) True)", _cite("hi_rb0", ["(inst g g)", "(inst r r)"], [_pr(P["hi"])]))
        have("h2", "(= (le (+ rb 152) (glo_of g)) True)", _cite("hi_hdr", ["(inst r r)"], [_pr(P["hi"])]))
        have("h4", "(= (le (gtop_of g) (gend_of g)) True)", _cite("hi_te", ["(inst rb rb)", "(inst r r)"], [_pr(P["hi"])]))
        have("h5", "(= (le (gend_of g) 4294967296) True)", _cite("hi_e32", ["(inst rb rb)", "(inst r r)"], [_pr(P["hi"])]))
        have("hv0", "(= (le 0 v) True)", _rows(("goal", 1), ("h1", 1), ("h2", 1), ("hlo", 1)))
        have("hv32", "(= (lt v 4294967296) True)", _rows(("goal", 1), ("hhi", 1), ("h4", 1), ("h5", 1), ("hn0", 8)))
        have("hv64", "(= (lt v 18446744073709551616) True)", _rows(("goal", 1), ("hhi", 1), ("h4", 1), ("h5", 1), ("hn0", 8)))
        have("hev", "(= (int_eq (mod v 2) 0) True)", _cite("mod8_even", [], [_pr("hal")]))
        have("hm2", "(= (mod v 2) 0)", _cite("int_eq_eq", ["(inst b 0)"], [_pr("hev")]))
        have("hb1", "(= (band v 1) (mod v 2))", _cite("band1_mod2", [], [_pr("hv0")]))
        have("hmv", "(= (mod v 18446744073709551616) v)", _cite("modu64_id", [], [_pr("hv0"), _pr("hv64")]))
        have("hbv", "(= (band v 4294967295) v)", _cite("bandu32_id", [], [_pr("hv0"), _pr("hv32")]))
        have("hgl", "(= (le mlo v) True)", _rows(("goal", 1), (P["mlo"], 1), ("h2", 1), ("hlo", 1)))
        have("hgh", "(= (le (+ v 8) msz) True)", _rows(("goal", 1), ("hhi", 1), ("h4", 1), (P["msz"], 1), ("hn0", 8)))
        have("hrd", "(= (load_le (iw8) m v) %s)" % HW, _cite("hr_rd_hdr", ["(inst rb rb)", "(inst g g)"], [_pr(P["hr"]), _pr(P["mem"])]))
        have("hcnt", "(= (band %s 4294967295) %s)" % (HW, CNT), _cite("hword_count", [], [_pr("hcb")]))
        if sat:
            have("hnlt", "(= (lt %s 2147483648) False)" % CNT, _rows(("goal", 1), (P["cnt"], 1)))
        else:
            have("hwlo", "(= (le 0 %s) True)" % HW, _cite("hword_lo", [], [_pr("hcb")]))
            have("hwe", "(= %s (+ %s (+ (* (htag_of (hfget (gcells_of g) v)) 4294967296) (* %s 281474976710656))))" % (HW, CNT, AR),
                 "(steps ((unfold hword lhs)) refl)")
            have("hw1lo", "(= (le 0 (+ %s 1)) True)" % HW, _rows(("goal", 1), ("hwlo", 1)))
            have("hw1hi", "(= (lt (+ %s 1) 18446744073709551616) True)" % HW,
                 _rows(("goal", 1), ("hwe", -1), (P["cnt"], 1), ("hthi", 4294967296), ("hahi", 281474976710656)))
            have("hmw1", "(= (mod (+ %s 1) 18446744073709551616) (+ %s 1))" % (HW, HW),
                 _cite("modu64_id", [], [_pr("hw1lo"), _pr("hw1hi")]))

    # ---- the steps ----
    C = "(compute lhs (stop iw8 hword hfget gcells_of hcount_of))"
    st = [C, _rw(P["dd"]), "(reduce lhs)", C, _rw("hmv"), C, _rw("hb1"), _rw("hm2"), C]
    if not imm:
        guard = [_rw("hpz"), _rw("hmv"), _rw("hbv"), _rw("hgl"), "(reduce lhs)", _rw("hgh"), "(reduce lhs)", C]
        st += guard + [_rw("hrd"), _rw("hcnt"), _rw("hnlt" if sat else P["cnt"]), C]
        if not sat:
            st += guard + [_rw("hmw1"), C]
    out.append("    (steps")
    out.append("      (" + "\n       ".join(st) + ")")
    out.append("      refl)))")
    return "\n".join(out)


def inc_block():
    return "\n\n".join(inc_leg(k) for k in ("imm", "ref", "sat"))


BANNER_INC = ";; --- THE INC ENGINE LEGS (generated by gen_rth.py inc — REGENERATE, never hand-patch) ---"
BLOCKS.append(("inc", BANNER_INC, inc_block))


# ---------------- dec: the three SIMPLE legs of rt_dec's law (C2b-5 part i) ----------------
# rt_dec's prefix is rt_inc's (the parity test, the header load, the count band,
# the immortal test); below the band the code splits once more on 1 < count:
# the SHARED leg stores hword − 1 (the exact mirror of inc's +1), the LAST leg
# (count = 1) is the cascade — parts ii/iii, not generated here.
DEC_TOWER = 13


def dec_leg(kind):
    imm = kind == "imm"
    sat = kind == "sat"
    name = {"imm": "rth_dec_imm_run", "shared": "rth_dec_shared_run", "sat": "rth_dec_sat_run"}[kind]
    doc = {"imm": "the IMMEDIATE leg: v odd, nothing moves",
           "shared": "the SHARED leg below the band: the one header store [v] := hword − 1",
           "sat": "the SATURATED leg: the count is in the immortal band, nothing moves"}[kind]
    HW = "(hword (hfget (gcells_of g) v))"
    CNT = "(hcount_of (hfget (gcells_of g) v))"
    AR = "(harity_of (hfget (gcells_of g) v))"
    T = DEC_TOWER
    if imm:
        prem = [("odd", "(= (int_eq (mod v 2) 1) True)"),
                ("v0", "(= (le 0 v) True)"),
                ("v64", "(= (lt v 18446744073709551616) True)"),
                ("dd", "(= (lt d dmax) True)")]
        binders = "(fs (List IpFn)) (mlo Int) (msz Int) (dmax Int) (d Int) (rb Int) (m Mem) (v Int) (fuel Nat)"
        concl = "(Some (IpRv 0 m))"
    else:
        prem = [("hr", "(= (heap_rep m rb g) True)"),
                ("hi", "(= (hinv rb g r) True)"),
                ("mem", "(= (memb (haddrs (gcells_of g)) v) True)"),
                ("cnt", "(= (lt %s 2147483648) True)" % CNT if not sat else "(= (le 2147483648 %s) True)" % CNT)]
        if not sat:
            prem.append(("cnt1", "(= (lt 1 %s) True)" % CNT))
        prem += [("mlo", "(= (le mlo rb) True)"),
                 ("msz", "(= (le (gend_of g) msz) True)"),
                 ("dd", "(= (lt d dmax) True)")]
        binders = ("(fs (List IpFn)) (mlo Int) (msz Int) (dmax Int) (d Int) (rb Int) (g GHeap) "
                   "(r (List Int)) (m Mem) (v Int) (fuel Nat)")
        concl = "(Some (IpRv 0 m))" if sat else "(Some (IpRv 0 (store_le (iw8) m v (- %s 1))))" % HW
    P = {k: i for i, (k, _) in enumerate(prem)}
    NP = len(prem)
    out = []
    out.append(f";; {doc}")
    out.append(f"(claim {name}")
    out.append("  (goal")
    out.append(f"    ({binders})")
    out.append("    (" + "\n     ".join(x for _, x in prem) + ")")
    out.append("    (=")
    out.append(f"      (ipcall (nS {T} (ntl {T} fuel)) (rt_app (rt_fns rb) fs) mlo msz dmax d 3 (Cons v Nil) m)")
    out.append(f"      {concl}))")
    out.append("  (chain")
    haves = []

    def have(nm, goal, proof):
        out.append(f"    (have {nm} {goal}")
        out.append(f"      {proof})")
        haves.append(nm)

    def ident(nm, goal):
        w = 1 + NP + len(haves)
        z = " ".join(["1"] + ["0"] * (w - 1))
        have(nm, goal, f"(by arith (list (list {z}) (list {z})))")

    if imm:
        have("hm2", "(= (mod v 2) 1)", _cite("int_eq_eq", ["(inst b 1)"], [_pr(P["odd"])]))
        have("hb1", "(= (band v 1) (mod v 2))", _cite("band1_mod2", [], [_pr(P["v0"])]))
        have("hmv", "(= (mod v 18446744073709551616) v)", _cite("modu64_id", [], [_pr(P["v0"]), _pr(P["v64"])]))
    else:
        ident("hpz", "(= (+ v 0) v)")
        have("hlv", "(= (live_ok g v) True)", _cite("live_facts", ["(inst rb rb)", "(inst r r)"], [_pr(P["hi"]), _pr(P["mem"])]))
        have(*_lv("hlo", "(= (le (glo_of g) v) True)", "lv_lo"))
        have(*_lv("hhi", "(= (le (+ v (+ 8 (* 8 %s))) (gtop_of g)) True)" % AR, "lv_hi"))
        have(*_lv("hal", "(= (int_eq (mod v 8) 0) True)", "lv_al", ["(inst g g)"]))
        have(*_lv("hcb", "(= (cbands (hfget (gcells_of g) v)) True)", "lv_bands"))
        have("hn0", "(= (le 0 %s) True)" % AR, _cite("cb_ar0", [], [_pr("hcb")]))
        have("h1", "(= (le 0 rb) True)", _cite("hi_rb0", ["(inst g g)", "(inst r r)"], [_pr(P["hi"])]))
        have("h2", "(= (le (+ rb 152) (glo_of g)) True)", _cite("hi_hdr", ["(inst r r)"], [_pr(P["hi"])]))
        have("h4", "(= (le (gtop_of g) (gend_of g)) True)", _cite("hi_te", ["(inst rb rb)", "(inst r r)"], [_pr(P["hi"])]))
        have("h5", "(= (le (gend_of g) 4294967296) True)", _cite("hi_e32", ["(inst rb rb)", "(inst r r)"], [_pr(P["hi"])]))
        have("hv0", "(= (le 0 v) True)", _rows(("goal", 1), ("h1", 1), ("h2", 1), ("hlo", 1)))
        have("hv32", "(= (lt v 4294967296) True)", _rows(("goal", 1), ("hhi", 1), ("h4", 1), ("h5", 1), ("hn0", 8)))
        have("hv64", "(= (lt v 18446744073709551616) True)", _rows(("goal", 1), ("hhi", 1), ("h4", 1), ("h5", 1), ("hn0", 8)))
        have("hev", "(= (int_eq (mod v 2) 0) True)", _cite("mod8_even", [], [_pr("hal")]))
        have("hm2", "(= (mod v 2) 0)", _cite("int_eq_eq", ["(inst b 0)"], [_pr("hev")]))
        have("hb1", "(= (band v 1) (mod v 2))", _cite("band1_mod2", [], [_pr("hv0")]))
        have("hmv", "(= (mod v 18446744073709551616) v)", _cite("modu64_id", [], [_pr("hv0"), _pr("hv64")]))
        have("hbv", "(= (band v 4294967295) v)", _cite("bandu32_id", [], [_pr("hv0"), _pr("hv32")]))
        have("hgl", "(= (le mlo v) True)", _rows(("goal", 1), (P["mlo"], 1), ("h2", 1), ("hlo", 1)))
        have("hgh", "(= (le (+ v 8) msz) True)", _rows(("goal", 1), ("hhi", 1), ("h4", 1), (P["msz"], 1), ("hn0", 8)))
        have("hrd", "(= (load_le (iw8) m v) %s)" % HW, _cite("hr_rd_hdr", ["(inst rb rb)", "(inst g g)"], [_pr(P["hr"]), _pr(P["mem"])]))
        have("hcnt", "(= (band %s 4294967295) %s)" % (HW, CNT), _cite("hword_count", [], [_pr("hcb")]))
        if sat:
            have("hnlt", "(= (lt %s 2147483648) False)" % CNT, _rows(("goal", 1), (P["cnt"], 1)))
        else:
            have("hwhi", "(= (lt %s 18446744073709551616) True)" % HW, _cite("hword_hi", [], [_pr("hcb")]))
            have("hc1", "(= (le 1 %s) True)" % CNT, _cite("cb_cnt1", [], [_pr("hcb")]))
            have("ht0", "(= (le 0 (htag_of (hfget (gcells_of g) v))) True)", _cite("cb_tag0", [], [_pr("hcb")]))
            have("hwe", "(= %s (+ %s (+ (* (htag_of (hfget (gcells_of g) v)) 4294967296) (* %s 281474976710656))))" % (HW, CNT, AR),
                 "(steps ((unfold hword lhs)) refl)")
            have("hw1lo", "(= (le 0 (- %s 1)) True)" % HW,
                 _rows(("goal", 1), ("hwe", 1), ("hc1", 1), ("ht0", 4294967296), ("hn0", 281474976710656)))
            have("hw1hi", "(= (lt (- %s 1) 18446744073709551616) True)" % HW, _rows(("goal", 1), ("hwhi", 1)))
            have("hmw1", "(= (mod (- %s 1) 18446744073709551616) (- %s 1))" % (HW, HW),
                 _cite("modu64_id", [], [_pr("hw1lo"), _pr("hw1hi")]))

    C = "(compute lhs (stop iw8 hword hfget gcells_of hcount_of))"
    st = [C, _rw(P["dd"]), "(reduce lhs)", C, _rw("hmv"), C, _rw("hb1"), _rw("hm2"), C]
    if not imm:
        guard = [_rw("hpz"), _rw("hmv"), _rw("hbv"), _rw("hgl"), "(reduce lhs)", _rw("hgh"), "(reduce lhs)", C]
        st += guard + [_rw("hrd"), _rw("hcnt"), _rw("hnlt" if sat else P["cnt"]), C]
        if not sat:
            st += [_rw(P["cnt1"]), C] + guard + [_rw("hmw1"), C]
    out.append("    (steps")
    out.append("      (" + "\n       ".join(st) + ")")
    out.append("      refl)))")
    return "\n".join(out)


def dec_block():
    return "\n\n".join(dec_leg(k) for k in ("imm", "shared", "sat"))


BANNER_DEC = ";; --- THE DEC ENGINE LEGS: the simple three (generated by gen_rth.py dec — REGENERATE, never hand-patch) ---"
BLOCKS.append(("dec", BANNER_DEC, dec_block))


# ---------------- slots: ONE ITERATION of rt_dec's inner loop (C2b-5 part ii) ----------------
# The body in its COMPUTED spelling (a citation matches syntactically against the
# goal after compute; the rt.shard helper spelling would never match), over an
# EXPLICIT locals list (0 v, 1 h, 2 c, 3 hd = the worklist
# head, 4 a = the dying cell, 5 n = its arity, 6 i = the slot index, 7..10
# scratch), at the slot word w = [a + 8 + 8i], in the code's four cases:
# odd (skip), immortal (skip), shared ([w] := hword − 1), dying ([w] :=
# hword − count + hd; hd := w).  The loop lemma composes these by
# induction on the remaining slots (hand-written, part ii-c); the fuel is a
# LITERAL S-tower over a Nat binder so the loop lemma can cite a leg at any
# remainder.
SLOTS_TOWER = 12
SLOTS_BODY = """(Cons (IpLoadW 7 (ITrunc U64 U32 (IBin U64 IAdd (ILoc 4) (IBin U64 IAdd (IConst 8) (IBin U64 IMul (ILoc 6) (IConst 8)))))) (Cons (IpIf (IBin U64 IEq (IBin U64 IAnd (ILoc 7) (IConst 1)) (IConst 0)) (Cons (IpLoadW 8 (ITrunc U64 U32 (IBin U64 IAdd (ILoc 7) (IConst 0)))) (Cons (IpSet 9 (IBin U64 IAnd (ILoc 8) (IConst 4294967295))) (Cons (IpIf (IBin U64 ILt (ILoc 9) (IConst 2147483648)) (Cons (IpIf (IBin U64 ILt (IConst 1) (ILoc 9)) (Cons (IpStoreW (ITrunc U64 U32 (IBin U64 IAdd (ILoc 7) (IConst 0))) (IBin U64 ISub (ILoc 8) (IConst 1))) Nil) (Cons (IpStoreW (ITrunc U64 U32 (IBin U64 IAdd (ILoc 7) (IConst 0))) (IBin U64 IAdd (IBin U64 ISub (ILoc 8) (ILoc 9)) (ILoc 3))) (Cons (IpSet 3 (ILoc 7)) Nil))) Nil) Nil) Nil))) Nil) (Cons (IpSet 6 (IBin U64 IAdd (ILoc 6) (IConst 1))) Nil)))"""


def slots_lc(hd="hd", i="i", x7="x7", x8="x8", x9="x9"):
    return (f"(Cons v (Cons h (Cons c (Cons {hd} (Cons a (Cons n (Cons {i} (Cons {x7} "
            f"(Cons {x8} (Cons {x9} (Cons x10 Nil)))))))))))")


def slots_leg(kind):
    odd, imm, shared, dying = (kind == k for k in ("odd", "imm", "shared", "dying"))
    name = "rth_slot_%s_run" % kind
    doc = {"odd": "an ODD slot: an immediate, nothing moves",
           "imm": "an IMMORTAL child: its count is in the band, nothing moves",
           "shared": "a SHARED child: its header loses one",
           "dying": "a DYING child: its header takes the link, it becomes the head"}[kind]
    M = "18446744073709551616"
    AD = "(+ a (+ 8 (* i 8)))"
    HW, CNT = "(hword d)", "(hcount_of d)"
    prem = [("pld", "(= (load_le (iw8) m %s) w)" % AD),
            ("par", "(= (mod w 2) %s)" % ("1" if odd else "0")),
            ("pa0", "(= (le 0 a) True)"), ("pi0", "(= (le 0 i) True)"),
            ("pad32", "(= (lt %s 4294967296) True)" % AD),
            ("pmlo", "(= (le mlo %s) True)" % AD), ("pmsz", "(= (le (+ %s 8) msz) True)" % AD),
            ("pw0", "(= (le 0 w) True)"), ("pw64", "(= (lt w %s) True)" % M)]
    binders = ("(fs (List IpFn)) (mlo Int) (msz Int) (dmax Int) (dd Int) (m Mem) (f Nat) (v Int) (h Int) (c Int) "
               "(hd Int) (a Int) (n Int) (i Int) (x7 Int) (x8 Int) (x9 Int) (x10 Int) (w Int)")
    if not odd:
        binders += " (d HCell)"
        prem += [("phd", "(= (load_le (iw8) m w) %s)" % HW), ("pcb", "(= (cbands d) True)"),
                 ("pw32", "(= (lt w 4294967296) True)"),
                 ("pwlo", "(= (le mlo w) True)"), ("pwhi", "(= (le (+ w 8) msz) True)")]
        if imm:
            prem.append(("pcnt", "(= (le 2147483648 %s) True)" % CNT))
        else:
            prem.append(("pcnt", "(= (lt %s 2147483648) True)" % CNT))
            prem.append(("pcnt1", "(= (lt 1 %s) %s)" % (CNT, "True" if shared else "False")))
        if dying:
            prem += [("phd0", "(= (le 0 hd) True)"), ("phd32", "(= (lt hd 4294967296) True)")]
    P = {k: i for i, (k, _) in enumerate(prem)}
    lc0 = slots_lc()
    if odd:
        lc1, m1 = slots_lc(i="(+ i 1)", x7="w"), "m"
    elif imm:
        lc1, m1 = slots_lc(i="(+ i 1)", x7="w", x8=HW, x9=CNT), "m"
    elif shared:
        lc1, m1 = slots_lc(i="(+ i 1)", x7="w", x8=HW, x9=CNT), "(store_le (iw8) m w (- %s 1))" % HW
    else:
        lc1, m1 = slots_lc(hd="w", i="(+ i 1)", x7="w", x8=HW, x9=CNT), "(store_le (iw8) m w (+ (- %s %s) hd))" % (HW, CNT)
    tower = "(S " * SLOTS_TOWER + "f" + ")" * SLOTS_TOWER
    out = [f";; {doc}", f"(claim {name}", "  (goal", f"    ({binders})",
           "    (" + "\n     ".join(x for _, x in prem) + ")",
           "    (=", f"      (ipstmts {tower} fs mlo msz dmax dd", f"        {SLOTS_BODY}",
           f"        {lc0} m)", f"      (Some (IpNorm {lc1} {m1}))))", "  (chain"]
    haves = []

    def have(nm, goal, proof):
        out.append(f"    (have {nm} {goal}")
        out.append(f"      {proof})")
        haves.append(nm)

    def ident(nm, goal):
        w = 1 + len(prem) + len(haves)
        z = " ".join(["1"] + ["0"] * (w - 1))
        have(nm, goal, f"(by arith (list (list {z}) (list {z})))")

    # the slot address a + 8 + 8i through the U64 ring and the U32 truncation
    have("hi8lo", "(= (le 0 (* i 8)) True)", _rows(("goal", 1), (P["pi0"], 8)))
    have("hi8hi", "(= (lt (* i 8) %s) True)" % M, _rows(("goal", 1), (P["pad32"], 1), (P["pa0"], 1)))
    have("hm8", "(= (mod (* i 8) %s) (* i 8))" % M, _cite("modu64_id", [], [_pr("hi8lo"), _pr("hi8hi")]))
    have("h8lo", "(= (le 0 (+ 8 (* i 8))) True)", _rows(("goal", 1), (P["pi0"], 8)))
    have("h8hi", "(= (lt (+ 8 (* i 8)) %s) True)" % M, _rows(("goal", 1), (P["pad32"], 1), (P["pa0"], 1)))
    have("hm8b", "(= (mod (+ 8 (* i 8)) %s) (+ 8 (* i 8)))" % M, _cite("modu64_id", [], [_pr("h8lo"), _pr("h8hi")]))
    have("hadlo", "(= (le 0 %s) True)" % AD, _rows(("goal", 1), (P["pa0"], 1), (P["pi0"], 8)))
    have("hadhi", "(= (lt %s %s) True)" % (AD, M), _rows(("goal", 1), (P["pad32"], 1)))
    have("hmad", "(= (mod %s %s) %s)" % (AD, M, AD), _cite("modu64_id", [], [_pr("hadlo"), _pr("hadhi")]))
    have("hbad", "(= (band %s 4294967295) %s)" % (AD, AD), _cite("bandu32_id", [], [_pr("hadlo"), _pr(P["pad32"])]))
    have("hi1lo", "(= (le 0 (+ i 1)) True)", _rows(("goal", 1), (P["pi0"], 1)))
    have("hi1hi", "(= (lt (+ i 1) %s) True)" % M, _rows(("goal", 8), (P["pad32"], 1), (P["pa0"], 1)))
    have("hi1", "(= (mod (+ i 1) %s) (+ i 1))" % M, _cite("modu64_id", [], [_pr("hi1lo"), _pr("hi1hi")]))
    have("hb1", "(= (band w 1) (mod w 2))", _cite("band1_mod2", [], [_pr(P["pw0"])]))
    if not odd:
        ident("hpz", "(= (+ w 0) w)")
        have("hmw", "(= (mod w %s) w)" % M, _cite("modu64_id", [], [_pr(P["pw0"]), _pr(P["pw64"])]))
        have("hbw", "(= (band w 4294967295) w)", _cite("bandu32_id", [], [_pr(P["pw0"]), _pr(P["pw32"])]))
        have("hcnt", "(= (band %s 4294967295) %s)" % (HW, CNT), _cite("hword_count", [], [_pr(P["pcb"])]))
        if imm:
            have("hnlt", "(= (lt %s 2147483648) False)" % CNT, _rows(("goal", 1), (P["pcnt"], 1)))
        else:
            have("hwhi", "(= (lt %s %s) True)" % (HW, M), _cite("hword_hi", [], [_pr(P["pcb"])]))
            have("hc1", "(= (le 1 %s) True)" % CNT, _cite("cb_cnt1", [], [_pr(P["pcb"])]))
            have("ht0", "(= (le 0 (htag_of d)) True)", _cite("cb_tag0", [], [_pr(P["pcb"])]))
            have("ha0", "(= (le 0 (harity_of d)) True)", _cite("cb_ar0", [], [_pr(P["pcb"])]))
            have("hthi", "(= (lt (htag_of d) 65536) True)", _cite("cb_taghi", [], [_pr(P["pcb"])]))
            have("hahi", "(= (lt (harity_of d) 65536) True)", _cite("cb_arhi", [], [_pr(P["pcb"])]))
            have("hwe", "(= %s (+ %s (+ (* (htag_of d) 4294967296) (* (harity_of d) 281474976710656))))" % (HW, CNT),
                 "(steps ((unfold hword lhs)) refl)")
            if shared:
                have("hw1lo", "(= (le 0 (- %s 1)) True)" % HW,
                     _rows(("goal", 1), ("hwe", 1), ("hc1", 1), ("ht0", 4294967296), ("ha0", 281474976710656)))
                have("hw1hi", "(= (lt (- %s 1) %s) True)" % (HW, M), _rows(("goal", 1), ("hwhi", 1)))
                have("hmw1", "(= (mod (- %s 1) %s) (- %s 1))" % (HW, M, HW), _cite("modu64_id", [], [_pr("hw1lo"), _pr("hw1hi")]))
            else:
                have("hwclo", "(= (le 0 (- %s %s)) True)" % (HW, CNT),
                     _rows(("goal", 1), ("hwe", 1), ("ht0", 4294967296), ("ha0", 281474976710656)))
                have("hwchi", "(= (lt (- %s %s) %s) True)" % (HW, CNT, M), _rows(("goal", 1), ("hwhi", 1), ("hc1", 1)))
                have("hwc", "(= (mod (- %s %s) %s) (- %s %s))" % (HW, CNT, M, HW, CNT), _cite("modu64_id", [], [_pr("hwclo"), _pr("hwchi")]))
                have("hwdlo", "(= (le 0 (+ (- %s %s) hd)) True)" % (HW, CNT),
                     _rows(("goal", 1), ("hwe", 1), ("ht0", 4294967296), ("ha0", 281474976710656), (P["phd0"], 1)))
                have("hwdhi", "(= (lt (+ (- %s %s) hd) %s) True)" % (HW, CNT, M),
                     _rows(("goal", 1), ("hwe", -1), ("hthi", 4294967296), ("hahi", 281474976710656), (P["phd32"], 1)))
                have("hwd", "(= (mod (+ (- %s %s) hd) %s) (+ (- %s %s) hd))" % (HW, CNT, M, HW, CNT),
                     _cite("modu64_id", [], [_pr("hwdlo"), _pr("hwdhi")]))
    C = "(compute lhs (stop iw8 ntl load_le store_le hword hcount_of htag_of harity_of whead))"
    st = [C, _rw("hm8"), _rw("hm8b"), _rw("hmad"), _rw("hbad"), _rw(P["pmlo"]), "(reduce lhs)", _rw(P["pmsz"]), "(reduce lhs)",
          C, _rw(P["pld"]), C, _rw("hb1"), _rw(P["par"]), C]
    if not odd:
        wguard = [_rw("hpz"), _rw("hmw"), _rw("hbw"), _rw(P["pwlo"]), "(reduce lhs)", _rw(P["pwhi"]), "(reduce lhs)", C]
        st += wguard + [_rw(P["phd"]), C, _rw("hcnt"), C, _rw("hnlt" if imm else P["pcnt"]), C]
        if shared:
            st += [_rw(P["pcnt1"]), C] + wguard + [_rw("hmw1"), C]
        elif dying:
            st += [_rw(P["pcnt1"]), C] + wguard + [_rw("hwc"), _rw("hwd"), C]
    st += [_rw("hi1"), C]
    out.append("    (steps")
    out.append("      (" + "\n       ".join(st) + ")")
    out.append("      refl)))")
    return "\n".join(out)


def slots_block():
    return "\n\n".join(slots_leg(k) for k in ("odd", "imm", "shared", "dying"))


BANNER_SLOTS = ";; --- THE INNER-LOOP STEP LEGS (generated by gen_rth.py slots — REGENERATE, never hand-patch) ---"
BLOCKS.append(("slots", BANNER_SLOTS, slots_block))


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
