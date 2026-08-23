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
    "scb":  ("ixf_scb",  "ixf_scbs",  "",     ""),
    "skok": ("ixf_skok", "ixf_skoks", "(k Int) ", "k "),
}
IF_S  = "(IpIf ce tb eb)"
WH_S  = "(IpWhile ce b)"
SET_S = "(IpSet i e)"
VARS_IF = "(ce IExp) (tb (List IpStmt)) (eb (List IpStmt)) "
VARS_WH = "(ce IExp) (b (List IpStmt)) "
VARS_SET = "(i Int) (e IExp) "
VARS_ST = "(ae IExp) (ve IExp) "
VARS_LW = "(i Int) (ae IExp) "
ST_S = "(IpStore ae ve)"
LW_S = "(IpLoadW i ae)"
SW_S = "(IpStoreW ae ve)"
M_ST = "(+ 1 (imax2 (ixf_dep ae) (ixf_dep ve)))"
# the call (A-5)
VARS_CALL = "(i Int) (k Int) (args (List IExp)) "
CALL_S = "(IpCall i k args)"

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
        if key == "scb":
            out.append(extraction(P,Ps,kv,ka,f"{key}_set_e",VARS_SET,SET_S,"(ixf_cb e)",[]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_if_c",VARS_IF,IF_S,"(ixf_cb ce)",[]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_if_t",VARS_IF,IF_S,f"({Pk}tb)",["if_f","if_ff"]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_if_e",VARS_IF,IF_S,f"({Pk}eb)",["if_ff","if_ff"]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_while_c",VARS_WH,WH_S,"(ixf_cb ce)",[]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_while_b",VARS_WH,WH_S,f"({Pk}b)",["if_ff"]))
            # the memory statements (A-3 part 3): the address is the scrutinee, the value a branch
            out.append(extraction(P,Ps,kv,ka,f"{key}_store_a",VARS_ST,ST_S,"(ixf_cb ae)",[]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_store_v",VARS_ST,ST_S,"(ixf_cb ve)",["if_ff"]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_loadw_a",VARS_LW,LW_S,"(ixf_cb ae)",[]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_storew_a",VARS_ST,SW_S,"(ixf_cb ae)",[]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_storew_v",VARS_ST,SW_S,"(ixf_cb ve)",["if_ff"]))
            out.append(extraction(P,Ps,kv,ka,f"{key}_call_args",VARS_CALL,CALL_S,"(ixf_cbs args)",[]))
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
    # the memory statements' own depth (A-3 part 3)
    for nm, svars, S, M in [("sdep_store",VARS_ST,ST_S,M_ST),("sdep_storew",VARS_ST,SW_S,M_ST),("sdep_loadw",VARS_LW,LW_S,"(ixf_dep ae)"),("sdep_call",VARS_CALL,CALL_S,"(ixf_deps args)")]:
        out.append(sdep(nm,svars,S,M,
            unf(S,M) + "\n" + ge("h1",M,"(ixf_sdep t)","l") + "\n"
            + "    (steps ((rewrite (premise hu) lr lhs true ()) (rewrite (premise h1) lr lhs true ())) refl)"))
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
def TR(f, r, lc, im): return f"(ipt_tr {f} fs mlo slo dmax dep fp nl own {r} {lc} {im})"
def TRS(f, s, lc, im): return f"(ipt_stmt {f} fs mlo slo dmax dep fp nl own {s} {lc} {im})"
def TRL(f, ss, lc, im): return f"(ipt_stmts {f} fs mlo slo dmax dep fp nl own {ss} {lc} {im})"
def TRW(f, ce, b, lc, im): return f"(ipt_while {f} fs mlo slo dmax dep fp nl own {ce} {b} {lc} {im})"
def TRC(f, fpc, k, lc, im): return f"(ipt_call {f} fs mlo slo dmax dep {fpc} {k} {lc} {im})"
def CALL(f, k, vs, im): return f"(ipcall {f} fs mlo slo dmax dep {k} {vs} {im})"
# the callee (A-5): its locals count, frame size, entry locals, zero patches, body and result
NLg = "(+ (ikn (ipparams_of g)) (ikn (ipextra_of g)))"
OWNg = "(ixf_own g)"
LCg = "(iapp (iband_args (ipparams_of g) lc) (izeros (ikn (ipextra_of g))))"
FZg = f"(fz_tr fp (ikn (ipparams_of g)) {NLg})"
RESg = "(ipresult_of g)"
BODYg = "(ipbody_of g)"
def STSd(f, ss, lc, im): return f"(ipstmts {f} fs mlo slo dmax (+ dep 1) {ss} {lc} {im})"
def TRLd(f, ss, lc, im): return f"(ipt_stmts {f} fs mlo slo dmax (+ dep 1) fp {NLg} {OWNg} {ss} {lc} {im})"
TA0 = lambda im: f"(ipt_args fp nl own 0 args lc {im} mlo slo)"
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

POST_CALL = " (unfold ipout_of_ret rhs) (reduce rhs)"
def absurd_run(engine, rewrites, lhs, subst, runp=0, pre="", post="", outn=None):
    """the run premise (index runp; = OUTN) against what the engine actually returns"""
    outn = outn or OUTN
    rw = "".join(f" {r} (reduce rhs)" for r in rewrites)
    body = f"""(have hn (= {lhs} {outn})
  (steps ({pre}(rewrite (premise {runp}) rl rhs true ()){SUBS(subst)} (unfold ipt_run rhs) (reduce rhs) (unfold {engine} rhs) (reduce rhs){rw}{post}) refl))"""
    if lhs == "None":
        return f"(chain {body} (absurd (premise hn)))"
    return f"(chain {body} (inject (premise hn) (hx)) (absurd (premise hx)))"

def run_fact(engine, name, lhs, rewrites, subst, runp=0, pre="", post="", outn=None):
    """have NAME (= LHS OUTN) by unfolding the run premise through the engine and the given facts"""
    outn = outn or OUTN
    rw = "".join(f" {r} (reduce rhs)" for r in rewrites)
    return f"""(have {name} (= {lhs} {outn})
  (steps ({pre}(rewrite (premise {runp}) rl rhs true ()){SUBS(subst)} (unfold ipt_run rhs) (reduce rhs) (unfold {engine} rhs) (reduce rhs){rw}{post}) refl))"""

RV = lambda h: f"(rewrite (premise {h}) lr rhs true ())"
SS = ['hr', 'hst']   # the statement arms' substitution
SL = ['hr', 'hss']   # the list arms'
SW = ['hr']          # the while request's
SC = ['hr']          # the call request's

def S_(sl, *names):
    """a copy of the slot tracker extended by the haves/inject products named, in order"""
    s2 = Slots(sl.names)
    for n in names: s2.add(n)
    return s2

def arm_set(leaf, sl, subst=None, runp=0, pre=""):
    """IqStmt (IpSet i e): hv (iexp = Some v), hset (ilset = Some lcs), hlc (lcs = lc2), hmem (IM = mem2)"""
    subst = subst or SS
    return f"""(case-on (iexp e lc {IM} mlo slo) Option
  ((case None (chain {cap('hv', f'(= (iexp e lc {IM} mlo slo) None)')} {absurd_run('ipstmt', [RV('hv')], '(Some IpTrap)', subst, runp, pre)}))
   (case Some (v)
     (chain
       {cap('hv', f'(= (iexp e lc {IM} mlo slo) (Some v))')}
       (case-on (ilset lc i v) Option
         ((case None (chain {cap('hset', '(= (ilset lc i v) None)')} {absurd_run('ipstmt', [RV('hv'), RV('hset')], '(Some IpTrap)', subst, runp, pre)}))
          (case Some (lcs)
            (chain
              {cap('hset', '(= (ilset lc i v) (Some lcs))')}
              {run_fact('ipstmt', 'hn1', f'(Some (IpNorm lcs {IM}))', [RV('hv'), RV('hset')], subst, runp, pre)}
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

def VALW(): return f"(load_le (iw8) {IM} ad)"
def arm_store(leaf, sl, subst=None, runp=0, pre="", word=False):
    """IqStmt (IpStore ae ve) / (IpStoreW ae ve): hv (iexp ae = Some ad), hg0 (mlo <= ad), hg1 (the span
    below the cut), hvv (iexp ve = Some vv), hn1 (the normal outcome), hlc (lc = lc2), hmem (the store = mem2)"""
    subst = subst or SS
    G1 = "(le (+ ad 8) slo)" if word else "(lt ad slo)"
    MEM2 = f"(store_le (iw8) {IM} ad vv)" if word else f"(mem_set {IM} ad vv)"
    AE = f"(iexp ae lc {IM} mlo slo)"; VE = f"(iexp ve lc {IM} mlo slo)"
    AR = lambda rws, lhs: absurd_run('ipstmt', rws, lhs, subst, runp, pre)
    return f"""(case-on {AE} Option
  ((case None (chain {cap('hv', f'(= {AE} None)')} {AR([RV('hv')], '(Some IpTrap)')}))
   (case Some (ad)
     (chain
       {cap('hv', f'(= {AE} (Some ad))')}
       (case-on (le mlo ad) Bool
         ((case False (chain {cap('hg0', '(= (le mlo ad) False)')} {AR([RV('hv'), RV('hg0')], '(Some IpTrap)')}))
          (case True
            (chain
              {cap('hg0', '(= (le mlo ad) True)')}
              (case-on {G1} Bool
                ((case False (chain {cap('hg1', f'(= {G1} False)')} {AR([RV('hv'), RV('hg0'), RV('hg1')], '(Some IpTrap)')}))
                 (case True
                   (chain
                     {cap('hg1', f'(= {G1} True)')}
                     (case-on {VE} Option
                       ((case None (chain {cap('hvv', f'(= {VE} None)')} {AR([RV('hv'), RV('hg0'), RV('hg1'), RV('hvv')], '(Some IpTrap)')}))
                        (case Some (vv)
                          (chain
                            {cap('hvv', f'(= {VE} (Some vv))')}
                            {run_fact('ipstmt', 'hn1', f'(Some (IpNorm lc {MEM2}))', [RV('hv'), RV('hg0'), RV('hg1'), RV('hvv')], subst, runp, pre)}
                            (inject (premise hn1) (hn2))
                            (inject (premise hn2) (hlc hmem))
{leaf(S_(sl, 'hv', 'hg0', 'hg1', 'hvv', 'hn1', 'hn2', 'hlc', 'hmem'))}))))))))))))))))"""

def arm_loadw(leaf, sl, subst=None, runp=0, pre=""):
    """IqStmt (IpLoadW i ae): hv (iexp ae = Some ad), hg0, hg1 (the word below the cut), hset (ilset of the
    loaded word = Some lcs), hn1, hlc (lcs = lc2), hmem (IM = mem2)"""
    subst = subst or SS
    AE = f"(iexp ae lc {IM} mlo slo)"; V = VALW()
    AR = lambda rws, lhs: absurd_run('ipstmt', rws, lhs, subst, runp, pre)
    return f"""(case-on {AE} Option
  ((case None (chain {cap('hv', f'(= {AE} None)')} {AR([RV('hv')], '(Some IpTrap)')}))
   (case Some (ad)
     (chain
       {cap('hv', f'(= {AE} (Some ad))')}
       (case-on (le mlo ad) Bool
         ((case False (chain {cap('hg0', '(= (le mlo ad) False)')} {AR([RV('hv'), RV('hg0')], '(Some IpTrap)')}))
          (case True
            (chain
              {cap('hg0', '(= (le mlo ad) True)')}
              (case-on (le (+ ad 8) slo) Bool
                ((case False (chain {cap('hg1', '(= (le (+ ad 8) slo) False)')} {AR([RV('hv'), RV('hg0'), RV('hg1')], '(Some IpTrap)')}))
                 (case True
                   (chain
                     {cap('hg1', '(= (le (+ ad 8) slo) True)')}
                     (case-on (ilset lc i {V}) Option
                       ((case None (chain {cap('hset', f'(= (ilset lc i {V}) None)')} {AR([RV('hv'), RV('hg0'), RV('hg1'), RV('hset')], '(Some IpTrap)')}))
                        (case Some (lcs)
                          (chain
                            {cap('hset', f'(= (ilset lc i {V}) (Some lcs))')}
                            {run_fact('ipstmt', 'hn1', f'(Some (IpNorm lcs {IM}))', [RV('hv'), RV('hg0'), RV('hg1'), RV('hset')], subst, runp, pre)}
                            (inject (premise hn1) (hn2))
                            (inject (premise hn2) (hlc hmem))
{leaf(S_(sl, 'hv', 'hg0', 'hg1', 'hset', 'hn1', 'hn2', 'hlc', 'hmem'))}))))))))))))))))"""

def arm_call(leaf, sl, subst=None, runp=0, pre=""):
    """IqStmt (IpCall i k args): hvs (ipexps = Some vs), hcall (ipcall f2 = Some r), hcall2 (r = IpRv v memc),
    hset (ilset lc i v = Some lcs), hn1, hlc (lcs = lc2), hmem (memc = mem2)"""
    subst = subst or SS
    E = f"(ipexps args lc {IM} mlo slo)"; CL = CALL('f2', 'k', 'vs', IM)
    AR = lambda rws, lhs: absurd_run('ipstmt', rws, lhs, subst, runp, pre)
    def ret(ctor, binders, body):
        b = f" ({binders})" if binders else ""
        return f"""(case {ctor}{b}
  (chain
    (have hcall2 (= {CL} (Some ({ctor} {binders}))) (steps ((rewrite (premise hcall) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
{body}))"""
    return f"""(case-on {E} Option
  ((case None (chain {cap('hvs', f'(= {E} None)')} {AR([RV('hvs')], '(Some IpTrap)')}))
   (case Some (vs)
     (chain
       {cap('hvs', f'(= {E} (Some vs))')}
       (case-on {CL} Option
         ((case None (chain {cap('hcall', f'(= {CL} None)')} {AR([RV('hvs'), RV('hcall')], 'None')}))
          (case Some (rt)
            (chain
              {cap('hcall', f'(= {CL} (Some rt))')}
              (case-on rt IpRet
                ({ret('IpRv', 'v memc', f"""    (case-on (ilset lc i v) Option
      ((case None (chain {cap('hset', '(= (ilset lc i v) None)')} {AR([RV('hvs'), RV('hcall2'), RV('hset')], '(Some IpTrap)')}))
       (case Some (lcs)
         (chain
           {cap('hset', '(= (ilset lc i v) (Some lcs))')}
           {run_fact('ipstmt', 'hn1', '(Some (IpNorm lcs memc))', [RV('hvs'), RV('hcall2'), RV('hset')], subst, runp, pre)}
           (inject (premise hn1) (hn2))
           (inject (premise hn2) (hlc hmem))
{leaf(S_(sl, 'hvs', 'hcall', 'hcall2', 'hset', 'hn1', 'hn2', 'hlc', 'hmem'))}))))""")}
                 {ret('IpRtrap', '', "    " + AR([RV('hvs'), RV('hcall2')], '(Some IpTrap)'))}
                 {ret('IpRfailed', 'fam', "    " + AR([RV('hvs'), RV('hcall2')], '(Some (IpFailed fam))'))}))))))))))"""

def arm_call_fail(leaf, sl, subst=None, runp=0, pre=""):
    """IqStmt (IpCall i k args) with out = IpFailed fam: hvs, hcall, hcall2 (rt = IpRfailed fam2), hn1, hfam (fam2 = fam)"""
    subst = subst or SS
    OUTF = "(Some (IpFailed fam))"
    E = f"(ipexps args lc {IM} mlo slo)"; CL = CALL('f2', 'k', 'vs', IM)
    AR = lambda rws, lhs: absurd_run('ipstmt', rws, lhs, subst, runp, pre, outn=OUTF)
    def ret(ctor, binders, body):
        b = f" ({binders})" if binders else ""
        return f"""(case {ctor}{b}
  (chain
    (have hcall2 (= {CL} (Some ({ctor} {binders}))) (steps ((rewrite (premise hcall) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
{body}))"""
    return f"""(case-on {E} Option
  ((case None (chain {cap('hvs', f'(= {E} None)')} {AR([RV('hvs')], '(Some IpTrap)')}))
   (case Some (vs)
     (chain
       {cap('hvs', f'(= {E} (Some vs))')}
       (case-on {CL} Option
         ((case None (chain {cap('hcall', f'(= {CL} None)')} {AR([RV('hvs'), RV('hcall')], 'None')}))
          (case Some (rt)
            (chain
              {cap('hcall', f'(= {CL} (Some rt))')}
              (case-on rt IpRet
                ({ret('IpRv', 'v memc', f"""    (case-on (ilset lc i v) Option
      ((case None (chain {cap('hset', '(= (ilset lc i v) None)')} {AR([RV('hvs'), RV('hcall2'), RV('hset')], '(Some IpTrap)')}))
       (case Some (lcs) (chain {cap('hset', '(= (ilset lc i v) (Some lcs))')} {AR([RV('hvs'), RV('hcall2'), RV('hset')], '(Some (IpNorm lcs memc))')}))))""")}
                 {ret('IpRtrap', '', "    " + AR([RV('hvs'), RV('hcall2')], '(Some IpTrap)'))}
                 {ret('IpRfailed', 'fam2', f"""    {run_fact('ipstmt', 'hn1', '(Some (IpFailed fam2))', [RV('hvs'), RV('hcall2')], subst, runp, pre, outn=OUTF)}
    (inject (premise hn1) (hn2))
    (inject (premise hn2) (hfam))
{leaf(S_(sl, 'hvs', 'hcall', 'hcall2', 'hn1', 'hn2', 'hfam'))}""")}))))))))))"""

def arm_qcall(leaf, sl, subst=None, runp=0, pre=""):
    """IqCall k at (S f2): hg (ipfn_at fs k = Some g), har (the arity check), hlt (dep < dmax), hb2 (the body's
    ipstmts f2 = Some (IpNorm lcb memb)), hv (iexp result = Some v), hn1, hlc ((Cons v Nil) = lc2), hmem (memb = mem2)"""
    subst = subst or SC
    AR = lambda rws, lhs: absurd_run('ipcall', rws, lhs, subst, runp, pre, POST_CALL)
    B = STSd('f2', BODYg, LCg, IM)
    AE = f"(int_eq (ilen lc) (ikn (ipparams_of g)))"
    V = f"(iexp {RESg} lcb memb mlo slo)"
    def ob(ctor, binders, body):
        b = f" ({binders})" if binders else ""
        return f"""(case {ctor}{b}
  (chain
    (have hb2 (= {B} (Some ({ctor} {binders}))) (steps ((rewrite (premise hb) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
{body}))"""
    return f"""(case-on (ipfn_at fs k) Option
  ((case None (chain {cap('hg', '(= (ipfn_at fs k) None)')} {AR([RV('hg')], '(Some IpTrap)')}))
   (case Some (g)
     (chain
       {cap('hg', '(= (ipfn_at fs k) (Some g))')}
       (case-on {AE} Bool
         ((case False (chain {cap('har', f'(= {AE} False)')} {AR([RV('hg'), RV('har')], '(Some IpTrap)')}))
          (case True
            (chain
              {cap('har', f'(= {AE} True)')}
              (case-on (lt dep dmax) Bool
                ((case False (chain {cap('hlt', '(= (lt dep dmax) False)')} {AR([RV('hg'), RV('har'), RV('hlt')], '(Some (IpFailed FStack))')}))
                 (case True
                   (chain
                     {cap('hlt', '(= (lt dep dmax) True)')}
                     (case-on {B} Option
                       ((case None (chain {cap('hb', f'(= {B} None)')} {AR([RV('hg'), RV('har'), RV('hlt'), RV('hb')], 'None')}))
                        (case Some (ob)
                          (chain
                            {cap('hb', f'(= {B} (Some ob))')}
                            (case-on ob IpOut
                              ({ob('IpNorm', 'lcb memb', f"""    (case-on {V} Option
      ((case None (chain {cap('hv', f'(= {V} None)')} {AR([RV('hg'), RV('har'), RV('hlt'), RV('hb2'), RV('hv')], '(Some IpTrap)')}))
       (case Some (v)
         (chain
           {cap('hv', f'(= {V} (Some v))')}
           {run_fact('ipcall', 'hn1', '(Some (IpNorm (Cons v Nil) memb))', [RV('hg'), RV('har'), RV('hlt'), RV('hb2'), RV('hv')], subst, runp, pre, POST_CALL)}
           (inject (premise hn1) (hn2))
           (inject (premise hn2) (hlc hmem))
{leaf(S_(sl, 'hg', 'har', 'hlt', 'hb', 'hb2', 'hv', 'hn1', 'hn2', 'hlc', 'hmem'))}))))""")}
                               {ob('IpTrap', '', "    " + AR([RV('hg'), RV('har'), RV('hlt'), RV('hb2')], '(Some IpTrap)'))}
                               {ob('IpFailed', 'fam', "    " + AR([RV('hg'), RV('har'), RV('hlt'), RV('hb2')], '(Some (IpFailed fam))'))}))))))))))))))))))"""

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
    def onec(ctor, binders, engine):
        return f"(case {ctor} ({binders}) (chain {cap('hr', f'(= r ({ctor} {binders}))')} {absurd_run(engine, [], 'None', SW, post=POST_CALL)}))"
    return f"""(case-on r IptReq
  ({one('IqStmt','s','ipstmt')}
   {one('IqStmts','ss','ipstmts')}
   {one('IqWhile','ce b','ipwhile')}
   {onec('IqCall','k','ipcall')}))"""

def hb3(P, body_list, subst):
    """the request-level side premise re-spelled on the body list (a 'fmt' side names its own shape and unfolds)"""
    if 'fmt' in P:
        unf = "".join(f" (unfold {u} rhs) (reduce rhs)" for u in P['unf'])
        return f"(have {P['h']} (= {P['fmt'].format(body=body_list)} True) (steps ((rewrite (premise {P['p']}) rl rhs true ()){SUBS(subst)}{unf}) refl))"
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
    slC = S_(sl0, 'hr')
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
                 {stmt_arm('IpStore', 'ae ve', '(IpStore ae ve)', arm_store(leaves['store'], slS))}
                 {stmt_arm('IpIf', 'ce tb eb', '(IpIf ce tb eb)', arm_if(leaves['if_t'], leaves['if_e'], slS))}
                 {stmt_arm('IpWhile', 'ce b', '(IpWhile ce b)', arm_while_stmt(leaves['while'], slS))}
                 {stmt_arm('IpCall', 'i k args', '(IpCall i k args)', arm_call(leaves['call'], slS))}
                 {stmt_arm('IpLoadW', 'i ae', '(IpLoadW i ae)', arm_loadw(leaves['loadw'], slS))}
                 {stmt_arm('IpStoreW', 'ae ve', '(IpStoreW ae ve)', arm_store(leaves['storew'], slS, word=True))}
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
              {arm_qwhile(leaves['qwhile_exit'], leaves['qwhile_iter'], slW)}))
          (case IqCall (k)
            (chain
              {cap('hr', '(= r (IqCall k))')}
              {goal_subst(['hr'])}
              {arm_qcall(leaves['qcall'], slC)}))))))))
"""

# ---------------- T1: the twin's memory law ----------------
# imp's memory after a normal run = the base under the twin's patches that lie below slo

T1_VARS = ("(f Nat) (r IptReq) (fs (List IpFn)) (mlo Int) (slo Int) (dmax Int) (dep Int) (fp Int) (nl Int) (own Int) "
           "(lc (List Int)) (mem0 Mem) (psx (List FPatch)) (lc2 (List Int)) (mem2 Mem)")
def T1C(f, r, lc, im, psx, mem2): return f"(= (fp_mem mem0 (fbelow slo (fp_app {TR(f, r, lc, im)} {psx}))) {mem2})"
T1_SIDES = []

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
            d3 = D('p3')
            return f"""  {HFB('ce', sl)}
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
    L['if_e'] = ifbr('eb', None)
    L['if_t'] = ifbr('tb', None)
    # IpWhile statement: the loop request's IH at the same state
    def while_(sl):
        d0 = "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw) lr lhs true ())) refl)"
        d3 = D('p3')
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
        d3h = D('p3')
        d0t = "(steps ((rewrite (premise hmems) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise ht) lr lhs true ())) refl)"
        d3t = D('p3')
        return f"""              {t1_ih('hmems', '(IqStmt s)', 'lc', 'psx', 'mems', d0h, d3h).replace('(inst lc2 lc2)', '(inst lc2 lcs)')}
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
        d3b = D('p3')
        d0w = "(steps ((rewrite (premise hmemb2) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw2) lr lhs true ())) refl)"
        d3w = D('p3')
        return f"""                            {HFB('ce', sl)}
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
    # the memory statements (A-3 part 3): one patch below the cut on top of the operand spills
    PSX2 = f"(Cons (FWord (+ fp (* 8 nl)) ad) (fp_app {FE_TR('ae')} psx))"
    TBI = f"(fe_tr fp nl 1 ve lc {IM} mlo slo)"
    def store_(word):
        keep = "fbelow_w_lo" if word else "fbelow_b_lo"
        cons = "fp_mem_cons_w" if word else "fp_mem_cons_b"
        def f(sl):
            I = "              "
            A = Arm(sl, I)
            A.arith('hnd0', "(= (le 0 (+ nl 0)) True)", {'p2': 1})
            A.arith('hnd1', "(= (le 0 (+ nl 1)) True)", {'p2': 1})
            A.arith('hlo8', "(= (le (+ (+ fp (* 8 nl)) 8) slo) False)", {'p1': 1, 'p2': 8})
            A.have('hc2', f"(= (fp_app (Cons (FWord (+ fp (* 8 nl)) ad) {FE_TR('ae')}) psx) {PSX2})", "(steps ((unfold fp_app lhs) (reduce lhs)) refl)")
            A.have('hfb', f"(= (fbelow slo {PSX2}) (fbelow slo psx))", f"(chain (rewrite-with (lemma fbelow_w_hi) lr lhs () ({D('hlo8')})) (rewrite-with (lemma fe_tr_below) lr lhs ((inst e ae)) ({D('p1')} {D('hnd0')})) refl)")
            A.have('hfbv', f"(= (fbelow slo (fp_app {TBI} {PSX2})) (fbelow slo {PSX2}))", f"(rewrite-with (lemma fe_tr_below) lr lhs ((inst e ve)) ({D('p1')} {D('hnd1')}) refl)")
            if word: A.have('hiw', "(= (xw8) (iw8))", "(steps ((compute both)) refl)")
            A.add(None, f"""(steps
{I}  ((rewrite (premise hmem) rl rhs true ())
{I}   (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs) (unfold ips_tr lhs) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hvv) lr lhs true ()) (reduce lhs)
{I}   (unfold fp_app lhs) (reduce lhs)))""")
            A.add(None, f"(rewrite-with (lemma {keep}) lr lhs () ({D('hg1')}))")
            tail = " (rewrite (premise hiw) lr lhs true ())" if word else ""
            A.add(None, f"(steps ((rewrite (lemma fp_app_assoc) lr lhs true ()) (rewrite (premise hc2) lr lhs true ()) (rewrite (premise hfbv) lr lhs true ()) (rewrite (premise hfb) lr lhs true ()) (rewrite (lemma {cons}) rl lhs true ()){tail}) refl)")
            return A.text()
        return f
    L['store'] = store_(False)
    L['storew'] = store_(True)
    def loadw_(sl):
        V = VALW(); I = "              "
        A = Arm(sl, I)
        A.have('hi0', "(= (le 0 i) True)", f"(rewrite-with (lemma ilset_lo) lr lhs ((inst ls lc) (inst v {V}) (inst ls2 lcs)) ({D('hset')}) refl)")
        A.arith('hlo8', "(= (le (+ (+ fp (* 8 i)) 8) slo) False)", {'p1': 1, 'hi0': 8})
        A.add('hfb', HFB('ae', A.sl))
        A.add(None, f"""(steps
{I}  ((rewrite (premise hmem) rl rhs true ())
{I}   (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs) (unfold ips_tr lhs) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)
{I}   (unfold fp_app lhs) (reduce lhs)))""")
        A.add(None, f"(rewrite-with (lemma fbelow_w_hi) lr lhs () ({D('hlo8')}))")
        A.add(None, "(steps ((rewrite (premise hfb) lr lhs true ())) refl)")
        return A.text()
    L['loadw'] = loadw_
    # the call statement (A-5): the result's slot word over the callee's patches (at fp + own) over the args'
    def call_(sl):
        I = "              "
        A = Arm(sl, I)
        TC = TRC('f2', '(+ fp own)', 'k', 'vs', IM)
        TCa = TRC('f2', '(+ fp own)', 'k', 'vs', IMx(f"(fp_app {TA0(IM)} psx)"))
        A.have('hi0', "(= (le 0 i) True)", f"(rewrite-with (lemma ilset_lo) lr lhs ((inst ls lc) (inst v v) (inst ls2 lcs)) ({D('hset')}) refl)")
        A.arith('hlo8', "(= (le (+ (+ fp (* 8 i)) 8) slo) False)", {'p1': 1, 'hi0': 8})
        A.arith('hj0', "(= (le 0 0) True)", {})
        A.have('hfb', f"(= (fbelow slo (fp_app {TA0(IM)} psx)) (fbelow slo psx))", f"(rewrite-with (lemma fa_below) lr lhs ((inst j 0) (inst args args)) ({D(2)} {D(3)} {D('hj0')} {D(1)}) refl)")
        A.arith('hslo2', "(= (le slo (+ fp own)) True)", {'p1': 1, 'p3': 1})
        d0 = f"(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hcall2) lr lhs true ()) (unfold ipout_of_ret lhs) (reduce lhs)) refl)"
        A.have('hih', T1C('f2', '(IqCall k)', 'vs', IMx(f"(fp_app {TA0(IM)} psx)"), f"(fp_app {TA0(IM)} psx)", 'memc').replace('(ipt_tr f2 fs mlo slo dmax dep fp nl own', '(ipt_tr f2 fs mlo slo dmax dep (+ fp own) nl own'),
               f"(rewrite-with (hyp ih) lr lhs ((inst lc2 (Cons v Nil)) (inst mem2 memc) (inst fp (+ fp own)) (inst lc vs) (inst nl nl) (inst own own)) ({d0} {D('hslo2')} {D(2)} {D(3)}) refl)")
        A.have('hih2', f"(= (fp_mem mem0 (fbelow slo (fp_app {TC} (fp_app {TA0(IM)} psx)))) memc)",
               "(steps ((rewrite (premise hih) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfb) lr rhs true ())) refl)")
        A.add(None, f"""(steps
{I}  ((rewrite (premise hmem) rl rhs true ())
{I}   (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs)
{I}   (rewrite (premise hvs) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hcall2) lr lhs true ()) (reduce lhs)
{I}   (unfold fp_app lhs) (reduce lhs)))""")
        A.add(None, f"(rewrite-with (lemma fbelow_w_hi) lr lhs () ({D('hlo8')}))")
        A.add(None, "(steps ((rewrite (lemma fp_app_assoc) lr lhs true ()) (rewrite (premise hih2) lr lhs true ())) refl)")
        return A.text()
    L['call'] = call_
    # the call request (A-5): the result's spills over the body's patches over the zeroing
    def qcall_(sl):
        I = "              "
        A = Arm(sl, I)
        TRr = f"(fe_tr fp {NLg} 0 {RESg} lcb memb mlo slo)"
        TB = TRLd('f2', BODYg, LCg, IM)
        PSXz = f"(fp_app {FZg} psx)"
        A.have('hnp0', "(= (le 0 (ikn (ipparams_of g))) True)", "(rewrite-with (lemma ikn_nonneg) lr lhs () () refl)")
        A.have('hne0', "(= (le 0 (ikn (ipextra_of g))) True)", "(rewrite-with (lemma ikn_nonneg) lr lhs () () refl)")
        A.arith('hnl0', f"(= (le 0 {NLg}) True)", {'hnp0': 1, 'hne0': 1})
        A.arith('hnld', f"(= (le 0 (+ {NLg} 0)) True)", {'hnp0': 1, 'hne0': 1})
        A.have('hown0', f"(= (le 0 {OWNg}) True)", "(rewrite-with (lemma ixf_own_nonneg) lr lhs () () refl)")
        A.have('hfz', f"(= (fbelow slo {PSXz}) (fbelow slo psx))", f"(rewrite-with (lemma fz_below) lr lhs () ({D(1)} {D('hnp0')}) refl)")
        d0 = f"(steps ((rewrite (premise hfz) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hb2) lr lhs true ())) refl)"
        A.have('hih', T1C('f2', f'(IqStmts {BODYg})', LCg, IMx(PSXz), PSXz, 'memb').replace('(ipt_tr f2 fs mlo slo dmax dep fp nl own', f'(ipt_tr f2 fs mlo slo dmax (+ dep 1) fp {NLg} {OWNg}'),
               f"(rewrite-with (hyp ih) lr lhs ((inst lc2 lcb) (inst mem2 memb) (inst dep (+ dep 1)) (inst nl {NLg}) (inst own {OWNg}) (inst lc {LCg})) ({d0} {D(1)} {D('hnl0')} {D('hown0')}) refl)")
        A.have('hih2', f"(= (fp_mem mem0 (fbelow slo (fp_app {TB} {PSXz}))) memb)",
               "(steps ((rewrite (premise hih) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfz) lr rhs true ())) refl)")
        A.add(None, f"""(steps
{I}  ((rewrite (premise hmem) rl rhs true ())
{I}   (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_call lhs) (reduce lhs)
{I}   (rewrite (premise hg) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise har) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hlt) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hb2) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (lemma fp_app_assoc) lr lhs true ()) (rewrite (lemma fp_app_assoc) lr lhs true ())))""")
        A.add(None, f"(rewrite-with (lemma fe_tr_below) lr lhs ((inst e {RESg})) ({D(1)} {D('hnld')}))")
        A.add(None, "(steps ((rewrite (premise hih2) lr lhs true ())) refl)")
        return A.text()
    L['qcall'] = qcall_
    return L

def t1():
    prem = [f"(= {RUN('f', 'r', 'lc', IM)} {OUTN})", "(= (le slo fp) True)", "(= (le 0 nl) True)", "(= (le 0 own) True)"]
    return dispatch("ipt_mem", T1_VARS, prem, T1C('f', 'r', 'lc', IM, 'psx', 'mem2'), t1_leaves(), T1_SIDES)

BANNER_T1 = ";; --- THE TWIN'S MEMORY LAW ipt_mem (generated by gen_fra4.py t1 — REGENERATE, never hand-patch) ---"
BLOCKS.append(("t1", BANNER_T1, t1))

# ---------------- T1.5: the twin's frame-disjointness law (A-5) ----------------
# every patch of a normal run's twin is a program patch or lies at or above fp — the callee
# (at fp + own) never touches the caller's locals. Stated over an ARBITRARY imp memory
# (no psx): the arms instantiate it at the sub-run's own memory.

T15_VARS = ("(f Nat) (r IptReq) (fs (List IpFn)) (mlo Int) (slo Int) (dmax Int) (dep Int) (fp Int) (nl Int) (own Int) "
            "(lc (List Int)) (mem Mem) (lc2 (List Int)) (mem2 Mem)")
def T15C(f, r, lc, mem): return f"(= (fp_wmin slo fp {TR(f, r, lc, mem)}) True)"
def t15_leaves():
    L = {}
    def FE(e, d): return f"(fe_tr fp nl {d} {e} lc mem mlo slo)"
    def wmin_tr(A, name, e, d):
        """name: fp_wmin slo fp (fe_tr … d e …) — the spill trace sits at or above fp + 8(nl + d) >= fp"""
        cert = Slots(A.sl.names + ['hm_' + name]).cert({'p2': 8})   # the inner haves are local to the justification
        A.have(name, f"(= (fp_wmin slo fp {FE(e, d)}) True)",
               f"""(chain
{A.ind}    (have hm_{name} (= (fp_min {FE(e, d)} (+ fp (* 8 (+ nl {d})))) True) (rewrite-with (lemma fe_tr_min) lr lhs ((inst e {e})) () refl))
{A.ind}    (have hl_{name} (= (le fp (+ fp (* 8 (+ nl {d})))) True) (by arith {cert}))
{A.ind}    (have hm2_{name} (= (fp_min {FE(e, d)} fp) True) (rewrite-with (lemma fp_min_weaken) lr lhs ((inst lo (+ fp (* 8 (+ nl {d}))))) ({D('hm_' + name)} {D('hl_' + name)}) refl))
{A.ind}    (rewrite-with (lemma fp_wmin_of_min) lr lhs ((inst lo fp)) ({D('hm2_' + name)}))
{A.ind}    refl)""")
    def ih(A, name, r, lc, mem, d0, pins=""):
        A.have(name, T15C('f2', r, lc, mem), f"(rewrite-with (hyp ih) lr lhs ((inst lc2 lc2) (inst mem2 mem2) {pins}) ({d0} {D(1)} {D(2)} {D(3)}) refl)")
    def unf(A, name, src, tw):
        A.have(name, f"(= (fp_wmin slo fp {tw}) True)", f"(steps ((rewrite (premise {src}) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs)) refl)")
    OPEN_S = "(unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs)"
    def set_like(V, e):
        def f(sl):
            I = "              "; A = Arm(sl, I)
            A.have('hi0', "(= (le 0 i) True)", f"(rewrite-with (lemma ilset_lo) lr lhs ((inst ls lc) (inst v {V}) (inst ls2 lcs)) ({D('hset')}) refl)")
            A.arith('hsa', "(= (le fp (+ fp (* 8 i))) True)", {'hi0': 8})
            wmin_tr(A, 'hw', e, 0)
            A.add(None, f"(steps ({OPEN_S} (unfold ips_tr lhs) (reduce lhs) (rewrite (premise hv) lr lhs true ()) (reduce lhs)))")
            A.add(None, f"(rewrite-with (lemma fp_wmin_cons_hi) lr lhs () ({D('hsa')} {D('hw')}))")
            A.add(None, "refl")
            return A.text()
        return f
    L['set'] = set_like('v', 'e')
    L['loadw'] = set_like(VALW(), 'ae')
    def store_(word):
        def f(sl):
            I = "              "; A = Arm(sl, I)
            wmin_tr(A, 'hwv', 've', 1)
            wmin_tr(A, 'hwa', 'ae', 0)
            A.arith('hsa', "(= (le fp (+ fp (* 8 nl))) True)", {'p2': 8})
            A.have('hwc', f"(= (fp_wmin slo fp (Cons (FWord (+ fp (* 8 nl)) ad) {FE('ae', 0)})) True)", f"(rewrite-with (lemma fp_wmin_cons_hi) lr lhs () ({D('hsa')} {D('hwa')}) refl)")
            A.have('hwapp', f"(= (fp_wmin slo fp (fp_app {FE('ve', 1)} (Cons (FWord (+ fp (* 8 nl)) ad) {FE('ae', 0)}))) True)", f"(rewrite-with (lemma fp_wmin_app) lr lhs () ({D('hwv')} {D('hwc')}) refl)")
            A.add(None, f"(steps ({OPEN_S} (unfold ips_tr lhs) (reduce lhs) (rewrite (premise hv) lr lhs true ()) (reduce lhs) (rewrite (premise hvv) lr lhs true ()) (reduce lhs)))")
            if word: A.add(None, f"(rewrite-with (lemma fp_wmin_cons_lo) lr lhs () ({D('hg1')} {D('hwapp')}))")
            else: A.add(None, f"(rewrite-with (lemma fp_wmin_cons_b) lr lhs () ({D('hwapp')}))")
            A.add(None, "refl")
            return A.text()
        return f
    L['store'] = store_(False); L['storew'] = store_(True)
    def ifbr(ss):
        def f(sl):
            I = "  "; A = Arm(sl, I)
            ih(A, 'hih', f'(IqStmts {ss})', 'lc', 'mem', "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hbr) lr lhs true ())) refl)")
            unf(A, 'hih2', 'hih', TRL('f2', ss, 'lc', 'mem'))
            wmin_tr(A, 'hwc', 'ce', 0)
            A.add(None, f"(steps ({OPEN_S} (rewrite (premise hv) lr lhs true ()) (reduce lhs) (rewrite (premise hcv) lr lhs true ()) (reduce lhs)))")
            A.add(None, f"(rewrite-with (lemma fp_wmin_app) lr lhs () ({D('hih2')} {D('hwc')}))")
            A.add(None, "refl")
            return A.text()
        return f
    L['if_t'] = ifbr('tb'); L['if_e'] = ifbr('eb')
    def while_(sl):
        I = "  "; A = Arm(sl, I)
        ih(A, 'hih', '(IqWhile ce b)', 'lc', 'mem', "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw) lr lhs true ())) refl)")
        unf(A, 'hih2', 'hih', TRW('f2', 'ce', 'b', 'lc', 'mem'))
        A.add(None, f"(steps ({OPEN_S} (rewrite (premise hih2) lr lhs true ())) refl)")
        return A.text()
    L['while'] = while_
    L['nil'] = lambda sl: "  (steps ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmts lhs) (reduce lhs) (unfold fp_wmin lhs) (reduce lhs)) refl)"
    def cons_(sl):
        I = "              "; A = Arm(sl, I)
        ih(A, 'hihs', '(IqStmt s)', 'lc', 'mem', "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hs2) lr lhs true ())) refl)")
        A.lines[-1] = A.lines[-1].replace('(inst lc2 lc2) (inst mem2 mem2)', '(inst lc2 lcs) (inst mem2 mems)')
        unf(A, 'hihs2', 'hihs', TRS('f2', 's', 'lc', 'mem'))
        ih(A, 'hiht', '(IqStmts t)', 'lcs', 'mems', "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise ht) lr lhs true ())) refl)", "(inst lc lcs) (inst mem mems)")
        unf(A, 'hiht2', 'hiht', TRL('f2', 't', 'lcs', 'mems'))
        A.add(None, "(steps ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmts lhs) (reduce lhs) (rewrite (premise hs2) lr lhs true ()) (reduce lhs)))")
        A.add(None, f"(rewrite-with (lemma fp_wmin_app) lr lhs () ({D('hiht2')} {D('hihs2')}))")
        A.add(None, "refl")
        return A.text()
    L['cons'] = cons_
    def qexit(sl):
        I = "              "; A = Arm(sl, I)
        wmin_tr(A, 'hwc', 'ce', 0)
        A.add(None, "(steps ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_while lhs) (reduce lhs) (rewrite (premise hv) lr lhs true ()) (reduce lhs) (rewrite (premise hcv) lr lhs true ()) (reduce lhs) (rewrite (premise hwc) lr lhs true ())) refl)")
        return A.text()
    L['qwhile_exit'] = qexit
    def qiter(sl):
        I = "                            "; A = Arm(sl, I)
        ih(A, 'hihb', '(IqStmts b)', 'lc', 'mem', "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hb2) lr lhs true ())) refl)")
        A.lines[-1] = A.lines[-1].replace('(inst lc2 lc2) (inst mem2 mem2)', '(inst lc2 lcb) (inst mem2 memb)')
        unf(A, 'hihb2', 'hihb', TRL('f2', 'b', 'lc', 'mem'))
        ih(A, 'hihw', '(IqWhile ce b)', 'lcb', 'memb', "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw2) lr lhs true ())) refl)", "(inst lc lcb) (inst mem memb)")
        unf(A, 'hihw2', 'hihw', TRW('f2', 'ce', 'b', 'lcb', 'memb'))
        wmin_tr(A, 'hwc', 'ce', 0)
        A.have('hin', f"(= (fp_wmin slo fp (fp_app {TRL('f2', 'b', 'lc', 'mem')} {FE('ce', 0)})) True)", f"(rewrite-with (lemma fp_wmin_app) lr lhs () ({D('hihb2')} {D('hwc')}) refl)")
        A.add(None, "(steps ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_while lhs) (reduce lhs) (rewrite (premise hv) lr lhs true ()) (reduce lhs) (rewrite (premise hcv) lr lhs true ()) (reduce lhs) (rewrite (premise hb2) lr lhs true ()) (reduce lhs)))")
        A.add(None, f"(rewrite-with (lemma fp_wmin_app) lr lhs () ({D('hihw2')} {D('hin')}))")
        A.add(None, "refl")
        return A.text()
    L['qwhile_iter'] = qiter
    def call_(sl):
        I = "              "; A = Arm(sl, I)
        TC = TRC('f2', '(+ fp own)', 'k', 'vs', 'mem')
        TA = "(ipt_args fp nl own 0 args lc mem mlo slo)"
        A.have('hi0', "(= (le 0 i) True)", f"(rewrite-with (lemma ilset_lo) lr lhs ((inst ls lc) (inst v v) (inst ls2 lcs)) ({D('hset')}) refl)")
        A.arith('hsa', "(= (le fp (+ fp (* 8 i))) True)", {'hi0': 8})
        A.arith('hslo2', "(= (le slo (+ fp own)) True)", {'p1': 1, 'p3': 1})
        A.arith('hle', "(= (le fp (+ fp own)) True)", {'p3': 1})
        A.arith('hj0', "(= (le 0 0) True)", {})
        d0 = "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hcall2) lr lhs true ()) (unfold ipout_of_ret lhs) (reduce lhs)) refl)"
        A.have('hihc', f"(= (fp_wmin slo (+ fp own) (ipt_tr f2 fs mlo slo dmax dep (+ fp own) nl own (IqCall k) vs mem)) True)",
               f"(rewrite-with (hyp ih) lr lhs ((inst lc2 (Cons v Nil)) (inst mem2 memc) (inst fp (+ fp own)) (inst lc vs) (inst nl nl) (inst own own)) ({d0} {D('hslo2')} {D(2)} {D(3)}) refl)")
        A.have('hihc2', f"(= (fp_wmin slo (+ fp own) {TC}) True)", "(steps ((rewrite (premise hihc) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs)) refl)")
        A.have('hmono', f"(= (fp_wmin slo fp {TC}) True)", f"(rewrite-with (lemma fp_wmin_mono) lr lhs ((inst lo (+ fp own))) ({D('hihc2')} {D('hle')}) refl)")
        A.have('hwa', f"(= (fp_wmin slo fp {TA}) True)", f"(rewrite-with (lemma fa_wmin) lr lhs ((inst j 0) (inst args args)) ({D(2)} {D(3)} {D('hj0')}) refl)")
        A.have('happ', f"(= (fp_wmin slo fp (fp_app {TC} {TA})) True)", f"(rewrite-with (lemma fp_wmin_app) lr lhs () ({D('hmono')} {D('hwa')}) refl)")
        A.add(None, f"(steps ({OPEN_S} (rewrite (premise hvs) lr lhs true ()) (reduce lhs) (rewrite (premise hcall2) lr lhs true ()) (reduce lhs)))")
        A.add(None, f"(rewrite-with (lemma fp_wmin_cons_hi) lr lhs () ({D('hsa')} {D('happ')}))")
        A.add(None, "refl")
        return A.text()
    L['call'] = call_
    def qcall_(sl):
        I = "              "; A = Arm(sl, I)
        TRr = f"(fe_tr fp {NLg} 0 {RESg} lcb memb mlo slo)"
        TB = TRLd('f2', BODYg, LCg, 'mem')
        A.have('hnp0', "(= (le 0 (ikn (ipparams_of g))) True)", "(rewrite-with (lemma ikn_nonneg) lr lhs () () refl)")
        A.have('hne0', "(= (le 0 (ikn (ipextra_of g))) True)", "(rewrite-with (lemma ikn_nonneg) lr lhs () () refl)")
        A.arith('hnl0', f"(= (le 0 {NLg}) True)", {'hnp0': 1, 'hne0': 1})
        A.have('hown0', f"(= (le 0 {OWNg}) True)", "(rewrite-with (lemma ixf_own_nonneg) lr lhs () () refl)")
        A.have('hmr', f"(= (fp_min {TRr} (+ fp (* 8 (+ {NLg} 0)))) True)", f"(rewrite-with (lemma fe_tr_min) lr lhs ((inst e {RESg})) () refl)")
        A.arith('hlr', f"(= (le fp (+ fp (* 8 (+ {NLg} 0)))) True)", {'hnp0': 8, 'hne0': 8})
        A.have('hmr2', f"(= (fp_min {TRr} fp) True)", f"(rewrite-with (lemma fp_min_weaken) lr lhs ((inst lo (+ fp (* 8 (+ {NLg} 0))))) ({D('hmr')} {D('hlr')}) refl)")
        A.have('hwr', f"(= (fp_wmin slo fp {TRr}) True)", f"(rewrite-with (lemma fp_wmin_of_min) lr lhs ((inst lo fp)) ({D('hmr2')}) refl)")
        d0 = "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hb2) lr lhs true ())) refl)"
        A.have('hihb', f"(= (fp_wmin slo fp (ipt_tr f2 fs mlo slo dmax (+ dep 1) fp {NLg} {OWNg} (IqStmts {BODYg}) {LCg} mem)) True)",
               f"(rewrite-with (hyp ih) lr lhs ((inst lc2 lcb) (inst mem2 memb) (inst dep (+ dep 1)) (inst nl {NLg}) (inst own {OWNg}) (inst lc {LCg})) ({d0} {D(1)} {D('hnl0')} {D('hown0')}) refl)")
        A.have('hihb2', f"(= (fp_wmin slo fp {TB}) True)", "(steps ((rewrite (premise hihb) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs)) refl)")
        A.have('hwz', f"(= (fp_wmin slo fp {FZg}) True)", f"(rewrite-with (lemma fz_wmin) lr lhs () ({D('hnp0')}) refl)")
        A.have('hin', f"(= (fp_wmin slo fp (fp_app {TB} {FZg})) True)", f"(rewrite-with (lemma fp_wmin_app) lr lhs () ({D('hihb2')} {D('hwz')}) refl)")
        A.add(None, f"""(steps
{I}  ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_call lhs) (reduce lhs)
{I}   (rewrite (premise hg) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise har) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hlt) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hb2) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)))""")
        A.add(None, f"(rewrite-with (lemma fp_wmin_app) lr lhs () ({D('hwr')} {D('hin')}))")
        A.add(None, "refl")
        return A.text()
    L['qcall'] = qcall_
    return L

def t15():
    global IM
    saved = IM; IM = "mem"
    try:
        prem = [f"(= {RUN('f', 'r', 'lc', 'mem')} {OUTN})", "(= (le slo fp) True)", "(= (le 0 nl) True)", "(= (le 0 own) True)"]
        text = dispatch("ipt_min", T15_VARS, prem, T15C('f', 'r', 'lc', 'mem'), t15_leaves(), [])
    finally:
        IM = saved
    return text

BANNER_T15 = ";; --- THE TWIN'S FRAME-DISJOINTNESS LAW ipt_min (generated by gen_fra4.py t15 — REGENERATE, never hand-patch) ---"
BLOCKS.append(("t15", BANNER_T15, t15))

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
def T2P(f, r, lc, im, psx, lc2): return f"(= (ixt_post {r} m mlo slo fp nl {lc2} (fp_app {TR(f, r, lc, im)} {psx})) True)"
FR_SIDE = {"fmt": "(ixf_fr nl own (ixf_sdep {body}) (ixf_maxown fs))", "p": 3, "h": "hfr", "unf": ["ixt_fr", "ixt_body"]}
T2_SIDES = [{"fn": "ixf_scb", "p": 2, "h": "hscb"}, FR_SIDE]
def fr_d(h): return f"(steps ((unfold ixt_fr lhs) (reduce lhs) (unfold ixt_body lhs) (reduce lhs) (rewrite (premise {h}) lr lhs true ())) refl)"
def fr_sub_have(name, sd, hsd, body):
    """name: ixf_fr nl own sd M from hfr (the request's bundle over body) and hsd (sd <= the body's depth)"""
    return f"(have {name} (= (ixf_fr nl own {sd} (ixf_maxown fs)) True) (rewrite-with (lemma fr_sub) lr lhs ((inst sd (ixf_sdep {body}))) ({D('hfr')} {D(hsd)}) refl))"

def t2_ih(hname, r, lc, psx, lc2, mem2, d0, d1, d2, d3):
    """the IH at f2 in the post-state form; d3 = the frame-bundle discharge"""
    return f"""(have {hname} {T2P('f2', r, lc, IMx(psx), psx, lc2)}
  (rewrite-with (hyp ih) lr lhs ((inst mem2 {mem2})) ({d0} {d1} {d2} {d3} {D(4)}) refl))"""

def t1_cite(hname, r, lc, psx, lc2, mem2, d0, d3):
    """the memory law for a sub-run (ipt_mem), at (r, lc, psx); d3 = the 0 <= own discharge"""
    return f"""(have {hname} {T1C('f2', r, lc, IMx(psx), psx, mem2)}
  (rewrite-with (lemma ipt_mem) lr lhs ((inst lc2 {lc2}) (inst mem2 {mem2})) ({d0} {D('hslo')} {D('hnl0')} {d3}) refl))"""

def body_side(h, pred, body):
    """a sub-request's side premise discharge: unfold ixt_body, then the extracted fact"""
    return f"(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise {h}) lr lhs true ())) refl)"

def t2_leaves():
    L = {}
    def set_like(sl, E, V, ext_lemma, ext_pins, band):
        """the slot store of V (an expression E's value, or the loaded word) into local i"""
        I = "              "
        cf, s2 = ctx_facts(sl, 'p1', I)
        A = Arm(s2, I)
        A.have('hi0', "(= (le 0 i) True)", f"(rewrite-with (lemma ilset_lo) lr lhs ((inst ls lc) (inst v {V}) (inst ls2 lcs)) ({D('hset')}) refl)")
        A.have('hii', "(= (lt i (ilen lc)) True)", f"(rewrite-with (lemma ilset_hi) lr lhs ((inst v {V}) (inst ls2 lcs)) ({D('hset')}) refl)")
        A.have('hcbe', f"(= (ixf_cb {E}) True)", f"(rewrite-with (lemma {ext_lemma}) lr lhs ({ext_pins}) ({D('hscb')}) refl)")
        band(A)
        A.have('hsome2', f"(= (ilset lc i {V}) (Some (fl_set lc i {V})))", f"(rewrite-with (lemma ilset_some) lr lhs () ({D('hi0')} {D('hii')}) refl)")
        A.have('hsl', f"(= (Some lcs) (Some (fl_set lc i {V})))", "(steps ((rewrite (premise hset) rl lhs true ()) (rewrite (premise hsome2) lr lhs true ())) refl)")
        A.add('hlcs0', "(inject (premise hsl) (hlcs0))")
        A.have('hlcs', f"(= lc2 (fl_set lc i {V}))", "(steps ((rewrite (premise hlc) rl lhs true ()) (rewrite (premise hlcs0) lr lhs true ())) refl)")
        A.arith('hlo8', "(= (le (+ (+ fp (* 8 i)) 8) slo) False)", {'hslo': 1, 'hi0': 8})
        A.arith('hsa', "(= (le slo (+ fp (* 8 i))) True)", {'hslo': 1, 'hi0': 8})
        A.have('hali', "(= (int_eq (mod (- (+ fp (* 8 i)) slo) 8) 0) True)", f"(rewrite-with (lemma al_shift) lr lhs ((inst k i)) ({D('hal')}) refl)")
        A.have('hdisca', f"(= (fp_disc slo {PSX1(E)}) True)", f"(rewrite-with (lemma fe_tr_disc) lr lhs ((inst e {E})) ({D('hdisc')} {D('hslo')} {D('hal')} {D('hnd0')}) refl)")
        A.have('hdisc2', f"(= (fp_disc slo (Cons (FWord (+ fp (* 8 i)) {V}) {PSX1(E)})) True)", f"(rewrite-with (lemma fp_disc_w_slot) lr lhs () ({D('hlo8')} {D('hsa')} {D('hali')} {D('hdisca')}) refl)")
        A.have('hlocsa', f"(= (fr_locs fp lc {PSX1(E)}) True)", f"(rewrite-with (lemma fe_tr_locs) lr lhs ((inst e {E})) ({D('hlocs')} {D('hnl')} {D('hd0')}) refl)")
        A.have('hlocs2', f"(= (fr_locs fp (fl_set lc i {V}) (Cons (FWord (+ fp (* 8 i)) {V}) {PSX1(E)})) True)", f"(rewrite-with (lemma fr_locs_set) lr lhs () ({D('hlocsa')} {D('hi0')} {D('hii')} {D('hb')}) refl)")
        A.have('hlen2', f"(= (le (ilen (fl_set lc i {V})) nl) True)", "(steps ((rewrite (lemma fl_set_len) lr lhs true ()) (rewrite (premise hnl) lr lhs true ())) refl)")
        A.add(None, f"""(steps
{I}  ((rewrite (premise hlcs) lr lhs true ())
{I}   (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs) (unfold ips_tr lhs) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)
{I}   (unfold fp_app lhs) (reduce lhs)))""")
        A.add(None, f"(rewrite-with (lemma ctx_intro) lr lhs () ({D('hdisc2')} {D('hlocs2')} {D('hxlo')} {D('hmlo')} {D('hslo')} {D('hal')} {D('hlen2')} {D('hd0')} {D('hslo0')} {D('hhi')}))")
        A.add(None, "refl")
        return cf + "\n" + A.text()
    def set_(sl):
        def band(A):
            A.have('hb', "(= (fe_inband v) True)", f"(rewrite-with (lemma fe_band) lr lhs ((inst lc lc) (inst mem {IM}) (inst mlo mlo) (inst msz slo) (inst fp fp) (inst psx psx) (inst e e)) ({D('hv')} {D('hlocs')} {D('hcbe')}) refl)")
        return set_like(sl, 'e', 'v', 'scb_set_e', '(inst i i) (inst t Nil)', band)
    L['set'] = set_
    def ifbr(ss, ext_cb, ext_a4):
        def f(sl):
            I = "  "
            cf, s2 = ctx_facts(sl, 'p1', I)
            d0 = "(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hbr) lr lhs true ())) refl)"
            return f"""{cf}
{ctx_after_cond('ce', I)}
{I}(have hcb (= (ixf_scb {ss}) True) (rewrite-with (lemma {ext_cb}) lr lhs ((inst ce ce) (inst tb tb) (inst eb eb) (inst t Nil)) ({D('hscb')}) refl))
{I}(have hsdb (= (le (ixf_sdep {ss}) (ixf_sdep (Cons (IpIf ce tb eb) Nil))) True) (rewrite-with (lemma sdep_if_{ss[0]}) lr lhs ((inst t Nil)) () refl))
{I}{fr_sub_have('hfrb', f'(ixf_sdep {ss})', 'hsdb', '(Cons (IpIf ce tb eb) Nil)')}
{I}{t2_ih('hih', f'(IqStmts {ss})', 'lc', PSX1('ce'), 'lc2', 'mem2', d0, D('hctx1'), body_side('hcb', None, None), fr_d('hfrb'))}
{I}(have hih2 {T2C('f2', f'(IqStmts {ss})', 'lc', IM, PSX1('ce'), 'lc2').replace(TR('f2', f'(IqStmts {ss})', 'lc', IM), TRL('f2', ss, 'lc', IM))}
{I}  (steps ((rewrite (premise hih) rl rhs true ()) (unfold ixt_post rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfb) lr rhs true ())) refl))
{I}(steps
{I}  ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hcv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (lemma fp_app_assoc) lr lhs true ())
{I}   (rewrite (premise hih2) lr lhs true ()))
{I}  refl)"""
        return f
    L['if_e'] = ifbr('eb', 'scb_if_e', None)
    L['if_t'] = ifbr('tb', 'scb_if_t', None)
    def while_(sl):
        d0 = "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw) lr lhs true ())) refl)"
        return f"""  {t2_ih('hih', '(IqWhile ce b)', 'lc', 'psx', 'lc2', 'mem2', d0, D('p1'), body_side('hscb', None, None), fr_d('hfr'))}
  (have hih2 {T2C('f2', '(IqWhile ce b)', 'lc', IM, 'psx', 'lc2').replace(TR('f2', '(IqWhile ce b)', 'lc', IM), TRW('f2', 'ce', 'b', 'lc', IM))}
    (steps ((rewrite (premise hih) rl rhs true ()) (unfold ixt_post rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs)) refl))
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
{I}(have hsdh0 (= (le (ixf_sdep (Cons s Nil)) (ixf_sdep (Cons s t))) True) (rewrite-with (lemma sdep_head) lr lhs () () refl))
{I}(have hsdt0 (= (le (ixf_sdep t) (ixf_sdep (Cons s t))) True) (rewrite-with (lemma sdep_tail) lr lhs ((inst s s)) () refl))
{I}{fr_sub_have('hfrh', '(ixf_sdep (Cons s Nil))', 'hsdh0', '(Cons s t)')}
{I}{fr_sub_have('hfrt', '(ixf_sdep t)', 'hsdt0', '(Cons s t)')}
{I}(have hfrn (= (le (* 8 (+ nl (+ 1 (ixf_sdep (Cons s t))))) own) True) (rewrite-with (lemma fr_nl) lr lhs ((inst mo (ixf_maxown fs))) ({D('hfr')}) refl))
{I}(have hsd0 (= (le 0 (ixf_sdep (Cons s t))) True) (rewrite-with (lemma ixf_sdep_nonneg) lr lhs () () refl))
{I}(have hown0 (= (le 0 own) True) (by arith {Slots(s2.names + ['hcbh','hcbt','hsdh0','hsdt0','hfrh','hfrt','hfrn','hsd0']).cert({'hfrn': 1, 'hnl0': 8, 'hsd0': 8})}))
{I}{t2_ih('hctxs', '(IqStmt s)', 'lc', 'psx', 'lcs', 'mems', d0h, D('p1'), body_side('hcbh', None, None), fr_d('hfrh'))}
{I}(have hctxs2 {T2C('f2', '(IqStmt s)', 'lc', IM, 'psx', 'lcs')} (steps ((rewrite (premise hctxs) rl rhs true ()) (unfold ixt_post rhs) (reduce rhs)) refl))
{I}{t1_cite('hmems', '(IqStmt s)', 'lc', 'psx', 'lcs', 'mems', d0h, D('hown0'))}
{I}{t2_ih('hih', '(IqStmts t)', 'lcs', PSXs, 'lc2', 'mem2', d0t, D('hctxs2'), body_side('hcbt', None, None), fr_d('hfrt'))}
{I}(have hih2 {T2C('f2', '(IqStmts t)', 'lcs', 'mems', PSXs2, 'lc2').replace(TR('f2', '(IqStmts t)', 'lcs', 'mems'), TRL('f2', 't', 'lcs', 'mems'))}
{I}  (steps ((rewrite (premise hih) rl rhs true ()) (unfold ixt_post rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hmems) lr rhs true ()) (unfold ipt_tr rhs) (reduce rhs)) refl))
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
{I}(have hsdb (= (le (ixf_sdep b) (ixf_sdep (Cons (IpWhile ce b) Nil))) True) (rewrite-with (lemma sdep_while_b) lr lhs ((inst t Nil)) () refl))
{I}{fr_sub_have('hfrb', '(ixf_sdep b)', 'hsdb', '(Cons (IpWhile ce b) Nil)')}
{I}(have hfrn (= (le (* 8 (+ nl (+ 1 (ixf_sdep (Cons (IpWhile ce b) Nil))))) own) True) (rewrite-with (lemma fr_nl) lr lhs ((inst mo (ixf_maxown fs))) ({D('hfr')}) refl))
{I}(have hsd0 (= (le 0 (ixf_sdep (Cons (IpWhile ce b) Nil))) True) (rewrite-with (lemma ixf_sdep_nonneg) lr lhs () () refl))
{I}(have hown0 (= (le 0 own) True) (by arith {Slots(s2.names + ['hdisca','hlocsa','hfb','hctx1','hcb','hsdb','hfrb','hfrn','hsd0']).cert({'hfrn': 1, 'hnl0': 8, 'hsd0': 8})}))
{I}{t2_ih('hctxb', '(IqStmts b)', 'lc', PSX1('ce'), 'lcb', 'memb', d0b, D('hctx1'), body_side('hcb', None, None), fr_d('hfrb'))}
{I}(have hctxb2 (= (fe_ctx m mlo slo fp nl 0 lcb {PSXb}) True)
{I}  (steps ((rewrite (premise hctxb) rl rhs true ()) (unfold ixt_post rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfb) lr rhs true ())) refl))
{I}{t1_cite('hmemb', '(IqStmts b)', 'lc', PSX1('ce'), 'lcb', 'memb', d0b, D('hown0'))}
{I}(have hmemb2 (= (fp_mem mem0 (fbelow slo {PSXb})) memb)
{I}  (steps ((rewrite (premise hmemb) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfb) lr rhs true ())) refl))
{I}{t2_ih('hih', '(IqWhile ce b)', 'lcb', PSXb, 'lc2', 'mem2', d0w, D('hctxb2'), body_side('hscb', None, None), fr_d('hfr'))}
{I}(have hih2 {T2C('f2', '(IqWhile ce b)', 'lcb', 'memb', PSXb, 'lc2').replace(TR('f2', '(IqWhile ce b)', 'lcb', 'memb'), TRW('f2', 'ce', 'b', 'lcb', 'memb'))}
{I}  (steps ((rewrite (premise hih) rl rhs true ()) (unfold ixt_post rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hmemb2) lr rhs true ())) refl))
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
    # the memory statements (A-3 part 3)
    def loadw_(sl):
        V = VALW()
        def band(A):
            A.have('hvl', f"(= (le 0 {V}) True)", "(rewrite-with (lemma ldw_lo) lr lhs () () refl)")
            A.have('hvh', f"(= (lt {V} 18446744073709551616) True)", "(rewrite-with (lemma ldw_hi) lr lhs () () refl)")
            A.have('hb', f"(= (fe_inband {V}) True)", f"(rewrite-with (lemma inband_intro) lr lhs () ({D('hvl')} {D('hvh')}) refl)")
        return set_like(sl, 'ae', V, 'scb_loadw_a', '(inst i i) (inst t Nil)', band)
    L['loadw'] = loadw_
    PSX2 = f"(Cons (FWord (+ fp (* 8 nl)) ad) (fp_app {FE_TR('ae')} psx))"
    TBI = f"(fe_tr fp nl 1 ve lc {IM} mlo slo)"
    def store_(word):
        disc = "fp_disc_w_lo" if word else "fp_disc_b"
        kind = "storew" if word else "store"
        PATCH = "(FWord ad vv)" if word else "(FByte ad vv)"
        NEW = f"(Cons {PATCH} (fp_app {TBI} {PSX2}))"
        def f(sl):
            I = "              "
            cf, s2 = ctx_facts(sl, 'p1', I)
            A = Arm(s2, I)
            A.arith('hnd1', "(= (le 0 (+ nl 1)) True)", {'hnl': 1, 'hnn': 1})
            A.arith('hd1', "(= (le 0 1) True)", {})
            A.arith('hlo8', "(= (le (+ (+ fp (* 8 nl)) 8) slo) False)", {'hslo': 1, 'hnl0': 8})
            A.arith('hsa', "(= (le slo (+ fp (* 8 nl))) True)", {'hslo': 1, 'hnl0': 8})
            A.have('hali', "(= (int_eq (mod (- (+ fp (* 8 nl)) slo) 8) 0) True)", f"(rewrite-with (lemma al_shift) lr lhs ((inst k nl)) ({D('hal')}) refl)")
            A.have('hdisca', f"(= (fp_disc slo {PSX1('ae')}) True)", f"(rewrite-with (lemma fe_tr_disc) lr lhs ((inst e ae)) ({D('hdisc')} {D('hslo')} {D('hal')} {D('hnd0')}) refl)")
            A.have('hdisc2', f"(= (fp_disc slo {PSX2}) True)", f"(rewrite-with (lemma fp_disc_w_slot) lr lhs () ({D('hlo8')} {D('hsa')} {D('hali')} {D('hdisca')}) refl)")
            A.have('hdiscb', f"(= (fp_disc slo (fp_app {TBI} {PSX2})) True)", f"(rewrite-with (lemma fe_tr_disc) lr lhs ((inst e ve)) ({D('hdisc2')} {D('hslo')} {D('hal')} {D('hnd1')}) refl)")
            A.have('hdisc3', f"(= (fp_disc slo {NEW}) True)", f"(rewrite-with (lemma {disc}) lr lhs () ({D('hg1')} {D('hdiscb')}) refl)")
            A.have('hlocsa', f"(= (fr_locs fp lc {PSX1('ae')}) True)", f"(rewrite-with (lemma fe_tr_locs) lr lhs ((inst e ae)) ({D('hlocs')} {D('hnl')} {D('hd0')}) refl)")
            A.arith('hpast', "(= (le (+ fp (* 8 (ilen lc))) (+ fp (* 8 nl))) True)", {'hnl': 8})
            A.have('hlocs2', f"(= (fr_locs fp lc {PSX2}) True)", f"(rewrite-with (lemma fr_locs_skip) lr lhs () ({D('hlocsa')} {D('hpast')}) refl)")
            A.have('hlocsb', f"(= (fr_locs fp lc (fp_app {TBI} {PSX2})) True)", f"(rewrite-with (lemma fe_tr_locs) lr lhs ((inst e ve)) ({D('hlocs2')} {D('hnl')} {D('hd1')}) refl)")
            if word:
                A.arith('hadf', "(= (lt ad fp) True)", {'hg1': 1, 'hslo': 1})
                A.have('hlocs3', f"(= (fr_locs fp lc {NEW}) True)", f"(rewrite-with (lemma fr_locs_skip_lo) lr lhs () ({D('hlocsb')} {D('hadf')}) refl)")
            else:
                A.have('hlocs3', f"(= (fr_locs fp lc {NEW}) True)", f"(rewrite-with (lemma fr_locs_b) lr lhs () ({D('hlocsb')}) refl)")
            A.have('hc2', f"(= (fp_app (Cons (FWord (+ fp (* 8 nl)) ad) {FE_TR('ae')}) psx) {PSX2})", "(steps ((unfold fp_app lhs) (reduce lhs)) refl)")
            A.add(None, f"""(steps
{I}  ((rewrite (premise hlc) rl lhs true ())
{I}   (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs) (unfold ips_tr lhs) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hvv) lr lhs true ()) (reduce lhs)
{I}   (unfold fp_app lhs) (reduce lhs)
{I}   (rewrite (lemma fp_app_assoc) lr lhs true ())
{I}   (rewrite (premise hc2) lr lhs true ())))""")
            A.add(None, f"(rewrite-with (lemma ctx_intro) lr lhs () ({D('hdisc3')} {D('hlocs3')} {D('hxlo')} {D('hmlo')} {D('hslo')} {D('hal')} {D('hnl')} {D('hd0')} {D('hslo0')} {D('hhi')}))")
            A.add(None, "refl")
            return cf + "\n" + A.text()
        return f
    L['store'] = store_(False)
    L['storew'] = store_(True)
    # ---- the call statement (A-5): the caller's context after the call ----
    def call_(sl):
        I = "              "
        cf, s2 = ctx_facts(sl, 'p1', I)
        A = Arm(s2, I)
        CALLS = "(Cons (IpCall i k args) Nil)"
        TA = TA0(IM); PSXa = f"(fp_app {TA} psx)"
        TC = TRC('f2', '(+ fp own)', 'k', 'vs', IM)
        TCa = f"(ipt_tr f2 fs mlo slo dmax dep (+ fp own) (ilen vs) own (IqCall k) vs {IMx(PSXa)})"
        P = "(FWord (+ fp (* 8 i)) v)"
        A.have('hcbs', "(= (ixf_cbs args) True)", f"(rewrite-with (lemma scb_call_args) lr lhs ((inst i i) (inst k k) (inst t Nil)) ({D('hscb')}) refl)")
        A.have('hsdc', f"(= (le (ixf_deps args) (ixf_sdep {CALLS})) True)", "(rewrite-with (lemma sdep_call) lr lhs ((inst t Nil)) () refl)")
        A.have('hfrn', f"(= (le (* 8 (+ nl (+ 1 (ixf_sdep {CALLS})))) own) True)", f"(rewrite-with (lemma fr_nl) lr lhs ((inst mo (ixf_maxown fs))) ({D('hfr')}) refl)")
        A.have('hfra', "(= (int_eq (mod own 8) 0) True)", f"(rewrite-with (lemma fr_al) lr lhs ((inst nl nl) (inst sd (ixf_sdep {CALLS})) (inst mo (ixf_maxown fs))) ({D('hfr')}) refl)")
        A.have('hsd0', f"(= (le 0 (ixf_sdep {CALLS})) True)", "(rewrite-with (lemma ixf_sdep_nonneg) lr lhs () () refl)")
        A.arith('hown0', "(= (le 0 own) True)", {'hfrn': 1, 'hnl0': 8, 'hsd0': 8})
        A.arith('hj0', "(= (le 0 0) True)", {})
        A.arith('hdo', "(= (le (* 8 (+ nl (ixf_deps args))) own) True)", {'hfrn': 1, 'hsdc': 8})
        A.arith('hnlo', "(= (le (* 8 nl) own) True)", {'hfrn': 1, 'hsd0': 8})
        A.have('hdisca', f"(= (fp_disc slo {PSXa}) True)", f"(rewrite-with (lemma fa_disc) lr lhs ((inst j 0) (inst args args)) ({D('hdisc')} {D('hslo')} {D('hal')} {D('hfra')} {D('hnl0')} {D('hown0')} {D('hj0')}) refl)")
        A.have('hfb', f"(= (fbelow slo {PSXa}) (fbelow slo psx))", f"(rewrite-with (lemma fa_below) lr lhs ((inst j 0) (inst args args)) ({D('hnl0')} {D('hown0')} {D('hj0')} {D('hslo')}) refl)")
        A.have('hslots0', f"(= (fr_locs (+ fp (+ own (* 8 0))) vs {PSXa}) True)", f"(rewrite-with (lemma fa_slots) lr lhs ((inst j 0) (inst args args) (inst vs vs) (inst q psx) (inst psx psx)) ({D('hvs')} {D('hlocs')} {D('hcbs')} {D('hdo')} {D('hj0')}) refl)")
        A.have('hfo', "(= (+ fp (+ own (* 8 0))) (+ fp own))", "(by arith (list (list 1) (list 1)))")
        A.have('hslots', f"(= (fr_locs (+ fp own) vs {PSXa}) True)", "(steps ((rewrite (premise hfo) rl lhs true ()) (rewrite (premise hslots0) lr lhs true ())) refl)")
        A.arith('hslo2', "(= (le slo (+ fp own)) True)", {'hslo': 1, 'hown0': 1})
        A.have('hal2', "(= (int_eq (mod (- (+ fp own) slo) 8) 0) True)", f"(rewrite-with (lemma al_add) lr lhs () ({D('hal')} {D('hfra')}) refl)")
        A.have('hlenv', "(= (le (ilen vs) (ilen vs)) True)", "(by arith (list 1))")
        A.have('hctxc', f"(= (fe_ctx m mlo slo (+ fp own) (ilen vs) 0 vs {PSXa}) True)", f"(rewrite-with (lemma ctx_intro) lr lhs () ({D('hdisca')} {D('hslots')} {D('hxlo')} {D('hmlo')} {D('hslo2')} {D('hal2')} {D('hlenv')} {D('hd0')} {D('hslo0')} {D('hhi')}) refl)")
        d0 = "(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hcall2) lr lhs true ()) (unfold ipout_of_ret lhs) (reduce lhs)) refl)"
        IHC = f"(= (ixt_post (IqCall k) m mlo slo (+ fp own) (ilen vs) (Cons v Nil) (fp_app {TCa} {PSXa})) True)"
        A.have('hih', IHC, f"(rewrite-with (hyp ih) lr lhs ((inst mem2 memc) (inst lc2 (Cons v Nil)) (inst fp (+ fp own)) (inst nl (ilen vs)) (inst own own) (inst lc vs)) ({d0} {D('hctxc')} (steps ((unfold ixt_body lhs) (reduce lhs) (unfold ixf_scb lhs) (reduce lhs)) refl) (steps ((unfold ixt_fr lhs) (reduce lhs) (rewrite (premise hown0) lr lhs true ())) refl) {D(4)}) refl)")
        A.have('hpc', f"(= (fe_ctx m mlo slo (+ fp own) (ilen vs) 0 Nil (fp_app {TCa} {PSXa})) True)", f"(rewrite-with (lemma post_call_ctx) lr lhs ((inst k k) (inst lc2 (Cons v Nil))) ({D('hih')}) refl)")
        A.have('hpc2', f"(= (fe_ctx m mlo slo (+ fp own) (ilen vs) 0 Nil (fp_app {TC} {PSXa})) True)", "(steps ((rewrite (premise hpc) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfb) lr rhs true ())) refl)")
        A.have('hpb0', "(= (fe_inband (ihd (Cons v Nil))) True)", f"(rewrite-with (lemma post_call_band) lr lhs ((inst k k) (inst m m) (inst mlo mlo) (inst slo slo) (inst fp (+ fp own)) (inst nl (ilen vs)) (inst ps (fp_app {TCa} {PSXa}))) ({D('hih')}) refl)")
        A.have('hpb', "(= (fe_inband v) True)", "(steps ((rewrite (premise hpb0) rl rhs true ()) (unfold ihd rhs) (reduce rhs)) refl)")
        A.have('hdiscc', f"(= (fp_disc slo (fp_app {TC} {PSXa})) True)", f"(rewrite-with (lemma ctx_disc) lr lhs ((inst m m) (inst mlo mlo) (inst fp (+ fp own)) (inst nl (ilen vs)) (inst d 0) (inst lc Nil)) ({D('hpc2')}) refl)")
        A.have('hi0', "(= (le 0 i) True)", f"(rewrite-with (lemma ilset_lo) lr lhs ((inst ls lc) (inst v v) (inst ls2 lcs)) ({D('hset')}) refl)")
        A.have('hii', "(= (lt i (ilen lc)) True)", f"(rewrite-with (lemma ilset_hi) lr lhs ((inst v v) (inst ls2 lcs)) ({D('hset')}) refl)")
        A.arith('hlo8', "(= (le (+ (+ fp (* 8 i)) 8) slo) False)", {'hslo': 1, 'hi0': 8})
        A.arith('hsa', "(= (le slo (+ fp (* 8 i))) True)", {'hslo': 1, 'hi0': 8})
        A.have('hali', "(= (int_eq (mod (- (+ fp (* 8 i)) slo) 8) 0) True)", f"(rewrite-with (lemma al_shift) lr lhs ((inst k i)) ({D('hal')}) refl)")
        A.have('hdisc2', f"(= (fp_disc slo (Cons {P} (fp_app {TC} {PSXa}))) True)", f"(rewrite-with (lemma fp_disc_w_slot) lr lhs () ({D('hlo8')} {D('hsa')} {D('hali')} {D('hdiscc')}) refl)")
        d0m = "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hcall2) lr lhs true ()) (unfold ipout_of_ret lhs) (reduce lhs)) refl)"
        A.have('hminc', f"(= (fp_wmin slo (+ fp own) (ipt_tr f2 fs mlo slo dmax dep (+ fp own) nl own (IqCall k) vs {IM})) True)", f"(rewrite-with (lemma ipt_min) lr lhs ((inst lc2 (Cons v Nil)) (inst mem2 memc) (inst fp (+ fp own)) (inst lc vs) (inst nl nl) (inst own own)) ({d0m} {D('hslo2')} {D('hnl0')} {D('hown0')}) refl)")
        A.have('hminc2', f"(= (fp_wmin slo (+ fp own) {TC}) True)", "(steps ((rewrite (premise hminc) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs)) refl)")
        A.arith('hpast', "(= (le (+ fp (* 8 (ilen lc))) (+ fp own)) True)", {'hnl': 8, 'hfrn': 1, 'hsd0': 8})
        A.have('hminc3', f"(= (fp_wmin slo (+ fp (* 8 (ilen lc))) {TC}) True)", f"(rewrite-with (lemma fp_wmin_mono) lr lhs ((inst lo (+ fp own))) ({D('hminc2')} {D('hpast')}) refl)")
        A.have('hlocsa', f"(= (fr_locs fp lc {PSXa}) True)", f"(rewrite-with (lemma fa_locs) lr lhs ((inst j 0) (inst args args)) ({D('hlocs')} {D('hnl')} {D('hnlo')} {D('hj0')}) refl)")
        A.have('hlocsc', f"(= (fr_locs fp lc (fp_app {TC} {PSXa})) True)", f"(rewrite-with (lemma fr_locs_app_wmin) lr lhs ((inst slo slo)) ({D('hlocsa')} {D('hminc3')} {D('hslo')}) refl)")
        A.have('hlocs2', f"(= (fr_locs fp (fl_set lc i v) (Cons {P} (fp_app {TC} {PSXa}))) True)", f"(rewrite-with (lemma fr_locs_set) lr lhs () ({D('hlocsc')} {D('hi0')} {D('hii')} {D('hpb')}) refl)")
        A.have('hsome2', "(= (ilset lc i v) (Some (fl_set lc i v)))", f"(rewrite-with (lemma ilset_some) lr lhs () ({D('hi0')} {D('hii')}) refl)")
        A.have('hsl', "(= (Some lcs) (Some (fl_set lc i v)))", "(steps ((rewrite (premise hset) rl lhs true ()) (rewrite (premise hsome2) lr lhs true ())) refl)")
        A.add('hlcs0', "(inject (premise hsl) (hlcs0))")
        A.have('hlcs', "(= lc2 (fl_set lc i v))", "(steps ((rewrite (premise hlc) rl lhs true ()) (rewrite (premise hlcs0) lr lhs true ())) refl)")
        A.have('hlen2', "(= (le (ilen (fl_set lc i v)) nl) True)", "(steps ((rewrite (lemma fl_set_len) lr lhs true ()) (rewrite (premise hnl) lr lhs true ())) refl)")
        A.add(None, f"""(steps
{I}  ((rewrite (premise hlcs) lr lhs true ())
{I}   (unfold ipt_tr lhs) (reduce lhs) (unfold ipt_stmt lhs) (reduce lhs)
{I}   (rewrite (premise hvs) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hcall2) lr lhs true ()) (reduce lhs)
{I}   (unfold fp_app lhs) (reduce lhs)
{I}   (rewrite (lemma fp_app_assoc) lr lhs true ())))""")
        A.add(None, f"(rewrite-with (lemma ctx_intro) lr lhs () ({D('hdisc2')} {D('hlocs2')} {D('hxlo')} {D('hmlo')} {D('hslo')} {D('hal')} {D('hlen2')} {D('hd0')} {D('hslo0')} {D('hhi')}))")
        A.add(None, "refl")
        return cf + "\n" + A.text()
    L['call'] = call_
    # ---- the call request (A-5): the callee's context at entry, the body's law, the result's band ----
    def qcall_(sl):
        I = "              "
        cf, s2 = ctx_facts(sl, 'p1', I)
        A = Arm(s2, I)
        NP = "(ikn (ipparams_of g))"; NE = "(ikn (ipextra_of g))"
        PSXz = f"(fp_app {FZg} psx)"
        TB = TRLd('f2', BODYg, LCg, IM)
        PSXb = f"(fp_app {TB} {PSXz})"
        IMb = IMx(PSXb)
        TRrI = f"(fe_tr fp {NLg} 0 {RESg} lcb {IMb} mlo slo)"
        TRr = f"(fe_tr fp {NLg} 0 {RESg} lcb memb mlo slo)"
        A.have('hfnok', "(= (ixf_fnok g) True)", f"(rewrite-with (lemma fnsok_at) lr lhs ((inst k k) (inst fs fs)) ({D('hg')} {D(4)}) refl)")
        A.have('hp64', "(= (ixf_p64 (ipparams_of g)) True)", f"(rewrite-with (lemma fnok_p64) lr lhs () ({D('hfnok')}) refl)")
        A.have('hscbb', f"(= (ixf_scb {BODYg}) True)", f"(rewrite-with (lemma fnok_scb) lr lhs () ({D('hfnok')}) refl)")
        A.have('hcbr', f"(= (ixf_cb {RESg}) True)", f"(rewrite-with (lemma fnok_cb) lr lhs () ({D('hfnok')}) refl)")
        A.have('hnp0', f"(= (le 0 {NP}) True)", "(rewrite-with (lemma ikn_nonneg) lr lhs () () refl)")
        A.have('hne0', f"(= (le 0 {NE}) True)", "(rewrite-with (lemma ikn_nonneg) lr lhs () () refl)")
        A.arith('hnlg', f"(= (le 0 {NLg}) True)", {'hnp0': 1, 'hne0': 1})
        A.arith('hnld', f"(= (le 0 (+ {NLg} 0)) True)", {'hnp0': 1, 'hne0': 1})
        A.have('hown0', f"(= (le 0 {OWNg}) True)", "(rewrite-with (lemma ixf_own_nonneg) lr lhs () () refl)")
        A.have('hfz', f"(= (fbelow slo {PSXz}) (fbelow slo psx))", f"(rewrite-with (lemma fz_below) lr lhs () ({D('hslo')} {D('hnp0')}) refl)")
        A.have('heq', f"(= (ilen lc) {NP})", f"(rewrite-with (lemma int_eq_eq) lr lhs ((inst a (ilen lc)) (inst b {NP})) ({D('har')}) refl)")
        A.have('hband', "(= (iband_args (ipparams_of g) lc) lc)", f"(rewrite-with (lemma iband_id) lr lhs ((inst x fp) (inst ps psx)) ({D('hp64')} {D('heq')} {D('hlocs')}) refl)")
        A.have('hdiscz', f"(= (fp_disc slo {PSXz}) True)", f"(rewrite-with (lemma fz_disc) lr lhs () ({D('hdisc')} {D('hslo')} {D('hal')} {D('hnp0')}) refl)")
        A.have('hminz', f"(= (fp_min {FZg} (+ fp (* 8 {NP}))) True)", "(rewrite-with (lemma fz_min) lr lhs () () refl)")
        A.have('hwminz', f"(= (fp_wmin slo (+ fp (* 8 {NP})) {FZg}) True)", f"(rewrite-with (lemma fp_wmin_of_min) lr lhs ((inst lo (+ fp (* 8 {NP})))) ({D('hminz')}) refl)")
        A.have('hwminz2', f"(= (fp_wmin slo (+ fp (* 8 (ilen lc))) {FZg}) True)", "(steps ((rewrite (premise heq) lr lhs true ()) (rewrite (premise hwminz) lr lhs true ())) refl)")
        A.have('hlocs1', f"(= (fr_locs fp lc {PSXz}) True)", f"(rewrite-with (lemma fr_locs_app_wmin) lr lhs ((inst slo slo)) ({D('hlocs')} {D('hwminz2')} {D('hslo')}) refl)")
        A.have('hlocsz', f"(= (fr_locs (+ fp (* 8 {NP})) (izeros (- {NLg} {NP})) {PSXz}) True)", f"(rewrite-with (lemma fz_locs) lr lhs () ({D('hnp0')}) refl)")
        A.have('hne_eq', f"(= (izeros (- {NLg} {NP})) (izeros {NE}))", f"(chain (have hx (= (- {NLg} {NP}) {NE}) (by arith (list (list 1) (list 1)))) (steps ((rewrite (premise hx) lr lhs true ())) refl))")
        A.have('hlocsz2', f"(= (fr_locs (+ fp (* 8 (ilen lc))) (izeros {NE}) {PSXz}) True)", "(steps ((rewrite (premise heq) lr lhs true ()) (rewrite (premise hne_eq) rl lhs true ()) (rewrite (premise hlocsz) lr lhs true ())) refl)")
        A.have('hlocs_all', f"(= (fr_locs fp (iapp lc (izeros {NE})) {PSXz}) True)", f"(rewrite-with (lemma fr_locs_app_l) lr lhs () ({D('hlocs1')} {D('hlocsz2')}) refl)")
        A.have('hlocsL', f"(= (fr_locs fp {LCg} {PSXz}) True)", "(steps ((rewrite (premise hband) lr lhs true ()) (rewrite (premise hlocs_all) lr lhs true ())) refl)")
        A.have('hlenL', f"(= (le (ilen {LCg}) {NLg}) True)", f"(chain (have hz (= (ilen (izeros {NE})) {NE}) (rewrite-with (lemma ilen_zeros) lr lhs () ({D('hne0')}) refl)) (steps ((rewrite (premise hband) lr lhs true ()) (rewrite (lemma ilen_app) lr lhs true ()) (rewrite (premise hz) lr lhs true ()) (rewrite (premise heq) lr lhs true ())) (by arith (list 1))))")
        A.have('hctxz', f"(= (fe_ctx m mlo slo fp {NLg} 0 {LCg} {PSXz}) True)", f"(rewrite-with (lemma ctx_intro) lr lhs () ({D('hdiscz')} {D('hlocsL')} {D('hxlo')} {D('hmlo')} {D('hslo')} {D('hal')} {D('hlenL')} {D('hd0')} {D('hslo0')} {D('hhi')}) refl)")
        A.have('hob', f"(= (le (* 8 (+ {NLg} (+ 1 (ixf_sdep {BODYg})))) {OWNg}) True)", "(rewrite-with (lemma ixf_own_body) lr lhs () () refl)")
        A.have('hoal', f"(= (int_eq (mod {OWNg} 8) 0) True)", "(rewrite-with (lemma ixf_own_al) lr lhs () () refl)")
        A.have('homo', f"(= (le {OWNg} (ixf_maxown fs)) True)", f"(rewrite-with (lemma ixf_maxown_at) lr lhs ((inst k k)) ({D('hg')}) refl)")
        A.have('hfrb', f"(= (ixf_fr {NLg} {OWNg} (ixf_sdep {BODYg}) (ixf_maxown fs)) True)", f"(rewrite-with (lemma fr_intro) lr lhs () ({D('hob')} {D('hoal')} {D('homo')}) refl)")
        d0 = "(steps ((rewrite (premise hfz) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hb2) lr lhs true ())) refl)"
        IHB = f"(= (ixt_post (IqStmts {BODYg}) m mlo slo fp {NLg} lcb (fp_app (ipt_tr f2 fs mlo slo dmax (+ dep 1) fp {NLg} {OWNg} (IqStmts {BODYg}) {LCg} {IMx(PSXz)}) {PSXz})) True)"
        A.have('hih', IHB, f"(rewrite-with (hyp ih) lr lhs ((inst mem2 memb) (inst lc2 lcb) (inst dep (+ dep 1)) (inst nl {NLg}) (inst own {OWNg}) (inst lc {LCg})) ({d0} {D('hctxz')} (steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise hscbb) lr lhs true ())) refl) (steps ((unfold ixt_fr lhs) (reduce lhs) (unfold ixt_body lhs) (reduce lhs) (rewrite (premise hfrb) lr lhs true ())) refl) {D(4)}) refl)")
        A.have('hih2', f"(= (fe_ctx m mlo slo fp {NLg} 0 lcb {PSXb}) True)", "(steps ((rewrite (premise hih) rl rhs true ()) (unfold ixt_post rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfz) lr rhs true ())) refl)")
        A.have('hmemb0', f"(= (fp_mem mem0 (fbelow slo (fp_app (ipt_tr f2 fs mlo slo dmax (+ dep 1) fp {NLg} {OWNg} (IqStmts {BODYg}) {LCg} {IMx(PSXz)}) {PSXz}))) memb)", f"(rewrite-with (lemma ipt_mem) lr lhs ((inst lc2 lcb) (inst mem2 memb) (inst dep (+ dep 1)) (inst nl {NLg}) (inst own {OWNg}) (inst lc {LCg})) ({d0} {D('hslo')} {D('hnlg')} {D('hown0')}) refl)")
        A.have('hmemb', f"(= {IMb} memb)", "(steps ((rewrite (premise hmemb0) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfz) lr rhs true ())) refl)")
        A.have('hv2', f"(= (iexp {RESg} lcb {IMb} mlo slo) (Some v))", "(steps ((rewrite (premise hmemb) lr lhs true ()) (rewrite (premise hv) lr lhs true ())) refl)")
        A.have('hlocsb', f"(= (fr_locs fp lcb {PSXb}) True)", f"(rewrite-with (lemma ctx_locs) lr lhs ((inst m m) (inst mlo mlo) (inst slo slo) (inst nl {NLg}) (inst d 0)) ({D('hih2')}) refl)")
        A.have('hbandv', "(= (fe_inband v) True)", f"(rewrite-with (lemma fe_band) lr lhs ((inst lc lcb) (inst mem {IMb}) (inst mlo mlo) (inst msz slo) (inst fp fp) (inst psx {PSXb}) (inst e {RESg})) ({D('hv2')} {D('hlocsb')} {D('hcbr')}) refl)")
        A.have('hdiscb', f"(= (fp_disc slo {PSXb}) True)", f"(rewrite-with (lemma ctx_disc) lr lhs ((inst m m) (inst mlo mlo) (inst fp fp) (inst nl {NLg}) (inst d 0) (inst lc lcb)) ({D('hih2')}) refl)")
        A.have('hdiscr', f"(= (fp_disc slo (fp_app {TRrI} {PSXb})) True)", f"(rewrite-with (lemma fe_tr_disc) lr lhs ((inst e {RESg})) ({D('hdiscb')} {D('hslo')} {D('hal')} {D('hnld')}) refl)")
        A.have('hdiscr2', f"(= (fp_disc slo (fp_app {TRr} {PSXb})) True)", "(steps ((rewrite (premise hmemb) rl lhs true ()) (rewrite (premise hdiscr) lr lhs true ())) refl)")
        A.have('hlocsN', f"(= (fr_locs fp Nil (fp_app {TRr} {PSXb})) True)", "(steps ((unfold fr_locs lhs) (reduce lhs)) refl)")
        A.have('hlenN', "(= (le (ilen Nil) nl) True)", "(steps ((unfold ilen lhs) (reduce lhs) (rewrite (premise hnl0) lr lhs true ())) refl)")
        A.have('hctxN', f"(= (fe_ctx m mlo slo fp nl 0 Nil (fp_app {TRr} {PSXb})) True)", f"(rewrite-with (lemma ctx_intro) lr lhs () ({D('hdiscr2')} {D('hlocsN')} {D('hxlo')} {D('hmlo')} {D('hslo')} {D('hal')} {D('hlenN')} {D('hd0')} {D('hslo0')} {D('hhi')}) refl)")
        A.add(None, f"""(steps
{I}  ((unfold ipt_tr lhs) (reduce lhs) (unfold ipt_call lhs) (reduce lhs)
{I}   (rewrite (premise hg) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise har) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hlt) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hb2) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hv) lr lhs true ()) (reduce lhs)
{I}   (rewrite (lemma fp_app_assoc) lr lhs true ()) (rewrite (lemma fp_app_assoc) lr lhs true ())
{I}   (rewrite (premise hctxN) lr lhs true ()) (reduce lhs)
{I}   (rewrite (premise hlc) rl lhs true ())
{I}   (unfold ihd lhs) (reduce lhs)
{I}   (rewrite (premise hbandv) lr lhs true ()))
{I}  refl)""")
        return cf + "\n" + A.text()
    L['qcall'] = qcall_
    return L

def t2():
    prem = [f"(= {RUN('f', 'r', 'lc', IM)} {OUTN})", "(= (fe_ctx m mlo slo fp nl 0 lc psx) True)",
            "(= (ixf_scb (ixt_body r)) True)", "(= (ixt_fr r nl own (ixf_maxown fs)) True)", "(= (ixf_fnsok fs) True)"]
    L = t2_leaves()
    # the post-state of a statement request is the context at its locals: open it once per leaf
    L = {k: (lambda f: (lambda sl: "  (steps ((unfold ixt_post lhs) (reduce lhs)))\n" + f(sl)))(v) for k, v in L.items()}
    return dispatch("ipt_ctx", T2_VARS, prem, T2P('f', 'r', 'lc', IM, 'psx', 'lc2'), L, T2_SIDES)

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
    """ixf_stmt s = Some iss  ->  xil iss <= ixf_scost s"""
    def arm(ctor, binders, S, body):
        b = f" ({binders})" if binders else ""
        return f"(case {ctor}{b} (chain {cap('hst', f'(= s {S})')} {body}))"
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
       (steps ((rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (unfold ixf_ecost lhs)) (by arith (list 1 0 0 0 -1 0 0 -1)))))))"""
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
                     (steps ((rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (unfold ixf_ecost lhs)) (by arith (list 1 0 0 0 -1 0 0 0 0 -1)))))))))))))))"""
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
              (steps ((rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (unfold ixf_ecost lhs)) (by arith (list 1 0 0 0 0 0 0 -1 1)))))))))))"""
    def mem_store(word):
        STI = "(XMem (XMStore64 (AReg R10) RAX))" if word else "(XStore8 (AReg R10) (SReg RAX))"
        EM = f"(ix_app ia (ix_app (ixf_spill nl 0) (ix_app iv (ix_app (ixf_reload10 nl 0) (list {STI})))))"
        sl = Slots(['p0','hst','hia','hiv','hlena','hlenv','hsome','his','hxs','hxr','hx1','hx'])
        return f"""(case-on (ixf_exp nl 0 ae) Option
  ((case None (chain {cap('hia', '(= (ixf_exp nl 0 ae) None)')} (have hn (= None (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hia) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
   (case Some (ia)
     (chain
       {cap('hia', '(= (ixf_exp nl 0 ae) (Some ia))')}
       (case-on (ixf_exp nl 1 ve) Option
         ((case None (chain {cap('hiv', '(= (ixf_exp nl 1 ve) None)')} (have hn (= None (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hia) lr rhs true ()) (reduce rhs) (rewrite (premise hiv) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
          (case Some (iv)
            (chain
              {cap('hiv', '(= (ixf_exp nl 1 ve) (Some iv))')}
              (have hlena (= (xil ia) (ixf_elen ae)) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d 0) (inst e ae)) ({D('hia')}) refl))
              (have hlenv (= (xil iv) (ixf_elen ve)) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d 1) (inst e ve)) ({D('hiv')}) refl))
              (have hsome (= (Some {EM}) (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hia) lr rhs true ()) (reduce rhs) (rewrite (premise hiv) lr rhs true ()) (reduce rhs)) refl))
              (inject (premise hsome) (his))
              (have hxs (= (xil (ixf_spill nl 0)) 3) (steps ((compute lhs)) refl))
              (have hxr (= (xil (ixf_reload10 nl 0)) 3) (steps ((compute lhs)) refl))
              (have hx1 (= (xil (list {STI})) 1) (steps ((compute lhs)) refl))
              (have hx (= (xil iss) (+ (xil ia) (+ (xil (ixf_spill nl 0)) (+ (xil iv) (+ (xil (ixf_reload10 nl 0)) (xil (list {STI}))))))) (steps ((rewrite (premise his) rl lhs true ()) (rewrite (lemma xil_app) lr lhs true ()) (rewrite (lemma xil_app) lr lhs true ()) (rewrite (lemma xil_app) lr lhs true ()) (rewrite (lemma xil_app) lr lhs true ())) refl))
              (steps ((rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (unfold ixf_ecost lhs) (unfold ixf_ecost lhs)) (by arith {sl.cert({'hlena':-1,'hlenv':-1,'hxs':-1,'hxr':-1,'hx1':-1,'hx':-1})}))))))))))"""
    LDW = "(XMem (XMLoad64 RAX (AReg RAX)))"
    sl_lw = Slots(['p0','hst','hie','hlen','hsome','his','hxc','hx'])
    loadw = f"""(case-on (ixf_exp nl 0 ae) Option
  ((case None (chain {cap('hie', '(= (ixf_exp nl 0 ae) None)')} (have hn (= None (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
   (case Some (ie)
     (chain
       {cap('hie', '(= (ixf_exp nl 0 ae) (Some ie))')}
       (have hlen (= (xil ie) (ixf_elen ae)) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d 0) (inst e ae)) ({D('hie')}) refl))
       (have hsome (= (Some (ix_app ie (Cons {LDW} (ixf_st i)))) (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs)) refl))
       (inject (premise hsome) (his))
       (have hxc (= (xil (Cons {LDW} (ixf_st i))) 4) (steps ((compute lhs)) refl))
       (have hx (= (xil iss) (+ (xil ie) (xil (Cons {LDW} (ixf_st i))))) (steps ((rewrite (premise his) rl lhs true ()) (rewrite (lemma xil_app) lr lhs true ())) refl))
       (steps ((rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (unfold ixf_ecost lhs)) (by arith {sl_lw.cert({'hlen':-1,'hxc':-1,'hx':-1})}))))))"""
    fail = f"""(chain
  (have hsome (= (Some (Cons (XMovRI RDI (match fam (FOverflow 70) (FOom 71) (FStack 72))) (Cons (XCall fail_ix) Nil))) (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs)) refl))
  (inject (premise hsome) (his))
  (steps ((rewrite (premise his) rl lhs true ()) (rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (compute lhs)) refl))"""
    L3C = "(Cons (XBin XAdd R15 (SImm own)) (Cons (XCall k) (Cons (XBin XSub R15 (SImm own)) Nil)))"
    sl_call = Slots(['p0','hst','hia','hla','hsome','his','hx3','hx'])
    call = f"""(case-on (ixf_args nl own 0 args) Option
  ((case None (chain {cap('hia', '(= (ixf_args nl own 0 args) None)')} (have hn (= None (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hia) lr rhs true ()) (reduce rhs)) refl)) (absurd (premise hn))))
   (case Some (iargs)
     (chain
       {cap('hia', '(= (ixf_args nl own 0 args) (Some iargs))')}
       (have hla (= (le (xil iargs) (ixf_argcost args)) True) (rewrite-with (lemma fa_len) lr lhs ((inst j 0) (inst nl nl) (inst own own)) ({D('hia')}) refl))
       (have hsome (= (Some (ix_app iargs (ix_app {L3C} (ixf_st i)))) (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hia) lr rhs true ()) (reduce rhs)) refl))
       (inject (premise hsome) (his))
       (have hx3 (= (xil {L3C}) 3) (steps ((compute lhs)) refl))
       (have hx (= (xil iss) (+ (xil iargs) (+ (xil {L3C}) 3))) (steps ((rewrite (premise his) rl lhs true ()) (rewrite (lemma xil_app) lr lhs true ()) (rewrite (lemma xil_app) lr lhs true ()) (rewrite (lemma xil_st) lr lhs true ())) refl))
       (steps ((rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs)) (by arith {sl_call.cert({'hx':-1,'hla':1,'hx3':-1})}))))))"""
    unreach = f"""(chain
  (have hsome (= (Some (Cons (XMovRI RAX 18446744073709547520) (Cons (XMem (XMLoad64 RAX (AReg RAX))) Nil))) (Some iss)) (steps ((rewrite (premise 0) rl rhs true ()) (rewrite (premise hst) lr rhs true ()) (unfold ixf_stmt rhs) (reduce rhs)) refl))
  (inject (premise hsome) (his))
  (steps ((rewrite (premise his) rl lhs true ()) (rewrite (premise hst) lr lhs true ()) (unfold ixf_scost lhs) (reduce lhs) (compute lhs)) refl))"""
    return f"""(claim slen_cost
  (goal ((s IpStmt) (iss (List XInstr)) (nl Int) (own Int) (fail_ix Int))
    ((= (ixf_stmt nl own fail_ix s) (Some iss)))
    (= (le (xil iss) (ixf_scost s)) True))
  (case-on s IpStmt
    ({arm('IpSet', 'i e', '(IpSet i e)', set_)}
     {arm('IpStore', 'ae ve', '(IpStore ae ve)', mem_store(False))}
     {arm('IpIf', 'ce tb eb', '(IpIf ce tb eb)', if_)}
     {arm('IpWhile', 'ce b', '(IpWhile ce b)', wh)}
     {arm('IpCall', 'i k args', '(IpCall i k args)', call)}
     {arm('IpLoadW', 'i ae', '(IpLoadW i ae)', loadw)}
     {arm('IpStoreW', 'ae ve', '(IpStoreW ae ve)', mem_store(True))}
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
      "(mlo Int) (slo Int) (m XModule) (fp Int) (dep Int) (fs (List IpFn)) (dmax Int) (xfs (List XFunc)) "
      "(a0 Int) (rcx Int) (dx Int) (rbx Int) (rbp Int) (rsi Int) (di Int) (r8 Int) (r9 Int) (s10 Int) (s11 Int) (r12 Int) (r13 Int)")
NPREM = 20
SHIM = "(Cons (XMovRI RAX 60) (Cons XSyscall Nil))"
SHIMF = f"(Cons (MkXFunc 0 {SHIM}) Nil)"   # the exit shim as the table's last fn
# the discharges every same-frame sub-request shares (shim, mlo, room, the depth facts, the table)
def TAIL_DS(frd): return [D(9), D(10), frd, D(12), D(13), D(14), D(15), D(16), D(17), D(18), D(19)]
N64 = "18446744073709551616"
def XR(cc, ff, r, is_, rs, xm): return f"(ixt_run (xt {cc} (kf K {ff})) m {r} {is_} {rs} {xm})"
def MEMOF(ff, r, lc, im, psx): return f"(fp_mem mem0 (fp_app {TR(ff, r, lc, im)} {psx}))"
def CONCL(ff, r, lc, im, psx, out, rs, run): return f"(= {run} (ixt_expect {r} {out} {rs} {run} {MEMOF(ff, r, lc, im, psx)}))"
CK = "(+ c K)"
KF2 = "(kf K f2)"
IH_PINS = "(inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst mlo mlo) (inst slo slo) (inst fs fs) (inst dmax dmax) (inst xfs xfs)"
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
    A.have('hfr', f"(= (ixf_fr nl own (ixf_sdep {body}) (ixf_maxown fs)) True)", f"(steps ((rewrite (premise 11) rl rhs true ()){SUBS(subst)} (unfold ixt_fr rhs) (reduce rhs) (unfold ixt_body rhs) (reduce rhs)) refl)")
    A.have('hfrn', f"(= (le (* 8 (+ nl (+ 1 (ixf_sdep {body})))) own) True)", f"(rewrite-with (lemma fr_nl) lr lhs ((inst mo (ixf_maxown fs))) ({D('hfr')}) refl)")
    A.have('hsd0', f"(= (le 0 (ixf_sdep {body})) True)", "(rewrite-with (lemma ixf_sdep_nonneg) lr lhs () () refl)")
    A.have('hnl_', "(= (le (ilen lc) nl) True)", f"(rewrite-with (lemma ctx_nl) lr lhs ((inst m m) (inst mlo mlo) (inst slo slo) (inst fp fp) (inst d 0) (inst psx psx)) ({D(2)}) refl)")
    A.have('hnn_', "(= (le 0 (ilen lc)) True)", "(rewrite-with (lemma ilen_nonneg) lr lhs () () refl)")
    A.arith('hown0', "(= (le 0 own) True)", {'hfrn': 1, 'hnl_': 8, 'hnn_': 8, 'hsd0': 8})

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
                C.have('himp', f"(= {ST('(S f2)', S, 'lc', IM)} (Some (IpNorm lc2 mem2)))", f"(steps ((rewrite (premise ho) rl rhs true ()) (rewrite (premise 1) rl rhs true ()){SUBS(SS)} (unfold ipt_run rhs) (reduce rhs)) refl)")
                C.have('hcbe', "(= (ixf_cb e) True)", f"(rewrite-with (lemma scb_set_e) lr lhs ((inst i i) (inst t Nil)) ({D('hscb')}) refl)")
                C.have('hsde', "(= (le (ixf_dep e) (ixf_sdep (Cons (IpSet i e) Nil))) True)", "(rewrite-with (lemma sdep_set_e) lr lhs ((inst t Nil)) () refl)")
                C.arith('hdepe', "(= (le (+ fp (* 8 (+ nl (ixf_dep e)))) (xmemhi_of m)) True)", {'hsd': 1, 'hsde': 8})
                C.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
                pins = "(inst i i) (inst e e) (inst ie ie) (inst v v) (inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst lc lc) (inst lc2 lc2) (inst mem2 mem2) (inst mlo mlo) (inst slo slo) (inst f f2) (inst fs fs) (inst dmax dmax)"
                C.add(None, f"(rewrite-with (lemma fs_step_set) lr lhs ({pins}) ({D('hem')} {D('himp')} {D(2)} {D('hcbe')} {D('hdepe')} {D('hcs')} {D('hie')} {D('hv')}))")
                C.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs) (unfold ixt_expect rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs) (unfold ipt_stmt rhs) (reduce rhs)) refl)")
                return C.text()
            B.add(None, arm_set(leaf, B.sl, SS, 1, "(rewrite (premise ho) rl rhs true ()) "))
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
        return f"(chain (have hn (= {lhs} {OUTF}) (steps ((rewrite (premise ho) rl rhs true ()) (rewrite (premise 1) rl rhs true ()){SUBS(SS)} (unfold ipt_run rhs) (reduce rhs) (unfold ipstmt rhs) (reduce rhs){rw}) refl)) (inject (premise hn) (hx)) (absurd (premise hx)))"
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
                    F.arith('hc6', f"(= (le (+ (xil ic) 6) {CK}) True)", {'hcs': 1, 'hlen': -1, 'hec': 1, 'p6': 1})
                    F.arith('hc8', f"(= (le (+ (xil ic) 8) {CK}) True)", {'hcs': 1, 'hlen': -1, 'hec': 1, 'p6': 1})
                    F.arith('hc0', f"(= (le 0 {CK}) True)", {'p8': 1, 'p6': 1})
                    rsc = RSC(RUNc)
                    def branch(flag, ss, iss, ext, cc, step):
                        G = Arm(S_(F.sl, 'hcvf'), I)
                        G.have('hbr', f"(= {STS('f2', ss, 'lc', IM)} (Some out))", f"(steps ((rewrite (premise himp) rl rhs true ()) (unfold ipstmt rhs) (reduce rhs) (rewrite (premise hcv) lr rhs true ()) (reduce rhs) (rewrite (premise hcvf) lr rhs true ()) (reduce rhs)) refl)")
                        G.have('hcbb', f"(= (ixf_scb {ss}) True)", f"(rewrite-with (lemma scb_if_{ext}) lr lhs ((inst ce ce) (inst tb tb) (inst eb eb) (inst t Nil)) ({D('hscb')}) refl)")
                        G.have('hsdb', f"(= (le (ixf_sdep {ss}) (ixf_sdep (Cons (IpIf ce tb eb) Nil))) True)", f"(rewrite-with (lemma sdep_if_{ext}) lr lhs ((inst t Nil)) () refl)")
                        G.arith('hsdt', f"(= (le (+ fp (* 8 (+ nl (ixf_sdep {ss})))) (xmemhi_of m)) True)", {'hsd': 1, 'hsdb': 8})
                        G.have('hkb', f"(= (ixf_skok K {ss}) True)", f"(rewrite-with (lemma skok_if_{ext}) lr lhs ((inst ce ce) (inst tb tb) (inst eb eb) (inst t Nil)) ({D('hkok')}) refl)")
                        G.add(None, fr_sub_have('hfrb', f'(ixf_sdep {ss})', 'hsdb', '(Cons (IpIf ce tb eb) Nil)')); G.sl.add('hfrb')
                        G.arith('hcb0', f"(= (le 0 {cc}) True)", {'hcs': 1, 'hlen': -1, 'hec': 1, 'p6': 1})
                        ds = [emit_d('h' + iss), f"(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hbr) lr lhs true ())) refl)", D('hctx1'), body_d('hcbb'), body_d('hsdt'), body_d('hkb'), D(6),
                              f"(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hcb0) lr lhs true ())) refl)", D('hcb0')] + TAIL_DS(fr_d('hfrb'))
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
                  "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hwc) lr lhs true ())) refl)", D('hw0')] + TAIL_DS(fr_d('hfr'))
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

COMMON_PINS = "(inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst lc lc) (inst lc2 lc2) (inst mem2 mem2) (inst mlo mlo) (inst slo slo) (inst f f2) (inst fs fs) (inst dmax dmax)"
def m_arm_mem(sl, kind):
    """IpStore / IpStoreW / IpLoadW: the engine decomposition (arm_store / arm_loadw) then the step lemma"""
    I = "    "
    word = kind == 'storew'
    loadw = kind == 'loadw'
    S = "(IpLoadW i ae)" if loadw else ("(IpStoreW ae ve)" if word else "(IpStore ae ve)")
    lemma = f"fs_step_{kind}"
    sub = SS + ['ho']
    def leaf(sl2):
        C = Arm(sl2, I + "          ")
        C.have('himp', f"(= {ST('(S f2)', S, 'lc', IM)} (Some (IpNorm lc2 mem2)))", f"(steps ((rewrite (premise ho) rl rhs true ()) (rewrite (premise 1) rl rhs true ()){SUBS(SS)} (unfold ipt_run rhs) (reduce rhs)) refl)")
        if loadw:
            C.have('hcba', "(= (ixf_cb ae) True)", f"(rewrite-with (lemma scb_loadw_a) lr lhs ((inst i i) (inst t Nil)) ({D('hscb')}) refl)")
            C.have('hsde', f"(= (le (ixf_dep ae) (ixf_sdep (Cons {S} Nil))) True)", "(rewrite-with (lemma sdep_loadw) lr lhs ((inst t Nil)) () refl)")
            C.arith('hdepe', "(= (le (+ fp (* 8 (+ nl (ixf_dep ae)))) (xmemhi_of m)) True)", {'hsd': 1, 'hsde': 8})
            pins = "(inst i i) (inst ae ae) (inst ie ie) (inst ad ad) " + COMMON_PINS
            ds = [D('hem'), D('himp'), D(2), D('hcba'), D('hdepe'), D('hcs'), D('hie'), D('hv')]
        else:
            C.have('hcba', "(= (ixf_cb ae) True)", f"(rewrite-with (lemma scb_{kind}_a) lr lhs ((inst ve ve) (inst t Nil)) ({D('hscb')}) refl)")
            C.have('hcbv', "(= (ixf_cb ve) True)", f"(rewrite-with (lemma scb_{kind}_v) lr lhs ((inst ae ae) (inst t Nil)) ({D('hscb')}) refl)")
            C.have('hsde', f"(= (le (+ 1 (imax2 (ixf_dep ae) (ixf_dep ve))) (ixf_sdep (Cons {S} Nil))) True)", f"(rewrite-with (lemma sdep_{kind}) lr lhs ((inst t Nil)) () refl)")
            C.arith('hdepe', "(= (le (+ fp (* 8 (+ nl (+ 1 (imax2 (ixf_dep ae) (ixf_dep ve)))))) (xmemhi_of m)) True)", {'hsd': 1, 'hsde': 8})
            pins = "(inst ae ae) (inst ve ve) (inst ia ia) (inst iv iv) (inst ad ad) (inst vv vv) " + COMMON_PINS
            ds = [D('hem'), D('himp'), D(2), D('hcba'), D('hcbv'), D('hdepe'), D('hcs'), D('hia'), D('hv'), D('hiv'), D('hvv')]
        C.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
        C.add(None, f"(rewrite-with (lemma {lemma}) lr lhs ({pins}) ({' '.join(ds)}))")
        C.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs) (unfold ixt_expect rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs) (unfold ipt_stmt rhs) (reduce rhs)) refl)")
        return C.text()
    PRE = "(rewrite (premise ho) rl rhs true ()) "
    def norm():
        A = Arm(S_(sl, 'ho'), I)
        A.add(None, "(steps ((rewrite (premise ho) lr rhs true ())))")
        A.have('hem', f"(= (ixf_stmt nl own fail_ix {S}) (Some is))", f"(steps ((rewrite (premise 0) rl rhs true ()){SUBS(SS)} (unfold ixt_emit rhs) (reduce rhs)) refl)")
        if loadw:
            def some():
                B = Arm(S_(A.sl, 'hie'), I)
                B.add(None, arm_loadw(leaf, B.sl, SS, 1, PRE))
                return B.text()
            A.add(None, case_opt("(ixf_exp nl 0 ae)", "ie", m_absurd_emit(A, None, "ixf_stmt", [RV('hie')]), some))
        else:
            def some_a():
                B = Arm(S_(A.sl, 'hia'), I)
                def some_v():
                    C0 = Arm(S_(B.sl, 'hiv'), I)
                    C0.add(None, arm_store(leaf, C0.sl, SS, 1, PRE, word))
                    return C0.text()
                B.add(None, case_opt("(ixf_exp nl 1 ve)", "iv", m_absurd_emit(B, None, "ixf_stmt", [RV('hia'), RV('hiv')]), some_v))
                return B.text()
            A.add(None, case_opt("(ixf_exp nl 0 ae)", "ia", m_absurd_emit(A, None, "ixf_stmt", [RV('hia')]), some_a))
        return A.text()
    def failed():
        A = Arm(S_(sl, 'ho'), I)
        A.add(None, arm_loadw_abs(A.sl, sub) if loadw else arm_store_abs(A.sl, sub, word))
        return A.text()
    return f"""(case-on out IpOut
  ((case IpNorm (lc2 mem2) (chain {cap('ho', '(= out (IpNorm lc2 mem2))')}
{norm()}))
   (case IpTrap (chain {cap('ho', '(= out IpTrap)')} (steps ((rewrite (premise ho) lr rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)))
   (case IpFailed (fam) (chain {cap('ho', '(= out (IpFailed fam))')}
{failed()}))))"""

def m_arm_store(sl): return m_arm_mem(sl, 'store')
def m_arm_storew(sl): return m_arm_mem(sl, 'storew')
def m_arm_loadw(sl): return m_arm_mem(sl, 'loadw')

def abs_leg(rewrites, lhs):
    """a leg of the engine against the failed outcome: the outcome is a trap or normal"""
    OUTF = "(Some (IpFailed fam))"
    rw = "".join(f" {r} (reduce rhs)" for r in rewrites)
    return f"(chain (have hn (= {lhs} {OUTF}) (steps ((rewrite (premise ho) rl rhs true ()) (rewrite (premise 1) rl rhs true ()){SUBS(SS)} (unfold ipt_run rhs) (reduce rhs) (unfold ipstmt rhs) (reduce rhs){rw}) refl)) (inject (premise hn) (hx)) (absurd (premise hx)))"

def arm_store_abs(sl, subst, word=False):
    G1 = "(le (+ ad 8) slo)" if word else "(lt ad slo)"
    MEM2 = f"(store_le (iw8) {IM} ad vv)" if word else f"(mem_set {IM} ad vv)"
    AE = f"(iexp ae lc {IM} mlo slo)"; VE = f"(iexp ve lc {IM} mlo slo)"; T = '(Some IpTrap)'
    return f"""(case-on {AE} Option
  ((case None (chain {cap('hv', f'(= {AE} None)')} {abs_leg([RV('hv')], T)}))
   (case Some (ad)
     (chain
       {cap('hv', f'(= {AE} (Some ad))')}
       (case-on (le mlo ad) Bool
         ((case False (chain {cap('hg0', '(= (le mlo ad) False)')} {abs_leg([RV('hv'), RV('hg0')], T)}))
          (case True
            (chain
              {cap('hg0', '(= (le mlo ad) True)')}
              (case-on {G1} Bool
                ((case False (chain {cap('hg1', f'(= {G1} False)')} {abs_leg([RV('hv'), RV('hg0'), RV('hg1')], T)}))
                 (case True
                   (chain
                     {cap('hg1', f'(= {G1} True)')}
                     (case-on {VE} Option
                       ((case None (chain {cap('hvv', f'(= {VE} None)')} {abs_leg([RV('hv'), RV('hg0'), RV('hg1'), RV('hvv')], T)}))
                        (case Some (vv) (chain {cap('hvv', f'(= {VE} (Some vv))')} {abs_leg([RV('hv'), RV('hg0'), RV('hg1'), RV('hvv')], f'(Some (IpNorm lc {MEM2}))')}))))))))))))))))"""

def arm_loadw_abs(sl, subst):
    AE = f"(iexp ae lc {IM} mlo slo)"; V = VALW(); T = '(Some IpTrap)'
    return f"""(case-on {AE} Option
  ((case None (chain {cap('hv', f'(= {AE} None)')} {abs_leg([RV('hv')], T)}))
   (case Some (ad)
     (chain
       {cap('hv', f'(= {AE} (Some ad))')}
       (case-on (le mlo ad) Bool
         ((case False (chain {cap('hg0', '(= (le mlo ad) False)')} {abs_leg([RV('hv'), RV('hg0')], T)}))
          (case True
            (chain
              {cap('hg0', '(= (le mlo ad) True)')}
              (case-on (le (+ ad 8) slo) Bool
                ((case False (chain {cap('hg1', '(= (le (+ ad 8) slo) False)')} {abs_leg([RV('hv'), RV('hg0'), RV('hg1')], T)}))
                 (case True
                   (chain
                     {cap('hg1', '(= (le (+ ad 8) slo) True)')}
                     (case-on (ilset lc i {V}) Option
                       ((case None (chain {cap('hset', f'(= (ilset lc i {V}) None)')} {abs_leg([RV('hv'), RV('hg0'), RV('hg1'), RV('hset')], T)}))
                        (case Some (lcs) (chain {cap('hset', f'(= (ilset lc i {V}) (Some lcs))')} {abs_leg([RV('hv'), RV('hg0'), RV('hg1'), RV('hset')], f'(Some (IpNorm lcs {IM}))')}))))))))))))))))"""

def call_facts(A, body):
    """the call statement's shared facts (after ctx_facts + m_fuel): the argument predicates, the frame bundle"""
    A.have('hcbs', "(= (ixf_cbs args) True)", f"(rewrite-with (lemma scb_call_args) lr lhs ((inst i i) (inst k k) (inst t Nil)) ({D('hscb')}) refl)")
    A.have('hsdc', f"(= (le (ixf_deps args) (ixf_sdep {body})) True)", "(rewrite-with (lemma sdep_call) lr lhs ((inst t Nil)) () refl)")
    A.have('hfra', "(= (int_eq (mod own 8) 0) True)", f"(rewrite-with (lemma fr_al) lr lhs ((inst nl nl) (inst sd (ixf_sdep {body})) (inst mo (ixf_maxown fs))) ({D('hfr')}) refl)")
    A.arith('hj0', "(= (le 0 0) True)", {})
    A.arith('hdo', "(= (le (* 8 (+ nl (ixf_deps args))) own) True)", {'hfrn': 1, 'hsdc': 8})
    A.arith('hnlo', "(= (le (* 8 nl) own) True)", {'hfrn': 1, 'hsd0': 8})
    A.arith('hwin', "(= (le (+ fp (* 8 (+ nl (ixf_deps args)))) (xmemhi_of m)) True)", {'hsd': 1, 'hsdc': 8})
    A.have('hM0', "(= (le 0 (ixf_maxown fs)) True)", "(rewrite-with (lemma ixf_maxown_nonneg) lr lhs () () refl)")
    A.have('hnext', "(= (le (+ (+ fp own) (ixf_maxown fs)) (xmemhi_of m)) True)", f"(rewrite-with (lemma room_next) lr lhs ((inst dep dep) (inst dmax dmax)) ({D(12)} {D(14)} {D('hM0')}) refl)")
    A.arith('hroomc', "(= (le (+ (+ (+ fp own) 0) (* (- (+ dmax 1) dep) (ixf_maxown fs))) (xmemhi_of m)) True)", {'p12': 1})
    A.have('hla', "(= (le (xil iargs) (ixf_argcost args)) True)", f"(rewrite-with (lemma fa_len) lr lhs ((inst j 0) (inst nl nl) (inst own own)) ({D('hia')}) refl)")
    A.arith('hcs12', "(= (le (+ (ixf_argcost args) 12) (+ c K)) True)", {'hcs': 1, 'p6': 1})

def callee_ctx(A, TA, PSXa):
    """the callee's entry context at fp + own over the argument patches (fa_disc / fa_slots / fa_below)"""
    A.have('hdisca', f"(= (fp_disc slo {PSXa}) True)", f"(rewrite-with (lemma fa_disc) lr lhs ((inst j 0) (inst args args)) ({D('hdisc')} {D('hslo')} {D('hal')} {D('hfra')} {D('hnl0')} {D('hown0')} {D('hj0')}) refl)")
    A.have('hfb', f"(= (fbelow slo {PSXa}) (fbelow slo psx))", f"(rewrite-with (lemma fa_below) lr lhs ((inst j 0) (inst args args)) ({D('hnl0')} {D('hown0')} {D('hj0')} {D('hslo')}) refl)")
    A.have('hslots0', f"(= (fr_locs (+ fp (+ own (* 8 0))) vs {PSXa}) True)", f"(rewrite-with (lemma fa_slots) lr lhs ((inst j 0) (inst args args) (inst vs vs) (inst q psx) (inst psx psx)) ({D('hvs')} {D('hlocs')} {D('hcbs')} {D('hdo')} {D('hj0')}) refl)")
    A.have('hfo', "(= (+ fp (+ own (* 8 0))) (+ fp own))", "(by arith (list (list 1) (list 1)))")
    A.have('hslots', f"(= (fr_locs (+ fp own) vs {PSXa}) True)", "(steps ((rewrite (premise hfo) rl lhs true ()) (rewrite (premise hslots0) lr lhs true ())) refl)")
    A.arith('hslo2', "(= (le slo (+ fp own)) True)", {'hslo': 1, 'hown0': 1})
    A.have('hal2', "(= (int_eq (mod (- (+ fp own) slo) 8) 0) True)", f"(rewrite-with (lemma al_add) lr lhs () ({D('hal')} {D('hfra')}) refl)")
    A.have('hlenv', "(= (le (ilen vs) (ilen vs)) True)", "(by arith (list 1))")
    A.have('hctxc', f"(= (fe_ctx m mlo slo (+ fp own) (ilen vs) 0 vs {PSXa}) True)", f"(rewrite-with (lemma ctx_intro) lr lhs () ({D('hdisca')} {D('hslots')} {D('hxlo')} {D('hmlo')} {D('hslo2')} {D('hal2')} {D('hlenv')} {D('hd0')} {D('hslo0')} {D('hhi')}) refl)")
    A.have('hvsl', "(= (ilen vs) (ixf_nargs args))", f"(rewrite-with (lemma ipexps_len) lr lhs ((inst args args) (inst lc lc) (inst mem {IM}) (inst mlo mlo) (inst msz slo)) ({D('hvs')}) refl)")

def call_ih(A, out, TA, PSXa, RSc, CC):
    """the call request's IH at f2 (its locals = the arguments, its frame fp + own, own = 0): hih, hihu, hih2 (the run as xeval_call)"""
    TCa = f"(ipt_tr f2 fs mlo slo dmax dep (+ fp own) (ilen vs) 0 (IqCall k) vs {IMx(PSXa)})"
    RUNq = f"(ixt_run (xt {CC} {KF2}) m (IqCall k) Nil {RSc} (fp_mem mem0 {PSXa}))"
    MEMq = f"(fp_mem mem0 (fp_app {TCa} {PSXa}))"
    A.have('hfit', "(= (le (* 8 (ilen vs)) (ixf_maxown fs)) True)", f"(rewrite-with (lemma ipcall_fit) lr lhs ((inst r RT) (inst f f2) (inst d dep) (inst mem {IM}) (inst mlo mlo) (inst msz slo) (inst dmax dmax) (inst k k)) ({D('hcall2')} (steps ((unfold ipr_trap lhs) (reduce lhs)) refl)) refl)")
    A.arith('hargwin', "(= (le (+ fp (+ own (* 8 (ixf_nargs args)))) (xmemhi_of m)) True)", {'hnext': 1, 'hfit': 1, 'hvsl': 8})
    A.arith('hcc0', f"(= (le 0 {CC}) True)", {'hcs': 1, 'hla': 1, 'p6': 1})
    A.arith('hwinc', "(= (le (+ (+ fp own) (* 8 (+ (ilen vs) 0))) (xmemhi_of m)) True)", {'hnext': 1, 'hfit': 1})
    ds = ["(steps ((unfold ixt_emit lhs) (reduce lhs)) refl)",
          "(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hcall2) lr lhs true ()) (unfold ipout_of_ret lhs) (reduce lhs)) refl)",
          D('hctxc'),
          "(steps ((unfold ixt_body lhs) (reduce lhs) (unfold ixf_scb lhs) (reduce lhs)) refl)",
          "(steps ((unfold ixt_body lhs) (reduce lhs) (unfold ixf_sdep lhs) (reduce lhs) (rewrite (premise hwinc) lr lhs true ())) refl)",
          "(steps ((unfold ixt_body lhs) (reduce lhs) (unfold ixf_skok lhs) (reduce lhs)) refl)",
          D(6),
          "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hcc0) lr lhs true ())) refl)",
          D('hcc0'), D(9), D(10),
          "(steps ((unfold ixt_fr lhs) (reduce lhs) (rewrite (premise hj0) lr lhs true ())) refl)",
          D('hroomc'), D(13), D(14), D(15), D(16), D(17), D(18), D(19)]
    A.have('hih', f"(= {RUNq} (ixt_expect (IqCall k) {out} {RSc} {RUNq} {MEMq}))",
           f"(rewrite-with (hyp ih) lr lhs ((inst out {out}) (inst lc vs) (inst nl (ilen vs)) (inst own 0) (inst fail_ix fail_ix) (inst mlo mlo) (inst slo slo) (inst fs fs) (inst dmax dmax) (inst xfs xfs)) ({' '.join(ds)}) refl)")
    RUNC = f"(xeval_call (xt {CC} {KF2}) m k {RSc} (fp_mem mem0 {PSXa}))"
    A.have('hihu', f"(= {RUNq} {RUNC})", "(steps ((unfold ixt_run lhs) (reduce lhs)) refl)")
    return RUNC, MEMq

def m_arm_call(sl):
    I = "    "
    S = "(IpCall i k args)"; BODY = f"(Cons {S} Nil)"
    A = Arm(sl, I)
    A.have('hem', f"(= (ixf_stmt nl own fail_ix {S}) (Some is))", f"(steps ((rewrite (premise 0) rl rhs true ()){SUBS(SS)} (unfold ixt_emit rhs) (reduce rhs)) refl)")
    def ia_some():
        B = Arm(S_(A.sl, 'hia'), I)
        B.have('himp', f"(= {ST('(S f2)', S, 'lc', IM)} (Some out))", f"(steps ((rewrite (premise 1) rl rhs true ()){SUBS(SS)} (unfold ipt_run rhs) (reduce rhs)) refl)")
        cf, B.sl = ctx_facts(B.sl, 'p2', I); B.lines.append(cf)
        m_fuel(B)
        call_facts(B, BODY)
        TA = TA0(IM); PSXa = f"(fp_app {TA} psx)"
        RUNA = f"(xeval_seq (xt {CK} {KF2}) m iargs {RS} {XM})"
        RSc = f"(MkRegs (rax_of (xo_regs {RUNA})) rcx (rdx_of (xo_regs {RUNA})) rbx rbp rsi di r8 r9 (r10_of (xo_regs {RUNA})) (r11_of (xo_regs {RUNA})) r12 r13 dep (+ fp own))"
        CC = f"(- (- (- {CK} (xil iargs)) 1) 2)"
        TC = TRC('f2', '(+ fp own)', 'k', 'vs', IM)
        PRE = "(rewrite (premise ho) rl rhs true ()) "
        def norm():
            C = Arm(S_(B.sl, 'ho'), I)
            C.add(None, "(steps ((rewrite (premise ho) lr rhs true ())))")
            def leaf(sl2):
                G = Arm(sl2, I + "      ")
                callee_ctx(G, TA, PSXa)
                RUNC, MEMq = call_ih(G, '(IpNorm (Cons v Nil) memc)', TA, PSXa, RSc, CC)
                G.lines = [l.replace("(inst r RT)", "(inst r (IpRv v memc))") for l in G.lines]
                MEMC = f"(fp_mem mem0 (fp_app {TC} {PSXa}))"
                G.have('hih2', f"(= {RUNC} (fe_out v {RSc} {RUNC} {MEMC}))", "(steps ((rewrite (premise hihu) rl both true ()) (rewrite (premise hih) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs) (unfold ihd lhs) (reduce lhs) (unfold ipt_tr lhs) (reduce lhs) (rewrite (premise hfb) lr lhs true ())) refl)")
                G.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs)))")
                pins = f"(inst i i) (inst k k) (inst args args) (inst iargs iargs) (inst vs vs) (inst v v) (inst tc {TC}) (inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst lc lc) (inst lc2 lcs) (inst mlo mlo) (inst slo slo)"
                G.add(None, f"(rewrite-with (lemma fs_step_call) lr lhs ({pins}) ({D('hem')} {D('hia')} {D('hvs')} {D(2)} {D('hcbs')} {D('hwin')} {D('hdo')} {D('hfra')} {D('hargwin')} {D('hcs12')} {D('hset')} {D('hih2')}))")
                G.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs) (unfold ixt_expect rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs) (unfold ipt_stmt rhs) (reduce rhs) (rewrite (premise hvs) lr rhs true ()) (reduce rhs) (rewrite (premise hcall2) lr rhs true ()) (reduce rhs) (unfold fp_app rhs) (reduce rhs) (rewrite (lemma fp_app_assoc) lr rhs true ())) refl)")
                return G.text()
            C.add(None, arm_call(leaf, C.sl, SS, 1, PRE))
            return C.text()
        def failed():
            C = Arm(S_(B.sl, 'ho'), I)
            C.add(None, "(steps ((rewrite (premise ho) lr rhs true ())))")
            def leaf(sl2):
                G = Arm(sl2, I + "      ")
                callee_ctx(G, TA, PSXa)
                RUNC, MEMq = call_ih(G, '(IpFailed fam2)', TA, PSXa, RSc, CC)
                G.lines = [l.replace("(inst r RT)", "(inst r (IpRfailed fam2))") for l in G.lines]
                G.have('hih2', f"(= {RUNC} (Some XTrap))", "(steps ((rewrite (premise hihu) rl lhs true ()) (rewrite (premise hih) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs)) refl)")
                G.add(None, "(steps ((unfold ixt_run lhs) (reduce lhs) (unfold ixt_expect rhs) (reduce rhs)))")
                pins = "(inst i i) (inst k k) (inst args args) (inst iargs iargs) (inst vs vs) (inst nl nl) (inst own own) (inst fail_ix fail_ix) (inst lc lc) (inst mlo mlo) (inst slo slo)"
                G.add(None, f"(rewrite-with (lemma fs_step_call_fail) lr lhs ({pins}) ({D('hem')} {D('hia')} {D('hvs')} {D(2)} {D('hcbs')} {D('hwin')} {D('hdo')} {D('hfra')} {D('hargwin')} {D('hcs12')} {D('hih2')}))")
                G.add(None, "refl")
                return G.text()
            C.add(None, arm_call_fail(leaf, C.sl, SS, 1, PRE))
            return C.text()
        B.add(None, f"""(case-on out IpOut
  ((case IpNorm (lc2 mem2) (chain {cap('ho', '(= out (IpNorm lc2 mem2))')}
{norm()}))
   (case IpTrap (chain {cap('ho', '(= out IpTrap)')} (steps ((rewrite (premise ho) lr rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl)))
   (case IpFailed (fam) (chain {cap('ho', '(= out (IpFailed fam))')}
{failed()}))))""")
        return B.text()
    A.add(None, case_opt("(ixf_args nl own 0 args)", "iargs", m_absurd_emit(A, None, "ixf_stmt", [RV('hiargs')]), ia_some))
    return A.text().replace("hiargs", "hia")

def m_arm_qcall(sl):
    """the call request at (S f2): the table, the callee's shape, the entry context, the body's IH, the result"""
    I = "    "
    NP = "(ikn (ipparams_of g))"; NE = "(ikn (ipextra_of g))"
    A = Arm(sl, I)
    A.have('himp', f"(= {CALL('(S f2)', 'k', 'lc', IM)} (Some rt0))", "(steps ((rewrite (premise hrt0) lr rhs true ())) refl)") if False else None
    # the engine at (S f2): every leg names the outcome by a have over premise 1
    def run_is(lhs, rws):
        rw = "".join(f" {r} (reduce rhs)" for r in rws)
        return f"(have hout (= {lhs} (Some out)) (steps ((rewrite (premise 1) rl rhs true ()){SUBS(SC)} (unfold ipt_run rhs) (reduce rhs) (unfold ipcall rhs) (reduce rhs){rw} (unfold ipout_of_ret rhs) (reduce rhs)) refl))"
    def trap_leg(rws):
        return f"(chain {run_is('(Some IpTrap)', rws)} (inject (premise hout) (ho)) (steps ((rewrite (premise ho) rl rhs true ()) (unfold ixt_expect rhs) (reduce rhs)) refl))"
    BODYX = "(ix_app (ixf_enter dmax fail_ix) (ix_app (ixf_zero NP NLg) (Cons (XBlock ib) (ix_app ir (ixf_leave)))))".replace("NP", NP).replace("NLg", NLg)
    def some_g():
        B = Arm(S_(A.sl, 'hg'), I)
        B.have('hfnok', "(= (ixf_fnok g) True)", f"(rewrite-with (lemma fnsok_at) lr lhs ((inst k k) (inst fs fs)) ({D('hg')} {D(18)}) refl)")
        B.have('hp64', "(= (ixf_p64 (ipparams_of g)) True)", f"(rewrite-with (lemma fnok_p64) lr lhs () ({D('hfnok')}) refl)")
        B.have('hscbb', f"(= (ixf_scb {BODYg}) True)", f"(rewrite-with (lemma fnok_scb) lr lhs () ({D('hfnok')}) refl)")
        B.have('hcbr', f"(= (ixf_cb {RESg}) True)", f"(rewrite-with (lemma fnok_cb) lr lhs () ({D('hfnok')}) refl)")
        B.have('hfnkok', "(= (ixf_fnkok K g) True)", f"(rewrite-with (lemma fnskok_at) lr lhs ((inst k k) (inst fs fs)) ({D('hg')} {D(19)}) refl)")
        B.have('hfcost', "(= (le (ixf_fcost g) K) True)", f"(rewrite-with (lemma fnkok_cost) lr lhs () ({D('hfnkok')}) refl)")
        B.have('hskokb', f"(= (ixf_skok K {BODYg}) True)", f"(rewrite-with (lemma fnkok_skok) lr lhs () ({D('hfnkok')}) refl)")
        B.have('hfc', f"(= (ixf_fcost g) (+ (* 4 {NE}) (+ (ixf_ecost {RESg}) 8)))", "(steps ((unfold ixf_fcost lhs)) refl)")
        B.have('hmo', f"(= (le {OWNg} (ixf_maxown fs)) True)", f"(rewrite-with (lemma ixf_maxown_at) lr lhs ((inst k k)) ({D('hg')}) refl)")
        B.have('hM0', "(= (le 0 (ixf_maxown fs)) True)", "(rewrite-with (lemma ixf_maxown_nonneg) lr lhs () () refl)")
        B.have('hxsome', "(= (xf_some (ixf_fn dmax fail_ix g)) True)", f"(rewrite-with (lemma ixf_fns_some) lr lhs ((inst k k) (inst xfs xfs) (inst fs fs)) ({D('hg')} {D(16)}) refl)")
        B.have('htbl', f"(= (xfunc_at (xf_app xfs {SHIMF}) k) (ixf_fn dmax fail_ix g))", f"(rewrite-with (lemma ixf_tbl_at) lr lhs ((inst fs fs) (inst tl {SHIMF}) (inst g g) (inst dmax dmax) (inst fail_ix fail_ix)) ({D('hg')} {D(16)}) refl)")
        B.have('htbl2', "(= (xfunc_at (xfuncs_of m) k) (ixf_fn dmax fail_ix g))", "(steps ((rewrite (premise 17) lr lhs true ()) (rewrite (premise htbl) lr lhs true ())) refl)")
        B.have('hnp0', f"(= (le 0 {NP}) True)", "(rewrite-with (lemma ikn_nonneg) lr lhs () () refl)")
        B.have('hne0', f"(= (le 0 {NE}) True)", "(rewrite-with (lemma ikn_nonneg) lr lhs () () refl)")
        B.arith('hnlg', f"(= (le 0 {NLg}) True)", {'hnp0': 1, 'hne0': 1})
        B.arith('hnld', f"(= (le 0 (+ {NLg} 0)) True)", {'hnp0': 1, 'hne0': 1})
        B.arith('hnpnl', f"(= (le {NP} {NLg}) True)", {'hne0': 1})
        B.have('hownz', f"(= (le 0 {OWNg}) True)", "(rewrite-with (lemma ixf_own_nonneg) lr lhs () () refl)")
        B.have('hob', f"(= (le (* 8 (+ {NLg} (+ 1 (ixf_sdep {BODYg})))) {OWNg}) True)", "(rewrite-with (lemma ixf_own_body) lr lhs () () refl)")
        B.have('hor', f"(= (le (* 8 (+ {NLg} (+ 1 (ixf_dep {RESg})))) {OWNg}) True)", "(rewrite-with (lemma ixf_own_res) lr lhs () () refl)")
        B.have('hoal', f"(= (int_eq (mod {OWNg} 8) 0) True)", "(rewrite-with (lemma ixf_own_al) lr lhs () () refl)")
        B.have('hsdb0', f"(= (le 0 (ixf_sdep {BODYg})) True)", "(rewrite-with (lemma ixf_sdep_nonneg) lr lhs () () refl)")
        B.have('hfrb', f"(= (ixf_fr {NLg} {OWNg} (ixf_sdep {BODYg}) (ixf_maxown fs)) True)", f"(rewrite-with (lemma fr_intro) lr lhs () ({D('hob')} {D('hoal')} {D('hmo')}) refl)")
        B.have('hel', f"(= (le 0 (ixf_elen {RESg})) True)", "(rewrite-with (lemma ixf_elen_nonneg) lr lhs () () refl)")
        B.have('hec', f"(= (ixf_ecost {RESg}) (+ (ixf_elen {RESg}) 5))", "(steps ((unfold ixf_ecost lhs)) refl)")
        B.arith('hdm0', "(= (le 0 dmax) True)", {'p13': 1, 'p14': 1})
        B.arith('hdmN', f"(= (lt dmax {N64}) True)", {'p15': 1})
        def fn_none():
            return f"(chain (have hn (= (xf_some None) True) (steps ((rewrite (premise hxsome) rl rhs true ()) (rewrite (premise hxf) lr rhs true ())) refl)) (have hff (= False True) (steps ((rewrite (premise hn) rl rhs true ()) (unfold xf_some rhs) (reduce rhs)) refl)) (absurd (premise hff)))"
        def fn_some():
            C = Arm(S_(B.sl, 'hxf'), I)
            def emit_none(rws):
                rw = "".join(f" {r} (reduce rhs)" for r in rws)
                return f"(chain (have hn (= None (Some xf)) (steps ((rewrite (premise hxf) rl rhs true ()) (unfold ixf_fn rhs) (reduce rhs) (rewrite (premise hp64) lr rhs true ()) (reduce rhs){rw}) refl)) (absurd (premise hn)))"
            def ib_some():
                E = Arm(S_(C.sl, 'hib'), I)
                def ir_some():
                    F = Arm(S_(E.sl, 'hir'), I)
                    F.have('hsome', f"(= (Some (MkXFunc 0 {BODYX})) (Some xf))", f"(steps ((rewrite (premise hxf) rl rhs true ()) (unfold ixf_fn rhs) (reduce rhs) (rewrite (premise hp64) lr rhs true ()) (reduce rhs) (rewrite (premise hib) lr rhs true ()) (reduce rhs) (rewrite (premise hir) lr rhs true ()) (reduce rhs)) refl)")
                    F.add('hxfe', "(inject (premise hsome) (hxfe))")
                    F.have('hfat', f"(= (xfunc_at (xfuncs_of m) k) (Some (MkXFunc 0 {BODYX})))", "(steps ((rewrite (premise htbl2) lr lhs true ()) (rewrite (premise hxf) lr lhs true ()) (rewrite (premise hxfe) rl lhs true ())) refl)")
                    F.have('hlen', f"(= (xil ir) (ixf_elen {RESg}))", f"(rewrite-with (lemma fe_len) lr lhs ((inst nl {NLg}) (inst d 0) (inst e {RESg})) ({D('hir')}) refl)")
                    cf, F.sl = ctx_facts(F.sl, 'p2', I); F.lines.append(cf)
                    m_fuel(F)
                    F.arith('hcost12', f"(= (le (+ (* 4 (- {NLg} {NP})) (+ (ixf_ecost {RESg}) 8)) {CK}) True)", {'hfcost': 1, 'hfc': 1, 'p8': 1})
                    F.arith('hfp0', "(= (le 0 fp) True)", {'hslo0': 1, 'hslo': 1})
                    F.arith('hxlofp', "(= (le (xmemlo_of m) fp) True)", {'hxlo': -1, 'hmlo': 1, 'hslo': 1})
                    def arity_true():
                        G = Arm(S_(F.sl, 'har'), I)
                        def stack_leg():
                            H = Arm(S_(G.sl, 'hltF'), I)
                            H.add(None, run_is('(Some (IpFailed FStack))', [RV('hg'), RV('har'), RV('hltF')])); H.sl.add('hout')
                            H.add('ho', "(inject (premise hout) (ho))")
                            H.add(None, "(steps ((rewrite (premise ho) rl rhs true ()) (unfold ixt_expect rhs) (reduce rhs) (unfold ixt_run lhs) (reduce lhs)))")
                            pins = f"(inst k k) (inst np {NP}) (inst nl {NLg}) (inst ib ib) (inst ir ir) (inst e {RESg}) (inst v 0) (inst tb Nil) (inst tr Nil) (inst dmax dmax) (inst fail_ix fail_ix)"
                            H.arith('hdepN', f"(= (lt (+ dep 1) {N64}) True)", {'p14': 1, 'p15': 1})
                            # the callee's frame fits even at the budget's edge: the room holds one frame more than the budget
                            H.have('hnext', "(= (le (+ (+ fp own) (ixf_maxown fs)) (xmemhi_of m)) True)", f"(rewrite-with (lemma room_next) lr lhs ((inst dep dep) (inst dmax dmax)) ({D(12)} {D(14)} {D('hM0')}) refl)")
                            H.arith('hfits', f"(= (le (+ fp {OWNg}) (xmemhi_of m)) True)", {'hnext': 1, 'hown0': 1, 'hmo': 1})
                            H.arith('hwinl', f"(= (le (+ fp (* 8 {NLg})) (xmemhi_of m)) True)", {'hfits': 1, 'hob': 1, 'hsdb0': 8})
                            H.add(None, f"(rewrite-with (lemma fc_step_stack) lr lhs ({pins}) ({D('hfat')} {D('hltF')} {D('hdm0')} {D('hdmN')} {D(13)} {D('hdepN')} {D('hnpnl')} {D('hnp0')} {D('hfp0')} {D('hxlofp')} {D('hwinl')} {D('hhi')} {D('hcost12')} {D('hlen')} {D(9)}))")
                            H.add(None, "refl")
                            return H.text()
                        def depth_true():
                            H = Arm(S_(G.sl, 'hlt'), I)
                            H.arith('hdepN', f"(= (lt (+ dep 1) {N64}) True)", {'p14': 1, 'p15': 1})
                            H.have('hroomb0', f"(= (le (+ (+ (+ fp own) {OWNg}) (* (- (+ dmax 1) (+ dep 1)) (ixf_maxown fs))) (xmemhi_of m)) True)", f"(rewrite-with (lemma room_step) lr lhs ((inst own2 {OWNg})) ({D(12)} {D('hmo')}) refl)")
                            H.arith('hroomb', f"(= (le (+ (+ fp {OWNg}) (* (- (+ dmax 1) (+ dep 1)) (ixf_maxown fs))) (xmemhi_of m)) True)", {'hroomb0': 1, 'hown0': 1})
                            H.arith('hdep1', "(= (le (+ dep 1) dmax) True)", {'hlt': 1})
                            H.have('hfits', f"(= (le (+ fp {OWNg}) (xmemhi_of m)) True)", f"(rewrite-with (lemma room_fits) lr lhs ((inst dep (+ dep 1)) (inst dmax dmax) (inst mo (ixf_maxown fs))) ({D('hroomb')} {D('hdep1')} {D('hM0')}) refl)")
                            H.arith('hwinl', "(= (le (+ fp (* 8 {NLg})) (xmemhi_of m)) True)".replace("{NLg}", NLg), {'hfits': 1, 'hob': 1, 'hsdb0': 8})
                            H.arith('hwinb', f"(= (le (+ fp (* 8 (+ {NLg} (ixf_sdep {BODYg})))) (xmemhi_of m)) True)", {'hfits': 1, 'hob': 1})
                            H.arith('hwinr', f"(= (le (+ fp (* 8 (+ {NLg} (+ 0 (ixf_dep {RESg}))))) (xmemhi_of m)) True)", {'hfits': 1, 'hor': 1})
                            # the callee's entry context (iband_id / fz_* / fr_locs_app_l)
                            PSXz = f"(fp_app {FZg} psx)"
                            H.have('hfz', f"(= (fbelow slo {PSXz}) (fbelow slo psx))", f"(rewrite-with (lemma fz_below) lr lhs () ({D('hslo')} {D('hnp0')}) refl)")
                            H.have('heq', f"(= (ilen lc) {NP})", f"(rewrite-with (lemma int_eq_eq) lr lhs ((inst a (ilen lc)) (inst b {NP})) ({D('har')}) refl)")
                            H.have('hband', "(= (iband_args (ipparams_of g) lc) lc)", f"(rewrite-with (lemma iband_id) lr lhs ((inst x fp) (inst ps psx)) ({D('hp64')} {D('heq')} {D('hlocs')}) refl)")
                            H.have('hdiscz', f"(= (fp_disc slo {PSXz}) True)", f"(rewrite-with (lemma fz_disc) lr lhs () ({D('hdisc')} {D('hslo')} {D('hal')} {D('hnp0')}) refl)")
                            H.have('hminz', f"(= (fp_min {FZg} (+ fp (* 8 {NP}))) True)", "(rewrite-with (lemma fz_min) lr lhs () () refl)")
                            H.have('hwminz', f"(= (fp_wmin slo (+ fp (* 8 {NP})) {FZg}) True)", f"(rewrite-with (lemma fp_wmin_of_min) lr lhs ((inst lo (+ fp (* 8 {NP})))) ({D('hminz')}) refl)")
                            H.have('hwminz2', f"(= (fp_wmin slo (+ fp (* 8 (ilen lc))) {FZg}) True)", "(steps ((rewrite (premise heq) lr lhs true ()) (rewrite (premise hwminz) lr lhs true ())) refl)")
                            H.have('hlocs1', f"(= (fr_locs fp lc {PSXz}) True)", f"(rewrite-with (lemma fr_locs_app_wmin) lr lhs ((inst slo slo)) ({D('hlocs')} {D('hwminz2')} {D('hslo')}) refl)")
                            H.have('hlocsz', f"(= (fr_locs (+ fp (* 8 {NP})) (izeros (- {NLg} {NP})) {PSXz}) True)", f"(rewrite-with (lemma fz_locs) lr lhs () ({D('hnp0')}) refl)")
                            H.have('hne_eq', f"(= (izeros (- {NLg} {NP})) (izeros {NE}))", f"(chain (have hx (= (- {NLg} {NP}) {NE}) (by arith (list (list 1) (list 1)))) (steps ((rewrite (premise hx) lr lhs true ())) refl))")
                            H.have('hlocsz2', f"(= (fr_locs (+ fp (* 8 (ilen lc))) (izeros {NE}) {PSXz}) True)", "(steps ((rewrite (premise heq) lr lhs true ()) (rewrite (premise hne_eq) rl lhs true ()) (rewrite (premise hlocsz) lr lhs true ())) refl)")
                            H.have('hlocs_all', f"(= (fr_locs fp (iapp lc (izeros {NE})) {PSXz}) True)", f"(rewrite-with (lemma fr_locs_app_l) lr lhs () ({D('hlocs1')} {D('hlocsz2')}) refl)")
                            H.have('hlocsL', f"(= (fr_locs fp {LCg} {PSXz}) True)", "(steps ((rewrite (premise hband) lr lhs true ()) (rewrite (premise hlocs_all) lr lhs true ())) refl)")
                            H.have('hlenL', f"(= (le (ilen {LCg}) {NLg}) True)", f"(chain (have hz (= (ilen (izeros {NE})) {NE}) (rewrite-with (lemma ilen_zeros) lr lhs () ({D('hne0')}) refl)) (steps ((rewrite (premise hband) lr lhs true ()) (rewrite (lemma ilen_app) lr lhs true ()) (rewrite (premise hz) lr lhs true ()) (rewrite (premise heq) lr lhs true ())) (by arith (list 1))))")
                            H.have('hctxz', f"(= (fe_ctx m mlo slo fp {NLg} 0 {LCg} {PSXz}) True)", f"(rewrite-with (lemma ctx_intro) lr lhs () ({D('hdiscz')} {D('hlocsL')} {D('hxlo')} {D('hmlo')} {D('hslo')} {D('hal')} {D('hlenL')} {D('hd0')} {D('hslo0')} {D('hhi')}) refl)")
                            B_ = STSd('f2', BODYg, LCg, IM)
                            TB = TRLd('f2', BODYg, LCg, IM)
                            RS1 = "(MkRegs a0 rcx dx rbx rbp rsi di r8 r9 s10 s11 r12 r13 (+ dep 1) fp)"
                            CZ = f"(- (- {CK} 1) 2)"
                            RUNZ = f"(xeval_seq (xt {CZ} {KF2}) m (ixf_zero {NP} {NLg}) {RS1} {XM})"
                            RS2 = f"(MkRegs (rax_of (xo_regs {RUNZ})) rcx (rdx_of (xo_regs {RUNZ})) rbx rbp rsi di r8 r9 (r10_of (xo_regs {RUNZ})) (r11_of (xo_regs {RUNZ})) r12 r13 (+ dep 1) fp)"
                            C2 = f"(- {CZ} (* 4 (- {NLg} {NP})))"
                            CB = f"(- {C2} 2)"; CR = f"(- {C2} 1)"
                            XMz = f"(fp_mem mem0 {PSXz})"
                            RUNBq = f"(ixt_run (xt {CB} {KF2}) m (IqStmts {BODYg}) ib {RS2} {XMz})"
                            RUNB = f"(xeval_seq (xt {CB} {KF2}) m ib {RS2} {XMz})"
                            PSXb = f"(fp_app {TB} {PSXz})"
                            XMb = f"(fp_mem mem0 {PSXb})"
                            IMb = IMx(PSXb)
                            TRrI = f"(fe_tr fp {NLg} 0 {RESg} lcb {IMb} mlo slo)"
                            TRr = f"(fe_tr fp {NLg} 0 {RESg} lcb memb mlo slo)"
                            RSB = f"(MkRegs (rax_of (xo_regs {RUNB})) rcx (rdx_of (xo_regs {RUNB})) rbx rbp rsi di r8 r9 (r10_of (xo_regs {RUNB})) (r11_of (xo_regs {RUNB})) r12 r13 (+ dep 1) fp)"
                            RUNR = f"(xeval_seq (xt {CR} {KF2}) m ir {RSB} {XMb})"
                            H.arith('hcb0', f"(= (le 0 {CB}) True)", {'hfcost': 1, 'hfc': 1, 'p8': 1, 'hec': 1, 'hel': 1})
                            H.arith('hcr', f"(= (le (ixf_ecost {RESg}) {CR}) True)", {'hfcost': 1, 'hfc': 1, 'p8': 1})
                            MEMBq = f"(fp_mem mem0 (fp_app (ipt_tr f2 fs mlo slo dmax (+ dep 1) fp {NLg} {OWNg} (IqStmts {BODYg}) {LCg} {IMx(PSXz)}) {PSXz}))"
                            def body_ih(K_, out, hbx):
                                ds = [f"(steps ((unfold ixt_emit lhs) (reduce lhs) (rewrite (premise hib) lr lhs true ())) refl)",
                                      f"(steps ((rewrite (premise hfz) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise {hbx}) lr lhs true ())) refl)",
                                      D('hctxz'),
                                      "(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise hscbb) lr lhs true ())) refl)",
                                      "(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise hwinb) lr lhs true ())) refl)",
                                      "(steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise hskokb) lr lhs true ())) refl)",
                                      D(6),
                                      "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hcb0) lr lhs true ())) refl)",
                                      D('hcb0'), D(9), D(10),
                                      "(steps ((unfold ixt_fr lhs) (reduce lhs) (unfold ixt_body lhs) (reduce lhs) (rewrite (premise hfrb) lr lhs true ())) refl)",
                                      D('hroomb'), f"(by arith {K_.sl.cert({'p13': 1})})", D('hdep1'), D(15), D(16), D(17), D(18), D(19)]
                                K_.have('hih', f"(= {RUNBq} (ixt_expect (IqStmts {BODYg}) {out} {RS2} {RUNBq} {MEMBq}))",
                                        f"(rewrite-with (hyp ih) lr lhs ((inst out {out}) (inst lc {LCg}) (inst nl {NLg}) (inst own {OWNg}) (inst fail_ix fail_ix) (inst mlo mlo) (inst slo slo) (inst fs fs) (inst dmax dmax) (inst xfs xfs)) ({' '.join(ds)}) refl)")
                                K_.have('hihu', f"(= {RUNBq} {RUNB})", "(steps ((unfold ixt_run lhs) (reduce lhs)) refl)")
                            def norm_leg():
                                K = Arm(S_(H.sl, 'hb', 'hb2'), I)
                                def res_some():
                                    L_ = Arm(S_(K.sl, 'hv'), I)
                                    L_.add(None, run_is('(Some (IpNorm (Cons v Nil) memb))', [RV('hg'), RV('har'), RV('hlt'), RV('hb2'), RV('hv')])); L_.sl.add('hout')
                                    L_.add('ho', "(inject (premise hout) (ho))")
                                    body_ih(L_, '(IpNorm lcb memb)', 'hb2')
                                    L_.have('hih2', f"(= {RUNB} (fs_out {RS2} {RUNB} {XMb}))", "(steps ((rewrite (premise hihu) rl both true ()) (rewrite (premise hih) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs) (unfold ipt_tr lhs) (reduce lhs) (rewrite (premise hfz) lr lhs true ())) refl)")
                                    d0 = "(steps ((rewrite (premise hfz) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hb2) lr lhs true ())) refl)"
                                    L_.have('hctxb', f"(= (ixt_post (IqStmts {BODYg}) m mlo slo fp {NLg} lcb (fp_app (ipt_tr f2 fs mlo slo dmax (+ dep 1) fp {NLg} {OWNg} (IqStmts {BODYg}) {LCg} {IMx(PSXz)}) {PSXz})) True)",
                                             f"(rewrite-with (lemma ipt_ctx) lr lhs ((inst mem2 memb) (inst lc2 lcb) (inst dep (+ dep 1)) (inst nl {NLg}) (inst own {OWNg}) (inst lc {LCg})) ({d0} {D('hctxz')} (steps ((unfold ixt_body lhs) (reduce lhs) (rewrite (premise hscbb) lr lhs true ())) refl) (steps ((unfold ixt_fr lhs) (reduce lhs) (unfold ixt_body lhs) (reduce lhs) (rewrite (premise hfrb) lr lhs true ())) refl) {D(18)}) refl)")
                                    L_.have('hctxb2', f"(= (fe_ctx m mlo slo fp {NLg} 0 lcb {PSXb}) True)", "(steps ((rewrite (premise hctxb) rl rhs true ()) (unfold ixt_post rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfz) lr rhs true ())) refl)")
                                    L_.have('hmemb0', f"(= (fp_mem mem0 (fbelow slo (fp_app (ipt_tr f2 fs mlo slo dmax (+ dep 1) fp {NLg} {OWNg} (IqStmts {BODYg}) {LCg} {IMx(PSXz)}) {PSXz}))) memb)", f"(rewrite-with (lemma ipt_mem) lr lhs ((inst lc2 lcb) (inst mem2 memb) (inst dep (+ dep 1)) (inst nl {NLg}) (inst own {OWNg}) (inst lc {LCg})) ({d0} {D('hslo')} {D('hnlg')} {D('hownz')}) refl)")
                                    L_.have('hmemb', f"(= {IMb} memb)", "(steps ((rewrite (premise hmemb0) rl rhs true ()) (unfold ipt_tr rhs) (reduce rhs) (rewrite (premise hfz) lr rhs true ())) refl)")
                                    L_.have('hv2', f"(= (iexp {RESg} lcb {IMb} mlo slo) (Some v))", "(steps ((rewrite (premise hmemb) lr lhs true ()) (rewrite (premise hv) lr lhs true ())) refl)")
                                    L_.have('hsim', f"(= {RUNR} (fe_out v {RSB} {RUNR} (fp_mem mem0 (fp_app {TRrI} {PSXb}))))", f"(rewrite-with (lemma fe_sound) lr lhs ((inst e {RESg}) (inst nl {NLg}) (inst d 0) (inst lc lcb) (inst mlo mlo) (inst slo slo) (inst v v)) ({D('hir')} {D('hv2')} {D('hctxb2')} {D('hcbr')} {D('hwinr')} {D('hcr')}) refl)")
                                    L_.have('hsim2', f"(= {RUNR} (fe_out v {RSB} {RUNR} (fp_mem mem0 (fp_app {TRr} {PSXb}))))", "(steps ((rewrite (premise hmemb) rl both true ()) (rewrite (premise hsim) lr lhs true ())) refl)")
                                    L_.add(None, "(steps ((rewrite (premise ho) rl rhs true ()) (unfold ixt_run lhs) (reduce lhs)))")
                                    pins = f"(inst k k) (inst np {NP}) (inst nl {NLg}) (inst ib ib) (inst ir ir) (inst e {RESg}) (inst v v) (inst tb {TB}) (inst tr {TRr}) (inst dmax dmax) (inst fail_ix fail_ix)"
                                    L_.add(None, f"(rewrite-with (lemma fc_step_norm) lr lhs ({pins}) ({D('hfat')} {D('hlt')} {D('hdm0')} {D('hdmN')} {D(13)} {D('hdepN')} {D('hnpnl')} {D('hnp0')} {D('hfp0')} {D('hxlofp')} {D('hwinl')} {D('hhi')} {D('hcost12')} {D('hlen')} {D('hih2')} {D('hsim2')}))")
                                    L_.add(None, "(steps ((unfold ixt_run rhs) (reduce rhs) (unfold ixt_expect rhs) (reduce rhs) (unfold ihd rhs) (reduce rhs) (unfold ipt_tr rhs) (reduce rhs) (unfold ipt_call rhs) (reduce rhs) (rewrite (premise hg) lr rhs true ()) (reduce rhs) (rewrite (premise har) lr rhs true ()) (reduce rhs) (rewrite (premise hlt) lr rhs true ()) (reduce rhs) (rewrite (premise hb2) lr rhs true ()) (reduce rhs) (rewrite (premise hv) lr rhs true ()) (reduce rhs) (rewrite (lemma fp_app_assoc) lr rhs true ()) (rewrite (lemma fp_app_assoc) lr rhs true ())) refl)")
                                    return L_.text()
                                V = f"(iexp {RESg} lcb memb mlo slo)"
                                K.add(None, f"""(case-on {V} Option
  ((case None (chain {cap('hv', f'(= {V} None)')} {trap_leg([RV('hg'), RV('har'), RV('hlt'), RV('hb2'), RV('hv')])}))
   (case Some (v) (chain {cap('hv', f'(= {V} (Some v))')}
{res_some()}))))""")
                                return K.text()
                            def fail_leg():
                                K = Arm(S_(H.sl, 'hb', 'hb2'), I)
                                K.add(None, run_is('(Some (IpFailed fam))', [RV('hg'), RV('har'), RV('hlt'), RV('hb2')])); K.sl.add('hout')
                                K.add('ho', "(inject (premise hout) (ho))")
                                body_ih(K, '(IpFailed fam)', 'hb2')
                                K.have('hih2', f"(= {RUNB} (Some XTrap))", "(steps ((rewrite (premise hihu) rl lhs true ()) (rewrite (premise hih) lr lhs true ()) (unfold ixt_expect lhs) (reduce lhs)) refl)")
                                K.add(None, "(steps ((rewrite (premise ho) rl rhs true ()) (unfold ixt_expect rhs) (reduce rhs) (unfold ixt_run lhs) (reduce lhs)))")
                                pins = f"(inst k k) (inst np {NP}) (inst nl {NLg}) (inst ib ib) (inst ir ir) (inst e {RESg}) (inst v 0) (inst tb Nil) (inst tr Nil) (inst dmax dmax) (inst fail_ix fail_ix)"
                                K.add(None, f"(rewrite-with (lemma fc_step_fail) lr lhs ({pins}) ({D('hfat')} {D('hlt')} {D('hdm0')} {D('hdmN')} {D(13)} {D('hdepN')} {D('hnpnl')} {D('hnp0')} {D('hfp0')} {D('hxlofp')} {D('hwinl')} {D('hhi')} {D('hcost12')} {D('hlen')} {D('hih2')}))")
                                K.add(None, "refl")
                                return K.text()
                            H.add(None, f"""(case-on {B_} Option
  ((case None (chain {cap('hb', f'(= {B_} None)')} (have hn (= None (Some out)) (steps ((rewrite (premise 1) rl rhs true ()){SUBS(SC)} (unfold ipt_run rhs) (reduce rhs) (unfold ipcall rhs) (reduce rhs) (rewrite (premise hg) lr rhs true ()) (reduce rhs) (rewrite (premise har) lr rhs true ()) (reduce rhs) (rewrite (premise hlt) lr rhs true ()) (reduce rhs) (rewrite (premise hb) lr rhs true ()) (reduce rhs) (unfold ipout_of_ret rhs) (reduce rhs)) refl)) (absurd (premise hn))))
   (case Some (ob)
     (chain
       {cap('hb', f'(= {B_} (Some ob))')}
       (case-on ob IpOut
         ((case IpNorm (lcb memb)
            (chain
              (have hb2 (= {B_} (Some (IpNorm lcb memb))) (steps ((rewrite (premise hb) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
{norm_leg()}))
          (case IpTrap
            (chain
              (have hb2 (= {B_} (Some IpTrap)) (steps ((rewrite (premise hb) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
              {trap_leg([RV('hg'), RV('har'), RV('hlt'), RV('hb2')])}))
          (case IpFailed (fam)
            (chain
              (have hb2 (= {B_} (Some (IpFailed fam))) (steps ((rewrite (premise hb) lr lhs true ()) (rewrite (hyp 0) lr lhs true ())) refl))
{fail_leg()}))))))))""")
                            return H.text()
                        G.add(None, f"""(case-on (lt dep dmax) Bool
  ((case False (chain {cap('hltF', '(= (lt dep dmax) False)')}
{stack_leg()}))
   (case True (chain {cap('hlt', '(= (lt dep dmax) True)')}
{depth_true()}))))""")
                        return G.text()
                    AE = f"(int_eq (ilen lc) (ikn (ipparams_of g)))"
                    F.add(None, f"""(case-on {AE} Bool
  ((case False (chain {cap('har', f'(= {AE} False)')} {trap_leg([RV('hg'), RV('har')])}))
   (case True (chain {cap('har', f'(= {AE} True)')}
{arity_true()}))))""")
                    return F.text()
                E.add(None, case_opt(f"(ixf_exp {NLg} 0 {RESg})", "ir", emit_none([RV('hib'), RV('hir')]), ir_some))
                return E.text()
            C.add(None, case_opt(f"(ixf_stmts {NLg} {OWNg} fail_ix {BODYg})", "ib", emit_none([RV('hib')]), ib_some))
            return C.text()
        B.add(None, case_opt("(ixf_fn dmax fail_ix g)", "xf", fn_none(), fn_some))
        return B.text()
    A.add(None, f"""(case-on (ipfn_at fs k) Option
  ((case None (chain {cap('hg', '(= (ipfn_at fs k) None)')} {trap_leg([RV('hg')])}))
   (case Some (g) (chain {cap('hg', '(= (ipfn_at fs k) (Some g))')}
{some_g()}))))""")
    return A.text()

def m_arm_fail(sl):
    I = "    "
    A = Arm(sl, I)
    A.have('hem', "(= (ixf_stmt nl own fail_ix (IpFail fam)) (Some is))", f"(steps ((rewrite (premise 0) rl rhs true ()){SUBS(SS)} (unfold ixt_emit rhs) (reduce rhs)) refl)")
    A.have('hout', "(= (Some (IpFailed fam)) (Some out))", f"(steps ((rewrite (premise 1) rl rhs true ()){SUBS(SS)} (unfold ipt_run rhs) (reduce rhs) (unfold ipstmt rhs) (reduce rhs)) refl)")
    A.add('ho', "(inject (premise hout) (ho))")
    A.add(None, "(steps ((rewrite (premise ho) rl rhs true ()) (unfold ixt_expect rhs) (reduce rhs) (unfold ixt_run lhs) (reduce lhs)))")
    A.add(None, f"(rewrite-with (lemma fs_step_fail) lr lhs ((inst fam fam) (inst nl nl) (inst own own) (inst fail_ix fail_ix)) ({D('hem')} {D(9)} {D('hcs')}))")
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
                                            ('hkh', 'skok_head', '(ixf_skok K (Cons s Nil))', '(inst t t)', 'hkok'), ('hkt', 'skok_tail', '(ixf_skok K t)', '(inst s s)', 'hkok'),
                                            ('hks', 'skok_cost', '(le (ixf_scost s) K)', '(inst t t)', 'hkok')]:
                C.have(nm, f"(= {fact} True)", f"(rewrite-with (lemma {lem}) lr lhs ({pin}) ({D(src)}) refl)")
            C.have('hsdh0', "(= (le (ixf_sdep (Cons s Nil)) (ixf_sdep (Cons s t))) True)", "(rewrite-with (lemma sdep_head) lr lhs () () refl)")
            C.have('hsdt0', "(= (le (ixf_sdep t) (ixf_sdep (Cons s t))) True)", "(rewrite-with (lemma sdep_tail) lr lhs ((inst s s)) () refl)")
            C.arith('hsdh', "(= (le (+ fp (* 8 (+ nl (ixf_sdep (Cons s Nil))))) (xmemhi_of m)) True)", {'hsd': 1, 'hsdh0': 8})
            C.arith('hsdt', "(= (le (+ fp (* 8 (+ nl (ixf_sdep t)))) (xmemhi_of m)) True)", {'hsd': 1, 'hsdt0': 8})
            C.add(None, fr_sub_have('hfrh', '(ixf_sdep (Cons s Nil))', 'hsdh0', '(Cons s t)')); C.sl.add('hfrh')
            C.add(None, fr_sub_have('hfrt', '(ixf_sdep t)', 'hsdt0', '(Cons s t)')); C.sl.add('hfrt')
            C.have('hsl', "(= (le (xil iss) (ixf_scost s)) True)", f"(rewrite-with (lemma slen_cost) lr lhs ((inst nl nl) (inst own own) (inst fail_ix fail_ix)) ({D('hiss')}) refl)")
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
                      "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hcsK) lr lhs true ())) refl)", D('hc0')] + TAIL_DS(fr_d('hfrh'))
                m_ih(G, 'hihs', '(IqStmt s)', CK, 'iss', RS, 'psx', 'lc', out, ds)
            def leaf_norm(sl2):
                G = Arm(sl2, I + "      ")
                G.have('ht', f"(= {STS('f2', 't', 'lcs', 'mems')} (Some out))", "(steps ((rewrite (premise himp) rl rhs true ()) (unfold ipstmts rhs) (reduce rhs) (rewrite (premise hs2) lr rhs true ()) (reduce rhs)) refl)")
                head_ih(G, '(IpNorm lcs mems)')
                d0 = "(steps ((unfold ipt_run lhs) (reduce lhs) (rewrite (premise hs2) lr lhs true ())) refl)"
                G.have('hmems', T1C('f2', '(IqStmt s)', 'lc', IM, 'psx', 'mems'), f"(rewrite-with (lemma ipt_mem) lr lhs ((inst lc2 lcs) (inst mem2 mems)) ({d0} {D('hslo')} {D('hnl0')} {D('hown0')}) refl)")
                G.have('hctxs', T2P('f2', '(IqStmt s)', 'lc', IM, 'psx', 'lcs'), f"(rewrite-with (lemma ipt_ctx) lr lhs ((inst mem2 mems)) ({d0} {D(2)} {body_d('hcbh')} {fr_d('hfrh')} {D(18)}) refl)")
                G.have('hctxs2', T2C('f2', '(IqStmt s)', 'lc', IM, 'psx', 'lcs'), "(steps ((rewrite (premise hctxs) rl rhs true ()) (unfold ixt_post rhs) (reduce rhs)) refl)")
                ds = [emit_d('hit'), "(steps ((rewrite (premise hmems) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise ht) lr lhs true ())) refl)", D('hctxs2'), body_d('hcbt'), body_d('hsdt'), body_d('hkt'), D(6),
                      "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hc0t) lr lhs true ())) refl)", D('hc0t')] + TAIL_DS(fr_d('hfrt'))
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
                F.arith('hc3', f"(= (le (+ (xil ic) 3) {CK}) True)", {'hcs': 1, 'hlen': -1, 'hec': 1, 'p6': 1})
                F.arith('hc5', f"(= (le (+ (xil ic) 5) {CK}) True)", {'hcs': 1, 'hlen': -1, 'hec': 1, 'p6': 1})
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
                    G.add(None, fr_sub_have('hfrb', '(ixf_sdep b)', 'hsdb', '(Cons (IpWhile ce b) Nil)')); G.sl.add('hfrb')
                    G.arith('hcb0', f"(= (le 0 {cc}) True)", {'hcs': 1, 'hlen': -1, 'hec': 1, 'p6': 1})
                    G.arith('hcw0', f"(= (le 0 (- {CK} 1)) True)", {'hcs': 1, 'hel': 1, 'hec': 1, 'p6': 1})
                    G.arith('hwc', f"(= (le (+ (ixf_ecost ce) 1) (- {CK} 1)) True)", {'hcs': 1, 'p6': 1})
                    def body_ih(H, out, hb):
                        ds = [emit_d('hib'), f"(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise {hb}) lr lhs true ())) refl)", D('hctx1'), body_d('hcbb'), body_d('hsdt'), body_d('hkb'), D(6),
                              "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hcb0) lr lhs true ())) refl)", D('hcb0')] + TAIL_DS(fr_d('hfrb'))
                        return m_ih(H, 'hihb', '(IqStmts b)', cc, 'ib', rsc, PSX1c, 'lc', out, ds)
                    def leaf_norm(sl2):
                        H = Arm(sl2, I + "          ")
                        H.have('hw2', f"(= {WH('f2', 'ce', 'b', 'lcb', 'memb')} (Some out))", "(steps ((rewrite (premise himp) rl rhs true ()) (unfold ipwhile rhs) (reduce rhs) (rewrite (premise hcv) lr rhs true ()) (reduce rhs) (rewrite (premise hcvf) lr rhs true ()) (reduce rhs) (rewrite (premise hb2) lr rhs true ()) (reduce rhs)) refl)")
                        runb, memb = body_ih(H, '(IpNorm lcb memb)', 'hb2')
                        d0b = "(steps ((rewrite (premise hfb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hb2) lr lhs true ())) refl)"
                        PSXb = f"(fp_app {TR('f2', '(IqStmts b)', 'lc', IMx(PSX1c))} {PSX1c})"
                        H.have('hmemb', T1C('f2', '(IqStmts b)', 'lc', IMx(PSX1c), PSX1c, 'memb'), f"(rewrite-with (lemma ipt_mem) lr lhs ((inst lc2 lcb) (inst mem2 memb)) ({d0b} {D('hslo')} {D('hnl0')} {D('hown0')}) refl)")
                        H.have('hctxb', T2P('f2', '(IqStmts b)', 'lc', IMx(PSX1c), PSX1c, 'lcb'), f"(rewrite-with (lemma ipt_ctx) lr lhs ((inst mem2 memb)) ({d0b} {D('hctx1')} {body_d('hcbb')} {fr_d('hfrb')} {D(18)}) refl)")
                        H.have('hctxb2', T2C('f2', '(IqStmts b)', 'lc', IMx(PSX1c), PSX1c, 'lcb'), "(steps ((rewrite (premise hctxb) rl rhs true ()) (unfold ixt_post rhs) (reduce rhs)) refl)")
                        rsb = RSP(runb)
                        ds = [D('hem'), "(steps ((rewrite (premise hmemb) lr lhs true ()) (unfold ipt_run lhs) (reduce lhs) (rewrite (premise hw2) lr lhs true ())) refl)", D('hctxb2'), body_d('hscb'), body_d('hsd'), body_d('hkok'), D(6),
                              "(steps ((unfold ixt_cost lhs) (reduce lhs) (rewrite (premise hwc) lr lhs true ())) refl)", D('hcw0')] + TAIL_DS(fr_d('hfr'))
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
            "(= (xfunc_at (xfuncs_of m) fail_ix) (Some (MkXFunc 0 (list (XMovRI RAX 60) XSyscall))))",
            "(= (le 0 mlo) True)",
            "(= (ixt_fr r nl own (ixf_maxown fs)) True)",
            "(= (le (+ (+ fp own) (* (- (+ dmax 1) dep) (ixf_maxown fs))) (xmemhi_of m)) True)",
            "(= (le 0 dep) True)",
            "(= (le dep dmax) True)",
            f"(= (lt (+ dmax 1) {N64}) True)",
            "(= (ixf_fns dmax fail_ix fs) (Some xfs))",
            f"(= (xfuncs_of m) (xf_app xfs {SHIMF}))",
            "(= (ixf_fnsok fs) True)",
            "(= (ixf_fnskok K fs) True)"]
    sl0 = Slots([f"p{i}" for i in range(NPREM)])
    def z_arm(ctor, binders, engine):
        b = f" ({binders})" if binders else ""
        post = POST_CALL if engine == "ipcall" else ""
        return f"(case {ctor}{b} (chain {cap('hr', f'(= r ({ctor} {binders}))')} (have hn (= None (Some out)) (steps ((rewrite (premise 1) rl rhs true ()) (rewrite (premise hr) lr rhs true ()) (unfold ipt_run rhs) (reduce rhs) (unfold {engine} rhs) (reduce rhs){post}) refl)) (absurd (premise hn))))"
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
    slS = S_(sl0, 'hr')
    # the list arm
    AL = Arm(S_(sl0, 'hr', 'hss'), "  "); m_sides(AL, "(Cons s t)", SL)
    m_cost(AL, "(IqStmts (Cons s t))", SL, ['ixt_cost']); AL.lines[-1] = AL.lines[-1].replace("{COST}", "0")
    AN = Arm(S_(sl0, 'hr', 'hss'), "  ")
    AW = Arm(S_(sl0, 'hr'), "  "); m_sides(AW, "(Cons (IpWhile ce b) Nil)", SW)
    m_cost(AW, "(IqWhile ce b)", SW, ['ixt_cost']); AW.lines[-1] = AW.lines[-1].replace("{COST}", "(+ (ixf_ecost ce) 1)")
    # the call request: no statement sides; its cost is 0 and its frame bundle says 0 <= own
    AC = Arm(S_(sl0, 'hr'), "  ")
    m_cost(AC, "(IqCall k)", SC, ['ixt_cost']); AC.lines[-1] = AC.lines[-1].replace("{COST}", "0")
    AC.have('hown0', "(= (le 0 own) True)", f"(steps ((rewrite (premise 11) rl rhs true ()){SUBS(SC)} (unfold ixt_fr rhs) (reduce rhs)) refl)")
    return f"""(claim ipt_sound
  (goal ({SV})
    ({chr(10).join('     ' + p for p in prem).strip()})
    {CONCL('f', 'r', 'lc', IM, 'psx', 'out', RS, XR('c', 'f', 'r', 'is', RS, XM))})
  (induct f
    ((case Z
       (case-on r IptReq
         ({z_arm('IqStmt', 's', 'ipstmt')}
          {z_arm('IqStmts', 'ss', 'ipstmts')}
          {z_arm('IqWhile', 'ce b', 'ipwhile')}
          {z_arm('IqCall', 'k', 'ipcall')})))
     (case S (f2)
       (case-on r IptReq
         ((case IqStmt (s)
            (chain
              {cap('hr', '(= r (IqStmt s))')}
              {gsub(['hr'])}
              (case-on s IpStmt
                ({stmt_arm('IpSet', 'i e', '(IpSet i e)', m_arm_set, slS, ['ixf_scost'])}
                 {stmt_arm('IpStore', 'ae ve', '(IpStore ae ve)', m_arm_store, slS, ['ixf_scost'])}
                 {stmt_arm('IpIf', 'ce tb eb', '(IpIf ce tb eb)', m_arm_if, slS, ['ixf_scost'])}
                 {stmt_arm('IpWhile', 'ce b', '(IpWhile ce b)', m_arm_while, slS, ['ixf_scost'])}
                 {stmt_arm('IpCall', 'i k args', '(IpCall i k args)', m_arm_call, slS, ['ixf_scost'])}
                 {stmt_arm('IpLoadW', 'i ae', '(IpLoadW i ae)', m_arm_loadw, slS, ['ixf_scost'])}
                 {stmt_arm('IpStoreW', 'ae ve', '(IpStoreW ae ve)', m_arm_storew, slS, ['ixf_scost'])}
                 {stmt_arm('IpFail', 'fam', '(IpFail fam)', m_arm_fail, slS, ['ixf_scost'])}
                 {stmt_arm('IpUnreach', '', 'IpUnreach', m_arm_unreach, slS, ['ixf_scost'])}))))
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
{m_arm_qwhile(AW.sl)}))
          (case IqCall (k)
            (chain
              {cap('hr', '(= r (IqCall k))')}
              {gsub(['hr'])}
{AC.text()}
{m_arm_qcall(AC.sl)}))))))))
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
