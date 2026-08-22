#!/usr/bin/env python3
"""models/imp/probes/gen_fra.py — the emitter for the GENERATED claims of
models/imp/probes/fra_kit.shard (Theorem A, docs/COVERAGE.md §11.1 slice A-2):
the structural-walk spines over IExp whose every leaf is the same hand
template punched per constructor / op / kind — fe_len (the emission's
length), the spill-trace twin's four laws (fe_tr_disc / fe_tr_locs /
fe_tr_below / fe_tr_min), fe_band (the value band), and the expression
lemma fe_sound.

IN-TREE ON PURPOSE: A1's walk emitters lived in a session scratchpad and are
gone (cert-arc-a.md) — the emitted nests could never be regenerated. This
file is the regeneration source; the emitted claims carry a banner naming
it, and are NEVER hand-patched (thread-division law).

TODO (shard#18 / #27): this is an ad-hoc proof-text generator, the
anti-pattern the 2026-07-12 ruling names; the intended home of this work
is tools/prove + the lock-step engine closing the leaves of a hand
skeleton. Kept until that machinery reaches the expression tier; the
ledger's A-2 record prices what it would replace.

RUN (from the repo root; then shardfmt the probe and bin/check it):
  python3 models/imp/probes/gen_fra.py fe_len    > /tmp/x && splice per the banner
  python3 models/imp/probes/gen_fra.py twin      (the four twin laws)
  python3 models/imp/probes/gen_fra.py fe_band
Each emitted block replaces the text from its banner line to the next
top-level form; a helper `splice` mode does this in place:
  python3 models/imp/probes/gen_fra.py splice    (rewrites fra_kit.shard's generated blocks)
"""
import sys

def cap(name, eq):
    return f"(have {name} {eq} (steps ((rewrite (hyp 0) lr lhs true ())) refl))"

def absurd_none(hp, rewrites):
    """None = Some ie from hp's unfolded RHS; `rewrites` = captured case eqs to push through (lr rhs)."""
    items = [f"(rewrite (premise {hp}) rl rhs true ())", "(unfold ixf_exp rhs)", "(reduce rhs)"]
    for r in rewrites:
        items.append(r)
        items.append("(reduce rhs)")
    return f"""(chain
  (have hn (= None (Some ie)) (steps ({' '.join(items)}) refl))
  (absurd (premise hn)))"""

def some_term(hp, term, rewrites, hie_rewrites_then_close):
    items = [f"(rewrite (premise {hp}) rl rhs true ())", "(unfold ixf_exp rhs)", "(reduce rhs)"]
    for r in rewrites:
        items.append(r)
        items.append("(reduce rhs)")
    return f"""(chain
  (have hsome (= (Some {term}) (Some ie)) (steps ({' '.join(items)}) refl))
  (inject (premise hsome) (hie))
  {hie_rewrites_then_close})"""

# ---------------- fe_len ----------------

def len_close_bin(oplist_term, ih_b_d="(+ d 1)"):
    """after hie: ie = (ix_app ia (ix_app (ixf_spill nl d) (ix_app ib (ix_app (ixf_reload10 nl d) OP))))"""
    return f"""(steps
    ((rewrite (premise hie) rl lhs true ())
     (rewrite (lemma xil_app) lr lhs true ())
     (rewrite (lemma xil_app) lr lhs true ())
     (rewrite (lemma xil_app) lr lhs true ())
     (rewrite (lemma xil_app) lr lhs true ())
     (rewrite (lemma xil_spill) lr lhs true ())
     (rewrite (lemma xil_reload) lr lhs true ())))
  (rewrite-with (hyp ih) lr lhs ((inst nl nl) (inst d d)) ((steps ((rewrite (premise ha) lr lhs true ())) refl)))
  (rewrite-with (hyp ih1) lr lhs ((inst nl nl) (inst d {ih_b_d})) ((steps ((rewrite (premise hb) lr lhs true ())) refl)))
  (steps
    ((compute lhs (stop ixf_elen))
     (unfold ixf_elen rhs)
     (reduce rhs)
     (unfold ixf_oplen rhs)
     (reduce rhs)
     (compute rhs (stop ixf_elen)))
    refl)"""

def len_close_unary(ih_hyp="ih", ih_d="d", extra_rewrites="", after=""):
    """ie = (ix_app ia TAIL): xil_app once, the tail ground"""
    return f"""(steps
    ((rewrite (premise hie) rl lhs true ())
     (rewrite (lemma xil_app) lr lhs true ()){extra_rewrites}))
  (rewrite-with (hyp {ih_hyp}) lr lhs ((inst nl nl) (inst d {ih_d})) ((steps ((rewrite (premise ha) lr lhs true ())) refl)))
  (steps
    ((compute lhs (stop ixf_elen)){after}
     (unfold ixf_elen rhs)
     (reduce rhs)
     (compute rhs (stop ixf_elen)))
    refl)"""

BIN_TERM = "(ix_app ia (ix_app (ixf_spill nl d) (ix_app ib (ix_app (ixf_reload10 nl d) {op}))))"

def bop_arm(op, xop):
    """IAdd/ISub/IMul/IAnd/IOr/IXor: k split (U8 refused)"""
    ha = "(rewrite (premise ha) lr rhs true ())"
    hb = "(rewrite (premise hb) lr rhs true ())"
    def kleaf(kind, instr):
        hk = "(rewrite (premise hk) lr rhs true ())"
        term = BIN_TERM.format(op=f"(list ({instr} {xop} R10 (SReg RAX)) (XMovRR RAX R10))")
        return f"""(chain
  {cap('hk', f'(= k {kind})')}
  (steps ((rewrite (premise hk) lr both true ())))
  {some_term('hp', term, [ha, hb, hk, '(unfold ixf_bop rhs)'], len_close_bin(None))})"""
    u8 = f"""(chain
  {cap('hk', '(= k U8)')}
  {absurd_none('hp', [ha, hb, '(rewrite (premise hk) lr rhs true ())', '(unfold ixf_bop rhs)'])})"""
    return f"""(case-on k IKind
  ((case U8 {u8})
   (case U32 {kleaf('U32', 'XBin32')})
   (case U64 {kleaf('U64', 'XBin')})))"""

def plain_arm(opterm):
    """IDiv/IRem/IEq/ILt/ILe: no k split; opterm = the op list as the reducer spells it"""
    ha = "(rewrite (premise ha) lr rhs true ())"
    hb = "(rewrite (premise hb) lr rhs true ())"
    term = BIN_TERM.format(op=opterm)
    return some_term('hp', term, [ha, hb], len_close_bin(None))

def general_arm(op, inner):
    """the a / b case-ons around `inner` (which assumes ha, hb captured)"""
    ha = "(rewrite (premise ha) lr rhs true ())"
    return f"""(chain
  {cap('ho', f'(= o {op})')}
  (have hp (= (ixf_exp nl d (IBin k {op} a b)) (Some ie))
    (steps ((rewrite (premise ho) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
  (steps ((rewrite (premise ho) lr both true ())))
  (case-on (ixf_exp nl d a) Option
    ((case None
       (chain
         {cap('ha', '(= (ixf_exp nl d a) None)')}
         {absurd_none('hp', [ha])}))
     (case Some
       (ia)
       (chain
         {cap('ha', '(= (ixf_exp nl d a) (Some ia))')}
         (case-on (ixf_exp nl (+ d 1) b) Option
           ((case None
              (chain
                {cap('hb', '(= (ixf_exp nl (+ d 1) b) None)')}
                {absurd_none('hp', [ha, '(rewrite (premise hb) lr rhs true ())'])}))
            (case Some
              (ib)
              (chain
                {cap('hb', '(= (ixf_exp nl (+ d 1) b) (Some ib))')}
                {inner})))))))))"""

def shift_arm(op, is_shl):
    """IShl / IShr: b must be IConst; IShl splits k (U8 refused)"""
    ha = "(rewrite (premise ha) lr rhs true ())"
    def notconst(ctor, binders):
        return f"""(case {ctor}{(' (' + binders + ')') if binders else ''}
  (chain
    {cap('hbc', f'(= b ({ctor} {binders}))' if binders else f'(= b {ctor})')}
    (steps ((rewrite (premise hbc) lr both true ())))
    (have hp2 (= (ixf_exp nl d (IBin k {op} a ({ctor} {binders}))) (Some ie))
      (steps ((rewrite (premise hbc) rl lhs true ()) (rewrite (premise hp) lr lhs true ())) refl))
    {absurd_none('hp2', [])}))"""
    others = "\n".join([
        notconst("ILoc", "j"),
        notconst("IBin", "k2 o2 a2 b2"),
        notconst("IRotr", "k2 a2 c2"),
        notconst("IExt", "k2 k3 a2"),
        notconst("ITrunc", "k2 k3 a2"),
        notconst("ILoad", "a2"),
    ])
    if is_shl:
        def kleaf(kind, instr):
            term = f"(ix_app ia (list ({instr} RAX cc)))"
            return f"""(chain
  {cap('hk', f'(= k {kind})')}
  (steps ((rewrite (premise hk) lr both true ())))
  {some_term('hp2', term, [ha, '(rewrite (premise hk) lr rhs true ())'], len_close_unary())})"""
        u8 = f"""(chain
  {cap('hk', '(= k U8)')}
  {absurd_none('hp2', [ha, '(rewrite (premise hk) lr rhs true ())'])})"""
        someleaf = f"""(case-on k IKind
  ((case U8 {u8})
   (case U32 {kleaf('U32', 'XShlI32')})
   (case U64 {kleaf('U64', 'XShlI')})))"""
    else:
        term = "(ix_app ia (list (XShrI RAX cc)))"
        someleaf = some_term('hp2', term, [ha], len_close_unary())
    const = f"""(case IConst
  (cc)
  (chain
    {cap('hbc', '(= b (IConst cc))')}
    (steps ((rewrite (premise hbc) lr both true ())))
    (have hp2 (= (ixf_exp nl d (IBin k {op} a (IConst cc))) (Some ie))
      (steps ((rewrite (premise hbc) rl lhs true ()) (rewrite (premise hp) lr lhs true ())) refl))
    (case-on (ixf_exp nl d a) Option
      ((case None
         (chain
           {cap('ha', '(= (ixf_exp nl d a) None)')}
           {absurd_none('hp2', [ha])}))
       (case Some
         (ia)
         (chain
           {cap('ha', '(= (ixf_exp nl d a) (Some ia))')}
           {someleaf}))))))"""
    return f"""(chain
  {cap('ho', f'(= o {op})')}
  (have hp (= (ixf_exp nl d (IBin k {op} a b)) (Some ie))
    (steps ((rewrite (premise ho) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
  (steps ((rewrite (premise ho) lr both true ())))
  (case-on b IExp
    ({const}
     {others})))"""

def ibin_case():
    arms = []
    for op, xop in [("IAdd","XAdd"),("ISub","XSub"),("IMul","XMul"),("IAnd","XAnd"),("IOr","XOr"),("IXor","XXor")]:
        arms.append(f"(case {op} {general_arm(op, bop_arm(op, xop))})")
    arms.append(f"(case IDiv {general_arm('IDiv', plain_arm('(ixf_div)'))})")
    arms.append(f"(case IRem {general_arm('IRem', plain_arm('(ix_app (ixf_div) (list (XMovRR RAX RDX)))'))})")
    arms.append(f"(case IShl {shift_arm('IShl', True)})")
    arms.append(f"(case IShr {shift_arm('IShr', False)})")
    for op, cd in [("IEq","CEq"),("ILt","CLtU"),("ILe","CLeU")]:
        arms.append(f"(case {op} {general_arm(op, plain_arm(f'(ixf_cmp ({cd} R10 (SReg RAX)))'))})")
    # the arms must be in IOp declaration order: IAdd ISub IMul IDiv IRem IAnd IOr IXor IShl IShr IEq ILt ILe
    order = ["IAdd","ISub","IMul","IDiv","IRem","IAnd","IOr","IXor","IShl","IShr","IEq","ILt","ILe"]
    bykey = {}
    for a in arms:
        key = a.split()[1]
        bykey[key] = a
    return "(case-on o IOp\n  (" + "\n   ".join(bykey[k] for k in order) + "))"

def const_case():
    return f"""(chain
  (have hsome (= (Some (list (XMovRI RAX n))) (Some ie))
    (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_exp rhs) (reduce rhs)) refl))
  (inject (premise hsome) (hie))
  (steps
    ((rewrite (premise hie) rl lhs true ()) (compute lhs) (unfold ixf_elen rhs) (reduce rhs))
    refl))"""

def loc_case():
    return f"""(chain
  (have hsome (= (Some (ixf_ld i)) (Some ie))
    (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_exp rhs) (reduce rhs)) refl))
  (inject (premise hsome) (hie))
  (steps
    ((rewrite (premise hie) rl lhs true ()) (rewrite (lemma xil_ld) lr lhs true ()) (unfold ixf_elen rhs) (reduce rhs))
    refl))"""

def rotr_case():
    ha = "(rewrite (premise ha) lr rhs true ())"
    def kabs(kind):
        return f"""(case {kind}
  (chain
    {cap('hk', f'(= k {kind})')}
    (have hp (= (ixf_exp nl d (IRotr {kind} a c)) (Some ie))
      (steps ((rewrite (premise hk) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
    {absurd_none('hp', [])}))"""
    def notconst(ctor, binders):
        return f"""(case {ctor}{(' (' + binders + ')') if binders else ''}
  (chain
    {cap('hcc', f'(= c ({ctor} {binders}))')}
    (have hp2 (= (ixf_exp nl d (IRotr U32 a ({ctor} {binders}))) (Some ie))
      (steps ((rewrite (premise hcc) rl lhs true ()) (rewrite (premise hp) lr lhs true ())) refl))
    {absurd_none('hp2', [])}))"""
    others = "\n".join([notconst("ILoc","j"), notconst("IBin","k2 o2 a2 b2"), notconst("IRotr","k2 a2 c2"),
                        notconst("IExt","k2 k3 a2"), notconst("ITrunc","k2 k3 a2"), notconst("ILoad","a2")])
    u32 = f"""(case U32
  (chain
    {cap('hk', '(= k U32)')}
    (have hp (= (ixf_exp nl d (IRotr U32 a c)) (Some ie))
      (steps ((rewrite (premise hk) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
    (steps ((rewrite (premise hk) lr both true ())))
    (case-on c IExp
      ((case IConst
         (cc)
         (chain
           {cap('hcc', '(= c (IConst cc))')}
           (steps ((rewrite (premise hcc) lr both true ())))
           (have hp2 (= (ixf_exp nl d (IRotr U32 a (IConst cc))) (Some ie))
             (steps ((rewrite (premise hcc) rl lhs true ()) (rewrite (premise hp) lr lhs true ())) refl))
           (case-on (ixf_exp nl d a) Option
             ((case None
                (chain
                  {cap('ha', '(= (ixf_exp nl d a) None)')}
                  {absurd_none('hp2', [ha])}))
              (case Some
                (ia)
                (chain
                  {cap('ha', '(= (ixf_exp nl d a) (Some ia))')}
                  {some_term('hp2', '(ix_app ia (list (XRorI32 RAX cc)))', [ha], len_close_unary())}))))))
       {others}))))"""
    return f"""(case-on k IKind
  ({kabs('U8')}
   {u32}
   {kabs('U64')}))"""

def ext_case():
    return f"""(chain
  (have hp (= (ixf_exp nl d a) (Some ie))
    (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_exp rhs) (reduce rhs)) refl))
  (rewrite-with (hyp ih) lr lhs ((inst nl nl) (inst d d)) ((steps ((rewrite (premise hp) lr lhs true ())) refl)))
  (steps ((unfold ixf_elen rhs) (reduce rhs)) refl))"""

def trunc_case():
    ha = "(rewrite (premise ha) lr rhs true ())"
    def kleaf(kind, term, extra):
        after = " (rewrite (lemma add0) lr lhs true ())" if kind == "U64" else ""
        return f"""(case {kind}
  (chain
    {cap('hk', f'(= kt {kind})')}
    (steps ((rewrite (premise hk) lr both true ())))
    (have hp2 (= (ixf_exp nl d (ITrunc k1 {kind} a)) (Some ie))
      (steps ((rewrite (premise hk) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
    {some_term('hp2', term, [ha], len_close_unary(extra_rewrites=extra, after=after))}))"""
    return f"""(case-on (ixf_exp nl d a) Option
  ((case None
     (chain
       {cap('ha', '(= (ixf_exp nl d a) None)')}
       {absurd_none('0', [ha])}))
   (case Some
     (ia)
     (chain
       {cap('ha', '(= (ixf_exp nl d a) (Some ia))')}
       (case-on kt IKind
         ({kleaf('U8', '(ix_app ia (list (XBin XAnd RAX (SImm 255))))', '')}
          {kleaf('U32', '(ix_app ia (list (XMovRR32 RAX RAX)))', '')}
          {kleaf('U64', '(ix_app ia Nil)', '')}))))))"""

def load_case():
    ha = "(rewrite (premise ha) lr rhs true ())"
    return f"""(case-on (ixf_exp nl d a) Option
  ((case None
     (chain
       {cap('ha', '(= (ixf_exp nl d a) None)')}
       {absurd_none('0', [ha])}))
   (case Some
     (ia)
     (chain
       {cap('ha', '(= (ixf_exp nl d a) (Some ia))')}
       {some_term('0', '(ix_app ia (list (XLoad8 RAX (AReg RAX))))', [ha], len_close_unary())}))))"""

def fe_len():
    return f""";; the emission's length is ixf_elen (generated by gen_fra.py — REGENERATE, never hand-patch)
(claim fe_len
  (goal ((e IExp) (nl Int) (d Int) (ie (List XInstr))) ((= (ixf_exp nl d e) (Some ie)))
    (= (xil ie) (ixf_elen e)))
  (induct e
    ((case IConst (n) {const_case()})
     (case ILoc (i) {loc_case()})
     (case IBin (k o a b) {ibin_case()})
     (case IRotr (k a c) {rotr_case()})
     (case IExt (k1 k2 a) {ext_case()})
     (case ITrunc (k1 kt a) {trunc_case()})
     (case ILoad (a) {load_case()}))))
"""


# ---------------- the twin's laws: a shared skeleton over fe_tr ----------------
# Each law: induct e; IConst/ILoc → fe_tr = Nil (nil_leaf); unary ctors → fe_tr a (unary_leaf);
# IBin: case-on o; shifts → unary_leaf; general → case-on (iexp a …): None → nil_leaf; Some va → bin_leaf.

OPS = ["IAdd","ISub","IMul","IDiv","IRem","IAnd","IOr","IXor","IShl","IShr","IEq","ILt","ILe"]
ADDR = "(+ fp (* 8 (+ nl d)))"
TA = "(fe_tr fp nl d a lc mem mlo msz)"
TB = "(fe_tr fp nl (+ d 1) b lc mem mlo msz)"

def twin_claim(name, goal, nil_leaf, unary_leaf, bin_leaf):
    """nil_leaf/unary_leaf/bin_leaf: proof text after the goal's fe_tr has been unfolded+reduced
    (unary: goal mentions (fe_tr fp nl d a …); bin: goal mentions (fp_app TB (Cons (FWord ADDR va) TA)))"""
    def ibin():
        arms=[]
        for op in OPS:
            if op in ("IShl","IShr"):
                body=f"""(chain
  {cap('ho', f'(= o {op})')}
  (steps ((rewrite (premise ho) lr both true ()) (unfold fe_tr lhs) (reduce lhs)))
  {unary_leaf})"""
            else:
                body=f"""(chain
  {cap('ho', f'(= o {op})')}
  (steps ((rewrite (premise ho) lr both true ()) (unfold fe_tr lhs) (reduce lhs)))
  (case-on (iexp a lc mem mlo msz) Option
    ((case None
       (chain
         {cap('hva', '(= (iexp a lc mem mlo msz) None)')}
         (steps ((rewrite (premise hva) lr lhs true ()) (reduce lhs)))
         {nil_leaf}))
     (case Some
       (va)
       (chain
         {cap('hva', '(= (iexp a lc mem mlo msz) (Some va))')}
         (steps ((rewrite (premise hva) lr lhs true ()) (reduce lhs)))
         {bin_leaf})))))"""
            arms.append(f"(case {op} {body})")
        return "(case-on o IOp\n  (" + "\n   ".join(arms) + "))"
    unary_pre = "(chain (steps ((unfold fe_tr lhs) (reduce lhs))) {leaf})"
    return f"""(claim {name}
  {goal}
  (induct e
    ((case IConst (n) (chain (steps ((unfold fe_tr lhs) (reduce lhs))) {nil_leaf}))
     (case ILoc (i) (chain (steps ((unfold fe_tr lhs) (reduce lhs))) {nil_leaf}))
     (case IBin (k o a b) {ibin()})
     (case IRotr (k a c) {unary_pre.format(leaf=unary_leaf)})
     (case IExt (k1 k2 a) {unary_pre.format(leaf=unary_leaf)})
     (case ITrunc (k1 kt a) {unary_pre.format(leaf=unary_leaf)})
     (case ILoad (a) {unary_pre.format(leaf=unary_leaf)}))))
"""

def fe_tr_disc():
    goal = """(goal
    ((e IExp) (fp Int) (nl Int) (d Int) (lc (List Int)) (mem Mem) (mlo Int) (msz Int) (slo Int) (psx (List FPatch)))
    ((= (fp_disc slo psx) True)
     (= (le slo fp) True)
     (= (int_eq (mod (- fp slo) 8) 0) True)
     (= (le 0 (+ nl d)) True))
    (= (fp_disc slo (fp_app (fe_tr fp nl d e lc mem mlo msz) psx)) True))"""
    nil_leaf = "(steps ((unfold fp_app lhs) (reduce lhs) (rewrite (premise 0) lr lhs true ())) refl)"
    d4 = "((steps ((rewrite (premise 0) lr lhs true ())) refl) (steps ((rewrite (premise 1) lr lhs true ())) refl) (steps ((rewrite (premise 2) lr lhs true ())) refl) (steps ((rewrite (premise 3) lr lhs true ())) refl))"
    unary_leaf = f"(rewrite-with (hyp ih) lr lhs () {d4} refl)"
    bin_leaf = f"""(chain
  (have hda (= (fp_disc slo (fp_app {TA} psx)) True)
    (rewrite-with (hyp ih) lr lhs () {d4} refl))
  (have hal (= (int_eq (mod (- {ADDR} slo) 8) 0) True)
    (rewrite-with (lemma al_shift) lr lhs ((inst k (+ nl d))) ((steps ((rewrite (premise 2) lr lhs true ())) refl)) refl))
  (have hlo (= (le (+ {ADDR} 8) slo) False) (by arith (list 1 0 1 0 8 0 0 0 0)))
  (have hge (= (le slo {ADDR}) True) (by arith (list 1 0 1 0 8 0 0 0 0 0)))
  (have hdc (= (fp_disc slo (Cons (FWord {ADDR} va) (fp_app {TA} psx))) True)
    (rewrite-with (lemma fp_disc_w_slot) lr lhs ()
      ((steps ((rewrite (premise hlo) lr lhs true ())) refl)
       (steps ((rewrite (premise hge) lr lhs true ())) refl)
       (steps ((rewrite (premise hal) lr lhs true ())) refl)
       (steps ((rewrite (premise hda) lr lhs true ())) refl))
      refl))
  (have hnd (= (le 0 (+ nl (+ d 1))) True) (by arith (list 1 0 0 0 1 0 0 0 0 0 0 0)))
  (have hc (= (fp_app (Cons (FWord {ADDR} va) {TA}) psx) (Cons (FWord {ADDR} va) (fp_app {TA} psx)))
    (steps ((unfold fp_app lhs) (reduce lhs)) refl))
  (steps ((rewrite (lemma fp_app_assoc) lr lhs true ()) (rewrite (premise hc) lr lhs true ())))
  (rewrite-with (hyp ih1) lr lhs ()
    ((steps ((rewrite (premise hdc) lr lhs true ())) refl)
     (steps ((rewrite (premise 1) lr lhs true ())) refl)
     (steps ((rewrite (premise 2) lr lhs true ())) refl)
     (steps ((rewrite (premise hnd) lr lhs true ())) refl)))
  refl)"""
    return twin_claim("fe_tr_disc", goal, nil_leaf, unary_leaf, bin_leaf)

def fe_tr_locs():
    goal = """(goal
    ((e IExp) (fp Int) (nl Int) (d Int) (lc (List Int)) (mem Mem) (mlo Int) (msz Int) (psx (List FPatch)))
    ((= (fr_locs fp lc psx) True) (= (le (ilen lc) nl) True) (= (le 0 d) True))
    (= (fr_locs fp lc (fp_app (fe_tr fp nl d e lc mem mlo msz) psx)) True))"""
    nil_leaf = "(steps ((unfold fp_app lhs) (reduce lhs) (rewrite (premise 0) lr lhs true ())) refl)"
    d3 = "((steps ((rewrite (premise 0) lr lhs true ())) refl) (steps ((rewrite (premise 1) lr lhs true ())) refl) (steps ((rewrite (premise 2) lr lhs true ())) refl))"
    unary_leaf = f"(rewrite-with (hyp ih) lr lhs () {d3} refl)"
    bin_leaf = f"""(chain
  (have hla (= (fr_locs fp lc (fp_app {TA} psx)) True)
    (rewrite-with (hyp ih) lr lhs () {d3} refl))
  (have hpast (= (le (+ fp (* 8 (ilen lc))) {ADDR}) True) (by arith (list 1 0 8 8 0 0 0)))
  (have hlc (= (fr_locs fp lc (Cons (FWord {ADDR} va) (fp_app {TA} psx))) True)
    (rewrite-with (lemma fr_locs_skip) lr lhs ()
      ((steps ((rewrite (premise hla) lr lhs true ())) refl)
       (steps ((rewrite (premise hpast) lr lhs true ())) refl))
      refl))
  (have hnd (= (le 0 (+ d 1)) True) (by arith (list 1 0 0 1 0 0 0 0 0)))
  (have hc (= (fp_app (Cons (FWord {ADDR} va) {TA}) psx) (Cons (FWord {ADDR} va) (fp_app {TA} psx)))
    (steps ((unfold fp_app lhs) (reduce lhs)) refl))
  (steps ((rewrite (lemma fp_app_assoc) lr lhs true ()) (rewrite (premise hc) lr lhs true ())))
  (rewrite-with (hyp ih1) lr lhs ()
    ((steps ((rewrite (premise hlc) lr lhs true ())) refl)
     (steps ((rewrite (premise 1) lr lhs true ())) refl)
     (steps ((rewrite (premise hnd) lr lhs true ())) refl)))
  refl)"""
    return twin_claim("fe_tr_locs", goal, nil_leaf, unary_leaf, bin_leaf)

def fe_tr_below():
    goal = """(goal
    ((e IExp) (fp Int) (nl Int) (d Int) (lc (List Int)) (mem Mem) (mlo Int) (msz Int) (slo Int) (psx (List FPatch)))
    ((= (le slo fp) True) (= (le 0 (+ nl d)) True))
    (= (fbelow slo (fp_app (fe_tr fp nl d e lc mem mlo msz) psx)) (fbelow slo psx)))"""
    nil_leaf = "(steps ((unfold fp_app lhs) (reduce lhs)) refl)"
    d2 = "((steps ((rewrite (premise 0) lr lhs true ())) refl) (steps ((rewrite (premise 1) lr lhs true ())) refl))"
    unary_leaf = f"(rewrite-with (hyp ih) lr lhs () {d2} refl)"
    bin_leaf = f"""(chain
  (have hlo (= (le (+ {ADDR} 8) slo) False) (by arith (list 1 1 8 0 0)))
  (have hnd (= (le 0 (+ nl (+ d 1))) True) (by arith (list 1 0 1 0 0 0)))
  (have hc (= (fp_app (Cons (FWord {ADDR} va) {TA}) psx) (Cons (FWord {ADDR} va) (fp_app {TA} psx)))
    (steps ((unfold fp_app lhs) (reduce lhs)) refl))
  (steps ((rewrite (lemma fp_app_assoc) lr lhs true ()) (rewrite (premise hc) lr lhs true ())))
  (rewrite-with (hyp ih1) lr lhs ()
    ((steps ((rewrite (premise 0) lr lhs true ())) refl)
     (steps ((rewrite (premise hnd) lr lhs true ())) refl)))
  (rewrite-with (lemma fbelow_w_hi) lr lhs ()
    ((steps ((rewrite (premise hlo) lr lhs true ())) refl)))
  (rewrite-with (hyp ih) lr lhs () {d2})
  refl)"""
    return twin_claim("fe_tr_below", goal, nil_leaf, unary_leaf, bin_leaf)

def fe_tr_min():
    goal = """(goal
    ((e IExp) (fp Int) (nl Int) (d Int) (lc (List Int)) (mem Mem) (mlo Int) (msz Int))
    ()
    (= (fp_min (fe_tr fp nl d e lc mem mlo msz) (+ fp (* 8 (+ nl d)))) True))"""
    nil_leaf = "(steps ((unfold fp_min lhs) (reduce lhs)) refl)"
    unary_leaf = "(rewrite-with (hyp ih) lr lhs () () refl)"
    bin_leaf = f"""(chain
  (have hmb (= (fp_min {TB} (+ fp (* 8 (+ nl (+ d 1))))) True)
    (rewrite-with (hyp ih1) lr lhs () () refl))
  (have hle (= (le (+ fp (* 8 (+ nl d))) (+ fp (* 8 (+ nl (+ d 1))))) True) (by arith (list 1 0)))
  (have hmb2 (= (fp_min {TB} (+ fp (* 8 (+ nl d)))) True)
    (rewrite-with (lemma fp_min_weaken) lr lhs ((inst lo (+ fp (* 8 (+ nl (+ d 1))))))
      ((steps ((rewrite (premise hmb) lr lhs true ())) refl)
       (steps ((rewrite (premise hle) lr lhs true ())) refl))
      refl))
  (have hma (= (fp_min {TA} (+ fp (* 8 (+ nl d)))) True)
    (rewrite-with (hyp ih) lr lhs () () refl))
  (have hself (= (le {ADDR} {ADDR}) True) (by arith (list 1 0 0 0 0)))
  (have hmc (= (fp_min (Cons (FWord {ADDR} va) {TA}) (+ fp (* 8 (+ nl d)))) True)
    (steps
      ((rewrite (lemma fp_min_unf_w) lr lhs true ())
       (rewrite (premise hself) lr lhs true ())
       (reduce lhs)
       (rewrite (premise hma) lr lhs true ()))
      refl))
  (rewrite-with (lemma fp_min_app) lr lhs ()
    ((steps ((rewrite (premise hmb2) lr lhs true ())) refl)
     (steps ((rewrite (premise hmc) lr lhs true ())) refl)))
  refl)"""
    return twin_claim("fe_tr_min", goal, nil_leaf, unary_leaf, bin_leaf)

def twin_laws():
    return "\n".join([fe_tr_disc(), fe_tr_locs(), fe_tr_below(), fe_tr_min()])

# ---------------- fe_band: every imp expression value is in [0, 2^64) ----------------

INST_ALL = "((inst lc lc) (inst mem mem) (inst mlo mlo) (inst msz msz) (inst fp fp) (inst psx psx))"

def d(name):  # a discharge by a named fact
    return f"(steps ((rewrite (premise {name}) lr lhs true ())) refl)"

def band_ih(hname, hyp, hv, hcb):
    return f"""(have {hname} (= (fe_inband {hv.split()[0] if False else hname[2:]}) True)
    (rewrite-with (hyp {hyp}) lr lhs {INST_ALL} ({d(hv)} {d('1')} {d(hcb)}) refl))"""

def unfold_prem(rewrites):
    """(rewrite (premise 0) rl rhs) + unfold iexp + reduce + the captured eqs"""
    items=["(rewrite (premise 0) rl rhs true ())","(unfold iexp rhs)","(reduce rhs)"]
    for r in rewrites:
        items.append(r); items.append("(reduce rhs)")
    return " ".join(items)

def absurd_some(rewrites):
    return f"""(chain
  (have hn (= None (Some v)) (steps ({unfold_prem(rewrites)}) refl))
  (absurd (premise hn)))"""

def close_with(term, rewrites, tail):
    """hv: (Some TERM) = (Some v) by the unfolded premise; inject; rewrite v -> TERM; then `tail` closes"""
    return f"""(chain
  (have hv (= (Some {term}) (Some v)) (steps ({unfold_prem(rewrites)}) refl))
  (inject (premise hv) (hvv))
  (steps ((rewrite (premise hvv) rl lhs true ())))
  {tail})"""

def fe_band():
    RA = "(rewrite (premise hva) lr rhs true ())"
    RB = "(rewrite (premise hvb) lr rhs true ())"
    RO = "(rewrite (premise ho) lr rhs true ())"
    ihs = f"""(have hba (= (fe_inband va) True)
    (rewrite-with (hyp ih) lr lhs {INST_ALL} ({d('hva')} {d('1')} {d('hcba')}) refl))
  (have hbb (= (fe_inband vb) True)
    (rewrite-with (hyp ih1) lr lhs {INST_ALL} ({d('hvb')} {d('1')} {d('hcbb')}) refl))
  (have hal (= (le 0 va) True) (rewrite-with (lemma inband_lo) lr lhs () ({d('hba')}) refl))
  (have hbl (= (le 0 vb) True) (rewrite-with (lemma inband_lo) lr lhs () ({d('hbb')}) refl))
  (have hbh (= (lt vb 18446744073709551616) True) (rewrite-with (lemma inband_hi) lr lhs () ({d('hbb')}) refl))"""
    def op_leaf(op):
        base = [RA, RB, RO, "(unfold iop_val rhs)"]
        if op in ("IAdd","ISub","IMul"):
            sym={"IAdd":"+","ISub":"-","IMul":"*"}[op]
            return close_with(f"(mod ({sym} va vb) (ikmod k))", base,
                              "(rewrite-with (lemma kmod_inband) lr lhs () () refl)")
        if op=="IDiv":
            return f"""(case-on (le 1 vb) Bool
  ((case False (chain {cap('hle','(= (le 1 vb) False)')} {absurd_some(base+['(rewrite (premise hle) lr rhs true ())'])}))
   (case True (chain {cap('hle','(= (le 1 vb) True)')}
     {close_with('(ediv va vb)', base+['(rewrite (premise hle) lr rhs true ())'],
                 f"(rewrite-with (lemma ediv_inband) lr lhs () ({d('hba')} {d('hle')}) refl)")}))))"""
        if op=="IRem":
            return f"""(case-on (le 1 vb) Bool
  ((case False (chain {cap('hle','(= (le 1 vb) False)')} {absurd_some(base+['(rewrite (premise hle) lr rhs true ())'])}))
   (case True (chain {cap('hle','(= (le 1 vb) True)')}
     {close_with('(mod va vb)', base+['(rewrite (premise hle) lr rhs true ())'],
                 f"(rewrite-with (lemma rem_inband) lr lhs () ({d('hle')} {d('hbh')}) refl)")}))))"""
        if op=="IAnd":
            return close_with("(band va vb)", base, f"(rewrite-with (lemma band_inband) lr lhs () ({d('hba')} {d('hbl')}) refl)")
        if op=="IOr":
            return close_with("(bor va vb)", base, f"(rewrite-with (lemma bor_inband) lr lhs () ({d('hba')} {d('hbb')}) refl)")
        if op=="IXor":
            return close_with("(bxor va vb)", base, f"(rewrite-with (lemma bxor_inband) lr lhs () ({d('hba')} {d('hbb')}) refl)")
        if op in ("IShl","IShr"):
            inner_term = "(mod (bshl va vb) (ikmod k))" if op=="IShl" else "(bshr va vb)"
            tail = "(rewrite-with (lemma kmod_inband) lr lhs () () refl)" if op=="IShl" else f"(rewrite-with (lemma shr_inband) lr lhs () ({d('hba')} {d('hs0')}) refl)"
            r0="(rewrite (premise hs0) lr rhs true ())"; r1="(rewrite (premise hs1) lr rhs true ())"
            return f"""(case-on (le 0 vb) Bool
  ((case False (chain {cap('hs0','(= (le 0 vb) False)')} {absurd_some(base+[r0])}))
   (case True (chain {cap('hs0','(= (le 0 vb) True)')}
     (case-on (lt vb (ikw k)) Bool
       ((case False (chain {cap('hs1','(= (lt vb (ikw k)) False)')} {absurd_some(base+[r0,r1])}))
        (case True (chain {cap('hs1','(= (lt vb (ikw k)) True)')}
          {close_with(inner_term, base+[r0,r1], tail)}))))))))"""
        cmp={"IEq":"int_eq","ILt":"lt","ILe":"le"}[op]
        return close_with(f"(ib2i ({cmp} va vb))", base, "(rewrite-with (lemma ib2i_inband) lr lhs () () refl)")
    arms="\n   ".join(f"(case {op} (chain {cap('ho', f'(= o {op})')} {op_leaf(op)}))" for op in OPS)
    ibin=f"""(chain
  (have hcba (= (ixf_cb a) True)
    (rewrite-with (lemma cb_bin_a) lr lhs ((inst k k) (inst o o) (inst b b)) ({d('2')}) refl))
  (have hcbb (= (ixf_cb b) True)
    (rewrite-with (lemma cb_bin_b) lr lhs ((inst k k) (inst o o) (inst a a)) ({d('2')}) refl))
  (case-on (iexp a lc mem mlo msz) Option
    ((case None (chain {cap('hva','(= (iexp a lc mem mlo msz) None)')} {absurd_some([RA])}))
     (case Some (va)
       (chain
         {cap('hva','(= (iexp a lc mem mlo msz) (Some va))')}
         (case-on (iexp b lc mem mlo msz) Option
           ((case None (chain {cap('hvb','(= (iexp b lc mem mlo msz) None)')} {absurd_some([RA,RB])}))
            (case Some (vb)
              (chain
                {cap('hvb','(= (iexp b lc mem mlo msz) (Some vb))')}
                {ihs}
                (case-on o IOp
                  ({arms})))))))))))"""
    iconst=f"""(chain
  (have hsome (= (Some n) (Some v)) (steps ({unfold_prem([])}) refl))
  (inject (premise hsome) (hnv))
  (have hcb (= (fe_inband n) True) (rewrite-with (lemma cb_const) lr lhs () ({d('2')}) refl))
  (steps ((rewrite (premise hnv) rl lhs true ()) (rewrite (premise hcb) lr lhs true ())) refl))"""
    iloc=f"""(chain
  (have hget (= (ilget lc i) (Some v)) (steps ({unfold_prem([])}) refl))
  (have hlo (= (le 0 v) True)
    (rewrite-with (lemma fr_get_lo) lr lhs ((inst fp fp) (inst lc lc) (inst ps psx) (inst i i)) ({d('1')} {d('hget')}) refl))
  (have hhi (= (lt v 18446744073709551616) True)
    (rewrite-with (lemma fr_get_hi) lr lhs ((inst fp fp) (inst lc lc) (inst ps psx) (inst i i)) ({d('1')} {d('hget')}) refl))
  (rewrite-with (lemma inband_intro) lr lhs () ({d('hlo')} {d('hhi')}))
  refl)"""
    RC = "(rewrite (premise hvc) lr rhs true ())"
    V1 = "(band va (- (ikmod k) 1))"
    M1 = "(mod vc (ikw k))"
    M2 = "(mod (- (ikw k) (mod vc (ikw k))) (ikw k))"
    ROT = f"(band (bor (bshr {V1} {M1}) (bshl {V1} {M2})) (- (ikmod k) 1))"
    irotr=f"""(chain
  (have hcba (= (ixf_cb a) True)
    (rewrite-with (lemma cb_rotr_a) lr lhs ((inst k k) (inst c c)) ({d('2')}) refl))
  (case-on (iexp a lc mem mlo msz) Option
    ((case None (chain {cap('hva','(= (iexp a lc mem mlo msz) None)')} {absurd_some([RA])}))
     (case Some (va)
       (chain
         {cap('hva','(= (iexp a lc mem mlo msz) (Some va))')}
         (case-on (iexp c lc mem mlo msz) Option
           ((case None (chain {cap('hvc','(= (iexp c lc mem mlo msz) None)')} {absurd_some([RA,RC])}))
            (case Some (vc)
              (chain
                {cap('hvc','(= (iexp c lc mem mlo msz) (Some vc))')}
                (have hba (= (fe_inband va) True)
                  (rewrite-with (hyp ih) lr lhs {INST_ALL} ({d('hva')} {d('1')} {d('hcba')}) refl))
                (have hal (= (le 0 va) True) (rewrite-with (lemma inband_lo) lr lhs () ({d('hba')}) refl))
                (have hw (= (lt 0 (ikw k)) True) (rewrite-with (lemma ikw_pos) lr lhs () () refl))
                (have hv1 (= (fe_inband {V1}) True) (rewrite-with (lemma mask_inband) lr lhs () ({d('hal')}) refl))
                (have hv1l (= (le 0 {V1}) True) (rewrite-with (lemma inband_lo) lr lhs () ({d('hv1')}) refl))
                (have hm1 (= (le 0 {M1}) True) (rewrite-with (lemma mod_lo) lr lhs ((inst n vc) (inst d (ikw k))) ({d('hw')}) refl))
                (have hm2 (= (le 0 {M2}) True) (rewrite-with (lemma mod_lo) lr lhs ((inst n (- (ikw k) (mod vc (ikw k)))) (inst d (ikw k))) ({d('hw')}) refl))
                (have hsr (= (fe_inband (bshr {V1} {M1})) True) (rewrite-with (lemma shr_inband) lr lhs () ({d('hv1')} {d('hm1')}) refl))
                (have hsrl (= (le 0 (bshr {V1} {M1})) True) (rewrite-with (lemma inband_lo) lr lhs () ({d('hsr')}) refl))
                (have hsl (= (le 0 (bshl {V1} {M2})) True) (rewrite-with (lemma shl_nonneg) lr lhs () ({d('hv1l')} {d('hm2')}) refl))
                (have hor (= (le 0 (bor (bshr {V1} {M1}) (bshl {V1} {M2}))) True) (rewrite-with (lemma bor_lo) lr lhs () ({d('hsrl')} {d('hsl')}) refl))
                {close_with(ROT, [RA, RC], f"(rewrite-with (lemma mask_inband) lr lhs () ({d('hor')}) refl)")})))))))))"""
    iext=f"""(chain
  (have hcb (= (ixf_cb a) True) (rewrite-with (lemma cb_ext) lr lhs ((inst k1 k1) (inst k2 k2)) ({d('2')}) refl))
  (have hp (= (iexp a lc mem mlo msz) (Some v)) (steps ({unfold_prem([])}) refl))
  (rewrite-with (hyp ih) lr lhs {INST_ALL} ({d('hp')} {d('1')} {d('hcb')}))
  refl)"""
    itrunc=f"""(chain
  (have hcb (= (ixf_cb a) True) (rewrite-with (lemma cb_trunc) lr lhs ((inst k1 k1) (inst kt kt)) ({d('2')}) refl))
  (case-on (iexp a lc mem mlo msz) Option
    ((case None (chain {cap('hva','(= (iexp a lc mem mlo msz) None)')} {absurd_some([RA])}))
     (case Some (va)
       (chain
         {cap('hva','(= (iexp a lc mem mlo msz) (Some va))')}
         (have hba (= (fe_inband va) True)
           (rewrite-with (hyp ih) lr lhs {INST_ALL} ({d('hva')} {d('1')} {d('hcb')}) refl))
         (have hal (= (le 0 va) True) (rewrite-with (lemma inband_lo) lr lhs () ({d('hba')}) refl))
         {close_with('(band va (- (ikmod kt) 1))', [RA], f"(rewrite-with (lemma mask_inband) lr lhs () ({d('hal')}) refl)")})))))"""
    RL0="(rewrite (premise hl0) lr rhs true ())"; RL1="(rewrite (premise hl1) lr rhs true ())"
    iload=f"""(chain
  (have hcb (= (ixf_cb a) True) (rewrite-with (lemma cb_load) lr lhs () ({d('2')}) refl))
  (case-on (iexp a lc mem mlo msz) Option
    ((case None (chain {cap('hva','(= (iexp a lc mem mlo msz) None)')} {absurd_some([RA])}))
     (case Some (va)
       (chain
         {cap('hva','(= (iexp a lc mem mlo msz) (Some va))')}
         (case-on (le mlo va) Bool
           ((case False (chain {cap('hl0','(= (le mlo va) False)')} {absurd_some([RA,RL0])}))
            (case True
              (chain
                {cap('hl0','(= (le mlo va) True)')}
                (case-on (lt va msz) Bool
                  ((case False (chain {cap('hl1','(= (lt va msz) False)')} {absurd_some([RA,RL0,RL1])}))
                   (case True
                     (chain
                       {cap('hl1','(= (lt va msz) True)')}
                       {close_with('(mem_get mem va)', [RA,RL0,RL1], "(rewrite-with (lemma get_inband) lr lhs () () refl)")})))))))))))))"""
    global _band_pieces
    _band_pieces = dict(iconst=iconst, iloc=iloc, ibin=ibin, irotr=irotr, iext=iext, itrunc=itrunc, iload=iload)
    return f""";; THE VALUE BAND (generated by gen_fra.py fe_band — REGENERATE, never hand-patch)
(claim fe_band
  (goal ((e IExp) (lc (List Int)) (mem Mem) (mlo Int) (msz Int) (v Int) (fp Int) (psx (List FPatch)))
    ((= (iexp e lc mem mlo msz) (Some v)) (= (fr_locs fp lc psx) True) (= (ixf_cb e) True))
    (= (fe_inband v) True))
  (induct e
    ((case IConst (n) {iconst})
     (case ILoc (i) {iloc})
     (case IBin (k o a b) {ibin})
     (case IRotr (k a c) {irotr})
     (case IExt (k1 k2 a) {iext})
     (case ITrunc (k1 kt a) {itrunc})
     (case ILoad (a) {iload}))))
"""


# ---------------- CLI ----------------

BANNERS = {
    "fe_len": (";; the emission's length is ixf_elen (generated by gen_fra.py — REGENERATE, never hand-patch)", "fe_len"),
    "twin": (";; --- the twin's four laws (generated by gen_fra.py twin_laws — REGENERATE, never hand-patch) ---", "twin_laws"),
    "fe_band": (";; THE VALUE BAND (generated by gen_fra.py fe_band — REGENERATE, never hand-patch)", "fe_band"),
}

def emit(which):
    return {"fe_len": fe_len, "twin_laws": twin_laws, "fe_band": fe_band}[which]()

def splice(path="models/imp/probes/fra_kit.shard"):
    import re
    s = open(path).read()
    for key, (banner, fn) in BANNERS.items():
        i = s.find(banner)
        if i < 0:
            print("banner not found:", key); continue
        new = emit(fn)
        # the block ends at the next top-level form that is not part of the emitted text
        tail = s[i:]
        # the emitted text starts with its banner; find the end of the OLD block: the next banner or "\n;; ===" section header
        ends = [tail.find(b, 1) for b in [bb for bb, _ in BANNERS.values()] + ["\n;; ====", "\n;; --- the twin's laws"]]
        ends = [e for e in ends if e > 0]
        j = i + (min(ends) if ends else len(tail))
        # keep the banner line as emitted by the generator
        s = s[:i] + new.rstrip("\n") + "\n\n" + s[j:].lstrip("\n")
    open(path, "w").write(s)

if __name__ == "__main__":
    arg = sys.argv[1] if len(sys.argv) > 1 else "fe_len"
    if arg == "splice":
        splice()
    else:
        sys.stdout.write(emit({"fe_len": "fe_len", "twin": "twin_laws", "fe_band": "fe_band"}[arg]))
