#!/usr/bin/env python3
"""models/imp/probes/gen_fra4.py — the A-4 (control) emitter for the GENERATED
claims of models/imp/probes/fra_kit.shard (Theorem A, docs/COVERAGE.md §11.1
slice A-4): the side-predicate extractions, the twin's two laws (memory,
context) by the request-dispatcher induction, the control step lemmas, and
the dispatcher induction itself (ipt_sound). Same status and law as
gen_fra.py (in-tree, banner-marked, NEVER hand-patched; #18 / #27 note there).

RUN (from the repo root):
  python3 models/imp/probes/gen_fra4.py extract | twin | ctl | sound    (stdout)
  python3 models/imp/probes/gen_fra4.py splice    (rewrites the A-4 blocks in place)
"""
import sys, os
sys.path.insert(0, os.path.dirname(__file__))
from gen_fra import Slots

PATH = "models/imp/probes/fra_kit.shard"

# ---------------- the side-predicate extractions ----------------
# Each list predicate P over (Cons s t) is (if (Ps s) (P t) False); the per-statement
# Ps unfolds one layer. An extraction case-splits its target Bool: True is the
# goal, False collapses P (Cons S t) to False against the premise — no match
# term is ever spelled. `nff` = how many (if x False False) collapses (if_ff) the
# collapse needs after the first reduce.

PREDS = {
    "a4":   ("ixf_a4",   "ixf_a4s",   "",     ""),
    "scb":  ("ixf_scb",  "ixf_scbs",  "",     ""),
    "skok": ("ixf_skok", "ixf_skoks", "(k Int) ", "k "),
}
IF_S  = "(IpIf ce tb eb)"
WH_S  = "(IpWhile ce b)"
SET_S = "(IpSet i e)"
VARS_IF = "(ce IExp) (tb (List IpStmt)) (eb (List IpStmt)) "
VARS_WH = "(ce IExp) (b (List IpStmt)) "
VARS_SET = "(i Int) (e IExp) "

def extraction(P, Ps, kv, ka, name, svars, S, target, collapse, pre="", presteps=""):
    """P (Cons S t) = True  ->  target = True.  `collapse` = the rewrites that fold the
    false-branch term to False after the target is rewritten (reduce does not enter the
    branches of a stuck if: if_f folds an inner (if False a b), if_ff an (if x False False))"""
    ff = "".join(f" (rewrite (lemma {l}) lr lhs true ())" for l in collapse)
    return f"""(claim {name}
  (goal ({kv}{svars}(t (List IpStmt))) ((= ({P} {ka}(Cons {S} t)) True)) (= {target} True))
  (case-on {target} Bool
    ((case True (steps ((rewrite (hyp True) lr lhs true ())) refl))
     (case False
       (chain{pre}
         (have hn (= ({P} {ka}(Cons {S} t)) False)
           (steps ((unfold {P} lhs) (reduce lhs) (unfold {Ps} lhs) (reduce lhs){presteps} (rewrite (hyp False) lr lhs true ()){ff} (reduce lhs)) refl))
         (have hn2 (= False True) (steps ((rewrite (premise hn) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
         (absurd (premise hn2)))))))
"""

def extraction_head(P, Ps, kv, ka, name):
    """P (Cons s t) = True  ->  P (Cons s Nil) = True"""
    return f"""(claim {name}
  (goal ({kv}(s IpStmt) (t (List IpStmt))) ((= ({P} {ka}(Cons s t)) True)) (= ({P} {ka}(Cons s Nil)) True))
  (case-on ({Ps} {ka}s) Bool
    ((case True
       (steps ((unfold {P} lhs) (reduce lhs) (rewrite (hyp True) lr lhs true ()) (reduce lhs) (unfold {P} lhs) (reduce lhs)) refl))
     (case False
       (chain
         (have hn (= ({P} {ka}(Cons s t)) False)
           (steps ((unfold {P} lhs) (reduce lhs) (rewrite (hyp False) lr lhs true ()) (reduce lhs)) refl))
         (have hn2 (= False True) (steps ((rewrite (premise hn) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
         (absurd (premise hn2)))))))
"""

def extractions():
    out = []
    for key,(P,Ps,kv,ka) in PREDS.items():
        Pk = f"{P} {ka}"
        out.append(extraction(P,Ps,kv,ka,f"{key}_tail","(s IpStmt) ","s",f"({Pk}t)",["if_ff"]))
        out.append(extraction_head(P,Ps,kv,ka,f"{key}_head"))
        if key == "a4":
            out.append(extraction(P,Ps,kv,ka,f"{key}_if_t",VARS_IF,IF_S,f"({Pk}tb)",[]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_if_e",VARS_IF,IF_S,f"({Pk}eb)",["if_ff"]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_while_b",VARS_WH,WH_S,f"({Pk}b)",[]))
        if key == "scb":
            out.append(extraction(P,Ps,kv,ka,f"{key}_set_e",VARS_SET,SET_S,"(ixf_cb e)",[]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_if_c",VARS_IF,IF_S,"(ixf_cb ce)",[]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_if_t",VARS_IF,IF_S,f"({Pk}tb)",["if_f","if_ff"]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_if_e",VARS_IF,IF_S,f"({Pk}eb)",["if_ff","if_ff"]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_while_c",VARS_WH,WH_S,"(ixf_cb ce)",[]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_while_b",VARS_WH,WH_S,f"({Pk}b)",["if_ff"]))
        if key == "skok":
            out.append(extraction(P,Ps,kv,ka,f"{key}_cost","(s IpStmt) ","s","(le (ixf_scost s) k)",[]))
            # the nested targets sit under the cost test: derive it (skok_cost) and rewrite it True first
            def hc(S):
                return (f"\n         (have hc (= (le (ixf_scost {S}) k) True) (rewrite-with (lemma skok_cost) lr lhs ((inst s {S}) (inst t t)) ((steps ((rewrite (premise 0) lr lhs true ())) refl)) refl))",
                        " (rewrite (premise hc) lr lhs true ()) (reduce lhs)")
            p,ps = hc(IF_S)
            out.append(extraction(P,Ps,kv,ka,f"{key}_if_t",VARS_IF,IF_S,f"({Pk}tb)",[],p,ps))
            out.append(extraction(P,Ps,kv,ka,f"{key}_if_e",VARS_IF,IF_S,f"({Pk}eb)",["if_ff"],p,ps))
            p,ps = hc(WH_S)
            out.append(extraction(P,Ps,kv,ka,f"{key}_while_b",VARS_WH,WH_S,f"({Pk}b)",[],p,ps))
    # the spill depth: imax2 inequalities over ixf_sdep's shape
    def sdep(name, svars, S, lhs, chain_items):
        return f"""(claim {name}
  (goal ({svars}(t (List IpStmt))) () (= (le {lhs} (ixf_sdep (Cons {S} t))) True))
  (chain
{chain_items}))
"""
    M_IF = "(imax2 (ixf_dep ce) (imax2 (ixf_sdep tb) (ixf_sdep eb)))"
    M_WH = "(imax2 (ixf_dep ce) (ixf_sdep b))"
    def unf(S, M):
        return f"    (have hu (= (ixf_sdep (Cons {S} t)) (imax2 {M} (ixf_sdep t))) (steps ((unfold ixf_sdep lhs) (reduce lhs) (unfold ixf_sdeps lhs) (reduce lhs)) refl))"
    def ge(nm, a, b, side):
        lem = "imax2_ge_l" if side=="l" else "imax2_ge_r"
        other = f"(inst b {b})" if side=="l" else f"(inst a {a})"
        x = a if side=="l" else b
        return f"    (have {nm} (= (le {x} (imax2 {a} {b})) True) (rewrite-with (lemma {lem}) lr lhs ({other}) () refl))"
    out.append(sdep("sdep_tail","(s IpStmt) ","s","(ixf_sdep t)",
        "    (have hu (= (ixf_sdep (Cons s t)) (imax2 (ixf_sdeps s) (ixf_sdep t))) (steps ((unfold ixf_sdep lhs) (reduce lhs)) refl))\n"
        + ge("h1","(ixf_sdeps s)","(ixf_sdep t)","r") + "\n"
        + "    (steps ((rewrite (premise hu) lr lhs true ()) (rewrite (premise h1) lr lhs true ())) refl)"))
    out.append(sdep("sdep_set_e",VARS_SET,SET_S,"(ixf_dep e)",
        unf(SET_S,"(ixf_dep e)") + "\n" + ge("h1","(ixf_dep e)","(ixf_sdep t)","l") + "\n"
        + "    (steps ((rewrite (premise hu) lr lhs true ()) (rewrite (premise h1) lr lhs true ())) refl)"))
    for nm, x, inner in [("sdep_if_c","(ixf_dep ce)", ge("h1","(ixf_dep ce)","(imax2 (ixf_sdep tb) (ixf_sdep eb))","l")),
                         ("sdep_if_t","(ixf_sdep tb)", ge("h0","(ixf_sdep tb)","(ixf_sdep eb)","l") + "\n" + ge("h0b","(ixf_dep ce)","(imax2 (ixf_sdep tb) (ixf_sdep eb))","r") + "\n    (have h1 (= (le (ixf_sdep tb) " + M_IF + ") True) (by arith (list 1 0 1 1)))"),
                         ("sdep_if_e","(ixf_sdep eb)", ge("h0","(ixf_sdep tb)","(ixf_sdep eb)","r") + "\n" + ge("h0b","(ixf_dep ce)","(imax2 (ixf_sdep tb) (ixf_sdep eb))","r") + "\n    (have h1 (= (le (ixf_sdep eb) " + M_IF + ") True) (by arith (list 1 0 1 1)))")]:
        out.append(sdep(nm,VARS_IF,IF_S,x,
            unf(IF_S,M_IF) + "\n" + inner + "\n" + ge("h2",M_IF,"(ixf_sdep t)","l") + "\n"
            + "    (steps ((rewrite (premise hu) lr lhs true ())) (by arith (list 1 0 0 0 1 1)))" if nm!="sdep_if_c" else
            unf(IF_S,M_IF) + "\n" + inner + "\n" + ge("h2",M_IF,"(ixf_sdep t)","l") + "\n"
            + "    (steps ((rewrite (premise hu) lr lhs true ())) (by arith (list 1 0 1 1)))"))
    for nm, x, side in [("sdep_while_c","(ixf_dep ce)","l"),("sdep_while_b","(ixf_sdep b)","r")]:
        out.append(sdep(nm,VARS_WH,WH_S,x,
            unf(WH_S,M_WH) + "\n" + ge("h1","(ixf_dep ce)","(ixf_sdep b)",side) + "\n" + ge("h2",M_WH,"(ixf_sdep t)","l") + "\n"
            + "    (steps ((rewrite (premise hu) lr lhs true ())) (by arith (list 1 0 1 1)))"))
    return "\n".join(out)

BANNER_EXTRACT = ";; --- the side-predicate extractions (generated by gen_fra4.py extract — REGENERATE, never hand-patch) ---"

BLOCKS = [("extract", BANNER_EXTRACT, extractions)]

def splice(path=PATH):
    s = open(path).read()
    for key, banner, fn in BLOCKS:
        new = banner + "\n\n" + fn().rstrip("\n") + "\n"
        i = s.find(banner)
        if i < 0:
            s = s.rstrip("\n") + "\n\n" + new
            continue
        tail = s[i:]
        ends = [tail.find(b, 1) for _, b, _ in BLOCKS] + [tail.find("\n;; ====", 1)]
        ends = [e for e in ends if e > 0]
        j = i + (min(ends) if ends else len(tail))
        s = s[:i] + new + ("\n" + s[j:].lstrip("\n") if j < len(s) else "")
    open(path, "w").write(s)


# ---------------- the dispatcher skeleton (shared by the twin laws and ipt_sound) ----------------
# The imp run premise is always premise 0, at fuel f over request r; the outcome form is the
# claim's (the twin laws: (Some (IpNorm lc2 mem2))). Every arm: the case-on scaffolding that
# decomposes the engine's run (absurd where the outcome cannot be normal), then the leaf.

IM = "(fp_mem mem0 (fbelow slo psx))"
def IMx(psx): return f"(fp_mem mem0 (fbelow slo {psx}))"
def RUN(f, r, lc, im): return f"(ipt_run {f} fs mlo slo dmax dep {r} {lc} {im})"
def TR(f, r, lc, im): return f"(ipt_tr {f} fs mlo slo dmax dep fp nl {r} {lc} {im})"
def TRS(f, s, lc, im): return f"(ipt_stmt {f} fs mlo slo dmax dep fp nl {s} {lc} {im})"
def TRL(f, ss, lc, im): return f"(ipt_stmts {f} fs mlo slo dmax dep fp nl {ss} {lc} {im})"
def TRW(f, ce, b, lc, im): return f"(ipt_while {f} fs mlo slo dmax dep fp nl {ce} {b} {lc} {im})"
def ST(f, s, lc, im): return f"(ipstmt {f} fs mlo slo dmax dep {s} {lc} {im})"
def STS(f, ss, lc, im): return f"(ipstmts {f} fs mlo slo dmax dep {ss} {lc} {im})"
def WH(f, ce, b, lc, im): return f"(ipwhile {f} fs mlo slo dmax dep {ce} {b} {lc} {im})"
def FE_TR(e): return f"(fe_tr fp nl 0 {e} lc {IM} mlo slo)"
def PSX1(e): return f"(fp_app {FE_TR(e)} psx)"
OUTN = "(Some (IpNorm lc2 mem2))"

def cap(name, eq): return f"(have {name} {eq} (steps ((rewrite (hyp 0) lr lhs true ())) refl))"
def PN(h):
    """a premise reference: the tracker's pN names are the claim premises, cited by index"""
    return h[1:] if (isinstance(h, str) and h[0] == 'p' and h[1:].isdigit()) else h
def D(h): return f"(steps ((rewrite (premise {PN(h)}) lr lhs true ())) refl)"
def SUBS(hs): return "".join(f" (rewrite (premise {h}) lr rhs true ())" for h in hs)
# case-on leaves the scrutinee VARIABLE in the goal and premises (the fact is hyp 0): every
# re-spelling of premise 0 / a side premise substitutes the captured ctor equations (hr, hst, hss)

def absurd_run(engine, rewrites, lhs, subst):
    """premise 0 (the run = OUTN) against what the engine actually returns"""
    rw = "".join(f" {r} (reduce rhs)" for r in rewrites)
    body = f"""(have hn (= {lhs} {OUTN})
  (steps ((rewrite (premise 0) rl rhs true ()){SUBS(subst)} (unfold ipt_run rhs) (reduce rhs) (unfold {engine} rhs) (reduce rhs){rw}) refl))"""
    if lhs == "None":
        return f"(chain {body} (absurd (premise hn)))"
    return f"(chain {body} (inject (premise hn) (hx)) (absurd (premise hx)))"

def run_fact(engine, name, lhs, rewrites, subst):
    """have NAME (= LHS OUTN) by unfolding premise 0 through the engine and the given facts"""
    rw = "".join(f" {r} (reduce rhs)" for r in rewrites)
    return f"""(have {name} (= {lhs} {OUTN})
  (steps ((rewrite (premise 0) rl rhs true ()){SUBS(subst)} (unfold ipt_run rhs) (reduce rhs) (unfold {engine} rhs) (reduce rhs){rw}) refl))"""

RV = lambda h: f"(rewrite (premise {h}) lr rhs true ())"
SS = ['hr', 'hst']   # the statement arms' substitution
SL = ['hr', 'hss']   # the list arms'
SW = ['hr']          # the while request's

def S_(sl, *names):
    """a copy of the slot tracker extended by the haves/inject products named, in order"""
    s2 = Slots(sl.names)
    for n in names: s2.add(n)
    return s2

def arm_set(leaf, sl):
    """IqStmt (IpSet i e): hv (iexp = Some v), hset (ilset = Some lcs), hlc (lcs = lc2), hmem (IM = mem2)"""
    return f"""(case-on (iexp e lc {IM} mlo slo) Option
  ((case None (chain {cap('hv', f'(= (iexp e lc {IM} mlo slo) None)')} {absurd_run('ipstmt', [RV('hv')], '(Some IpTrap)', SS)}))
   (case Some (v)
     (chain
       {cap('hv', f'(= (iexp e lc {IM} mlo slo) (Some v))')}
       (case-on (ilset lc i v) Option
         ((case None (chain {cap('hset', '(= (ilset lc i v) None)')} {absurd_run('ipstmt', [RV('hv'), RV('hset')], '(Some IpTrap)', SS)}))
          (case Some (lcs)
            (chain
              {cap('hset', '(= (ilset lc i v) (Some lcs))')}
              {run_fact('ipstmt', 'hn1', f'(Some (IpNorm lcs {IM}))', [RV('hv'), RV('hset')], SS)}
              (inject (premise hn1) (hn2))
              (inject (premise hn2) (hlc hmem))
{leaf(S_(sl, 'hv', 'hset', 'hn1', 'hn2', 'hlc', 'hmem'))}))))))))"""

def arm_if(leaf_t, leaf_e, sl):
    """IqStmt (IpIf ce tb eb): hv (iexp ce = Some cv), hcv, hbr (the branch's ipstmts = OUTN)"""
    def br(which, flag):
        ss = "eb" if which=="e" else "tb"
        leaf = leaf_e if which=="e" else leaf_t
        return f"""(chain
  {cap('hcv', f'(= (int_eq cv 0) {flag})')}
  {run_fact('ipstmt', 'hbr', STS('f2', ss, 'lc', IM), [RV('hv'), RV('hcv')], SS)}
{leaf(S_(sl, 'hv', 'hcv', 'hbr'))})"""
    return f"""(case-on (iexp ce lc {IM} mlo slo) Option
  ((case None (chain {cap('hv', f'(= (iexp ce lc {IM} mlo slo) None)')} {absurd_run('ipstmt', [RV('hv')], '(Some IpTrap)', SS)}))
   (case Some (cv)
     (chain
       {cap('hv', f'(= (iexp ce lc {IM} mlo slo) (Some cv))')}
       (case-on (int_eq cv 0) Bool
         ((case True {br('e','True')})
          (case False {br('t','False')})))))))"""

def arm_while_stmt(leaf, sl):
    """IqStmt (IpWhile ce b): hw (ipwhile f2 = OUTN)"""
    return f"""(chain
  {run_fact('ipstmt', 'hw', WH('f2', 'ce', 'b', 'lc', IM), [], SS)}
{leaf(S_(sl, 'hw'))})"""

def arm_fenced(S):
    """the A-4 fence refuses S: ixf_a4 (Cons S Nil) = False against hb3"""
    return f"""(chain
  (have hn (= False True)
    (steps ((rewrite (premise ha4) rl rhs true ()) (unfold ixf_a4 rhs) (reduce rhs) (unfold ixf_a4s rhs) (reduce rhs)) refl))
  (absurd (premise hn)))"""

def arm_nonnorm(lhs):
    """IpFail / IpUnreach: the engine's outcome is not normal"""
    return absurd_run('ipstmt', [], lhs, SS)

def arm_nil(leaf, sl):
    return f"""(chain
  {run_fact('ipstmts', 'hn1', f'(Some (IpNorm lc {IM}))', [], SL)}
  (inject (premise hn1) (hn2))
  (inject (premise hn2) (hlc hmem))
{leaf(S_(sl, 'hn1', 'hn2', 'hlc', 'hmem'))})"""

def arm_cons(leaf, sl):
    """IqStmts (Cons s t): hs (ipstmt f2 s = Some (IpNorm lcs mems)), ht (ipstmts f2 t lcs mems = OUTN)"""
    S = ST('f2', 's', 'lc', IM)
    return f"""(case-on {S} Option
  ((case None (chain {cap('hs', f'(= {S} None)')} {absurd_run('ipstmts', [RV('hs')], 'None', SL)}))
   (case Some (o)
     (chain
       {cap('hs', f'(= {S} (Some o))')}
       (case-on o IpOut
         ((case IpNorm (lcs mems)
            (chain
              (have hs2 (= {S} (Some (IpNorm lcs mems))) (steps ((rewrite (premise hs) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
              {run_fact('ipstmts', 'ht', STS('f2', 't', 'lcs', 'mems'), [RV('hs2')], SL)}
{leaf(S_(sl, 'hs', 'hs2', 'ht'))}))
          (case IpTrap
            (chain
              (have hs2 (= {S} (Some IpTrap)) (steps ((rewrite (premise hs) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
              {absurd_run('ipstmts', [RV('hs2')], '(Some IpTrap)', SL)}))
          (case IpFailed (fam)
            (chain
              (have hs2 (= {S} (Some (IpFailed fam))) (steps ((rewrite (premise hs) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
              {absurd_run('ipstmts', [RV('hs2')], '(Some (IpFailed fam))', SL)}))))))))"""

def arm_qwhile(leaf_exit, leaf_iter, sl):
    """IqWhile ce b at (S f2): exit (cv = 0: hlc hmem) or iterate (hb: ipstmts f2 b = Some (IpNorm lcb memb), hw2: ipwhile f2 lcb memb = OUTN)"""
    B = STS('f2', 'b', 'lc', IM)
    return f"""(case-on (iexp ce lc {IM} mlo slo) Option
  ((case None (chain {cap('hv', f'(= (iexp ce lc {IM} mlo slo) None)')} {absurd_run('ipwhile', [RV('hv')], '(Some IpTrap)', SW)}))
   (case Some (cv)
     (chain
       {cap('hv', f'(= (iexp ce lc {IM} mlo slo) (Some cv))')}
       (case-on (int_eq cv 0) Bool
         ((case True
            (chain
              {cap('hcv', '(= (int_eq cv 0) True)')}
              {run_fact('ipwhile', 'hn1', f'(Some (IpNorm lc {IM}))', [RV('hv'), RV('hcv')], SW)}
              (inject (premise hn1) (hn2))
              (inject (premise hn2) (hlc hmem))
{leaf_exit(S_(sl, 'hv', 'hcv', 'hn1', 'hn2', 'hlc', 'hmem'))}))
          (case False
            (chain
              {cap('hcv', '(= (int_eq cv 0) False)')}
              (case-on {B} Option
                ((case None (chain {cap('hb', f'(= {B} None)')} {absurd_run('ipwhile', [RV('hv'), RV('hcv'), RV('hb')], 'None', SW)}))
                 (case Some (ob)
                   (chain
                     {cap('hb', f'(= {B} (Some ob))')}
                     (case-on ob IpOut
                       ((case IpNorm (lcb memb)
                          (chain
                            (have hb2 (= {B} (Some (IpNorm lcb memb))) (steps ((rewrite (premise hb) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
                            {run_fact('ipwhile', 'hw2', WH('f2', 'ce', 'b', 'lcb', 'memb'), [RV('hv'), RV('hcv'), RV('hb2')], SW)}
{leaf_iter(S_(sl, 'hv', 'hcv', 'hb', 'hb2', 'hw2'))}))
                        (case IpTrap
                          (chain
                            (have hb2 (= {B} (Some IpTrap)) (steps ((rewrite (premise hb) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
                            {absurd_run('ipwhile', [RV('hv'), RV('hcv'), RV('hb2')], '(Some IpTrap)', SW)}))
                        (case IpFailed (fam)
                          (chain
                            (have hb2 (= {B} (Some (IpFailed fam))) (steps ((rewrite (premise hb) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
                            {absurd_run('ipwhile', [RV('hv'), RV('hcv'), RV('hb2')], '(Some (IpFailed fam))', SW)}))))))))))))))))"""

def z_case():
    """fuel Z: every engine returns None"""
    def one(ctor, binders, engine):
        b = f" ({binders})" if binders else ""
        return f"(case {ctor}{b} (chain {cap('hr', f'(= r ({ctor} {binders}))')} {absurd_run(engine, [], 'None', SW)}))"
    return f"""(case-on r IptReq
  ({one('IqStmt','s','ipstmt')}
   {one('IqStmts','ss','ipstmts')}
   {one('IqWhile','ce b','ipwhile')}))"""

def hb3(P, body_list, subst):
    """the request-level side premise re-spelled on the body list"""
    return f"(have {P['h']} (= ({P['fn']} {body_list}) True) (steps ((rewrite (premise {P['p']}) rl rhs true ()){SUBS(subst)} (unfold ixt_body rhs) (reduce rhs)) refl))"

def goal_subst(hs):
    return "(steps (" + " ".join(f"(rewrite (premise {h}) lr lhs true ())" for h in hs) + "))"

def dispatch(name, vars_, premises, concl, leaves, sides):
    """leaves: dict arm -> fn(sl) -> text (built with the arm's facts in scope);
    sides: list of dicts {fn, p, h} — the request-level list predicates re-spelled as hb* per arm"""
    sl0 = Slots([f"p{i}" for i in range(len(premises))])
    side_names = [P['h'] for P in sides]
    def hbs(body_list, subst):
        return "\n".join("  " + hb3(P, body_list, subst) for P in sides)
    def stmt_arm(ctor, binders, S, body):
        b = f" ({binders})" if binders else ""
        return f"""(case {ctor}{b} (chain
  {cap('hst', f'(= s {S})')}
  {goal_subst(['hst'])}
{hbs(f'(Cons {S} Nil)', SS)}
  {body}))"""
    slS = S_(sl0, 'hr', 'hst', *side_names)
    slL = S_(sl0, 'hr', 'hss', *side_names)
    slN = S_(sl0, 'hr', 'hss')
    slW = S_(sl0, 'hr', *side_names)
    prem = "\n     ".join(premises)
    return f"""(claim {name}
  (goal ({vars_})
    ({prem})
    {concl})
  (induct f
    ((case Z {z_case()})
     (case S (f2)
       (case-on r IptReq
         ((case IqStmt (s)
            (chain
              {cap('hr', '(= r (IqStmt s))')}
              {goal_subst(['hr'])}
              (case-on s IpStmt
                ({stmt_arm('IpSet', 'i e', '(IpSet i e)', arm_set(leaves['set'], slS))}
                 {stmt_arm('IpStore', 'ae ve', '(IpStore ae ve)', arm_fenced('(IpStore ae ve)'))}
                 {stmt_arm('IpIf', 'ce tb eb', '(IpIf ce tb eb)', arm_if(leaves['if_t'], leaves['if_e'], slS))}
                 {stmt_arm('IpWhile', 'ce b', '(IpWhile ce b)', arm_while_stmt(leaves['while'], slS))}
                 {stmt_arm('IpCall', 'i k args', '(IpCall i k args)', arm_fenced('(IpCall i k args)'))}
                 {stmt_arm('IpLoadW', 'i ae', '(IpLoadW i ae)', arm_fenced('(IpLoadW i ae)'))}
                 {stmt_arm('IpStoreW', 'ae ve', '(IpStoreW ae ve)', arm_fenced('(IpStoreW ae ve)'))}
                 {stmt_arm('IpFail', 'fam', '(IpFail fam)', arm_nonnorm('(Some (IpFailed fam))'))}
                 {stmt_arm('IpUnreach', '', 'IpUnreach', arm_nonnorm('(Some IpTrap)'))}))))
          (case IqStmts (ss)
            (chain
              {cap('hr', '(= r (IqStmts ss))')}
              {goal_subst(['hr'])}
              (case-on ss List
                ((case Nil (chain {cap('hss', '(= ss Nil)')} {goal_subst(['hss'])} {arm_nil(leaves['nil'], slN)}))
                 (case Cons (s t) (chain
                   {cap('hss', '(= ss (Cons s t))')}
                   {goal_subst(['hss'])}
{hbs('(Cons s t)', SL)}
                   {arm_cons(leaves['cons'], slL)}))))))
          (case IqWhile (ce b)
            (chain
              {cap('hr', '(= r (IqWhile ce b))')}
              {goal_subst(['hr'])}
{hbs('(Cons (IpWhile ce b) Nil)', SW)}
              {arm_qwhile(leaves['qwhile_exit'], leaves['qwhile_iter'], slW)}))))))))
"""

# ---------------- T1: the twin's memory law ----------------
# imp's memory after a normal run = the base under the twin's patches that lie below slo

T1_VARS = ("(f Nat) (r IptReq) (fs (List IpFn)) (mlo Int) (slo Int) (dmax Int) (dep Int) (fp Int) (nl Int) "
           "(lc (List Int)) (mem0 Mem) (psx (List FPatch)) (lc2 (List Int)) (mem2 Mem)")
def T1C(f, r, lc, im, psx, mem2): return f"(= (fp_mem mem0 (fbelow slo (fp_app {TR(f, r, lc, im)} {psx}))) {mem2})"
T1_SIDES = [{"fn": "ixf_a4", "p": 3, "h": "ha4"}]

def t1_ih(hname, r, lc, psx, mem2, d0, d3):
    """the IH at f2: r, lc, psx, with the twin at IMx(psx); d0/d3 = the run/fence discharges"""
    return f"""(have {hname} {T1C('f2', r, lc, IMx(psx), psx, mem2)}
  (rewrite-with (hyp ih) lr lhs ((inst lc2 lc2) (inst mem2 {mem2})) ({d0} {D('p1')} {D('p2')} {d3}) refl))"""

def t1_leaves():
    L = {}
    def HFB(e, sl): return f"(have hfb (= (fbelow slo {PSX1(e)}) (fbelow slo psx)) (rewrite-with (lemma fe_tr_below) lr lhs () ({D('p1')} (by arith {sl.cert({'p2':1})})) refl))"
    # IpSet: the slot word and the spills lie at or above slo
    def set_(sl):
        s2 = S_(sl, 'hi0')
        return f"""              (have hi0 (= (le 0 i) True) (rewrite-with (lemma ilset_lo) lr lhs ((inst ls lc) (inst v v) (inst ls2 lcs)) ({D('hset')}) refl))
              (have hlo8 (= (le (+ (+ fp (* 8 i)) 8) slo) False) (by arith {s2.cert({'p1':1,'hi0':8})}))
              {HFB('e', S_(s2, 'hlo8'))}
              (steps
                ((rewrite (premise hmem) rl rhs true ())
                 (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs) (unfold ips_tr lhs) (reduce lhs)
                 (rewrite (premise hv) lr lhs true ()) (reduce lhs)
                 (unfold fp_app lhs) (reduce lhs)))
              (rewrite-with (lemma fbelow_w_hi) lr lhs () ({D('hlo8')}))
              (steps ((rewrite (premise hfb) lr lhs true ())) refl)"""
    L['set'] = set_
    # IpIf: the branch's IH at the post-cond patch list
    def ifbr(ss, ext):
        def f(sl):
            d0 = f"(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hbr) lr lhs true ())) refl)"
            d3 = f"(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise hfa) lr lhs true ())) refl)"
            return f"""  {HFB('ce', sl)}
  (have hfa (= (ixf_a4 {ss}) True) (rewrite-with (lemma {ext}) lr lhs ((inst ce ce) (inst tb tb) (inst eb eb) (inst t Nil)) ({D('ha4')}) refl))
  {t1_ih('hih', f'(IqStmts {ss})', 'lc', PSX1('ce'), 'mem2', d0, d3)}
  (have hih2 {T1C('f2', f'(IqStmts {ss})', 'lc', IM, PSX1('ce'), 'mem2').replace(TR('f2', f'(IqStmts {ss})', 'lc', IM), TRL('f2', ss, 'lc', IM))}
    (steps ((rewrite (premise hih) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfb) lr rhs true ())) refl))
  (steps
    ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs)
     (rewrite (premise hv) lr lhs true ()) (reduce lhs)
     (rewrite (premise hcv) lr lhs true ()) (reduce lhs)
     (rewrite (lemma fp_app_assoc) lr lhs true ())
     (rewrite (premise hih2) lr lhs true ()))
    refl)"""
        return f
    L['if_e'] = ifbr('eb', 'a4_if_e')
    L['if_t'] = ifbr('tb', 'a4_if_t')
    # IpWhile statement: the loop request's IH at the same state
    def while_(sl):
        d0 = "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw) lr lhs true ())) refl)"
        d3 = "(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise ha4) lr lhs true ())) refl)"
        return f"""  {t1_ih('hih', '(IqWhile ce b)', 'lc', 'psx', 'mem2', d0, d3)}
  (steps
    ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs)
     (rewrite (premise hih) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs))
    refl)"""
    L['while'] = while_
    L['nil'] = lambda sl: """  (steps
    ((rewrite (premise hmem) rl rhs true ())
     (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmts lhs) (reduce lhs) (unfold fp_app lhs) (reduce lhs))
    refl)"""
    # Cons: head IH (IqStmt s at psx) gives mems; tail IH (IqStmts t at lcs, PSX') gives mem2
    def cons_(sl):
        PSXs = f"(fp_app {TR('f2', '(IqStmt s)', 'lc', IM)} psx)"
        PSXs2 = f"(fp_app {TRS('f2', 's', 'lc', IM)} psx)"
        d0h = "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hs2) lr lhs true ())) refl)"
        d3h = "(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise hfa) lr lhs true ())) refl)"
        d0t = "(steps ((rewrite (premise hmems) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise ht) lr lhs true ())) refl)"
        d3t = "(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise hfat) lr lhs true ())) refl)"
        return f"""              (have hfa (= (ixf_a4 (Cons s Nil)) True) (rewrite-with (lemma a4_head) lr lhs ((inst t t)) ({D('ha4')}) refl))
              (have hfat (= (ixf_a4 t) True) (rewrite-with (lemma a4_tail) lr lhs ((inst s s)) ({D('ha4')}) refl))
              {t1_ih('hmems', '(IqStmt s)', 'lc', 'psx', 'mems', d0h, d3h).replace('(inst lc2 lc2)', '(inst lc2 lcs)')}
              {t1_ih('hih', '(IqStmts t)', 'lcs', PSXs, 'mem2', d0t, d3t)}
              (have hih2 {T1C('f2', '(IqStmts t)', 'lcs', 'mems', PSXs2, 'mem2').replace(TR('f2', '(IqStmts t)', 'lcs', 'mems'), TRL('f2', 't', 'lcs', 'mems'))}
                (steps ((rewrite (premise hih) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hmems) lr rhs true ()) (unfold ipt_tr rhs) (reduce rhs)) refl))
              (steps
                ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmts lhs) (reduce lhs)
                 (rewrite (premise hs2) lr lhs true ()) (reduce lhs)
                 (rewrite (lemma fp_app_assoc) lr lhs true ())
                 (rewrite (premise hih2) lr lhs true ()))
                refl)"""
    L['cons'] = cons_
    def qexit(sl):
        return f"""              {HFB('ce', sl)}
              (steps
                ((rewrite (premise hmem) rl rhs true ())
                 (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_while lhs) (reduce lhs)
                 (rewrite (premise hv) lr lhs true ()) (reduce lhs)
                 (rewrite (premise hcv) lr lhs true ()) (reduce lhs)
                 (rewrite (premise hfb) lr lhs true ()))
                refl)"""
    L['qwhile_exit'] = qexit
    def qiter(sl):
        PSXb = f"(fp_app {TRL('f2', 'b', 'lc', IM)} {PSX1('ce')})"
        d0b = f"(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hb2) lr lhs true ())) refl)"
        d3b = "(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise hfa) lr lhs true ())) refl)"
        d0w = "(steps ((rewrite (premise hmemb2) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw2) lr lhs true ())) refl)"
        d3w = "(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise ha4) lr lhs true ())) refl)"
        return f"""                            {HFB('ce', sl)}
                            (have hfa (= (ixf_a4 b) True) (rewrite-with (lemma a4_while_b) lr lhs ((inst ce ce) (inst b b) (inst t Nil)) ({D('ha4')}) refl))
                            {t1_ih('hmemb', '(IqStmts b)', 'lc', PSX1('ce'), 'memb', d0b, d3b).replace('(inst lc2 lc2)', '(inst lc2 lcb)')}
                            (have hmemb2 (= (fp_mem mem0 (fbelow slo {PSXb})) memb)
                              (steps ((rewrite (premise hmemb) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfb) lr rhs true ())) refl))
                            {t1_ih('hih', '(IqWhile ce b)', 'lcb', PSXb, 'mem2', d0w, d3w)}
                            (have hih2 {T1C('f2', '(IqWhile ce b)', 'lcb', 'memb', PSXb, 'mem2').replace(TR('f2', '(IqWhile ce b)', 'lcb', 'memb'), TRW('f2', 'ce', 'b', 'lcb', 'memb'))}
                              (steps ((rewrite (premise hih) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hmemb2) lr rhs true ())) refl))
                            (steps
                              ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_while lhs) (reduce lhs)
                               (rewrite (premise hv) lr lhs true ()) (reduce lhs)
                               (rewrite (premise hcv) lr lhs true ()) (reduce lhs)
                               (rewrite (premise hb2) lr lhs true ()) (reduce lhs)
                               (rewrite (lemma fp_app_assoc) lr lhs true ())
                               (rewrite (lemma fp_app_assoc) lr lhs true ())
                               (rewrite (premise hih2) lr lhs true ()))
                              refl)"""
    L['qwhile_iter'] = qiter
    return L

def t1():
    prem = [f"(= {RUN('f', 'r', 'lc', IM)} {OUTN})", "(= (le slo fp) True)", "(= (le 0 nl) True)", "(= (ixf_a4 (ixt_body r)) True)"]
    return dispatch("ipt_mem", T1_VARS, prem, T1C('f', 'r', 'lc', IM, 'psx', 'mem2'), t1_leaves(), T1_SIDES)

BANNER_T1 = ";; --- THE TWIN'S MEMORY LAW ipt_mem (generated by gen_fra4.py t1 — REGENERATE, never hand-patch) ---"
BLOCKS.append(("t1", BANNER_T1, t1))

# ---------------- T2: the twin's context law ----------------
# the frame context (discipline, locals, window facts) survives a normal run, at the post-state

from gen_fra import CTX_INSTS, CTX_FACT, CTX_LEMMA
CTX_NAMES = ["hdisc","hlocs","hxlo","hmlo","hslo","hal","hnl","hd0","hslo0","hhi"]

def ctx_facts(sl, ctxp, ind):
    """the ten ctx_* extractions of the ctx premise (by name/index ctxp) at d = 0, plus
    hnn (0 <= ilen lc), hnd0 (0 <= nl + 0), hnl0 (0 <= nl); returns (text, sl2)"""
    lines = []
    s2 = Slots(sl.names)
    for nm in CTX_NAMES:
        insts = CTX_INSTS[nm].replace("(inst d d)", "(inst d 0)")
        fact = CTX_FACT[nm].replace("(le 0 d)", "(le 0 0)")
        lines.append(f"{ind}(have {nm} (= {fact} True) (rewrite-with (lemma {CTX_LEMMA[nm]}) lr lhs {insts} ({D(ctxp)}) refl))"); s2.add(nm)
    lines.append(f"{ind}(have hnn (= (le 0 (ilen lc)) True) (rewrite-with (lemma ilen_nonneg) lr lhs () () refl))"); s2.add("hnn")
    lines.append(f"{ind}(have hnd0 (= (le 0 (+ nl 0)) True) (by arith {s2.cert({'hnl':1,'hnn':1})}))"); s2.add("hnd0")
    lines.append(f"{ind}(have hnl0 (= (le 0 nl) True) (by arith {s2.cert({'hnl':1,'hnn':1})}))"); s2.add("hnl0")
    return "\n".join(lines), s2

def ctx_after_cond(e, ind):
    """hctx1: the context at (lc, PSX1(e)) — the cond's spills keep the discipline and the locals"""
    return f"""{ind}(have hdisca (= (fp_disc slo {PSX1(e)}) True) (rewrite-with (lemma fe_tr_disc) lr lhs ((inst e {e})) ({D('hdisc')} {D('hslo')} {D('hal')} {D('hnd0')}) refl))
{ind}(have hlocsa (= (fr_locs fp lc {PSX1(e)}) True) (rewrite-with (lemma fe_tr_locs) lr lhs ((inst e {e})) ({D('hlocs')} {D('hnl')} {D('hd0')}) refl))
{ind}(have hfb (= (fbelow slo {PSX1(e)}) (fbelow slo psx)) (rewrite-with (lemma fe_tr_below) lr lhs () ({D('hslo')} {D('hnd0')}) refl))
{ind}(have hctx1 (= (fe_ctx m mlo slo fp nl 0 lc {PSX1(e)}) True) (rewrite-with (lemma ctx_intro) lr lhs () ({D('hdisca')} {D('hlocsa')} {D('hxlo')} {D('hmlo')} {D('hslo')} {D('hal')} {D('hnl')} {D('hd0')} {D('hslo0')} {D('hhi')}) refl))"""

T2_VARS = T1_VARS + " (m XModule)"
def T2C(f, r, lc, im, psx, lc2): return f"(= (fe_ctx m mlo slo fp nl 0 {lc2} (fp_app {TR(f, r, lc, im)} {psx})) True)"
T2_SIDES = [{"fn": "ixf_scb", "p": 2, "h": "hscb"}, {"fn": "ixf_a4", "p": 3, "h": "ha4"}]

def t2_ih(hname, r, lc, psx, lc2, mem2, d0, d1, d2, d3):
    return f"""(have {hname} {T2C('f2', r, lc, IMx(psx), psx, lc2)}
  (rewrite-with (hyp ih) lr lhs ((inst mem2 {mem2})) ({d0} {d1} {d2} {d3}) refl))"""

def t1_cite(hname, r, lc, psx, lc2, mem2, d0, d3):
    """the memory law for a sub-run (ipt_mem), at (r, lc, psx)"""
    return f"""(have {hname} {T1C('f2', r, lc, IMx(psx), psx, mem2)}
  (rewrite-with (lemma ipt_mem) lr lhs ((inst lc2 {lc2}) (inst mem2 {mem2})) ({d0} {D('hslo')} {D('hnl0')} {d3}) refl))"""

def body_side(h, pred, body):
    """a sub-request's side premise discharge: unfold ixt_body, then the extracted fact"""
    return f"(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise {h}) lr lhs true ())) refl)"

def t2_leaves():
    L = {}
    def set_(sl):
        I = "              "
        cf, s2 = ctx_facts(sl, 'p1', I)
        s2 = S_(s2, 'hi0', 'hii', 'hcbe', 'hb', 'hsome2', 'hsl', 'hlcs0', 'hlcs')
        return f"""{cf}
{I}(have hi0 (= (le 0 i) True) (rewrite-with (lemma ilset_lo) lr lhs ((inst ls lc) (inst v v) (inst ls2 lcs)) ({D('hset')}) refl))
{I}(have hii (= (lt i (ilen lc)) True) (rewrite-with (lemma ilset_hi) lr lhs ((inst v v) (inst ls2 lcs)) ({D('hset')}) refl))
{I}(have hcbe (= (ixf_cb e) True) (rewrite-with (lemma scb_set_e) lr lhs ((inst i i) (inst t Nil)) ({D('hscb')}) refl))
{I}(have hb (= (fe_inband v) True) (rewrite-with (lemma fe_band) lr lhs ((inst lc lc) (inst mem {IM}) (inst mlo mlo) (inst msz slo) (inst fp fp) (inst psx psx) (inst e e)) ({D('hv')} {D('hlocs')} {D('hcbe')}) refl))
{I}(have hsome2 (= (ilset lc i v) (Some (fl_set lc i v))) (rewrite-with (lemma ilset_some) lr lhs () ({D('hi0')} {D('hii')}) refl))
{I}(have hsl (= (Some lcs) (Some (fl_set lc i v))) (steps ((rewrite (premise hset) rl lhs true ()) (rewrite (premise hsome2) lr lhs true ())) refl))
{I}(inject (premise hsl) (hlcs0))
{I}(have hlcs (= lc2 (fl_set lc i v)) (steps ((rewrite (premise hlc) rl lhs true ()) (rewrite (premise hlcs0) lr lhs true ())) refl))
{I}(have hlo8 (= (le (+ (+ fp (* 8 i)) 8) slo) False) (by arith {s2.cert({'hslo':1,'hi0':8})}))
{I}(have hsa (= (le slo (+ fp (* 8 i))) True) (by arith {S_(s2,'hlo8').cert({'hslo':1,'hi0':8})}))
{I}(have hali (= (int_eq (mod (- (+ fp (* 8 i)) slo) 8) 0) True) (rewrite-with (lemma al_shift) lr lhs ((inst k i)) ({D('hal')}) refl))
{I}(have hdisca (= (fp_disc slo {PSX1('e')}) True) (rewrite-with (lemma fe_tr_disc) lr lhs ((inst e e)) ({D('hdisc')} {D('hslo')} {D('hal')} {D('hnd0')}) refl))
{I}(have hdisc2 (= (fp_disc slo (Cons (FWord (+ fp (* 8 i)) v) {PSX1('e')})) True) (rewrite-with (lemma fp_disc_w_slot) lr lhs () ({D('hlo8')} {D('hsa')} {D('hali')} {D('hdisca')}) refl))
{I}(have hlocsa (= (fr_locs fp lc {PSX1('e')}) True) (rewrite-with (lemma fe_tr_locs) lr lhs ((inst e e)) ({D('hlocs')} {D('hnl')} {D('hd0')}) refl))
{I}(have hlocs2 (= (fr_locs fp (fl_set lc i v) (Cons (FWord (+ fp (* 8 i)) v) {PSX1('e')})) True) (rewrite-with (lemma fr_locs_set) lr lhs () ({D('hlocsa')} {D('hi0')} {D('hii')} {D('hb')}) refl))
{I}(have hlen2 (= (le (ilen (fl_set lc i v)) nl) True) (steps ((rewrite (lemma fl_set_len) lr lhs true ()) (rewrite (premise hnl) lr lhs true ())) refl))
{I}(steps
{I}  ((rewrite (premise hlcs) lr lhs true ())
{I}   (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs) (unfold ips_tr lhs) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)
{I}   (unfold fp_app lhs) (reduce lhs)))
{I}(rewrite-with (lemma ctx_intro) lr lhs () ({D('hdisc2')} {D('hlocs2')} {D('hxlo')} {D('hmlo')} {D('hslo')} {D('hal')} {D('hlen2')} {D('hd0')} {D('hslo0')} {D('hhi')}))
{I}refl"""
    L['set'] = set_
    def ifbr(ss, ext_cb, ext_a4):
        def f(sl):
            I = "  "
            cf, s2 = ctx_facts(sl, 'p1', I)
            d0 = "(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hbr) lr lhs true ())) refl)"
            return f"""{cf}
{ctx_after_cond('ce', I)}
{I}(have hcb (= (ixf_scb {ss}) True) (rewrite-with (lemma {ext_cb}) lr lhs ((inst ce ce) (inst tb tb) (inst eb eb) (inst t Nil)) ({D('hscb')}) refl))
{I}(have hfa (= (ixf_a4 {ss}) True) (rewrite-with (lemma {ext_a4}) lr lhs ((inst ce ce) (inst tb tb) (inst eb eb) (inst t Nil)) ({D('ha4')}) refl))
{I}{t2_ih('hih', f'(IqStmts {ss})', 'lc', PSX1('ce'), 'lc2', 'mem2', d0, D('hctx1'), body_side('hcb', None, None), body_side('hfa', None, None))}
{I}(have hih2 {T2C('f2', f'(IqStmts {ss})', 'lc', IM, PSX1('ce'), 'lc2').replace(TR('f2', f'(IqStmts {ss})', 'lc', IM), TRL('f2', ss, 'lc', IM))}
{I}  (steps ((rewrite (premise hih) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfb) lr rhs true ())) refl))
{I}(steps
{I}  ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hcv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (lemma fp_app_assoc) lr lhs true ())
{I}   (rewrite (premise hih2) lr lhs true ()))
{I}  refl)"""
        return f
    L['if_e'] = ifbr('eb', 'scb_if_e', 'a4_if_e')
    L['if_t'] = ifbr('tb', 'scb_if_t', 'a4_if_t')
    def while_(sl):
        d0 = "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw) lr lhs true ())) refl)"
        return f"""  {t2_ih('hih', '(IqWhile ce b)', 'lc', 'psx', 'lc2', 'mem2', d0, D('p1'), body_side('hscb', None, None), body_side('ha4', None, None))}
  (have hih2 {T2C('f2', '(IqWhile ce b)', 'lc', IM, 'psx', 'lc2').replace(TR('f2', '(IqWhile ce b)', 'lc', IM), TRW('f2', 'ce', 'b', 'lc', IM))}
    (steps ((rewrite (premise hih) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs)) refl))
  (steps
    ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs)
     (rewrite (premise hih2) lr lhs true ()))
    refl)"""
    L['while'] = while_
    L['nil'] = lambda sl: """  (steps
    ((rewrite (premise hlc) rl lhs true ())
     (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmts lhs) (reduce lhs) (unfold fp_app lhs) (reduce lhs)
     (rewrite (premise 1) lr lhs true ()))
    refl)"""
    def cons_(sl):
        I = "              "
        cf, s2 = ctx_facts(sl, 'p1', I)
        PSXs = f"(fp_app {TR('f2', '(IqStmt s)', 'lc', IM)} psx)"
        PSXs2 = f"(fp_app {TRS('f2', 's', 'lc', IM)} psx)"
        d0h = "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hs2) lr lhs true ())) refl)"
        d0t = "(steps ((rewrite (premise hmems) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise ht) lr lhs true ())) refl)"
        return f"""{cf}
{I}(have hcbh (= (ixf_scb (Cons s Nil)) True) (rewrite-with (lemma scb_head) lr lhs ((inst t t)) ({D('hscb')}) refl))
{I}(have hcbt (= (ixf_scb t) True) (rewrite-with (lemma scb_tail) lr lhs ((inst s s)) ({D('hscb')}) refl))
{I}(have hfa (= (ixf_a4 (Cons s Nil)) True) (rewrite-with (lemma a4_head) lr lhs ((inst t t)) ({D('ha4')}) refl))
{I}(have hfat (= (ixf_a4 t) True) (rewrite-with (lemma a4_tail) lr lhs ((inst s s)) ({D('ha4')}) refl))
{I}{t2_ih('hctxs', '(IqStmt s)', 'lc', 'psx', 'lcs', 'mems', d0h, D('p1'), body_side('hcbh', None, None), body_side('hfa', None, None))}
{I}{t1_cite('hmems', '(IqStmt s)', 'lc', 'psx', 'lcs', 'mems', d0h, body_side('hfa', None, None))}
{I}{t2_ih('hih', '(IqStmts t)', 'lcs', PSXs, 'lc2', 'mem2', d0t, D('hctxs'), body_side('hcbt', None, None), body_side('hfat', None, None))}
{I}(have hih2 {T2C('f2', '(IqStmts t)', 'lcs', 'mems', PSXs2, 'lc2').replace(TR('f2', '(IqStmts t)', 'lcs', 'mems'), TRL('f2', 't', 'lcs', 'mems'))}
{I}  (steps ((rewrite (premise hih) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hmems) lr rhs true ()) (unfold ipt_tr rhs) (reduce rhs)) refl))
{I}(steps
{I}  ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmts lhs) (reduce lhs)
{I}   (rewrite (premise hs2) lr lhs true ()) (reduce lhs)
{I}   (rewrite (lemma fp_app_assoc) lr lhs true ())
{I}   (rewrite (premise hih2) lr lhs true ()))
{I}  refl)"""
    L['cons'] = cons_
    def qexit(sl):
        I = "              "
        cf, s2 = ctx_facts(sl, 'p1', I)
        return f"""{cf}
{ctx_after_cond('ce', I)}
{I}(steps
{I}  ((rewrite (premise hlc) rl lhs true ())
{I}   (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_while lhs) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hcv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hctx1) lr lhs true ()))
{I}  refl)"""
    L['qwhile_exit'] = qexit
    def qiter(sl):
        I = "                            "
        cf, s2 = ctx_facts(sl, 'p1', I)
        PSXb = f"(fp_app {TRL('f2', 'b', 'lc', IM)} {PSX1('ce')})"
        d0b = "(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hb2) lr lhs true ())) refl)"
        d0w = "(steps ((rewrite (premise hmemb2) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw2) lr lhs true ())) refl)"
        return f"""{cf}
{ctx_after_cond('ce', I)}
{I}(have hcb (= (ixf_scb b) True) (rewrite-with (lemma scb_while_b) lr lhs ((inst ce ce) (inst b b) (inst t Nil)) ({D('hscb')}) refl))
{I}(have hfa (= (ixf_a4 b) True) (rewrite-with (lemma a4_while_b) lr lhs ((inst ce ce) (inst b b) (inst t Nil)) ({D('ha4')}) refl))
{I}{t2_ih('hctxb', '(IqStmts b)', 'lc', PSX1('ce'), 'lcb', 'memb', d0b, D('hctx1'), body_side('hcb', None, None), body_side('hfa', None, None))}
{I}(have hctxb2 (= (fe_ctx m mlo slo fp nl 0 lcb {PSXb}) True)
{I}  (steps ((rewrite (premise hctxb) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfb) lr rhs true ())) refl))
{I}{t1_cite('hmemb', '(IqStmts b)', 'lc', PSX1('ce'), 'lcb', 'memb', d0b, body_side('hfa', None, None))}
{I}(have hmemb2 (= (fp_mem mem0 (fbelow slo {PSXb})) memb)
{I}  (steps ((rewrite (premise hmemb) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfb) lr rhs true ())) refl))
{I}{t2_ih('hih', '(IqWhile ce b)', 'lcb', PSXb, 'lc2', 'mem2', d0w, D('hctxb2'), body_side('hscb', None, None), body_side('ha4', None, None))}
{I}(have hih2 {T2C('f2', '(IqWhile ce b)', 'lcb', 'memb', PSXb, 'lc2').replace(TR('f2', '(IqWhile ce b)', 'lcb', 'memb'), TRW('f2', 'ce', 'b', 'lcb', 'memb'))}
{I}  (steps ((rewrite (premise hih) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hmemb2) lr rhs true ())) refl))
{I}(steps
{I}  ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_while lhs) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hcv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hb2) lr lhs true ()) (reduce lhs)
{I}   (rewrite (lemma fp_app_assoc) lr lhs true ())
{I}   (rewrite (lemma fp_app_assoc) lr lhs true ())
{I}   (rewrite (premise hih2) lr lhs true ()))
{I}  refl)"""
    L['qwhile_iter'] = qiter
    return L

def t2():
    prem = [f"(= {RUN('f', 'r', 'lc', IM)} {OUTN})", "(= (fe_ctx m mlo slo fp nl 0 lc psx) True)",
            "(= (ixf_scb (ixt_body r)) True)", "(= (ixf_a4 (ixt_body r)) True)"]
    return dispatch("ipt_ctx", T2_VARS, prem, T2C('f', 'r', 'lc', IM, 'psx', 'lc2'), t2_leaves(), T2_SIDES)

BANNER_T2 = ";; --- THE TWIN'S CONTEXT LAW ipt_ctx (generated by gen_fra4.py t2 — REGENERATE, never hand-patch) ---"
BLOCKS.append(("t2", BANNER_T2, t2))

# ---------------- the CONTROL STEP LEMMAS (x86 side) ----------------
# Stated at a generic fuel g and explicit heights; sub-runs' simulations are premises in
# the ixt_expect form; the conclusion is the request's ixt_expect form.

from gen_fra import ACC14
ACC15 = ["rax"] + ACC14
RS = "(MkRegs a0 rcx dx rbx rbp rsi di r8 r9 s10 s11 r12 r13 dep fp)"
XM = "(fp_mem mem0 psx)"
CVARS = ("(is (List XInstr)) (nl Int) (own Int) (fail_ix Int) (lc (List Int)) (mem0 Mem) (psx (List FPatch)) "
         "(mlo Int) (slo Int) (c Int) (g Nat) (m XModule) (fp Int) (dep Int) (f Nat) (fs (List IpFn)) (dmax Int) "
         "(a0 Int) (rcx Int) (dx Int) (rbx Int) (rbp Int) (rsi Int) (di Int) (r8 Int) (r9 Int) (s10 Int) (s11 Int) (r12 Int) (r13 Int)")
def XRUN(is_, c="c", rs=RS, xm=XM): return f"(xeval_seq (xt {c} g) m {is_} {rs} {xm})"
def MTW(f, r, lc, im, psx): return f"(fp_mem mem0 (fp_app {TR(f, r, lc, im)} {psx}))"
def EXPECT(r, out, rs, run, mem): return f"(ixt_expect {r} {out} {rs} {run} {mem})"
ACC_RHS = "\n".join(f"       (rewrite (lemma {a}_of_mk) lr rhs true ())" for a in ACC15)

def ctl_nil():
    run = XRUN("is")
    return f"""(claim fs_step_nil
  (goal ((out IpOut) {CVARS})
    ((= (ixf_stmts nl own fail_ix Nil) (Some is))
     (= {STS('(S f)', 'Nil', 'lc', IM)} (Some out))
     (= (le 1 c) True))
    (= {run} {EXPECT('(IqStmts Nil)', 'out', RS, run, MTW('(S f)', '(IqStmts Nil)', 'lc', IM, 'psx'))}))
  (chain
    (have hsome (= (Some Nil) (Some is)) (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_stmts rhs) (reduce rhs)) refl))
    (inject (premise hsome) (his))
    (have hn1 (= (Some (IpNorm lc {IM})) (Some out)) (steps ((rewrite (premise 1) rl rhs true ()) (unfold ipstmts rhs) (reduce rhs)) refl))
    (inject (premise hn1) (hout))
    (have hc (= (lt 0 c) True) (by arith (list 1 0 0 1 0 0)))
    (have hrun (= {run} (Some (XNorm {RS} {XM})))
      (chain
        (steps ((rewrite (premise his) rl lhs true ())))
        (rewrite-with (lemma xt_peel) lr lhs () ({D('hc')}))
        (steps ((unfold xeval_seq lhs) (reduce lhs)) refl)))
    (steps
      ((rewrite (premise hrun) lr both true ())
       (rewrite (premise hout) rl rhs true ())
       (unfold ixt_expect rhs) (reduce rhs)
       (rewrite (lemma fs_out_of_norm) lr rhs true ())
{ACC_RHS}
       (unfold ipt_tr rhs) (reduce rhs) (unfold ipt_stmts rhs) (reduce rhs) (unfold fp_app rhs) (reduce rhs))
      refl)))
"""

def RSP(run):
    """the post-run register file: scratch as the run's own, the rest carried"""
    return f"(MkRegs (rax_of (xo_regs {run})) rcx (rdx_of (xo_regs {run})) rbx rbp rsi di r8 r9 (r10_of (xo_regs {run})) (r11_of (xo_regs {run})) r12 r13 dep fp)"
ACC_CARRIED = ["rcx","rbx","rbp","rsi","rdi","r8","r9","r12","r13","r14","r15"]
def acc_rw(side):
    return "".join(f" (rewrite (lemma {a}_of_mk) lr {side} true ())" for a in ACC_CARRIED)

def ctl_cons():
    PSXs = f"(fp_app {TR('f', '(IqStmt s)', 'lc', IM)} psx)"
    RUN = XRUN("is"); RUNs = XRUN("iss")
    RSP_ = RSP(RUNs)
    RUNt = f"(xeval_seq (xt (- c (xil iss)) g) m it {RSP_} (fp_mem mem0 {PSXs}))"
    MEMt = f"(fp_mem mem0 (fp_app {TR('f', '(IqStmts t)', 'lcs', IMx(PSXs))} {PSXs}))"
    MEM = MTW('(S f)', '(IqStmts (Cons s t))', 'lc', IM, 'psx')
    NORM = f"(fp_mem mem0 (fp_app {TRL('f', 't', 'lcs', 'mems')} (fp_app {TRS('f', 's', 'lc', IM)} psx)))"
    return f"""(claim fs_step_cons
  (goal ((s IpStmt) (t (List IpStmt)) (iss (List XInstr)) (it (List XInstr)) (out IpOut) (lcs (List Int)) (mems Mem) {CVARS})
    ((= (ixf_stmts nl own fail_ix (Cons s t)) (Some is))
     (= (ixf_stmt nl own fail_ix s) (Some iss))
     (= (ixf_stmts nl own fail_ix t) (Some it))
     (= {STS('(S f)', '(Cons s t)', 'lc', IM)} (Some out))
     (= {ST('f', 's', 'lc', IM)} (Some (IpNorm lcs mems)))
     (= (le (xil iss) c) True)
     (= (fp_mem mem0 (fbelow slo {PSXs})) mems)
     (= {RUNs} {EXPECT('(IqStmt s)', '(IpNorm lcs mems)', RS, RUNs, f'(fp_mem mem0 {PSXs})')})
     (= {RUNt} {EXPECT('(IqStmts t)', 'out', RSP_, RUNt, MEMt)}))
    (= {RUN} {EXPECT('(IqStmts (Cons s t))', 'out', RS, RUN, MEM)}))
  (chain
    (have hsome (= (Some (ix_app iss it)) (Some is))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_stmts rhs) (reduce rhs) (rewrite (premise 2) lr rhs true ()) (reduce rhs) (rewrite (premise 1) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hsome) (his))
    (have hout (= {STS('f', 't', 'lcs', 'mems')} (Some out))
      (steps ((rewrite (premise 3) rl rhs true ()) (unfold ipstmts rhs) (reduce rhs) (rewrite (premise 4) lr rhs true ()) (reduce rhs)) refl))
    (have h7 (= {RUNs} (fs_out {RS} {RUNs} (fp_mem mem0 {PSXs})))
      (steps ((rewrite (premise 7) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs)) refl))
    (have hrun (= {RUN} {RUNt})
      (chain
        (steps ((rewrite (premise his) rl lhs true ())))
        (rewrite-with (lemma xseq_app) lr lhs () ({D(5)}))
        (steps ((rewrite (premise h7) lr lhs true ()) (unfold fs_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs){acc_rw('lhs')}) refl)))
    (have hmem (= {MEM} {NORM})
      (steps ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmts lhs) (reduce lhs) (rewrite (premise 4) lr lhs true ()) (reduce lhs) (rewrite (lemma fp_app_assoc) lr lhs true ())) refl))
    (have hmemt (= {MEMt} {NORM})
      (steps ((unfold ipt_tr lhs) (reduce lhs) (rewrite (premise 6) lr lhs true ()) (unfold ipt_tr lhs) (reduce lhs)) refl))
    (steps ((rewrite (premise hrun) lr both true ()) (rewrite (premise hmem) lr rhs true ())))
    (case-on out IpOut
      ((case IpNorm (lc2 mem2)
         (chain
           {cap('ho', '(= out (IpNorm lc2 mem2))')}
           (have h8 (= {RUNt} (fs_out {RSP_} {RUNt} {NORM}))
             (steps ((rewrite (premise 8) lr lhs true ()) (rewrite (premise ho) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs) (rewrite (premise hmemt) lr lhs true ())) refl))
           (steps
             ((rewrite (premise ho) lr rhs true ())
              (unfold ixt_expect rhs) (reduce rhs)
              (rewrite (premise h8) lr lhs true ())
              (unfold fs_out both) (reduce both){acc_rw('both')})
             refl)))
       (case IpTrap
         (chain
           {cap('ho', '(= out IpTrap)')}
           (steps ((rewrite (premise ho) lr rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)))
       (case IpFailed (fam)
         (chain
           {cap('ho', '(= out (IpFailed fam))')}
           (have h8 (= {RUNt} (Some XTrap))
             (steps ((rewrite (premise 8) lr lhs true ()) (rewrite (premise ho) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs)) refl))
           (steps ((rewrite (premise ho) lr rhs true ()) (unfold ixt_expect rhs) (reduce rhs) (rewrite (premise h8) lr lhs true ())) refl)))))))
"""

def ctl_cons_fail():
    PSXs = f"(fp_app {TR('f', '(IqStmt s)', 'lc', IM)} psx)"
    RUN = XRUN("is"); RUNs = XRUN("iss")
    MEM = MTW('(S f)', '(IqStmts (Cons s t))', 'lc', IM, 'psx')
    return f"""(claim fs_step_cons_fail
  (goal ((s IpStmt) (t (List IpStmt)) (iss (List XInstr)) (it (List XInstr)) (out IpOut) (fam IFam) {CVARS})
    ((= (ixf_stmts nl own fail_ix (Cons s t)) (Some is))
     (= (ixf_stmt nl own fail_ix s) (Some iss))
     (= (ixf_stmts nl own fail_ix t) (Some it))
     (= {STS('(S f)', '(Cons s t)', 'lc', IM)} (Some out))
     (= {ST('f', 's', 'lc', IM)} (Some (IpFailed fam)))
     (= (le (xil iss) c) True)
     (= {RUNs} {EXPECT('(IqStmt s)', '(IpFailed fam)', RS, RUNs, f'(fp_mem mem0 {PSXs})')}))
    (= {RUN} {EXPECT('(IqStmts (Cons s t))', 'out', RS, RUN, MEM)}))
  (chain
    (have hsome (= (Some (ix_app iss it)) (Some is))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_stmts rhs) (reduce rhs) (rewrite (premise 2) lr rhs true ()) (reduce rhs) (rewrite (premise 1) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hsome) (his))
    (have hout (= (Some (IpFailed fam)) (Some out))
      (steps ((rewrite (premise 3) rl rhs true ()) (unfold ipstmts rhs) (reduce rhs) (rewrite (premise 4) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hout) (ho))
    (have h7 (= {RUNs} (Some XTrap))
      (steps ((rewrite (premise 6) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs)) refl))
    (have hrun (= {RUN} (Some XTrap))
      (chain
        (steps ((rewrite (premise his) rl lhs true ())))
        (rewrite-with (lemma xseq_app) lr lhs () ({D(5)}))
        (steps ((rewrite (premise h7) lr lhs true ()) (unfold xcont lhs) (reduce lhs)) refl)))
    (steps ((rewrite (premise hrun) lr both true ()) (rewrite (premise ho) rl rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)))
"""

# --- the x86 BLOCK LEMMAS: the if/loop choreography over an explicit S-tower, once ---
def Sn(n, F="F"): return F if n == 0 else f"(S {Sn(n-1, F)})"
UR = lambda fn: f"(unfold {fn} lhs) (reduce lhs)"
IF_IS = "(Cons (XBlock (Cons (XBlock (Cons (XBrIf (CNz RAX) 0) (Cons (XBlock ieb) (Cons (XBr 1) Nil)))) itb)) Nil)"
REST = "(Cons (XBrIf (CEqz RAX) 1) (Cons (XBlock ib) (Cons (XBr 0) Nil)))"
LOOP_IS = "(Cons (XBlock (Cons (XLoop l) Nil)) Nil)"
def blk(name, vars_, prems, lhs, rhs, steps):
    z = " 0" * len(prems)
    return f"""(claim {name}
  (goal ({vars_}) ({' '.join(prems)}) (= {lhs} {rhs}))
  (chain
    (have h00 (= (int_eq 0 0) True) (by arith (list)))
    (have h10 (= (int_eq 1 0) False) (by arith (list -1{z} 0)))
    (have h11 (= (- 1 1) 0) (by arith (list (list 1{z} 0 0) (list 1{z} 0 0))))
    (steps ({' '.join(steps)}) refl)))
"""
BV = "(F Nat) (m XModule) (rs Regs) (mem Mem) (rs2 Regs) (mem2 Mem)"
P0F = "(= (int_eq (rax_of rs) 0) False)"; P0T = "(= (int_eq (rax_of rs) 0) True)"
def cond_steps():   # the XBrIf: xcond → rget → the premise on RAX
    return [UR("xcond"), UR("rget"), "(rewrite (premise 0) lr lhs true ()) (reduce lhs)"]
def exit0():        # xblock_exit (XBrk 0 …) → XNorm
    return [UR("xblock_exit"), "(rewrite (premise h00) lr lhs true ()) (reduce lhs)"]

def ctl_blocks():
    out = []
    # if, then-branch taken (RAX ≠ 0): B2 exits at once, itb runs at S^3 F, the outer block and Nil tail close
    for nm, res in [("fs_if_then_norm", "(Some (XNorm rs2 mem2))"), ("fs_if_then_trap", "(Some XTrap)")]:
        out.append(blk(nm, BV + " (ieb (List XInstr)) (itb (List XInstr))",
            [P0F, f"(= (xeval_seq {Sn(3)} m itb rs mem) {res})"],
            f"(xeval_seq {Sn(6)} m {IF_IS} rs mem)", res,
            [UR("xeval_seq"), UR("xeval_instr"), UR("xeval_seq"), UR("xeval_instr"), UR("xeval_seq"), UR("xeval_instr")]
            + cond_steps() + exit0()
            + ["(rewrite (premise 1) lr lhs true ()) (reduce lhs)", UR("xblock_exit")] + ([UR("xeval_seq")] if "norm" in nm else [])))
    # if, else-branch taken (RAX = 0): ieb runs at S F inside its block, XBr 1 unwinds B2, B1 closes
    for nm, res in [("fs_if_else_norm", "(Some (XNorm rs2 mem2))"), ("fs_if_else_trap", "(Some XTrap)")]:
        tail = ([UR("xblock_exit"), UR("xeval_seq"), UR("xeval_instr"), UR("xblock_exit"),
                 "(rewrite (premise h10) lr lhs true ()) (reduce lhs) (rewrite (premise h11) lr lhs true ())"]
                + exit0() + [UR("xeval_seq")]) if "norm" in nm else \
               [UR("xblock_exit"), UR("xblock_exit"), UR("xblock_exit")]
        out.append(blk(nm, BV + " (ieb (List XInstr)) (itb (List XInstr))",
            [P0T, f"(= (xeval_seq {Sn(1)} m ieb rs mem) {res})"],
            f"(xeval_seq {Sn(8)} m {IF_IS} rs mem)", res,
            [UR("xeval_seq"), UR("xeval_instr"), UR("xeval_seq"), UR("xeval_instr"), UR("xeval_seq"), UR("xeval_instr")]
            + cond_steps()
            + [UR("xeval_seq"), UR("xeval_instr"), "(rewrite (premise 1) lr lhs true ()) (reduce lhs)"] + tail))
    # the loop body after the cond: exit (RAX = 0) is a depth-1 branch
    out.append(blk("fw_rest_exit", BV + " (ib (List XInstr))", [P0T],
        f"(xeval_seq {Sn(2)} m {REST} rs mem)", "(Some (XBrk 1 rs mem))",
        [UR("xeval_seq"), UR("xeval_instr")] + cond_steps()))
    # … iterate (RAX ≠ 0): the body block at S F, then the back edge (depth 0)
    for nm, res, fin in [("fw_rest_norm", "(Some (XNorm rs2 mem2))", "(Some (XBrk 0 rs2 mem2))"), ("fw_rest_trap", "(Some XTrap)", "(Some XTrap)")]:
        tail = [UR("xblock_exit"), UR("xeval_seq"), UR("xeval_instr")] if "norm" in nm else [UR("xblock_exit")]
        out.append(blk(nm, BV + " (ib (List XInstr))", [P0F, f"(= (xeval_seq {Sn(1)} m ib rs mem) {res})"],
            f"(xeval_seq {Sn(4)} m {REST} rs mem)", fin,
            [UR("xeval_seq"), UR("xeval_instr")] + cond_steps()
            + [UR("xeval_seq"), UR("xeval_instr"), "(rewrite (premise 1) lr lhs true ()) (reduce lhs)"] + tail))
    # the while statement: the loop engine's depth-0 exit is the statement's normal completion
    for nm, res, fin in [("fs_loop_brk", "(Some (XBrk 0 rs2 mem2))", "(Some (XNorm rs2 mem2))"), ("fs_loop_trap", "(Some XTrap)", "(Some XTrap)")]:
        tail = exit0() + [UR("xeval_seq")] if "brk" in nm else [UR("xblock_exit")]
        out.append(blk(nm, BV + " (l (List XInstr))", [f"(= (xeval_loop F m l rs mem) {res})"],
            f"(xeval_seq {Sn(4)} m {LOOP_IS} rs mem)", fin,
            [UR("xeval_seq"), UR("xeval_instr"), UR("xeval_seq"), UR("xeval_instr"),
             "(rewrite (premise 0) lr lhs true ()) (reduce lhs)"] + tail))
    return "\n".join(out)

# --- the control STEP LEMMAS ---
PSX1c = PSX1('ce')
Mc = f"(fp_mem mem0 {PSX1c})"
def RSC(runc): return f"(MkRegs cv rcx (rdx_of (xo_regs {runc})) rbx rbp rsi di r8 r9 (r10_of (xo_regs {runc})) (r11_of (xo_regs {runc})) r12 r13 dep fp)"
FSO = lambda rs, r, mem: f"(MkRegs (rax_of (xo_regs {r})) (rcx_of {rs}) (rdx_of (xo_regs {r})) (rbx_of {rs}) (rbp_of {rs}) (rsi_of {rs}) (rdi_of {rs}) (r8_of {rs}) (r9_of {rs}) (r10_of (xo_regs {r})) (r11_of (xo_regs {r})) (r12_of {rs}) (r13_of {rs}) (r14_of {rs}) (r15_of {rs}))"
SCR_RHS = "".join(f" (rewrite (lemma {a}_of_mk) lr rhs true ())" for a in ["rax","rdx","r10","r11"])
IFV = "(ce IExp) (tb (List IpStmt)) (eb (List IpStmt)) (ic (List XInstr)) (itb (List XInstr)) (ieb (List XInstr)) (cv Int) (out IpOut)"

def step_if(which):
    """which = 't' (cv ≠ 0, then) or 'e' (cv = 0, else)"""
    then = which == 't'
    ss = "tb" if then else "eb"; iss = "itb" if then else "ieb"
    flag = "False" if then else "True"
    RUN = XRUN("is"); RUNc = XRUN("ic"); rsc = RSC(RUNc)
    H = "(- c (xil ic))"
    if then:
        cost = 6; H3 = f"(- {H} 3)"; fuel_b = f"(xt {H3} g)"; F = f"(xt (- {H3} 3) g)"
        peels = f"""    (have h3a (= (le 3 {H}) True) (by arith {{C7}}))
    (have h3b (= (le 3 {H3}) True) (by arith {{C7}}))
    (have hp1 (= (xt {H} g) (S (S (S (xt {H3} g))))) (rewrite-with (lemma xt_peel3) lr lhs () ({D('h3a')}) refl))
    (have hp2 (= (xt {H3} g) (S (S (S {F})))) (rewrite-with (lemma xt_peel3) lr lhs () ({D('h3b')}) refl))"""
        bfuel = f"(S (S (S {F})))"; lem = "fs_if_then"; nS = 6
    else:
        cost = 8; H7 = f"(- {H} 7)"; fuel_b = f"(xt {H7} g)"; F = f"(xt (- {H7} 1) g)"
        peels = f"""    (have h3a (= (le 7 {H}) True) (by arith {{C7}}))
    (have h3b (= (lt 0 {H7}) True) (by arith {{C7}}))
    (have hp1 (= (xt {H} g) (S (S (S (S (S (S (S (xt {H7} g))))))))) (rewrite-with (lemma xt_peel7) lr lhs () ({D('h3a')}) refl))
    (have hp2 (= (xt {H7} g) (S {F})) (rewrite-with (lemma xt_peel) lr lhs () ({D('h3b')}) refl))"""
        bfuel = f"(S {F})"; lem = "fs_if_else"; nS = 8
    RUNb = f"(xeval_seq {fuel_b} m {iss} {rsc} {Mc})"
    MEMb = f"(fp_mem mem0 (fp_app {TR('f', f'(IqStmts {ss})', 'lc', IMx(PSX1c))} {PSX1c}))"
    MEM = MTW('(S f)', '(IqStmt (IpIf ce tb eb))', 'lc', IM, 'psx')
    NORM = f"(fp_mem mem0 (fp_app {TRL('f', ss, 'lc', IM)} {PSX1c}))"
    BRUN = f"(xeval_seq {bfuel} m {iss} {rsc} {Mc})"
    RSF = FSO(rsc, RUNb, None)
    sl = Slots([f"p{i}" for i in range(12)] + ["hsome", "his", "hout"])
    C7 = sl.cert({'p7': 1}); sl.add('hle')
    peels = peels.replace("{C7}", sl.cert({'p7': 1}), 1); sl.add('h3a')
    peels = peels.replace("{C7}", sl.cert({'p7': 1}), 1)
    return f"""(claim fs_step_if_{which}
  (goal ({IFV} {CVARS})
    ((= (ixf_stmt nl own fail_ix (IpIf ce tb eb)) (Some is))
     (= (ixf_exp nl 0 ce) (Some ic))
     (= (ixf_stmts nl own fail_ix tb) (Some itb))
     (= (ixf_stmts nl own fail_ix eb) (Some ieb))
     (= {ST('(S f)', '(IpIf ce tb eb)', 'lc', IM)} (Some out))
     (= (iexp ce lc {IM} mlo slo) (Some cv))
     (= (int_eq cv 0) {flag})
     (= (le (+ (xil ic) {cost}) c) True)
     (= {RUNc} (fe_out cv {RS} {RUNc} {Mc}))
     (= {RUNb} {EXPECT(f'(IqStmts {ss})', 'out', rsc, RUNb, MEMb)})
     (= (fbelow slo {PSX1c}) (fbelow slo psx))
     (= (le 0 c) True))
    (= {RUN} {EXPECT('(IqStmt (IpIf ce tb eb))', 'out', RS, RUN, MEM)}))
  (chain
    (have hsome (= (Some (ix_app ic {IF_IS})) (Some is))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise 1) lr rhs true ()) (reduce rhs) (rewrite (premise 2) lr rhs true ()) (reduce rhs) (rewrite (premise 3) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hsome) (his))
    (have hout (= {STS('f', ss, 'lc', IM)} (Some out))
      (steps ((rewrite (premise 4) rl rhs true ()) (unfold ipstmt rhs) (reduce rhs) (rewrite (premise 5) lr rhs true ()) (reduce rhs) (rewrite (premise 6) lr rhs true ()) (reduce rhs)) refl))
    (have hle (= (le (xil ic) c) True) (by arith {C7}))
{peels}
    (have hrax (= (int_eq (rax_of {rsc}) 0) {flag}) (steps ((rewrite (lemma rax_of_mk) lr lhs true ()) (rewrite (premise 6) lr lhs true ())) refl))
    (have h00 (= (int_eq 0 0) True) (by arith (list)))
    (have hmem (= {MEM} {NORM})
      (steps ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs) (rewrite (premise 5) lr lhs true ()) (reduce lhs) (rewrite (premise 6) lr lhs true ()) (reduce lhs) (rewrite (lemma fp_app_assoc) lr lhs true ())) refl))
    (have hmemb (= {MEMb} {NORM})
      (steps ((unfold ipt_tr lhs) (reduce lhs) (rewrite (premise 10) lr lhs true ())) refl))
    (have hpre (= {RUN} (xeval_seq {Sn(nS, F)} m {IF_IS} {rsc} {Mc}))
      (chain
        (steps ((rewrite (premise his) rl lhs true ())))
        (rewrite-with (lemma xseq_app) lr lhs () ({D('hle')}))
        (steps ((rewrite (premise 8) lr lhs true ()) (unfold fe_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs){acc_rw('lhs')} (rewrite (premise hp1) lr lhs true ()) (rewrite (premise hp2) lr lhs true ())) refl)))
    (case-on out IpOut
      ((case IpNorm (lc2 mem2)
         (chain
           {cap('ho', '(= out (IpNorm lc2 mem2))')}
           (have hb (= {BRUN} (Some (XNorm {RSF} {NORM})))
             (steps ((rewrite (premise hp2) rl lhs true ()) (rewrite (premise 9) lr lhs true ()) (rewrite (premise ho) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs) (unfold fs_out lhs) (reduce lhs) (rewrite (premise hmemb) lr lhs true ())) refl))
           (have hrun (= {RUN} (Some (XNorm {RSF} {NORM})))
             (chain
               (steps ((rewrite (premise hpre) lr lhs true ())))
               (rewrite-with (lemma {lem}_norm) lr lhs ((inst rs2 {RSF}) (inst mem2 {NORM})) ({D('hrax')} {D('hb')}))
               refl))
           (steps
             ((rewrite (premise hrun) lr both true ())
              (rewrite (premise ho) lr rhs true ())
              (unfold ixt_expect rhs) (reduce rhs)
              (rewrite (lemma fs_out_of_norm) lr rhs true ())
              (rewrite (premise hmem) lr rhs true ()){SCR_RHS}{acc_rw('both')})
             refl)))
       (case IpTrap
         (chain
           {cap('ho', '(= out IpTrap)')}
           (steps ((rewrite (premise ho) lr rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)))
       (case IpFailed (fam)
         (chain
           {cap('ho', '(= out (IpFailed fam))')}
           (have hb (= {BRUN} (Some XTrap))
             (steps ((rewrite (premise hp2) rl lhs true ()) (rewrite (premise 9) lr lhs true ()) (rewrite (premise ho) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs)) refl))
           (have hrun (= {RUN} (Some XTrap))
             (chain
               (steps ((rewrite (premise hpre) lr lhs true ())))
               (rewrite-with (lemma {lem}_trap) lr lhs ((inst rs2 {RS}) (inst mem2 mem0)) ({D('hrax')} {D('hb')}))
               refl))
           (steps ((rewrite (premise hrun) lr both true ()) (rewrite (premise ho) lr rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)))))))
"""

WHV = "(ce IExp) (b (List IpStmt)) (ic (List XInstr)) (ib (List XInstr)) (cv Int) (out IpOut)"
LOOPL = f"(ix_app ic {REST})"
def step_while():
    """the statement wrapper: XBlock [XLoop L]; the loop request's simulation is the premise"""
    RUN = XRUN("is"); F = "(xt (- c 4) g)"
    RUNw = f"(xeval_loop {F} m {LOOPL} {RS} {XM})"
    MEMw = f"(fp_mem mem0 (fp_app {TR('f', '(IqWhile ce b)', 'lc', IM)} psx))"
    MEM = MTW('(S f)', '(IqStmt (IpWhile ce b))', 'lc', IM, 'psx')
    NORM = f"(fp_mem mem0 (fp_app {TRW('f', 'ce', 'b', 'lc', IM)} psx))"
    RSW = f"(MkRegs (rax_of (xo_bregs {RUNw})) (rcx_of {RS}) (rdx_of (xo_bregs {RUNw})) (rbx_of {RS}) (rbp_of {RS}) (rsi_of {RS}) (rdi_of {RS}) (r8_of {RS}) (r9_of {RS}) (r10_of (xo_bregs {RUNw})) (r11_of (xo_bregs {RUNw})) (r12_of {RS}) (r13_of {RS}) (r14_of {RS}) (r15_of {RS}))"
    return f"""(claim fs_step_while
  (goal ((ce IExp) (b (List IpStmt)) (ic (List XInstr)) (ib (List XInstr)) (out IpOut) {CVARS})
    ((= (ixf_stmt nl own fail_ix (IpWhile ce b)) (Some is))
     (= (ixf_exp nl 0 ce) (Some ic))
     (= (ixf_stmts nl own fail_ix b) (Some ib))
     (= {ST('(S f)', '(IpWhile ce b)', 'lc', IM)} (Some out))
     (= (le 4 c) True)
     (= {RUNw} {EXPECT('(IqWhile ce b)', 'out', RS, RUNw, MEMw)}))
    (= {RUN} {EXPECT('(IqStmt (IpWhile ce b))', 'out', RS, RUN, MEM)}))
  (chain
    (have hsome (= (Some (Cons (XBlock (Cons (XLoop {LOOPL}) Nil)) Nil)) (Some is))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise 1) lr rhs true ()) (reduce rhs) (rewrite (premise 2) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hsome) (his))
    (have hout (= {WH('f', 'ce', 'b', 'lc', IM)} (Some out))
      (steps ((rewrite (premise 3) rl rhs true ()) (unfold ipstmt rhs) (reduce rhs)) refl))
    (have hp (= (xt c g) (S (S (S (S {F}))))) (rewrite-with (lemma xt_peel4) lr lhs () ({D(4)}) refl))
    (have hmem (= {MEM} {NORM}) (steps ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs)) refl))
    (have hmemw (= {MEMw} {NORM}) (steps ((unfold ipt_tr lhs) (reduce lhs)) refl))
    (case-on out IpOut
      ((case IpNorm (lc2 mem2)
         (chain
           {cap('ho', '(= out (IpNorm lc2 mem2))')}
           (have hw (= {RUNw} (Some (XBrk 0 {RSW} {NORM})))
             (steps ((rewrite (premise 5) lr lhs true ()) (rewrite (premise ho) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs) (unfold fw_out lhs) (reduce lhs) (rewrite (premise hmemw) lr lhs true ())) refl))
           (have hrun (= {RUN} (Some (XNorm {RSW} {NORM})))
             (chain
               (steps ((rewrite (premise his) rl lhs true ()) (rewrite (premise hp) lr lhs true ())))
               (rewrite-with (lemma fs_loop_brk) lr lhs ((inst rs2 {RSW}) (inst mem2 {NORM})) ({D('hw')}))
               refl))
           (steps
             ((rewrite (premise hrun) lr both true ())
              (rewrite (premise ho) lr rhs true ())
              (unfold ixt_expect rhs) (reduce rhs)
              (rewrite (lemma fs_out_of_norm) lr rhs true ())
              (rewrite (premise hmem) lr rhs true ()){SCR_RHS}{acc_rw('both')})
             refl)))
       (case IpTrap
         (chain
           {cap('ho', '(= out IpTrap)')}
           (steps ((rewrite (premise ho) lr rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)))
       (case IpFailed (fam)
         (chain
           {cap('ho', '(= out (IpFailed fam))')}
           (have hw (= {RUNw} (Some XTrap))
             (steps ((rewrite (premise 5) lr lhs true ()) (rewrite (premise ho) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs)) refl))
           (have hrun (= {RUN} (Some XTrap))
             (chain
               (steps ((rewrite (premise his) rl lhs true ()) (rewrite (premise hp) lr lhs true ())))
               (rewrite-with (lemma fs_loop_trap) lr lhs ((inst rs2 {RS}) (inst mem2 mem0)) ({D('hw')}))
               refl))
           (steps ((rewrite (premise hrun) lr both true ()) (rewrite (premise ho) lr rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)))))))
"""

def step_wexit():
    RUN = f"(xeval_loop (xt c g) m is {RS} {XM})"
    RUNc = f"(xeval_seq (xt (- c 1) g) m ic {RS} {XM})"; rsc = RSC(RUNc)
    Hp = "(- (- c 1) (xil ic))"
    MEM = MTW('(S f)', '(IqWhile ce b)', 'lc', IM, 'psx')
    sl = Slots([f"p{i}" for i in range(8)] + ["hsome", "his", "hout", "ho", "hxn"])
    Chc1 = sl.cert({'p6': 1, 'hxn': 1}); sl.add('hc1')
    Chle = sl.cert({'p6': 1}); sl.add('hle')
    Ch2 = sl.cert({'p6': 1}); sl.add('h2')
    for n in ["hp1", "hp2", "hrax"]: sl.add(n)
    Ch10 = sl.cert({}, G=-1); sl.add('h10')
    Ch11 = sl.cert2({}, {})
    return f"""(claim fw_step_exit
  (goal ({WHV} {CVARS})
    ((= (ixt_emit nl own fail_ix (IqWhile ce b)) (Some is))
     (= (ixf_exp nl 0 ce) (Some ic))
     (= (ixf_stmts nl own fail_ix b) (Some ib))
     (= {WH('(S f)', 'ce', 'b', 'lc', IM)} (Some out))
     (= (iexp ce lc {IM} mlo slo) (Some cv))
     (= (int_eq cv 0) True)
     (= (le (+ (xil ic) 3) c) True)
     (= {RUNc} (fe_out cv {RS} {RUNc} {Mc})))
    (= {RUN} {EXPECT('(IqWhile ce b)', 'out', RS, RUN, MEM)}))
  (chain
    (have hsome (= (Some {LOOPL}) (Some is))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixt_emit rhs) (reduce rhs) (rewrite (premise 1) lr rhs true ()) (reduce rhs) (rewrite (premise 2) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hsome) (his))
    (have hout (= (Some (IpNorm lc {IM})) (Some out))
      (steps ((rewrite (premise 3) rl rhs true ()) (unfold ipwhile rhs) (reduce rhs) (rewrite (premise 4) lr rhs true ()) (reduce rhs) (rewrite (premise 5) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hout) (ho))
    (have hxn (= (le 0 (xil ic)) True) (rewrite-with (lemma xil_nonneg) lr lhs () () refl))
    (have hc1 (= (lt 0 c) True) (by arith {Chc1}))
    (have hle (= (le (xil ic) (- c 1)) True) (by arith {Chle}))
    (have h2 (= (le 2 {Hp}) True) (by arith {Ch2}))
    (have hp1 (= (xt c g) (S (xt (- c 1) g))) (rewrite-with (lemma xt_peel) lr lhs () ({D('hc1')}) refl))
    (have hp2 (= (xt {Hp} g) (S (S (xt (- {Hp} 2) g)))) (rewrite-with (lemma xt_peel2) lr lhs () ({D('h2')}) refl))
    (have hrax (= (int_eq (rax_of {rsc}) 0) True) (steps ((rewrite (lemma rax_of_mk) lr lhs true ()) (rewrite (premise 5) lr lhs true ())) refl))
    (have h10 (= (int_eq 1 0) False) (by arith {Ch10}))
    (have h11 (= (- 1 1) 0) (by arith {Ch11}))
    (have hrun (= {RUN} (Some (XBrk 0 {rsc} {Mc})))
      (chain
        (steps ((rewrite (premise his) rl lhs true ()) (rewrite (premise hp1) lr lhs true ()) (unfold xeval_loop lhs) (reduce lhs)))
        (rewrite-with (lemma xseq_app) lr lhs () ({D('hle')}))
        (steps ((rewrite (premise 7) lr lhs true ()) (unfold fe_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs){acc_rw('lhs')} (rewrite (premise hp2) lr lhs true ())))
        (rewrite-with (lemma fw_rest_exit) lr lhs ((inst rs2 {RS}) (inst mem2 mem0)) ({D('hrax')}))
        (steps ((reduce lhs) (rewrite (premise h10) lr lhs true ()) (reduce lhs) (rewrite (premise h11) lr lhs true ())) refl)))
    (steps
      ((rewrite (premise hrun) lr both true ())
       (rewrite (premise ho) rl rhs true ())
       (unfold ixt_expect rhs) (reduce rhs)
       (rewrite (lemma fw_out_of_brk) lr rhs true ()){SCR_RHS}{acc_rw('rhs')}
       (unfold ipt_tr rhs) (reduce rhs) (unfold ipt_while rhs) (reduce rhs)
       (rewrite (premise 4) lr rhs true ()) (reduce rhs) (rewrite (premise 5) lr rhs true ()) (reduce rhs))
      refl)))
"""

def step_witer(fail):
    RUN = f"(xeval_loop (xt c g) m is {RS} {XM})"
    RUNc = f"(xeval_seq (xt (- c 1) g) m ic {RS} {XM})"; rsc = RSC(RUNc)
    Hp = "(- (- c 1) (xil ic))"; H3 = f"(- {Hp} 3)"; F = f"(xt (- {H3} 1) g)"
    RUNb = f"(xeval_seq (xt {H3} g) m ib {rsc} {Mc})"
    PSXb = f"(fp_app {TR('f', '(IqStmts b)', 'lc', IMx(PSX1c))} {PSX1c})"
    MEMb = f"(fp_mem mem0 {PSXb})"
    MEM = MTW('(S f)', '(IqWhile ce b)', 'lc', IM, 'psx')
    bout = "(IpFailed fam)" if fail else "(IpNorm lcb memb)"
    RSB = RSP(RUNb)
    RUNw2 = f"(xeval_loop (xt (- c 1) g) m is {RSB} {MEMb})"
    RUNw2p = f"(xeval_loop (xt (- c 1) g) m {LOOPL} {RSB} {MEMb})"
    MEMw2 = f"(fp_mem mem0 (fp_app {TR('f', '(IqWhile ce b)', 'lcb', IMx(PSXb))} {PSXb}))"
    NORM2 = f"(fp_mem mem0 (fp_app {TRW('f', 'ce', 'b', 'lcb', 'memb')} (fp_app {TRL('f', 'b', 'lc', IM)} {PSX1c})))"
    if fail:
        extra_vars = "(fam IFam)"
        prems = f"""     (= {STS('f', 'b', 'lc', IM)} (Some (IpFailed fam)))
     (= {RUNb} {EXPECT('(IqStmts b)', '(IpFailed fam)', rsc, RUNb, MEMb)}))"""
        nprem = 10
    else:
        extra_vars = "(lcb (List Int)) (memb Mem)"
        prems = f"""     (= {STS('f', 'b', 'lc', IM)} (Some (IpNorm lcb memb)))
     (= (fp_mem mem0 (fbelow slo {PSXb})) memb)
     (= {RUNb} {EXPECT('(IqStmts b)', '(IpNorm lcb memb)', rsc, RUNb, MEMb)})
     (= {RUNw2} {EXPECT('(IqWhile ce b)', 'out', RSB, RUNw2, MEMw2)})
     (= (fbelow slo {PSX1c}) (fbelow slo psx)))"""
        nprem = 13
    sl = Slots([f"p{i}" for i in range(nprem)] + ["hsome", "his", "hxn"])
    Chc1 = sl.cert({'p6': 1, 'hxn': 1}); sl.add('hc1')
    Chle = sl.cert({'p6': 1}); sl.add('hle')
    Ch3 = sl.cert({'p6': 1}); sl.add('h3')
    Ch1 = sl.cert({'p6': 1}); sl.add('h1')
    name = "fw_step_iter_fail" if fail else "fw_step_iter"
    common = f"""  (chain
    (have hsome (= (Some {LOOPL}) (Some is))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixt_emit rhs) (reduce rhs) (rewrite (premise 1) lr rhs true ()) (reduce rhs) (rewrite (premise 2) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hsome) (his))
    (have hxn (= (le 0 (xil ic)) True) (rewrite-with (lemma xil_nonneg) lr lhs () () refl))
    (have hc1 (= (lt 0 c) True) (by arith {Chc1}))
    (have hle (= (le (xil ic) (- c 1)) True) (by arith {Chle}))
    (have h3 (= (le 3 {Hp}) True) (by arith {Ch3}))
    (have h1 (= (lt 0 {H3}) True) (by arith {Ch1}))
    (have hp1 (= (xt c g) (S (xt (- c 1) g))) (rewrite-with (lemma xt_peel) lr lhs () ({D('hc1')}) refl))
    (have hp3 (= (xt {Hp} g) (S (S (S (xt {H3} g))))) (rewrite-with (lemma xt_peel3) lr lhs () ({D('h3')}) refl))
    (have hp4 (= (xt {H3} g) (S {F})) (rewrite-with (lemma xt_peel) lr lhs () ({D('h1')}) refl))
    (have hrax (= (int_eq (rax_of {rsc}) 0) False) (steps ((rewrite (lemma rax_of_mk) lr lhs true ()) (rewrite (premise 5) lr lhs true ())) refl))
    (have h00 (= (int_eq 0 0) True) (by arith (list)))"""
    if fail:
        body = f"""
    (have hout (= (Some (IpFailed fam)) (Some out))
      (steps ((rewrite (premise 3) rl rhs true ()) (unfold ipwhile rhs) (reduce rhs) (rewrite (premise 4) lr rhs true ()) (reduce rhs) (rewrite (premise 5) lr rhs true ()) (reduce rhs) (rewrite (premise 8) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hout) (ho))
    (have hb (= (xeval_seq (S {F}) m ib {rsc} {Mc}) (Some XTrap))
      (steps ((rewrite (premise hp4) rl lhs true ()) (rewrite (premise 9) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs)) refl))
    (have hrun (= {RUN} (Some XTrap))
      (chain
        (steps ((rewrite (premise his) rl lhs true ()) (rewrite (premise hp1) lr lhs true ()) (unfold xeval_loop lhs) (reduce lhs)))
        (rewrite-with (lemma xseq_app) lr lhs () ({D('hle')}))
        (steps ((rewrite (premise 7) lr lhs true ()) (unfold fe_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs){acc_rw('lhs')} (rewrite (premise hp3) lr lhs true ()) (rewrite (premise hp4) lr lhs true ())))
        (rewrite-with (lemma fw_rest_trap) lr lhs ((inst rs2 {RS}) (inst mem2 mem0)) ({D('hrax')} {D('hb')}))
        (steps ((reduce lhs)) refl)))
    (steps ((rewrite (premise hrun) lr both true ()) (rewrite (premise ho) rl rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)))
"""
    else:
        body = f"""
    (have hout (= {WH('f', 'ce', 'b', 'lcb', 'memb')} (Some out))
      (steps ((rewrite (premise 3) rl rhs true ()) (unfold ipwhile rhs) (reduce rhs) (rewrite (premise 4) lr rhs true ()) (reduce rhs) (rewrite (premise 5) lr rhs true ()) (reduce rhs) (rewrite (premise 8) lr rhs true ()) (reduce rhs)) refl))
    (have hb (= (xeval_seq (S {F}) m ib {rsc} {Mc}) (Some (XNorm {RSB} {MEMb})))
      (steps ((rewrite (premise hp4) rl lhs true ()) (rewrite (premise 10) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs) (unfold fs_out lhs) (reduce lhs){acc_rw('lhs')}) refl))
    (have hrun (= {RUN} {RUNw2p})
      (chain
        (steps ((rewrite (premise his) rl lhs true ()) (rewrite (premise hp1) lr lhs true ()) (unfold xeval_loop lhs) (reduce lhs)))
        (rewrite-with (lemma xseq_app) lr lhs () ({D('hle')}))
        (steps ((rewrite (premise 7) lr lhs true ()) (unfold fe_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs){acc_rw('lhs')} (rewrite (premise hp3) lr lhs true ()) (rewrite (premise hp4) lr lhs true ())))
        (rewrite-with (lemma fw_rest_norm) lr lhs ((inst rs2 {RSB}) (inst mem2 {MEMb})) ({D('hrax')} {D('hb')}))
        (steps ((reduce lhs) (rewrite (premise h00) lr lhs true ()) (reduce lhs)) refl)))
    (have h11n (= {RUNw2p} {EXPECT('(IqWhile ce b)', 'out', RSB, RUNw2p, MEMw2)})
      (steps ((rewrite (premise his) lr both true ()) (rewrite (premise 11) lr lhs true ())) refl))
    (have hmem (= {MEM} {NORM2})
      (steps ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_while lhs) (reduce lhs) (rewrite (premise 4) lr lhs true ()) (reduce lhs) (rewrite (premise 5) lr lhs true ()) (reduce lhs) (rewrite (premise 8) lr lhs true ()) (reduce lhs) (rewrite (lemma fp_app_assoc) lr lhs true ()) (rewrite (lemma fp_app_assoc) lr lhs true ())) refl))
    (have hmemw (= {MEMw2} {NORM2})
      (steps ((unfold ipt_tr lhs) (reduce lhs) (rewrite (premise 9) lr lhs true ()) (unfold ipt_tr lhs) (reduce lhs) (rewrite (premise 12) lr lhs true ())) refl))
    (steps ((rewrite (premise hrun) lr both true ())))
    (case-on out IpOut
      ((case IpNorm (lc2 mem2)
         (chain
           {cap('ho', '(= out (IpNorm lc2 mem2))')}
           (have h11w (= {RUNw2p} (fw_out {RSB} {RUNw2p} {NORM2}))
             (steps ((rewrite (premise h11n) lr lhs true ()) (rewrite (premise ho) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs) (rewrite (premise hmemw) lr lhs true ())) refl))
           (steps
             ((rewrite (premise ho) lr rhs true ())
              (unfold ixt_expect rhs) (reduce rhs)
              (rewrite (premise h11w) lr lhs true ())
              (rewrite (premise hmem) lr rhs true ())
              (unfold fw_out both) (reduce both){acc_rw('both')})
             refl)))
       (case IpTrap
         (chain
           {cap('ho', '(= out IpTrap)')}
           (steps ((rewrite (premise ho) lr rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)))
       (case IpFailed (fam)
         (chain
           {cap('ho', '(= out (IpFailed fam))')}
           (have h11t (= {RUNw2p} (Some XTrap))
             (steps ((rewrite (premise h11n) lr lhs true ()) (rewrite (premise ho) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs)) refl))
           (steps ((rewrite (premise ho) lr rhs true ()) (unfold ixt_expect rhs) (reduce rhs) (rewrite (premise h11t) lr lhs true ())) refl)))))))
"""
    return f"""(claim {name}
  (goal ({WHV} {extra_vars} {CVARS})
    ((= (ixt_emit nl own fail_ix (IqWhile ce b)) (Some is))
     (= (ixf_exp nl 0 ce) (Some ic))
     (= (ixf_stmts nl own fail_ix b) (Some ib))
     (= {WH('(S f)', 'ce', 'b', 'lc', IM)} (Some out))
     (= (iexp ce lc {IM} mlo slo) (Some cv))
     (= (int_eq cv 0) False)
     (= (le (+ (xil ic) 5) c) True)
     (= {RUNc} (fe_out cv {RS} {RUNc} {Mc}))
{prems}
    (= {RUN} {EXPECT('(IqWhile ce b)', 'out', RS, RUN, MEM)}))
{common}{body}"""

def ctl():
    return "\n".join([ctl_blocks(), ctl_nil(), ctl_cons(), ctl_cons_fail(), step_if('t'), step_if('e'), step_while(), step_wexit(), step_witer(False), step_witer(True)])

BANNER_CTL = ";; --- the CONTROL STEP LEMMAS (generated by gen_fra4.py ctl — REGENERATE, never hand-patch) ---"
BLOCKS.append(("ctl", BANNER_CTL, ctl))

# ---------------- the emission length is bounded by the statement cost ----------------
def slen_cost():
    """ixf_stmt s = Some iss  ->  xil iss <= ixf_scost s   (under the A-4 fence)"""
    def arm(ctor, binders, S, body):
        b = f" ({binders})" if binders else ""
        return f"(case {ctor}{b} (chain {cap('hst', f'(= s {S})')} {body}))"
    def fenced(S):
        return f"""(have hn (= False True) (steps ((rewrite (premise 1) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_a4s rhs) (reduce rhs)) refl)) (absurd (premise hn))"""
    def via_exp(S, e, tail_len, cost_k):
        # emission = ix_app ie TAIL with |TAIL| = tail_len; cost = ecost e + cost_k = elen e + 5 + cost_k
        return f"""(case-on (ixf_exp nl 0 {e}) Option
  ((case None (chain {cap('hie', f'(= (ixf_exp nl 0 {e}) None)')} (have hn (= None (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
   (case Some (ie)
     (chain
       {cap('hie', f'(= (ixf_exp nl 0 {e}) (Some ie))')}
       (have hlen (= (xil ie) (ixf_elen {e})) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d 0) (inst e {e})) ({D('hie')}) refl))
       (have hsome (= (Some {{EM}}) (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs)) refl))
       (inject (premise hsome) (his))
       (have hx (= (xil iss) (+ (xil ie) {tail_len})) (steps ((rewrite (premise his) rl lhs true ()) (rewrite (lemma xil_app) lr lhs true ()) (rewrite (lemma xil_st) lr lhs true ())) refl))
       (steps ((rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (unfold ixf_ecost lhs)) (by arith (list 1 0 0 0 0 -1 0 0 -1)))))))"""
    set_ = via_exp("(IpSet i e)", "e", 3, 4).replace("{EM}", "(ix_app ie (ixf_st i))")
    if_ = via_exp("(IpIf ce tb eb)", "ce", 1, 3)
    # the if: the emission's tail is one block; needs the sub-lists' emissions too
    if_ = f"""(case-on (ixf_exp nl 0 ce) Option
  ((case None (chain {cap('hie', '(= (ixf_exp nl 0 ce) None)')} (have hn (= None (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
   (case Some (ie)
     (chain
       {cap('hie', '(= (ixf_exp nl 0 ce) (Some ie))')}
       (have hlen (= (xil ie) (ixf_elen ce)) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d 0) (inst e ce)) ({D('hie')}) refl))
       (case-on (ixf_stmts nl own fail_ix tb) Option
         ((case None (chain {cap('htb', '(= (ixf_stmts nl own fail_ix tb) None)')} (have hn (= None (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs) (rewrite (premise htb) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
          (case Some (itb)
            (chain
              {cap('htb', '(= (ixf_stmts nl own fail_ix tb) (Some itb))')}
              (case-on (ixf_stmts nl own fail_ix eb) Option
                ((case None (chain {cap('heb', '(= (ixf_stmts nl own fail_ix eb) None)')} (have hn (= None (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs) (rewrite (premise htb) lr rhs true ()) (reduce rhs) (rewrite (premise heb) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
                 (case Some (ieb)
                   (chain
                     {cap('heb', '(= (ixf_stmts nl own fail_ix eb) (Some ieb))')}
                     (have hsome (= (Some (ix_app ie (Cons (XBlock (Cons (XBlock (Cons (XBrIf (CNz RAX) 0) (Cons (XBlock ieb) (Cons (XBr 1) Nil)))) itb)) Nil))) (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs) (rewrite (premise htb) lr rhs true ()) (reduce rhs) (rewrite (premise heb) lr rhs true ()) (reduce rhs)) refl))
                     (inject (premise hsome) (his))
                     (have hx (= (xil iss) (+ (xil ie) 1)) (steps ((rewrite (premise his) rl lhs true ()) (rewrite (lemma xil_app) lr lhs true ()) (rewrite (lemma xil_one) lr lhs true ())) refl))
                     (steps ((rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (unfold ixf_ecost lhs)) (by arith (list 1 0 0 0 0 -1 0 0 0 0 -1)))))))))))))))"""
    wh = f"""(case-on (ixf_exp nl 0 ce) Option
  ((case None (chain {cap('hie', '(= (ixf_exp nl 0 ce) None)')} (have hn (= None (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
   (case Some (ie)
     (chain
       {cap('hie', '(= (ixf_exp nl 0 ce) (Some ie))')}
       (case-on (ixf_stmts nl own fail_ix b) Option
         ((case None (chain {cap('hb', '(= (ixf_stmts nl own fail_ix b) None)')} (have hn (= None (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs) (rewrite (premise hb) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
          (case Some (ib)
            (chain
              {cap('hb', '(= (ixf_stmts nl own fail_ix b) (Some ib))')}
              (have hsome (= (Some (Cons (XBlock (Cons (XLoop (ix_app ie (Cons (XBrIf (CEqz RAX) 1) (Cons (XBlock ib) (Cons (XBr 0) Nil))))) Nil)) Nil)) (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs) (rewrite (premise hb) lr rhs true ()) (reduce rhs)) refl))
              (inject (premise hsome) (his))
              (have hx (= (xil iss) 1) (steps ((rewrite (premise his) rl lhs true ()) (compute lhs)) refl))
              (have he (= (le 0 (ixf_elen ce)) True) (rewrite-with (lemma ixf_elen_nonneg) lr lhs () () refl))
              (steps ((rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (unfold ixf_ecost lhs)) (by arith (list 1 0 0 0 0 0 0 0 -1 1)))))))))))"""
    fail = f"""(chain
  (have hsome (= (Some (Cons (XMovRI RDI (match fam (FOverflow 70) (FOom 71) (FStack 72))) (Cons (XCall fail_ix) Nil))) (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs)) refl))
  (inject (premise hsome) (his))
  (steps ((rewrite (premise his) rl lhs true ()) (rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (compute lhs)) refl))"""
    unreach = f"""(chain
  (have hsome (= (Some (Cons (XMovRI RAX 18446744073709547520) (Cons (XMem (XMLoad64 RAX (AReg RAX))) Nil))) (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs)) refl))
  (inject (premise hsome) (his))
  (steps ((rewrite (premise his) rl lhs true ()) (rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (compute lhs)) refl))"""
    return f"""(claim slen_cost
  (goal ((s IpStmt) (iss (List XInstr)) (nl Int) (own Int) (fail_ix Int))
    ((= (ixf_stmt nl own fail_ix s) (Some iss)) (= (ixf_a4s s) True))
    (= (le (xil iss) (ixf_scost s)) True))
  (case-on s IpStmt
    ({arm('IpSet', 'i e', '(IpSet i e)', set_)}
     {arm('IpStore', 'ae ve', '(IpStore ae ve)', fenced('(IpStore ae ve)'))}
     {arm('IpIf', 'ce tb eb', '(IpIf ce tb eb)', if_)}
     {arm('IpWhile', 'ce b', '(IpWhile ce b)', wh)}
     {arm('IpCall', 'i k args', '(IpCall i k args)', fenced('(IpCall i k args)'))}
     {arm('IpLoadW', 'i ae', '(IpLoadW i ae)', fenced('(IpLoadW i ae)'))}
     {arm('IpStoreW', 'ae ve', '(IpStoreW ae ve)', fenced('(IpStoreW ae ve)'))}
     {arm('IpFail', 'fam', '(IpFail fam)', fail)}
     {arm('IpUnreach', '', 'IpUnreach', unreach)})))
"""

BANNER_SLEN = ";; --- the emission length is bounded by the cost (generated by gen_fra4.py slen — REGENERATE, never hand-patch) ---"
BLOCKS.append(("slen", BANNER_SLEN, slen_cost))

# ---------------- ipt_sound: THE DISPATCHER INDUCTION (x86 side) ----------------
# One claim over the request type; induction on the imp fuel f; every arm decomposes the
# engine's run, derives the sub-requests' premises (side predicates by extraction, the
# post-state by the twin laws, the heights by arithmetic), re-enters the IH at
# xt (c+K) (kf K f2), and cites the step lemma.

SV = ("(f Nat) (r IptReq) (K Int) (c Int) (out IpOut) (is (List XInstr)) (nl Int) (own Int) (fail_ix Int) (lc (List Int)) (mem0 Mem) (psx (List FPatch)) "
      "(mlo Int) (slo Int) (m XModule) (fp Int) (dep Int) (fs (List IpFn)) (dmax Int) "
      "(a0 Int) (rcx Int) (dx Int) (rbx Int) (rbp Int) (rsi Int) (di Int) (r8 Int) (r9 Int) (s10 Int) (s11 Int) (r12 Int) (r13 Int)")
NPREM = 12
def XR(cc, ff, r, is_, rs, xm): return f"(ixt_run (xt {cc} (kf K {ff})) m {r} {is_} {rs} {xm})"
def MEMOF(ff, r, lc, im, psx): return f"(fp_mem mem0 (fp_app {TR(ff, r, lc, im)} {psx}))"
def CONCL(ff, r, lc, im, psx, out, rs, run): return f"(= {run} (ixt_expect {r} {out} {rs} {run} {MEMOF(ff, r, lc, im, psx)}))"
CK = "(+ c K)"
KF2 = "(kf K f2)"
IH_PINS = "(inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst mlo mlo) (inst slo slo) (inst fs fs) (inst dmax dmax)"
def MK(a0, dx, s10, s11): return f"(MkRegs {a0} rcx {dx} rbx rbp rsi di r8 r9 {s10} {s11} r12 r13 dep fp)"

class Arm:
    """an arm's text builder with the Farkas slot tracker kept in step"""
    def __init__(self, sl, ind):
        self.sl = Slots(sl.names); self.lines = []; self.ind = ind
    def add(self, name, text):
        self.lines.append(self.ind + text); 
        if name: self.sl.add(name)
    def have(self, name, fact, just):
        self.add(name, f"(have {name} {fact}\n{self.ind}  {just})")
    def arith(self, name, fact, m, G=1):
        self.have(name, fact, f"(by arith {self.sl.cert(m, G)})")
    def arith2(self, name, fact, le, ge):
        self.have(name, fact, f"(by arith {self.sl.cert2(le, ge)})")
    def text(self): return "\n".join(self.lines)

def side_have(A, name, fact, prem, subst):
    A.have(name, f"(= {fact} True)", f"(steps ((rewrite (premise {prem}) rl rhs true ()){SUBS(subst)} (unfold ixt_body rhs) (reduce rhs)) refl)")

def m_sides(A, body, subst):
    side_have(A, 'hscb', f"(ixf_scb {body})", 3, subst)
    side_have(A, 'hsd', f"(le (+ fp (* 8 (+ nl (ixf_sdep {body})))) (xmemhi_of m))", 4, subst)
    side_have(A, 'hkok', f"(ixf_skok K {body})", 5, subst)
    side_have(A, 'ha4', f"(ixf_a4 {body})", 9, subst)

def m_cost(A, req, subst, unfolds):
    """hcost: the request's cost premise at the concrete request, unfolded as asked"""
    u = "".join(f" (unfold {fn} rhs) (reduce rhs)" for fn in unfolds)
    A.have('hcs', f"(= (le {{COST}} c) True)", f"(steps ((rewrite (premise 7) rl rhs true ()){SUBS(subst)}{u}) refl)")

def m_fuel(A):
    A.have('hkf', f"(= (kf K (S f2)) (xt K {KF2}))", "(rewrite-with (lemma kf_s) lr lhs () () refl)")
    A.arith('hk0', "(= (le 0 K) True)", {'p6': 1})
    A.have('hadd', f"(= (xt c (xt K {KF2})) (xt {CK} {KF2}))", f"(rewrite-with (lemma xt_add) lr lhs () ({D(8)} {D('hk0')}) refl)")
    A.add(None, "(steps ((rewrite (premise hkf) lr both true ()) (rewrite (premise hadd) lr both true ())))")

def m_ih(A, name, r, cc, is_, rs, psx, lc, out, ds):
    """the IH at f2: (name) in ixt_run form, (name2) in the engine's form; ds = the 12 discharges"""
    run = f"(ixt_run (xt {cc} {KF2}) m {r} {is_} {rs} (fp_mem mem0 {psx}))"
    mem = f"(fp_mem mem0 (fp_app {TR('f2', r, lc, IMx(psx))} {psx}))"
    A.have(name, f"(= {run} (ixt_expect {r} {out} {rs} {run} {mem}))",
           f"(rewrite-with (hyp ih) lr lhs ((inst out {out}) (inst lc {lc}) {IH_PINS}) ({' '.join(ds)}) refl)")
    eng = "xeval_loop" if r.startswith("(IqWhile") else "xeval_seq"
    runx = f"({eng} (xt {cc} {KF2}) m {is_} {rs} (fp_mem mem0 {psx}))"
    A.have(name + 'u', f"(= {run} {runx})", "(steps ((unfold ixt_run lhs) (reduce lhs)) refl)")
    A.have(name + '2', f"(= {runx} (ixt_expect {r} {out} {rs} {runx} {mem}))",
           f"(steps ((rewrite (premise {name}u) rl both true ()) (rewrite (premise {name}) lr lhs true ())) refl)")
    return runx, mem

def body_d(h): return f"(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise {h}) lr lhs true ())) refl)"
def emit_d(h): return f"(steps ((unfold ixt_emit lhs) (reduce lhs) (rewrite (premise {h}) lr lhs true ())) refl)"

def m_trap(A, engine, rewrites, subst):
    """the imp run is a trap: the conclusion is the run itself"""
    rw = "".join(f" {r} (reduce rhs)" for r in rewrites)
    A.have('hout', "(= (Some IpTrap) (Some out))", f"(steps ((rewrite (premise 1) rl rhs true ()){SUBS(subst)} (unfold ipt_run rhs) (reduce rhs) (unfold {engine} rhs) (reduce rhs){rw}) refl)")
    A.add('ho', "(inject (premise hout) (ho))")
    A.add(None, "(steps ((rewrite (premise ho) rl rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)")

def m_absurd_emit(A, hem_term, unfold, rewrites):
    rw = "".join(f" {r} (reduce rhs)" for r in rewrites)
    return f"(chain (have hn (= None (Some is)) (steps ((rewrite (premise hem) rl rhs true ()) (unfold {unfold} rhs) (reduce rhs){rw}) refl)) (absurd (premise hn)))"

def case_opt(scrut, var, none_text, some_fn):
    """a case-on an Option-valued emission/evaluation: None => none_text, Some var => some_fn() (with h{var} captured)"""
    return f"""(case-on {scrut} Option
  ((case None (chain {cap('h' + var, f'(= {scrut} None)')} {none_text}))
   (case Some ({var})
     (chain
       {cap('h' + var, f'(= {scrut} (Some {var}))')}
{some_fn()}))))"""

# ---- the arms ----
def m_arm_set(sl):
    I = "    "
    S = "(IpSet i e)"; sub = SS + ['ho']
    def norm():
        A = Arm(S_(sl, 'ho'), I)
        A.add(None, "(steps ((rewrite (premise ho) lr rhs true ())))")
        A.have('hem', f"(= (ixf_stmt nl own fail_ix {S}) (Some is))", f"(steps ((rewrite (premise 0) rl rhs true ()){SUBS(SS)} (unfold ixt_emit rhs) (reduce rhs)) refl)")
        def some():
            B = Arm(S_(A.sl, 'hie'), I)
            def leaf(sl2):
                C = Arm(sl2, I + "          ")
                C.have('himp', f"(= {ST('(S f2)', S, 'lc', IM)} (Some (IpNorm lc2 mem2)))", f"(steps ((rewrite (premise 1) rl rhs true ()){SUBS(sub)} (unfold ipt_run rhs) (reduce rhs)) refl)")
                C.have('hcbe', "(= (ixf_cb e) True)", f"(rewrite-with (lemma scb_set_e) lr lhs ((inst i i) (inst t Nil)) ({D('hscb')}) refl)")
                C.have('hsde', "(= (le (ixf_dep e) (ixf_sdep (Cons (IpSet i e) Nil))) True)", "(rewrite-with (lemma sdep_set_e) lr lhs ((inst t Nil)) () refl)")
                C.arith('hdepe', "(= (le (+ fp (* 8 (+ nl (ixf_dep e)))) (xmemhi_of m)) True)", {'hsd': 1, 'hsde': 8})
                C.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
                pins = "(inst i i) (inst e e) (inst ie ie) (inst v v) (inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst lc lc) (inst lc2 lc2) (inst mem2 mem2) (inst mlo mlo) (inst slo slo) (inst f f2) (inst fs fs) (inst dmax dmax)"
                C.add(None, f"(rewrite-with (lemma fs_step_set) lr lhs ({pins}) ({D('hem')} {D('himp')} {D(2)} {D('hcbe')} {D('hdepe')} {D('hcs')} {D('hie')} {D('hv')}))")
                C.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs) (unfold ixt_expect rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs) (unfold ipt_stmt rhs) (reduce rhs)) refl)")
                return C.text()
            B.add(None, arm_set(leaf, B.sl, sub))
            return B.text()
        A.add(None, case_opt("(ixf_exp nl 0 e)", "ie", m_absurd_emit(A, None, "ixf_stmt", [RV('hie')]), some))
        return A.text()
    def failed():
        A = Arm(S_(sl, 'ho'), I)
        A.add(None, arm_set_abs(A.sl, sub))
        return A.text()
    return f"""(case-on out IpOut
  ((case IpNorm (lc2 mem2) (chain {cap('ho', '(= out (IpNorm lc2 mem2))')}
{norm()}))
   (case IpTrap (chain {cap('ho', '(= out IpTrap)')} (steps ((rewrite (premise ho) lr rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)))
   (case IpFailed (fam) (chain {cap('ho', '(= out (IpFailed fam))')}
{failed()}))))"""

def arm_set_abs(sl, subst):
    """IpSet never fails: every leg of the engine is a trap or a normal outcome"""
    OUTF = "(Some (IpFailed fam))"
    def ab(rewrites, lhs):
        rw = "".join(f" {r} (reduce rhs)" for r in rewrites)
        return f"(chain (have hn (= {lhs} {OUTF}) (steps ((rewrite (premise 1) rl rhs true ()){SUBS(subst)} (unfold ipt_run rhs) (reduce rhs) (unfold ipstmt rhs) (reduce rhs){rw}) refl)) (inject (premise hn) (hx)) (absurd (premise hx)))"
    return f"""(case-on (iexp e lc {IM} mlo slo) Option
  ((case None (chain {cap('hv', f'(= (iexp e lc {IM} mlo slo) None)')} {ab([RV('hv')], '(Some IpTrap)')}))
   (case Some (v)
     (chain
       {cap('hv', f'(= (iexp e lc {IM} mlo slo) (Some v))')}
       (case-on (ilset lc i v) Option
         ((case None (chain {cap('hset', '(= (ilset lc i v) None)')} {ab([RV('hv'), RV('hset')], '(Some IpTrap)')}))
          (case Some (lcs) (chain {cap('hset', '(= (ilset lc i v) (Some lcs))')} {ab([RV('hv'), RV('hset')], f'(Some (IpNorm lcs {IM}))')}))))))))"""

def m_arm_if(sl):
    I = "    "
    S = "(IpIf ce tb eb)"
    A = Arm(sl, I)
    A.have('hem', f"(= (ixf_stmt nl own fail_ix {S}) (Some is))", f"(steps ((rewrite (premise 0) rl rhs true ()){SUBS(SS)} (unfold ixt_emit rhs) (reduce rhs)) refl)")
    def ic_some():
        B = Arm(S_(A.sl, 'hic'), I)
        def tb_some():
            C = Arm(S_(B.sl, 'hitb'), I)
            def eb_some():
                E = Arm(S_(C.sl, 'hieb'), I)
                E.have('himp', f"(= {ST('(S f2)', S, 'lc', IM)} (Some out))", f"(steps ((rewrite (premise 1) rl rhs true ()){SUBS(SS)} (unfold ipt_run rhs) (reduce rhs)) refl)")
                def cv_none():
                    F = Arm(S_(E.sl, 'hcv0'), I); return "(chain " + m_trap_text(F, 'ipstmt', [RV('hcv0')], 'himp') + ")"
                def cv_some():
                    F = Arm(S_(E.sl, 'hcv'), I)
                    cf, F.sl = ctx_facts(F.sl, 'p2', I); F.lines.append(cf)
                    F.lines.append(ctx_after_cond('ce', I)); 
                    for n in ['hdisca', 'hlocsa', 'hfb', 'hctx1']: F.sl.add(n)
                    m_fuel(F)
                    F.have('hlen', "(= (xil ic) (ixf_elen ce))", f"(rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d 0) (inst e ce)) ({D('hic')}) refl)")
                    F.have('hcbc', "(= (ixf_cb ce) True)", f"(rewrite-with (lemma scb_if_c) lr lhs ((inst tb tb) (inst eb eb) (inst t Nil)) ({D('hscb')}) refl)")
                    F.have('hsdc', "(= (le (ixf_dep ce) (ixf_sdep (Cons (IpIf ce tb eb) Nil))) True)", "(rewrite-with (lemma sdep_if_c) lr lhs ((inst t Nil)) () refl)")
                    F.arith('hdepc', "(= (le (+ fp (* 8 (+ nl (+ 0 (ixf_dep ce))))) (xmemhi_of m)) True)", {'hsd': 1, 'hsdc': 8})
                    F.have('hec', "(= (ixf_ecost ce) (+ (ixf_elen ce) 5))", "(steps ((unfold ixf_ecost lhs)) refl)")
                    F.arith('hecost', f"(= (le (ixf_ecost ce) {CK}) True)", {'hcs': 1, 'p6': 1})
                    RUNc = f"(xeval_seq (xt {CK} {KF2}) m ic {RS} {XM})"
                    F.have('hsim', f"(= {RUNc} (fe_out cv {RS} {RUNc} {Mc}))", f"(rewrite-with (lemma fe_sound) lr lhs ((inst e ce) (inst nl nl) (inst d 0) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst v cv)) ({D('hic')} {D('hcv')} {D(2)} {D('hcbc')} {D('hdepc')} {D('hecost')}) refl)")
                    F.arith('hc6', f"(= (le (+ (xil ic) 6) {CK}) True)", {'hcs': 1, 'hlen': 1, 'hec': 1, 'p6': 1})
                    F.arith('hc8', f"(= (le (+ (xil ic) 8) {CK}) True)", {'hcs': 1, 'hlen': 1, 'hec': 1, 'p6': 1})
                    F.arith('hc0', f"(= (le 0 {CK}) True)", {'p8': 1, 'p6': 1})
                    rsc = RSC(RUNc)
                    def branch(flag, ss, iss, ext, cc, step):
                        G = Arm(S_(F.sl, 'hcvf'), I)
                        G.have('hbr', f"(= {STS('f2', ss, 'lc', IM)} (Some out))", f"(steps ((rewrite (premise himp) rl rhs true ()) (unfold ipstmt rhs) (reduce rhs) (rewrite (premise hcv) lr rhs true ()) (reduce rhs) (rewrite (premise hcvf) lr rhs true ()) (reduce rhs)) refl)")
                        G.have('hcbb', f"(= (ixf_scb {ss}) True)", f"(rewrite-with (lemma scb_if_{ext}) lr lhs ((inst ce ce) (inst tb tb) (inst eb eb) (inst t Nil)) ({D('hscb')}) refl)")
                        G.have('hsdb', f"(= (le (ixf_sdep {ss}) (ixf_sdep (Cons (IpIf ce tb eb) Nil))) True)", f"(rewrite-with (lemma sdep_if_{ext}) lr lhs ((inst t Nil)) () refl)")
                        G.arith('hsdt', f"(= (le (+ fp (* 8 (+ nl (ixf_sdep {ss})))) (xmemhi_of m)) True)", {'hsd': 1, 'hsdb': 8})
                        G.have('hkb', f"(= (ixf_skok K {ss}) True)", f"(rewrite-with (lemma skok_if_{ext}) lr lhs ((inst ce ce) (inst tb tb) (inst eb eb) (inst t Nil)) ({D('hkok')}) refl)")
                        G.have('hab', f"(= (ixf_a4 {ss}) True)", f"(rewrite-with (lemma a4_if_{ext}) lr lhs ((inst ce ce) (inst tb tb) (inst eb eb) (inst t Nil)) ({D('ha4')}) refl)")
                        G.arith('hcb0', f"(= (le 0 {cc}) True)", {'hcs': 1, 'hlen': 1, 'hec': 1, 'p6': 1})
                        ds = [emit_d('h' + iss), f"(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hbr) lr lhs true ())) refl)", D('hctx1'), body_d('hcbb'), body_d('hsdt'), body_d('hkb'), D(6),
                              f"(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hcb0) lr lhs true ())) refl)", D('hcb0'), body_d('hab'), D(10), D(11)]
                        m_ih(G, 'hih', f"(IqStmts {ss})", cc, iss, rsc, PSX1c, 'lc', 'out', ds)
                        G.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
                        pins = "(inst ce ce) (inst tb tb) (inst eb eb) (inst ic ic) (inst itb itb) (inst ieb ieb) (inst cv cv) (inst out out) (inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst f f2) (inst fs fs) (inst dmax dmax)"
                        cost = 'hc6' if step == 't' else 'hc8'
                        G.add(None, f"(rewrite-with (lemma fs_step_if_{step}) lr lhs ({pins}) ({D('hem')} {D('hic')} {D('hitb')} {D('hieb')} {D('himp')} {D('hcv')} {D('hcvf')} {D(cost)} {D('hsim')} {D('hih2')} {D('hfb')} {D('hc0')}))")
                        G.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs)) refl)")
                        return G.text()
                    F.add(None, f"""(case-on (int_eq cv 0) Bool
  ((case True (chain {cap('hcvf', '(= (int_eq cv 0) True)')}
{branch('True', 'eb', 'ieb', 'e', f'(- (- {CK} (xil ic)) 7)', 'e')}))
   (case False (chain {cap('hcvf', '(= (int_eq cv 0) False)')}
{branch('False', 'tb', 'itb', 't', f'(- (- {CK} (xil ic)) 3)', 't')}))))""")
                    return F.text()
                E.add(None, f"""(case-on (iexp ce lc {IM} mlo slo) Option
  ((case None (chain {cap('hcv0', f'(= (iexp ce lc {IM} mlo slo) None)')}
{m_trap_text(Arm(S_(E.sl, 'hcv0'), I), 'ipstmt', [RV('hcv0')], 'himp')}))
   (case Some (cv) (chain {cap('hcv', f'(= (iexp ce lc {IM} mlo slo) (Some cv))')}
{cv_some()}))))""")
                return E.text()
            C.add(None, case_opt("(ixf_stmts nl own fail_ix eb)", "ieb", m_absurd_emit(C, None, "ixf_stmt", [RV('hic'), RV('hitb'), RV('hieb')]), eb_some))
            return C.text()
        B.add(None, case_opt("(ixf_stmts nl own fail_ix tb)", "itb", m_absurd_emit(B, None, "ixf_stmt", [RV('hic'), RV('hitb')]), tb_some))
        return B.text()
    A.add(None, case_opt("(ixf_exp nl 0 ce)", "ic", m_absurd_emit(A, None, "ixf_stmt", [RV('hic')]), ic_some))
    return A.text()

def m_trap_text(A, engine, rewrites, himp):
    """the imp run is a trap (via the engine fact himp): the conclusion is the run itself"""
    rw = "".join(f" {r} (reduce rhs)" for r in rewrites)
    A.have('hout', "(= (Some IpTrap) (Some out))", f"(steps ((rewrite (premise {himp}) rl rhs true ()) (unfold {engine} rhs) (reduce rhs){rw}) refl)")
    A.add('ho', "(inject (premise hout) (ho))")
    A.add(None, "(steps ((rewrite (premise ho) rl rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)")
    return A.text()

def m_arm_while(sl):
    I = "    "
    S = "(IpWhile ce b)"
    A = Arm(sl, I)
    A.have('hem', f"(= (ixf_stmt nl own fail_ix {S}) (Some is))", f"(steps ((rewrite (premise 0) rl rhs true ()){SUBS(SS)} (unfold ixt_emit rhs) (reduce rhs)) refl)")
    def ic_some():
        B = Arm(S_(A.sl, 'hic'), I)
        def ib_some():
            C = Arm(S_(B.sl, 'hib'), I)
            C.have('himp', f"(= {ST('(S f2)', S, 'lc', IM)} (Some out))", f"(steps ((rewrite (premise 1) rl rhs true ()){SUBS(SS)} (unfold ipt_run rhs) (reduce rhs)) refl)")
            C.have('hw', f"(= {WH('f2', 'ce', 'b', 'lc', IM)} (Some out))", "(steps ((rewrite (premise himp) rl rhs true ()) (unfold ipstmt rhs) (reduce rhs)) refl)")
            m_fuel(C)
            C.have('hel', "(= (le 0 (ixf_elen ce)) True)", "(rewrite-with (lemma ixf_elen_nonneg) lr lhs () () refl)")
            C.have('hec', "(= (ixf_ecost ce) (+ (ixf_elen ce) 5))", "(steps ((unfold ixf_ecost lhs)) refl)")
            cc = f"(- {CK} 4)"
            C.arith('hwc', f"(= (le (+ (ixf_ecost ce) 1) {cc}) True)", {'hcs': 1, 'p6': 1})
            C.arith('hw0', f"(= (le 0 {cc}) True)", {'hcs': 1, 'hel': 1, 'hec': 1, 'p6': 1})
            C.arith('hc4', f"(= (le 4 {CK}) True)", {'hcs': 1, 'hel': 1, 'hec': 1, 'p6': 1})
            ds = [f"(steps ((unfold ixt_emit lhs) (reduce lhs) (rewrite (premise hic) lr lhs true ()) (reduce lhs) (rewrite (premise hib) lr lhs true ()) (reduce lhs)) refl)",
                  "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw) lr lhs true ())) refl)", D(2), body_d('hscb'), body_d('hsd'), body_d('hkok'), D(6),
                  "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hwc) lr lhs true ())) refl)", D('hw0'), body_d('ha4'), D(10), D(11)]
            m_ih(C, 'hih', "(IqWhile ce b)", cc, LOOPL, RS, 'psx', 'lc', 'out', ds)
            C.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
            pins = "(inst ce ce) (inst b b) (inst ic ic) (inst ib ib) (inst out out) (inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst f f2) (inst fs fs) (inst dmax dmax)"
            C.add(None, f"(rewrite-with (lemma fs_step_while) lr lhs ({pins}) ({D('hem')} {D('hic')} {D('hib')} {D('himp')} {D('hc4')} {D('hih2')}))")
            C.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs)) refl)")
            return C.text()
        B.add(None, case_opt("(ixf_stmts nl own fail_ix b)", "ib", m_absurd_emit(B, None, "ixf_stmt", [RV('hic'), RV('hib')]), ib_some))
        return B.text()
    A.add(None, case_opt("(ixf_exp nl 0 ce)", "ic", m_absurd_emit(A, None, "ixf_stmt", [RV('hic')]), ic_some))
    return A.text()

def m_arm_fail(sl):
    I = "    "
    A = Arm(sl, I)
    A.have('hem', "(= (ixf_stmt nl own fail_ix (IpFail fam)) (Some is))", f"(steps ((rewrite (premise 0) rl rhs true ()){SUBS(SS)} (unfold ixt_emit rhs) (reduce rhs)) refl)")
    A.have('hout', "(= (Some (IpFailed fam)) (Some out))", f"(steps ((rewrite (premise 1) rl rhs true ()){SUBS(SS)} (unfold ipt_run rhs) (reduce rhs) (unfold ipstmt rhs) (reduce rhs)) refl)")
    A.add('ho', "(inject (premise hout) (ho))")
    A.add(None, "(steps ((rewrite (premise ho) rl rhs true ()) (unfold ixt_expect rhs) (reduce rhs) (unfold ixt_run lhs) (reduce lhs)))")
    A.add(None, f"(rewrite-with (lemma fs_step_fail) lr lhs ((inst fam fam) (inst nl nl) (inst own own) (inst fail_ix fail_ix)) ({D('hem')} {D(10)} {D('hcs')}))")
    A.add(None, "refl")
    return A.text()

def m_arm_unreach(sl):
    I = "    "
    A = Arm(sl, I)
    A.have('hout', "(= (Some IpTrap) (Some out))", f"(steps ((rewrite (premise 1) rl rhs true ()){SUBS(SS)} (unfold ipt_run rhs) (reduce rhs) (unfold ipstmt rhs) (reduce rhs)) refl)")
    A.add('ho', "(inject (premise hout) (ho))")
    A.add(None, "(steps ((rewrite (premise ho) rl rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)")
    return A.text()

def m_arm_nil(sl):
    I = "    "
    A = Arm(sl, I)
    A.have('hem', "(= (ixf_stmts nl own fail_ix Nil) (Some is))", f"(steps ((rewrite (premise 0) rl rhs true ()){SUBS(SL)} (unfold ixt_emit rhs) (reduce rhs)) refl)")
    A.have('himp', f"(= {STS('(S f2)', 'Nil', 'lc', IM)} (Some out))", f"(steps ((rewrite (premise 1) rl rhs true ()){SUBS(SL)} (unfold ipt_run rhs) (reduce rhs)) refl)")
    m_fuel(A)
    A.arith('hc1', f"(= (le 1 {CK}) True)", {'p8': 1, 'p6': 1})
    A.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
    pins = "(inst out out) (inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst f f2) (inst fs fs) (inst dmax dmax)"
    A.add(None, f"(rewrite-with (lemma fs_step_nil) lr lhs ({pins}) ({D('hem')} {D('himp')} {D('hc1')}))")
    A.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs)) refl)")
    return A.text()

def m_arm_cons(sl):
    I = "    "
    A = Arm(sl, I)
    A.have('hem', "(= (ixf_stmts nl own fail_ix (Cons s t)) (Some is))", f"(steps ((rewrite (premise 0) rl rhs true ()){SUBS(SL)} (unfold ixt_emit rhs) (reduce rhs)) refl)")
    def it_some():
        B = Arm(S_(A.sl, 'hit'), I)
        def iss_some():
            C = Arm(S_(B.sl, 'hiss'), I)
            C.have('himp', f"(= {STS('(S f2)', '(Cons s t)', 'lc', IM)} (Some out))", f"(steps ((rewrite (premise 1) rl rhs true ()){SUBS(SL)} (unfold ipt_run rhs) (reduce rhs)) refl)")
            cf, C.sl = ctx_facts(C.sl, 'p2', I); C.lines.append(cf)
            m_fuel(C)
            for nm, lem, fact, pin, src in [('hcbh', 'scb_head', '(ixf_scb (Cons s Nil))', '(inst t t)', 'hscb'), ('hcbt', 'scb_tail', '(ixf_scb t)', '(inst s s)', 'hscb'),
                                            ('hfa', 'a4_head', '(ixf_a4 (Cons s Nil))', '(inst t t)', 'ha4'), ('hfat', 'a4_tail', '(ixf_a4 t)', '(inst s s)', 'ha4'),
                                            ('hkh', 'skok_head', '(ixf_skok K (Cons s Nil))', '(inst t t)', 'hkok'), ('hkt', 'skok_tail', '(ixf_skok K t)', '(inst s s)', 'hkok'),
                                            ('hks', 'skok_cost', '(le (ixf_scost s) K)', '(inst t t)', 'hkok')]:
                C.have(nm, f"(= {fact} True)", f"(rewrite-with (lemma {lem}) lr lhs ({pin}) ({D(src)}) refl)")
            C.have('hsdh0', "(= (le (ixf_sdep (Cons s Nil)) (ixf_sdep (Cons s t))) True)", "(rewrite-with (lemma sdep_head) lr lhs () () refl)")
            C.have('hsdt0', "(= (le (ixf_sdep t) (ixf_sdep (Cons s t))) True)", "(rewrite-with (lemma sdep_tail) lr lhs ((inst s s)) () refl)")
            C.arith('hsdh', "(= (le (+ fp (* 8 (+ nl (ixf_sdep (Cons s Nil))))) (xmemhi_of m)) True)", {'hsd': 1, 'hsdh0': 8})
            C.arith('hsdt', "(= (le (+ fp (* 8 (+ nl (ixf_sdep t)))) (xmemhi_of m)) True)", {'hsd': 1, 'hsdt0': 8})
            C.add(None, f"""(have ha4s (= (ixf_a4s s) True)
{I}  (case-on (ixf_a4s s) Bool
{I}    ((case True (steps ((rewrite (hyp True) lr lhs true ())) refl))
{I}     (case False (chain (have hn (= (ixf_a4 (Cons s Nil)) False) (steps ((unfold ixf_a4 lhs) (reduce lhs) (rewrite (hyp False) lr lhs true ()) (reduce lhs)) refl)) (have hn2 (= False True) (steps ((rewrite (premise hn) rl lhs true ()) (rewrite (premise hfa) lr lhs true ())) refl)) (absurd (premise hn2)))))))""")
            C.sl.add('ha4s')
            C.have('hsl', "(= (le (xil iss) (ixf_scost s)) True)", f"(rewrite-with (lemma slen_cost) lr lhs ((inst nl nl) (inst own own) (inst fail_ix fail_ix)) ({D('hiss')} {D('ha4s')}) refl)")
            C.arith('hle', f"(= (le (xil iss) {CK}) True)", {'hsl': 1, 'hks': 1, 'p8': 1})
            C.arith('hc0', f"(= (le 0 {CK}) True)", {'p8': 1, 'p6': 1})
            C.arith('hcsK', f"(= (le (ixf_scost s) {CK}) True)", {'hks': 1, 'p8': 1})
            cc2 = f"(- {CK} (xil iss))"
            C.arith('hc0t', f"(= (le 0 {cc2}) True)", {'hsl': 1, 'hks': 1, 'p8': 1})
            RUNs = f"(xeval_seq (xt {CK} {KF2}) m iss {RS} {XM})"
            rsp = RSP(RUNs)
            PSXs = f"(fp_app {TR('f2', '(IqStmt s)', 'lc', IM)} psx)"
            def head_ih(G, out):
                ds = [emit_d('hiss'), "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hs2) lr lhs true ())) refl)", D(2), body_d('hcbh'), body_d('hsdh'), body_d('hkh'), D(6),
                      "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hcsK) lr lhs true ())) refl)", D('hc0'), body_d('hfa'), D(10), D(11)]
                m_ih(G, 'hihs', '(IqStmt s)', CK, 'iss', RS, 'psx', 'lc', out, ds)
            def leaf_norm(sl2):
                G = Arm(sl2, I + "      ")
                G.have('ht', f"(= {STS('f2', 't', 'lcs', 'mems')} (Some out))", "(steps ((rewrite (premise himp) rl rhs true ()) (unfold ipstmts rhs) (reduce rhs) (rewrite (premise hs2) lr rhs true ()) (reduce rhs)) refl)")
                head_ih(G, '(IpNorm lcs mems)')
                d0 = "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hs2) lr lhs true ())) refl)"
                G.have('hmems', T1C('f2', '(IqStmt s)', 'lc', IM, 'psx', 'mems'), f"(rewrite-with (lemma ipt_mem) lr lhs ((inst lc2 lcs) (inst mem2 mems)) ({d0} {D('hslo')} {D('hnl0')} {body_d('hfa')}) refl)")
                G.have('hctxs', T2C('f2', '(IqStmt s)', 'lc', IM, 'psx', 'lcs'), f"(rewrite-with (lemma ipt_ctx) lr lhs ((inst mem2 mems)) ({d0} {D(2)} {body_d('hcbh')} {body_d('hfa')}) refl)")
                ds = [emit_d('hit'), "(steps ((rewrite (premise hmems) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise ht) lr lhs true ())) refl)", D('hctxs'), body_d('hcbt'), body_d('hsdt'), body_d('hkt'), D(6),
                      "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hc0t) lr lhs true ())) refl)", D('hc0t'), body_d('hfat'), D(10), D(11)]
                m_ih(G, 'hiht', '(IqStmts t)', cc2, 'it', rsp, PSXs, 'lcs', 'out', ds)
                G.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
                pins = "(inst s s) (inst t t) (inst iss iss) (inst it it) (inst out out) (inst lcs lcs) (inst mems mems) (inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst f f2) (inst fs fs) (inst dmax dmax)"
                G.add(None, f"(rewrite-with (lemma fs_step_cons) lr lhs ({pins}) ({D('hem')} {D('hiss')} {D('hit')} {D('himp')} {D('hs2')} {D('hle')} {D('hmems')} {D('hihs2')} {D('hiht2')}))")
                G.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs)) refl)")
                return G.text()
            def leaf_fail(sl2):
                G = Arm(sl2, I + "      ")
                head_ih(G, '(IpFailed fam)')
                G.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
                pins = "(inst s s) (inst t t) (inst iss iss) (inst it it) (inst out out) (inst fam fam) (inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst f f2) (inst fs fs) (inst dmax dmax)"
                G.add(None, f"(rewrite-with (lemma fs_step_cons_fail) lr lhs ({pins}) ({D('hem')} {D('hiss')} {D('hit')} {D('himp')} {D('hs2')} {D('hle')} {D('hihs2')}))")
                G.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs)) refl)")
                return G.text()
            Sx = ST('f2', 's', 'lc', IM)
            C.add(None, f"""(case-on {Sx} Option
  ((case None (chain {cap('hs', f'(= {Sx} None)')} (have hn (= None (Some out)) (steps ((rewrite (premise himp) rl rhs true ()) (unfold ipstmts rhs) (reduce rhs) (rewrite (premise hs) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
   (case Some (o)
     (chain
       {cap('hs', f'(= {Sx} (Some o))')}
       (case-on o IpOut
         ((case IpNorm (lcs mems)
            (chain
              (have hs2 (= {Sx} (Some (IpNorm lcs mems))) (steps ((rewrite (premise hs) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
{leaf_norm(S_(C.sl, 'hs', 'hs2'))}))
          (case IpTrap
            (chain
              (have hs2 (= {Sx} (Some IpTrap)) (steps ((rewrite (premise hs) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
{m_trap_text(Arm(S_(C.sl, 'hs', 'hs2'), I + "      "), 'ipstmts', [RV('hs2')], 'himp')}))
          (case IpFailed (fam)
            (chain
              (have hs2 (= {Sx} (Some (IpFailed fam))) (steps ((rewrite (premise hs) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
{leaf_fail(S_(C.sl, 'hs', 'hs2'))}))))))))""")
            return C.text()
        B.add(None, case_opt("(ixf_stmt nl own fail_ix s)", "iss", m_absurd_emit(B, None, "ixf_stmts", [RV('hit'), RV('hiss')]), iss_some))
        return B.text()
    A.add(None, case_opt("(ixf_stmts nl own fail_ix t)", "it", m_absurd_emit(A, None, "ixf_stmts", [RV('hit')]), it_some))
    return A.text()

def m_arm_qwhile(sl):
    I = "    "
    A = Arm(sl, I)
    A.have('hem', "(= (ixt_emit nl own fail_ix (IqWhile ce b)) (Some is))", f"(steps ((rewrite (premise 0) rl rhs true ()){SUBS(SW)}) refl)")
    def ic_some():
        B = Arm(S_(A.sl, 'hic'), I)
        def ib_some():
            C = Arm(S_(B.sl, 'hib'), I)
            C.have('himp', f"(= {WH('(S f2)', 'ce', 'b', 'lc', IM)} (Some out))", f"(steps ((rewrite (premise 1) rl rhs true ()){SUBS(SW)} (unfold ipt_run rhs) (reduce rhs)) refl)")
            cf, C.sl = ctx_facts(C.sl, 'p2', I); C.lines.append(cf)
            m_fuel(C)
            C.have('hlen', "(= (xil ic) (ixf_elen ce))", f"(rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d 0) (inst e ce)) ({D('hic')}) refl)")
            C.have('hec', "(= (ixf_ecost ce) (+ (ixf_elen ce) 5))", "(steps ((unfold ixf_ecost lhs)) refl)")
            C.have('hel', "(= (le 0 (ixf_elen ce)) True)", "(rewrite-with (lemma ixf_elen_nonneg) lr lhs () () refl)")
            C.have('hcbc', "(= (ixf_cb ce) True)", f"(rewrite-with (lemma scb_while_c) lr lhs ((inst b b) (inst t Nil)) ({D('hscb')}) refl)")
            C.have('hsdc', "(= (le (ixf_dep ce) (ixf_sdep (Cons (IpWhile ce b) Nil))) True)", "(rewrite-with (lemma sdep_while_c) lr lhs ((inst t Nil)) () refl)")
            C.arith('hdepc', "(= (le (+ fp (* 8 (+ nl (+ 0 (ixf_dep ce))))) (xmemhi_of m)) True)", {'hsd': 1, 'hsdc': 8})
            C.arith('hecost', f"(= (le (ixf_ecost ce) (- {CK} 1)) True)", {'hcs': 1, 'p6': 1})
            def cv_some():
                F = Arm(S_(C.sl, 'hcv'), I)
                F.lines.append(ctx_after_cond('ce', I))
                for n in ['hdisca', 'hlocsa', 'hfb', 'hctx1']: F.sl.add(n)
                RUNc = f"(xeval_seq (xt (- {CK} 1) {KF2}) m ic {RS} {XM})"
                F.have('hsim', f"(= {RUNc} (fe_out cv {RS} {RUNc} {Mc}))", f"(rewrite-with (lemma fe_sound) lr lhs ((inst e ce) (inst nl nl) (inst d 0) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst v cv)) ({D('hic')} {D('hcv')} {D(2)} {D('hcbc')} {D('hdepc')} {D('hecost')}) refl)")
                F.arith('hc3', f"(= (le (+ (xil ic) 3) {CK}) True)", {'hcs': 1, 'hlen': 1, 'hec': 1, 'p6': 1})
                F.arith('hc5', f"(= (le (+ (xil ic) 5) {CK}) True)", {'hcs': 1, 'hlen': 1, 'hec': 1, 'p6': 1})
                rsc = RSC(RUNc)
                basepins = "(inst ce ce) (inst b b) (inst ic ic) (inst ib ib) (inst cv cv) (inst out out) (inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst f f2) (inst fs fs) (inst dmax dmax)"
                def exit_():
                    G = Arm(S_(F.sl, 'hcvf'), I)
                    G.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
                    G.add(None, f"(rewrite-with (lemma fw_step_exit) lr lhs ({basepins}) ({D('hem')} {D('hic')} {D('hib')} {D('himp')} {D('hcv')} {D('hcvf')} {D('hc3')} {D('hsim')}))")
                    G.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs)) refl)")
                    return G.text()
                def iter_():
                    G = Arm(S_(F.sl, 'hcvf'), I)
                    Bx = STS('f2', 'b', 'lc', IM)
                    cc = f"(- (- (- {CK} 1) (xil ic)) 3)"
                    G.have('hcbb', "(= (ixf_scb b) True)", f"(rewrite-with (lemma scb_while_b) lr lhs ((inst ce ce) (inst b b) (inst t Nil)) ({D('hscb')}) refl)")
                    G.have('hsdb', "(= (le (ixf_sdep b) (ixf_sdep (Cons (IpWhile ce b) Nil))) True)", "(rewrite-with (lemma sdep_while_b) lr lhs ((inst t Nil)) () refl)")
                    G.arith('hsdt', "(= (le (+ fp (* 8 (+ nl (ixf_sdep b)))) (xmemhi_of m)) True)", {'hsd': 1, 'hsdb': 8})
                    G.have('hkb', "(= (ixf_skok K b) True)", f"(rewrite-with (lemma skok_while_b) lr lhs ((inst ce ce) (inst b b) (inst t Nil)) ({D('hkok')}) refl)")
                    G.have('hab', "(= (ixf_a4 b) True)", f"(rewrite-with (lemma a4_while_b) lr lhs ((inst ce ce) (inst b b) (inst t Nil)) ({D('ha4')}) refl)")
                    G.arith('hcb0', f"(= (le 0 {cc}) True)", {'hcs': 1, 'hlen': 1, 'hec': 1, 'p6': 1})
                    G.arith('hcw0', f"(= (le 0 (- {CK} 1)) True)", {'hcs': 1, 'hel': 1, 'hec': 1, 'p6': 1})
                    G.arith('hwc', f"(= (le (+ (ixf_ecost ce) 1) (- {CK} 1)) True)", {'hcs': 1, 'p6': 1})
                    def body_ih(H, out, hb):
                        ds = [emit_d('hib'), f"(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise {hb}) lr lhs true ())) refl)", D('hctx1'), body_d('hcbb'), body_d('hsdt'), body_d('hkb'), D(6),
                              "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hcb0) lr lhs true ())) refl)", D('hcb0'), body_d('hab'), D(10), D(11)]
                        return m_ih(H, 'hihb', '(IqStmts b)', cc, 'ib', rsc, PSX1c, 'lc', out, ds)
                    def leaf_norm(sl2):
                        H = Arm(sl2, I + "          ")
                        H.have('hw2', f"(= {WH('f2', 'ce', 'b', 'lcb', 'memb')} (Some out))", "(steps ((rewrite (premise himp) rl rhs true ()) (unfold ipwhile rhs) (reduce rhs) (rewrite (premise hcv) lr rhs true ()) (reduce rhs) (rewrite (premise hcvf) lr rhs true ()) (reduce rhs) (rewrite (premise hb2) lr rhs true ()) (reduce rhs)) refl)")
                        runb, memb = body_ih(H, '(IpNorm lcb memb)', 'hb2')
                        d0b = "(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hb2) lr lhs true ())) refl)"
                        PSXb = f"(fp_app {TR('f2', '(IqStmts b)', 'lc', IMx(PSX1c))} {PSX1c})"
                        H.have('hmemb', T1C('f2', '(IqStmts b)', 'lc', IMx(PSX1c), PSX1c, 'memb'), f"(rewrite-with (lemma ipt_mem) lr lhs ((inst lc2 lcb) (inst mem2 memb)) ({d0b} {D('hslo')} {D('hnl0')} {body_d('hab')}) refl)")
                        H.have('hctxb', T2C('f2', '(IqStmts b)', 'lc', IMx(PSX1c), PSX1c, 'lcb'), f"(rewrite-with (lemma ipt_ctx) lr lhs ((inst mem2 memb)) ({d0b} {D('hctx1')} {body_d('hcbb')} {body_d('hab')}) refl)")
                        rsb = RSP(runb)
                        ds = [D('hem'), "(steps ((rewrite (premise hmemb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw2) lr lhs true ())) refl)", D('hctxb'), body_d('hscb'), body_d('hsd'), body_d('hkok'), D(6),
                              "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hwc) lr lhs true ())) refl)", D('hcw0'), body_d('ha4'), D(10), D(11)]
                        m_ih(H, 'hihw', '(IqWhile ce b)', f"(- {CK} 1)", 'is', rsb, PSXb, 'lcb', 'out', ds)
                        H.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
                        pins = basepins + " (inst lcb lcb) (inst memb memb)"
                        H.add(None, f"(rewrite-with (lemma fw_step_iter) lr lhs ({pins}) ({D('hem')} {D('hic')} {D('hib')} {D('himp')} {D('hcv')} {D('hcvf')} {D('hc5')} {D('hsim')} {D('hb2')} {D('hmemb')} {D('hihb2')} {D('hihw2')} {D('hfb')}))")
                        H.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs)) refl)")
                        return H.text()
                    def leaf_fail(sl2):
                        H = Arm(sl2, I + "          ")
                        body_ih(H, '(IpFailed fam)', 'hb2')
                        H.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
                        pins = basepins + " (inst fam fam)"
                        H.add(None, f"(rewrite-with (lemma fw_step_iter_fail) lr lhs ({pins}) ({D('hem')} {D('hic')} {D('hib')} {D('himp')} {D('hcv')} {D('hcvf')} {D('hc5')} {D('hsim')} {D('hb2')} {D('hihb2')}))")
                        H.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs)) refl)")
                        return H.text()
                    G.add(None, f"""(case-on {Bx} Option
  ((case None (chain {cap('hb', f'(= {Bx} None)')} (have hn (= None (Some out)) (steps ((rewrite (premise himp) rl rhs true ()) (unfold ipwhile rhs) (reduce rhs) (rewrite (premise hcv) lr rhs true ()) (reduce rhs) (rewrite (premise hcvf) lr rhs true ()) (reduce rhs) (rewrite (premise hb) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
   (case Some (ob)
     (chain
       {cap('hb', f'(= {Bx} (Some ob))')}
       (case-on ob IpOut
         ((case IpNorm (lcb memb)
            (chain
              (have hb2 (= {Bx} (Some (IpNorm lcb memb))) (steps ((rewrite (premise hb) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
{leaf_norm(S_(G.sl, 'hb', 'hb2'))}))
          (case IpTrap
            (chain
              (have hb2 (= {Bx} (Some IpTrap)) (steps ((rewrite (premise hb) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
{m_trap_text(Arm(S_(G.sl, 'hb', 'hb2'), I + "          "), 'ipwhile', [RV('hcv'), RV('hcvf'), RV('hb2')], 'himp')}))
          (case IpFailed (fam)
            (chain
              (have hb2 (= {Bx} (Some (IpFailed fam))) (steps ((rewrite (premise hb) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
{leaf_fail(S_(G.sl, 'hb', 'hb2'))}))))))))""")
                    return G.text()
                F.add(None, f"""(case-on (int_eq cv 0) Bool
  ((case True (chain {cap('hcvf', '(= (int_eq cv 0) True)')}
{exit_()}))
   (case False (chain {cap('hcvf', '(= (int_eq cv 0) False)')}
{iter_()}))))""")
                return F.text()
            C.add(None, f"""(case-on (iexp ce lc {IM} mlo slo) Option
  ((case None (chain {cap('hcv0', f'(= (iexp ce lc {IM} mlo slo) None)')}
{m_trap_text(Arm(S_(C.sl, 'hcv0'), I), 'ipwhile', [RV('hcv0')], 'himp')}))
   (case Some (cv) (chain {cap('hcv', f'(= (iexp ce lc {IM} mlo slo) (Some cv))')}
{cv_some()}))))""")
            return C.text()
        B.add(None, case_opt("(ixf_stmts nl own fail_ix b)", "ib", m_absurd_emit(B, None, "ixt_emit", [RV('hic'), RV('hib')]), ib_some))
        return B.text()
    A.add(None, case_opt("(ixf_exp nl 0 ce)", "ic", m_absurd_emit(A, None, "ixt_emit", [RV('hic')]), ic_some))
    return A.text()

def t3():
    prem = [f"(= (ixt_emit nl own fail_ix r) (Some is))",
            f"(= {RUN('f', 'r', 'lc', IM)} (Some out))",
            "(= (fe_ctx m mlo slo fp nl 0 lc psx) True)",
            "(= (ixf_scb (ixt_body r)) True)",
            "(= (le (+ fp (* 8 (+ nl (ixf_sdep (ixt_body r))))) (xmemhi_of m)) True)",
            "(= (ixf_skok K (ixt_body r)) True)",
            "(= (le 1 K) True)",
            "(= (le (ixt_cost r) c) True)",
            "(= (le 0 c) True)",
            "(= (ixf_a4 (ixt_body r)) True)",
            "(= (xfunc_at (xfuncs_of m) fail_ix) (Some (MkXFunc 0 (list (XMovRI RAX 60) XSyscall))))",
            "(= (le 0 mlo) True)"]
    sl0 = Slots([f"p{i}" for i in range(NPREM)])
    def z_arm(ctor, binders, engine):
        b = f" ({binders})" if binders else ""
        return f"(case {ctor}{b} (chain {cap('hr', f'(= r ({ctor} {binders}))')} (have hn (= None (Some out)) (steps ((rewrite (premise 1) rl rhs true ()) (rewrite (premise hr) lr rhs true ()) (unfold ipt_run rhs) (reduce rhs) (unfold {engine} rhs) (reduce rhs)) refl)) (absurd (premise hn))))"
    def gsub(hs): return "(steps (" + " ".join(f"(rewrite (premise {h}) lr both true ())" for h in hs) + "))"
    def stmt_arm(ctor, binders, S, body_fn, sl_base, costunf):
        b = f" ({binders})" if binders else ""
        A = Arm(S_(sl_base, 'hst'), "  ")
        m_sides(A, f"(Cons {S} Nil)", SS)
        m_cost(A, f"(IqStmt {S})", SS, ['ixt_cost'] + costunf)
        A.lines[-1] = A.lines[-1].replace("{COST}", COSTS[ctor])
        return f"""(case {ctor}{b} (chain
  {cap('hst', f'(= s {S})')}
  {gsub(['hst'])}
{A.text()}
{body_fn(A.sl)}))"""
    fenced = lambda sl: f"(chain (have hn (= False True) (steps ((rewrite (premise ha4) rl rhs true ()) (unfold ixf_a4 rhs) (reduce rhs) (unfold ixf_a4s rhs) (reduce rhs)) refl)) (absurd (premise hn)))"
    slS = S_(sl0, 'hr')
    # the list arm
    AL = Arm(S_(sl0, 'hr', 'hss'), "  "); m_sides(AL, "(Cons s t)", SL)
    m_cost(AL, "(IqStmts (Cons s t))", SL, ['ixt_cost']); AL.lines[-1] = AL.lines[-1].replace("{COST}", "0")
    AN = Arm(S_(sl0, 'hr', 'hss'), "  ")
    AW = Arm(S_(sl0, 'hr'), "  "); m_sides(AW, "(Cons (IpWhile ce b) Nil)", SW)
    m_cost(AW, "(IqWhile ce b)", SW, ['ixt_cost']); AW.lines[-1] = AW.lines[-1].replace("{COST}", "(+ (ixf_ecost ce) 1)")
    return f"""(claim ipt_sound
  (goal ({SV})
    ({chr(10).join('     ' + p for p in prem).strip()})
    {CONCL('f', 'r', 'lc', IM, 'psx', 'out', RS, XR('c', 'f', 'r', 'is', RS, XM))})
  (induct f
    ((case Z
       (case-on r IptReq
         ({z_arm('IqStmt', 's', 'ipstmt')}
          {z_arm('IqStmts', 'ss', 'ipstmts')}
          {z_arm('IqWhile', 'ce b', 'ipwhile')})))
     (case S (f2)
       (case-on r IptReq
         ((case IqStmt (s)
            (chain
              {cap('hr', '(= r (IqStmt s))')}
              {gsub(['hr'])}
              (case-on s IpStmt
                ({stmt_arm('IpSet', 'i e', '(IpSet i e)', m_arm_set, slS, ['ixf_scost'])}
                 {stmt_arm('IpStore', 'ae ve', '(IpStore ae ve)', fenced, slS, [])}
                 {stmt_arm('IpIf', 'ce tb eb', '(IpIf ce tb eb)', m_arm_if, slS, ['ixf_scost'])}
                 {stmt_arm('IpWhile', 'ce b', '(IpWhile ce b)', m_arm_while, slS, ['ixf_scost'])}
                 {stmt_arm('IpCall', 'i k args', '(IpCall i k args)', fenced, slS, [])}
                 {stmt_arm('IpLoadW', 'i ae', '(IpLoadW i ae)', fenced, slS, [])}
                 {stmt_arm('IpStoreW', 'ae ve', '(IpStoreW ae ve)', fenced, slS, [])}
                 {stmt_arm('IpFail', 'fam', '(IpFail fam)', m_arm_fail, slS, ['ixf_scost'])}
                 {stmt_arm('IpUnreach', '', 'IpUnreach', m_arm_unreach, slS, [])}))))
          (case IqStmts (ss)
            (chain
              {cap('hr', '(= r (IqStmts ss))')}
              {gsub(['hr'])}
              (case-on ss List
                ((case Nil (chain {cap('hss', '(= ss Nil)')} {gsub(['hss'])}
{m_arm_nil(AN.sl)}))
                 (case Cons (s t) (chain
                   {cap('hss', '(= ss (Cons s t))')}
                   {gsub(['hss'])}
{AL.text()}
{m_arm_cons(AL.sl)}))))))
          (case IqWhile (ce b)
            (chain
              {cap('hr', '(= r (IqWhile ce b))')}
              {gsub(['hr'])}
{AW.text()}
{m_arm_qwhile(AW.sl)}))))))))
"""

COSTS = {'IpSet': '(+ (ixf_ecost e) 4)', 'IpStore': '(+ (ixf_ecost ae) (+ (ixf_ecost ve) 8))', 'IpIf': '(+ (ixf_ecost ce) 3)', 'IpWhile': '(+ (ixf_ecost ce) 4)',
         'IpCall': '(+ (ixf_argcost args) 12)', 'IpLoadW': '(+ (ixf_ecost ae) 4)', 'IpStoreW': '(+ (ixf_ecost ae) (+ (ixf_ecost ve) 8))', 'IpFail': '8', 'IpUnreach': '4'}

BANNER_T3 = ";; --- THEOREM A's STATEMENT TIER: ipt_sound, the dispatcher induction (generated by gen_fra4.py t3 — REGENERATE, never hand-patch) ---"
BLOCKS.append(("t3", BANNER_T3, t3))

# ---------------- entry point (keep last: the blocks register above) ----------------
if __name__ == "__main__":
    arg = sys.argv[1] if len(sys.argv) > 1 else "extract"
    if arg == "splice": splice()
    else: sys.stdout.write({k: fn for k, _, fn in BLOCKS}[arg]())
