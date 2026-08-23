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
  python3 models/imp/probes/gen_fra.py MODE      (MODE = a key of BANNERS: fe_len twin fe_band
                                              unary binary sound stmt — the block to stdout)
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
    # the U32 bitwise trio is refused by the frame tier (A-2): its U32 arm is an absurd too
    if op in ("IAnd", "IOr", "IXor"):
        u32 = f"""(chain
  {cap('hk', '(= k U32)')}
  {absurd_none('hp', [ha, hb, '(rewrite (premise hk) lr rhs true ())', '(unfold ixf_bop rhs)'])})"""
    else:
        u32 = kleaf('U32', 'XBin32')
    return f"""(case-on k IKind
  ((case U8 {u8})
   (case U32 {u32})
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
    (unary: goal mentions (fe_tr fp nl d a …); bin: goal mentions (fp_app TB (Cons (FWord ADDR va) TA)));
    unary_leaf may be a function (E, B2) -> text of the constructor form E and its second operand B2 (or None)"""
    def UL(E, B2): return unary_leaf(E, B2) if callable(unary_leaf) else unary_leaf
    def BL(op): return bin_leaf(op) if callable(bin_leaf) else bin_leaf
    def ibin():
        arms=[]
        for op in OPS:
            if op in ("IShl","IShr"):
                body=f"""(chain
  {cap('ho', f'(= o {op})')}
  (steps ((rewrite (premise ho) lr both true ()) (unfold fe_tr lhs) (reduce lhs)))
  {UL(f'(IBin k {op} a b)', 'b')})"""
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
         {BL(op)})))))"""
            arms.append(f"(case {op} {body})")
        return "(case-on o IOp\n  (" + "\n   ".join(arms) + "))"
    unary_pre = "(chain (steps ((unfold fe_tr lhs) (reduce lhs))) {leaf})"
    return f"""(claim {name}
  {goal}
  (induct e
    ((case IConst (n) (chain (steps ((unfold fe_tr lhs) (reduce lhs))) {nil_leaf}))
     (case ILoc (i) (chain (steps ((unfold fe_tr lhs) (reduce lhs))) {nil_leaf}))
     (case IBin (k o a b) {ibin()})
     (case IRotr (k a c) {unary_pre.format(leaf=UL('(IRotr k a c)', 'c'))})
     (case IExt (k1 k2 a) {unary_pre.format(leaf=UL('(IExt k1 k2 a)', None))})
     (case ITrunc (k1 kt a) {unary_pre.format(leaf=UL('(ITrunc k1 kt a)', None))})
     (case ILoad (a) {unary_pre.format(leaf=UL('(ILoad a)', None))}))))
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

def fe_tr_max():
    """the spill trace's UPPER bound (A-5): every spill ends at or below fp + 8(nl + d + dep e) —
    with fe_tr_min, the argument slots at fp + own + 8j (own >= 8(nl + 1 + dep)) are never a spill"""
    def HI(E): return f"(+ fp (* 8 (+ nl (+ d (ixf_dep {E})))))"
    goal = f"""(goal
    ((e IExp) (fp Int) (nl Int) (d Int) (lc (List Int)) (mem Mem) (mlo Int) (msz Int))
    ()
    (= (fp_max (fe_tr fp nl d e lc mem mlo msz) {HI('e')}) True))"""
    nil_leaf = "(steps ((unfold fp_max lhs) (reduce lhs)) refl)"
    def unary_leaf(E, B2):
        if B2 is None:
            hdl = f"(have hdl (= (ixf_dep {E}) (ixf_dep a)) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))"
            cert = "(list 1 0 8)"
        else:
            hdl = f"""(have hdl (= (ixf_dep {E}) (+ 1 (imax2 (ixf_dep a) (ixf_dep {B2})))) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))
  (have hm (= (le (ixf_dep a) (imax2 (ixf_dep a) (ixf_dep {B2}))) True) (rewrite-with (lemma imax2_ge_l) lr lhs () () refl))"""
            cert = "(list 1 0 0 8 8)" if E.startswith("(IBin") else "(list 1 0 8 8)"   # the shift arms carry the `ho` slot
        return f"""(chain
  (have hma (= (fp_max (fe_tr fp nl d a lc mem mlo msz) {HI('a')}) True) (rewrite-with (hyp ih) lr lhs () () refl))
  {hdl}
  (have hle (= (le {HI('a')} {HI(E)}) True) (by arith {cert}))
  (rewrite-with (lemma fp_max_weaken) lr lhs ((inst hi {HI('a')}))
    ((steps ((rewrite (premise hma) lr lhs true ())) refl)
     (steps ((rewrite (premise hle) lr lhs true ())) refl)))
  refl)"""
    HIb = "(+ fp (* 8 (+ nl (+ (+ d 1) (ixf_dep b)))))"
    HIX = "(+ fp (* 8 (+ nl (+ d (+ 1 (imax2 (ixf_dep a) (ixf_dep b)))))))"
    # slots in the bin leaf: ho hva | hmb hma hdep hga hgb hda hleb hmb2 hlea hma2 hself hmc
    def bin_leaf(op): return f"""(chain
  (have hmb (= (fp_max {TB} {HIb}) True) (rewrite-with (hyp ih1) lr lhs () () refl))
  (have hma (= (fp_max {TA} {HI('a')}) True) (rewrite-with (hyp ih) lr lhs () () refl))
  (have hdep (= (ixf_dep (IBin k {op} a b)) (+ 1 (imax2 (ixf_dep a) (ixf_dep b)))) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))
  (have hga (= (le (ixf_dep a) (imax2 (ixf_dep a) (ixf_dep b))) True) (rewrite-with (lemma imax2_ge_l) lr lhs () () refl))
  (have hgb (= (le (ixf_dep b) (imax2 (ixf_dep a) (ixf_dep b))) True) (rewrite-with (lemma imax2_ge_r) lr lhs () () refl))
  (have hda (= (le 0 (ixf_dep a)) True) (rewrite-with (lemma ixf_dep_nonneg) lr lhs () () refl))
  (have hleb (= (le {HIb} {HIX}) True) (by arith (list 1 0 0 0 0 0 0 8 0)))
  (have hmb2 (= (fp_max {TB} {HIX}) True)
    (rewrite-with (lemma fp_max_weaken) lr lhs ((inst hi {HIb}))
      ((steps ((rewrite (premise hmb) lr lhs true ())) refl)
       (steps ((rewrite (premise hleb) lr lhs true ())) refl))
      refl))
  (have hlea (= (le {HI('a')} {HIX}) True) (by arith (list 1 0 0 0 0 0 8 0 0)))
  (have hma2 (= (fp_max {TA} {HIX}) True)
    (rewrite-with (lemma fp_max_weaken) lr lhs ((inst hi {HI('a')}))
      ((steps ((rewrite (premise hma) lr lhs true ())) refl)
       (steps ((rewrite (premise hlea) lr lhs true ())) refl))
      refl))
  (have hself (= (le (+ {ADDR} 8) {HIX}) True) (by arith (list 1 0 0 0 0 0 8 0 8)))
  (have hmc (= (fp_max (Cons (FWord {ADDR} va) {TA}) {HIX}) True)
    (steps
      ((rewrite (lemma fp_max_unf_w) lr lhs true ())
       (rewrite (premise hself) lr lhs true ())
       (reduce lhs)
       (rewrite (premise hma2) lr lhs true ()))
      refl))
  (steps ((rewrite (premise hdep) lr lhs true ())))
  (rewrite-with (lemma fp_max_app) lr lhs ()
    ((steps ((rewrite (premise hmb2) lr lhs true ())) refl)
     (steps ((rewrite (premise hmc) lr lhs true ())) refl)))
  refl)"""
    return twin_claim("fe_tr_max", goal, nil_leaf, unary_leaf, bin_leaf)

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

# every generated block of fra_kit.shard: mode -> (its banner line in the file, the emitter's
# name — resolved at call time, the emitters are defined below in file order)
BANNERS = {
    "fe_len": (";; the emission's length is ixf_elen (generated by gen_fra.py — REGENERATE, never hand-patch)", "fe_len"),
    "twin": (";; --- the twin's four laws (generated by gen_fra.py twin_laws — REGENERATE, never hand-patch) ---", "twin_laws"),
    "fe_band": (";; THE VALUE BAND (generated by gen_fra.py fe_band — REGENERATE, never hand-patch)", "fe_band"),
    "unary": (";; --- STEP lemmas, unary-tail family (generated by gen_fra.py unary — REGENERATE, never hand-patch) ---", "step_lemmas_unary"),
    "binary": (";; --- STEP lemmas, binary family (generated by gen_fra.py binary — REGENERATE, never hand-patch) ---", "step_lemmas_binary"),
    "sound": (";; THE EXPRESSION LEMMA (generated by gen_fra.py fe_sound — REGENERATE, never hand-patch):", "fe_sound"),
    "stmt": (";; --- STEP lemmas, statements (generated by gen_fra.py stmt — REGENERATE, never hand-patch) ---", "step_lemmas_stmt"),
    # A-5: the fifth twin law lives AFTER the imax2 / ixf_dep_nonneg / fp_max laws it cites (citations resolve in file order)
    "trmax": (";; --- the twin's fifth law: the spill bound fe_tr_max (generated by gen_fra.py trmax — REGENERATE, never hand-patch) ---", "fe_tr_max"),
}

def emit(which):
    return globals()[which]()

def splice(path="models/imp/probes/fra_kit.shard"):
    import re
    s = open(path).read()
    for key, (banner, fn) in BANNERS.items():
        i = s.find(banner)
        if i < 0:
            print("banner not found:", key); continue
        new = emit(fn)
        if not new.startswith(banner):
            new = banner + "\n\n" + new
        # the block ends at the next top-level form that is not part of the emitted text
        tail = s[i:]
        # the emitted text starts with its banner; find the end of the OLD block: the next banner or "\n;; ===" section header
        # ... or the next hand section header (";; --- ..." / ";; ===="): no emitter writes one
        assert "\n;; --- " not in new and "\n;; ====" not in new, key
        ends = [tail.find(b, 1) for b in [bb for bb, _ in BANNERS.values()] + ["\n;; ====", "\n;; --- "]]
        ends = [e for e in ends if e > 0]
        j = i + (min(ends) if ends else len(tail))
        # keep the banner line as emitted by the generator
        s = s[:i] + new.rstrip("\n") + "\n\n" + s[j:].lstrip("\n")
    open(path, "w").write(s)


# ---------------- the expression lemma's STEP LEMMAS (unary-tail family) ----------------
# One lemma per constructor; the sub-run's IH is an explicit premise. Slot bookkeeping for
# Farkas certs is done by a Slots tracker (premises then haves in order).

RS = "(MkRegs a0 rcx dx rbx rbp rsi di r8 r9 s10 s11 r12 r13 dep fp)"
XM = "(fp_mem mem0 psx)"
IM = "(fp_mem mem0 (fbelow slo psx))"
def RUN(ie): return f"(xeval_seq (xt c g) m {ie} {RS} {XM})"
def MA(sub="a"): return f"(fp_mem mem0 (fp_app (fe_tr fp nl d {sub} lc {IM} mlo slo) psx))"
def ME(e): return f"(fp_mem mem0 (fp_app (fe_tr fp nl d {e} lc {IM} mlo slo) psx))"
GOALVARS = ("(nl Int) (d Int) (ie (List XInstr)) (lc (List Int)) (mem0 Mem) (psx (List FPatch)) "
            "(mlo Int) (slo Int) (v Int) (c Int) (g Nat) (m XModule) (fp Int) (dep Int) "
            "(a0 Int) (rcx Int) (dx Int) (rbx Int) (rbp Int) (rsi Int) (di Int) (r8 Int) (r9 Int) "
            "(s10 Int) (s11 Int) (r12 Int) (r13 Int)")
ACC14 = ["rcx","rdx","rbx","rbp","rsi","rdi","r8","r9","r10","r11","r12","r13","r14","r15"]
STOPS_L = "(stop xt xo_regs xeval_seq fp_mem fbelow fe_tr fp_app rdx_of r10_of r11_of xil xmemlo_of xmemhi_of)"

class Slots:
    def __init__(self, premises):
        self.names = list(premises)   # index i -> slot i+1
    def add(self, name): self.names.append(name)
    def cert(self, m, G=1):
        l = [0]*(len(self.names)+1); l[0]=G
        for k,v in m.items():
            l[self.names.index(k)+1] = v
        return "(list " + " ".join(str(x) for x in l) + ")"
    def cert2(self, le, ge):
        return "(list " + self.cert(le) + " " + self.cert(ge) + ")"

def common_premises(e):
    return f"""((= (ixf_exp nl d {e}) (Some ie))
     (= (iexp {e} lc {IM} mlo slo) (Some v))
     (= (fe_ctx m mlo slo fp nl d lc psx) True)
     (= (ixf_cb {e}) True)
     (= (le (+ fp (* 8 (+ nl (+ d (ixf_dep {e}))))) (xmemhi_of m)) True)
     (= (le (ixf_ecost {e}) c) True)"""

def ih_premises(sub="a", iesub="ia", vsub="va"):
    return f"""     (= (ixf_exp nl d {sub}) (Some {iesub}))
     (= (iexp {sub} lc {IM} mlo slo) (Some {vsub}))
     (= {RUN(iesub)} (fe_out {vsub} {RS} {RUN(iesub)} {MA(sub)}))"""

def conclusion(e):
    return f"""    (=
      {RUN('ie')}
      (fe_out v {RS} {RUN('ie')} {ME(e)})))"""

def hrun_rhs(rax, mem, sub_run, extra_fields=None):
    """the re-spelled run: RAX := rax, the scratch as projections of the sub-run"""
    f = extra_fields or {}
    fields = dict(rdx=f"(rdx_of (xo_regs {sub_run}))", r10=f"(r10_of (xo_regs {sub_run}))", r11=f"(r11_of (xo_regs {sub_run}))")
    fields.update(f)
    return (f"(Some (XNorm (MkRegs {rax} rcx {fields['rdx']} rbx rbp rsi di r8 r9 {fields['r10']} {fields['r11']} r12 r13 dep fp) {mem}))")

def tail_steps(n_instr, tail_extra=None):
    """unfold/reduce choreography for n straight-line tail instructions then Nil;
    tail_extra = rewrites inserted after the FIRST instruction's compute (window guards etc.)"""
    items=[]
    for i in range(n_instr):
        items += ["(unfold xeval_seq lhs)","(reduce lhs)","(unfold xeval_instr lhs)","(reduce lhs)",f"(compute lhs {STOPS_L})"]
        if i == 0 and tail_extra: items += tail_extra
        items += ["(reduce lhs)"]
    items += ["(unfold xeval_seq lhs)","(reduce lhs)",f"(compute lhs {STOPS_L})"]
    return items

def closing(extra_rhs_rewrites):
    acc = "\n".join(f"       (rewrite (lemma {f}_of_mk) lr rhs true ())" for f in ACC14)
    ex = "\n".join("       "+r for r in extra_rhs_rewrites)
    return f"""    (steps
      ((rewrite (premise hrun) lr both true ())
{ex}
       (rewrite (lemma fe_out_of_norm) lr rhs true ())
{acc}
       (unfold fe_tr rhs)
       (reduce rhs))
      refl)"""

def step_unary(name, e, binders, tail_list, n_tail, k_len, imp_value, imp_cases, x_rax, rax_rewrites, extra_haves_fn, extra_rhs_rewrites, extra_premises="", extra_goal_vars="", extra_unfolds=None, tail_extra=None):
    """
    e: the ctor term; binders: goal binders for its fields; tail_list: the emitted tail as spelled
    (a (list …) or Nil); n_tail: instruction count; k_len: elen e = elen a + k_len;
    imp_value: the Some-payload the reducer yields for iexp e (after imp_cases' rewrites);
    imp_cases: list of (guard, name) case splits on the imp side (False arm absurd);
    x_rax: RAX as the x86 leaves it after the tail (spelled as compute leaves it, before rax_rewrites);
    rax_rewrites: rewrites (lr lhs) turning x_rax into the form matching imp_value's rewrite;
    extra_haves_fn(slots) -> list of have-text lines appended before hrun (may add slots).
    """
    premises = ["p0","p1","p2","p3","p4","p5","p6","p7","p8"]
    sl = Slots(premises)
    out=[]
    out.append(f"""(claim {name}
  (goal
    ({binders} (ia (List XInstr)) (va Int) {GOALVARS}{extra_goal_vars})
    ({common_premises(e)[1:]}
{ih_premises()}{extra_premises})
{conclusion(e)}
  (chain""")
    # hsome: the emission
    out.append(f"""    (have hsome (= (Some (ix_app ia {tail_list})) (Some ie))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_exp rhs) (reduce rhs) (rewrite (premise 6) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hsome) (hie))""")
    sl.add("hsome"); sl.add("hie")
    # imp-side case splits
    case_rw = list(extra_unfolds or [])
    for guard, nm in imp_cases:
        # each split: (case-on GUARD Bool ((case False absurd) (case True continue…))) — we open the True arm and continue the chain inside
        out.append(f"""    (case-on {guard} Bool
      ((case False
         (chain
           {cap(nm, f'(= {guard} False)')}
           (have hn (= None (Some v))
             (steps ((rewrite (premise 1) rl rhs true ()) (unfold iexp rhs) (reduce rhs) (rewrite (premise 7) lr rhs true ()) (reduce rhs){''.join(' '+r for r in case_rw)} (rewrite (premise {nm}) lr rhs true ()) (reduce rhs)) refl))
           (absurd (premise hn))))
       (case True
         (chain
           {cap(nm, f'(= {guard} True)')}""")
        sl.add(nm)
        case_rw.append(f"(rewrite (premise {nm}) lr rhs true ()) (reduce rhs)")
    # hv: the value
    out.append(f"""    (have hv (= (Some {imp_value}) (Some v))
      (steps ((rewrite (premise 1) rl rhs true ()) (unfold iexp rhs) (reduce rhs) (rewrite (premise 7) lr rhs true ()) (reduce rhs){''.join(' '+r for r in case_rw)}) refl))
    (inject (premise hv) (hvv))""")
    sl.add("hv"); sl.add("hvv")
    # lengths and towers
    costrhs = f"(+ (+ (ixf_elen a) {k_len}) 5)" if k_len else "(+ (ixf_elen a) 5)"
    out.append(f"""    (have hlen (= (xil ia) (ixf_elen a))
      (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d d) (inst e a)) ((steps ((rewrite (premise 6) lr lhs true ())) refl)) refl))
    (have hcost (= (ixf_ecost {e}) {costrhs})
      (steps ((unfold ixf_ecost lhs) (unfold ixf_elen lhs) (reduce lhs)) refl))""")
    sl.add("hlen"); sl.add("hcost")
    out.append(f"    (have hle (= (le (xil ia) c) True) (by arith {sl.cert({'p5':1,'hlen':-1,'hcost':1})}))")
    sl.add("hle")
    npeel = n_tail + 1
    hcN_goal = "(lt 0 (- c (xil ia)))" if npeel == 1 else f"(le {npeel} (- c (xil ia)))"
    out.append(f"    (have hcN (= {hcN_goal} True) (by arith {sl.cert({'p5':1,'hlen':-1,'hcost':1})}))")
    sl.add("hcN")
    # extra haves (value alignment facts etc.)
    for line in extra_haves_fn(sl):
        out.append(line)
    peel = {1:"xt_peel",2:"xt_peel2",3:"xt_peel3",4:"xt_peel4",8:"xt_peel8"}[npeel]
    rr = "".join(f" {r}" for r in rax_rewrites)
    out.append(f"""    (have hrun (= {RUN('ie')} {hrun_rhs(x_rax, MA('a'), RUN('ia'))})
      (chain
        (steps ((rewrite (premise hie) rl lhs true ())))
        (rewrite-with (lemma xseq_app) lr lhs () ((steps ((rewrite (premise hle) lr lhs true ())) refl)))
        (steps ((rewrite (premise 8) lr lhs true ()) (unfold fe_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs)))
        (rewrite-with (lemma {peel}) lr lhs () ((steps ((rewrite (premise hcN) lr lhs true ())) refl)))
        (steps ({' '.join(tail_steps(n_tail, tail_extra))}{rr}) refl)))""")
    sl.add("hrun")
    out.append(closing(extra_rhs_rewrites))
    # close the chain and the case-on True arms
    closers = ")" * (1 + 4*len(imp_cases))  # chain + per case: True-arm chain, case True, arm list, case-on
    out.append(closers + ")")
    return "\n".join(out) + "\n"

def steps_trunc(kind):
    if kind == "U32":
        return step_unary("fe_step_trunc32_g", "(ITrunc k1 U32 a)", "(k1 IKind) (a IExp)", "(list (XMovRR32 RAX RAX))", 1, 1,
            "(band va (- (ikmod U32) 1))", [], "(band va 4294967295)", [],
            lambda sl: ["    (have hmask (= (- (ikmod U32) 1) 4294967295) (steps ((compute lhs)) refl))"] or sl.add("hmask"),
            ["(rewrite (premise hvv) rl rhs true ())", "(rewrite (premise hmask) lr rhs true ())"])
    if kind == "U8":
        return step_unary("fe_step_trunc8", "(ITrunc k1 U8 a)", "(k1 IKind) (a IExp)", "(list (XBin XAnd RAX (SImm 255)))", 1, 1,
            "(band va (- (ikmod U8) 1))", [], "(band va 255)", [],
            lambda sl: ["    (have hmask (= (- (ikmod U8) 1) 255) (steps ((compute lhs)) refl))"] or sl.add("hmask"),
            ["(rewrite (premise hvv) rl rhs true ())", "(rewrite (premise hmask) lr rhs true ())"])
    # U64: the tail is empty; v = band va (2^64 - 1) = va (mask_word64 + wrap64_id on the in-band va)
    def extra(sl):
        lines = [
            "    (have hlocs (= (fr_locs fp lc psx) True) (rewrite-with (lemma ctx_locs) lr lhs ((inst m m) (inst mlo mlo) (inst slo slo) (inst nl nl) (inst d d)) ((steps ((rewrite (premise 2) lr lhs true ())) refl)) refl))",
            "    (have hcb (= (ixf_cb a) True) (rewrite-with (lemma cb_trunc) lr lhs ((inst k1 k1) (inst kt U64)) ((steps ((rewrite (premise 3) lr lhs true ())) refl)) refl))",
            f"    (have hba (= (fe_inband va) True) (rewrite-with (lemma fe_band) lr lhs ((inst lc lc) (inst mem {IM}) (inst mlo mlo) (inst msz slo) (inst fp fp) (inst psx psx) (inst e a)) ((steps ((rewrite (premise 7) lr lhs true ())) refl) (steps ((rewrite (premise hlocs) lr lhs true ())) refl) (steps ((rewrite (premise hcb) lr lhs true ())) refl)) refl))",
            "    (have hal (= (le 0 va) True) (rewrite-with (lemma inband_lo) lr lhs () ((steps ((rewrite (premise hba) lr lhs true ())) refl)) refl))",
            "    (have hah (= (lt va 18446744073709551616) True) (rewrite-with (lemma inband_hi) lr lhs () ((steps ((rewrite (premise hba) lr lhs true ())) refl)) refl))",
            "    (have hmask (= (- (ikmod U64) 1) 18446744073709551615) (steps ((compute lhs)) refl))",
            "    (have hmw (= (band va 18446744073709551615) va) (chain (rewrite-with (lemma mask_word64) lr lhs () ((steps ((rewrite (premise hal) lr lhs true ())) refl))) (rewrite-with (lemma wrap64_id) lr lhs () ((steps ((rewrite (premise hal) lr lhs true ())) refl) (steps ((rewrite (premise hah) lr lhs true ())) refl))) refl))",
        ]
        for n in ["hlocs","hcb","hba","hal","hah","hmask","hmw"]: sl.add(n)
        return lines
    return step_unary("fe_step_trunc64", "(ITrunc k1 U64 a)", "(k1 IKind) (a IExp)", "Nil", 0, 0,
        "(band va (- (ikmod U64) 1))", [], "va", [], extra,
        ["(rewrite (premise hvv) rl rhs true ())", "(rewrite (premise hmask) lr rhs true ())", "(rewrite (premise hmw) lr rhs true ())"])



CTX_INSTS = {
 "hdisc": "((inst m m) (inst mlo mlo) (inst fp fp) (inst nl nl) (inst d d) (inst lc lc))",
 "hlocs": "((inst m m) (inst mlo mlo) (inst slo slo) (inst nl nl) (inst d d))",
 "hxlo":  "((inst slo slo) (inst fp fp) (inst nl nl) (inst d d) (inst lc lc) (inst psx psx))",
 "hmlo":  "((inst m m) (inst fp fp) (inst nl nl) (inst d d) (inst lc lc) (inst psx psx))",
 "hslo":  "((inst m m) (inst mlo mlo) (inst nl nl) (inst d d) (inst lc lc) (inst psx psx))",
 "hal":   "((inst m m) (inst mlo mlo) (inst nl nl) (inst d d) (inst lc lc) (inst psx psx))",
 "hnl":   "((inst m m) (inst mlo mlo) (inst slo slo) (inst fp fp) (inst d d) (inst psx psx))",
 "hd0":   "((inst m m) (inst mlo mlo) (inst slo slo) (inst fp fp) (inst nl nl) (inst lc lc) (inst psx psx))",
 "hslo0": "((inst m m) (inst mlo mlo) (inst fp fp) (inst nl nl) (inst d d) (inst lc lc) (inst psx psx))",
 "hhi":   "((inst mlo mlo) (inst slo slo) (inst fp fp) (inst nl nl) (inst d d) (inst lc lc) (inst psx psx))",
}
CTX_LEMMA = {"hdisc":"ctx_disc","hlocs":"ctx_locs","hxlo":"ctx_lo","hmlo":"ctx_mlo","hslo":"ctx_slo","hal":"ctx_al","hnl":"ctx_nl","hd0":"ctx_d","hslo0":"ctx_slo0","hhi":"ctx_hi"}
CTX_FACT = {"hdisc":"(fp_disc slo psx)","hlocs":"(fr_locs fp lc psx)","hxlo":"(int_eq (xmemlo_of m) mlo)","hmlo":"(le mlo slo)","hslo":"(le slo fp)","hal":"(int_eq (mod (- fp slo) 8) 0)","hnl":"(le (ilen lc) nl)","hd0":"(le 0 d)","hslo0":"(le 0 slo)","hhi":"(le (xmemhi_of m) 4294967296)"}
def ctx_have(nm, dval="d"):
    insts = CTX_INSTS[nm].replace("(inst d d)", f"(inst d {dval})")
    fact = CTX_FACT[nm].replace("(le 0 d)", f"(le 0 {dval})")
    return f"    (have {nm} (= {fact} True) (rewrite-with (lemma {CTX_LEMMA[nm]}) lr lhs {insts} ((steps ((rewrite (premise 2) lr lhs true ())) refl)) refl))"

def steps_shift(kind_or_shr):
    if kind_or_shr == "shl64":
        def extra(sl):
            lines=["    (have hw (= (ikw U64) 64) (steps ((compute lhs)) refl))"]; sl.add("hw")
            lines.append(f"    (have hlt (= (lt cc 64) True) (by arith {sl.cert({'hs1':1,'hw':-1})}))"); sl.add("hlt")
            lines.append("    (have hm64 (= (mod cc 64) cc) (rewrite-with (lemma mod64_id) lr lhs () ((steps ((rewrite (premise hs0) lr lhs true ())) refl) (steps ((rewrite (premise hlt) lr lhs true ())) refl)) refl))"); sl.add("hm64")
            lines.append("    (have hk (= (ikmod U64) 18446744073709551616) (steps ((compute lhs)) refl))"); sl.add("hk")
            return lines
        return step_unary("fe_step_shl64", "(IBin U64 IShl a (IConst cc))", "(a IExp) (cc Int)", "(list (XShlI RAX cc))", 1, 1,
            "(mod (bshl va cc) (ikmod U64))", [("(le 0 cc)","hs0"),("(lt cc (ikw U64))","hs1")],
            "(mod (bshl va cc) 18446744073709551616)", ["(rewrite (premise hm64) lr lhs true ())"], extra,
            ["(rewrite (premise hvv) rl rhs true ())","(rewrite (premise hk) lr rhs true ())"],
            extra_unfolds=["(unfold iexp rhs)","(reduce rhs)","(unfold iop_val rhs)","(reduce rhs)"])
    if kind_or_shr == "shl32":
        def extra(sl):
            lines=["    (have hw (= (ikw U32) 32) (steps ((compute lhs)) refl))"]; sl.add("hw")
            lines.append(f"    (have hlt (= (lt cc 32) True) (by arith {sl.cert({'hs1':1,'hw':-1})}))"); sl.add("hlt")
            lines.append("    (have hm32 (= (mod cc 32) cc) (rewrite-with (lemma mod32_id) lr lhs () ((steps ((rewrite (premise hs0) lr lhs true ())) refl) (steps ((rewrite (premise hlt) lr lhs true ())) refl)) refl))"); sl.add("hm32")
            lines.append("    (have hk (= (ikmod U32) 4294967296) (steps ((compute lhs)) refl))"); sl.add("hk")
            return lines
        return step_unary("fe_step_shl32", "(IBin U32 IShl a (IConst cc))", "(a IExp) (cc Int)", "(list (XShlI32 RAX cc))", 1, 1,
            "(mod (bshl va cc) (ikmod U32))", [("(le 0 cc)","hs0"),("(lt cc (ikw U32))","hs1")],
            "(mod (bshl va cc) 4294967296)", ["(rewrite (premise hm32) lr lhs true ())"], extra,
            ["(rewrite (premise hvv) rl rhs true ())","(rewrite (premise hk) lr rhs true ())"],
            extra_unfolds=["(unfold iexp rhs)","(reduce rhs)","(unfold iop_val rhs)","(reduce rhs)"])
    # shr at any kind
    def extra(sl):
        lines=["    (have hw (= (le (ikw k) 64) True) (rewrite-with (lemma ikw_le64) lr lhs () () refl))"]; sl.add("hw")
        lines.append(f"    (have hlt (= (lt cc 64) True) (by arith {sl.cert({'hs1':1,'hw':1})}))"); sl.add("hlt")
        lines.append("    (have hm64 (= (mod cc 64) cc) (rewrite-with (lemma mod64_id) lr lhs () ((steps ((rewrite (premise hs0) lr lhs true ())) refl) (steps ((rewrite (premise hlt) lr lhs true ())) refl)) refl))"); sl.add("hm64")
        return lines
    return step_unary("fe_step_shr", "(IBin k IShr a (IConst cc))", "(k IKind) (a IExp) (cc Int)", "(list (XShrI RAX cc))", 1, 1,
        "(bshr va cc)", [("(le 0 cc)","hs0"),("(lt cc (ikw k))","hs1")],
        "(bshr va cc)", ["(rewrite (premise hm64) lr lhs true ())"], extra,
        ["(rewrite (premise hvv) rl rhs true ())"],
        extra_unfolds=["(unfold iexp rhs)","(reduce rhs)","(unfold iop_val rhs)","(reduce rhs)"])

def steps_rotr():
    V1="(band va (- (ikmod U32) 1))"
    imp=f"(band (bor (bshr {V1} (mod cc (ikw U32))) (bshl {V1} (mod (- (ikw U32) (mod cc (ikw U32))) (ikw U32)))) (- (ikmod U32) 1))"
    x="(band (bor (bshr (band va 4294967295) (mod cc 32)) (bshl (band va 4294967295) (mod (- 32 (mod cc 32)) 32))) 4294967295)"
    def extra(sl):
        lines=["    (have hw (= (ikw U32) 32) (steps ((compute lhs)) refl))"]; sl.add("hw")
        lines.append("    (have hmask (= (- (ikmod U32) 1) 4294967295) (steps ((compute lhs)) refl))"); sl.add("hmask")
        return lines
    return step_unary("fe_step_rotr32", "(IRotr U32 a (IConst cc))", "(a IExp) (cc Int)", "(list (XRorI32 RAX cc))", 1, 1,
        imp, [], x, [], extra,
        ["(rewrite (premise hvv) rl rhs true ())","(rewrite (premise hw) lr rhs true ())","(rewrite (premise hmask) lr rhs true ())"],
        extra_unfolds=["(unfold iexp rhs)","(reduce rhs)"])

def steps_load():
    TA="(fe_tr fp nl d a lc (fp_mem mem0 (fbelow slo psx)) mlo slo)"
    MAa=f"(fp_mem mem0 (fp_app {TA} psx))"
    def extra(sl):
        lines=[]
        for nm in ["hdisc","hxlo","hmlo","hslo","hal","hnl","hd0"]:
            lines.append(ctx_have(nm)); sl.add(nm)
        lines.append("    (have hnn (= (le 0 (ilen lc)) True) (rewrite-with (lemma ilen_nonneg) lr lhs () () refl))"); sl.add("hnn")
        lines.append("    (have hdep (= (le 0 (ixf_dep a)) True) (rewrite-with (lemma ixf_dep_nonneg) lr lhs () () refl))"); sl.add("hdep")
        lines.append("    (have hdepl (= (ixf_dep (ILoad a)) (ixf_dep a)) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))"); sl.add("hdepl")
        lines.append(f"    (have hxl (= (xmemlo_of m) mlo) (by arith {sl.cert2({'hxlo':-1},{'hxlo':1})}))"); sl.add("hxl")
        lines.append(f"    (have hwlo (= (le (xmemlo_of m) va) True) (steps ((rewrite (premise hxl) lr lhs true ())) (by arith {sl.cert({'hl0':1})})))"); sl.add("hwlo")
        lines.append(f"    (have hwhi (= (le (+ va 1) (xmemhi_of m)) True) (by arith {sl.cert({'p4':1,'hdepl':8,'hdep':8,'hnl':8,'hnn':8,'hd0':8,'hslo':1,'hl1':1})}))"); sl.add("hwhi")
        lines.append(f"    (have hnd0 (= (le 0 (+ nl d)) True) (by arith {sl.cert({'hnl':1,'hnn':1,'hd0':1})}))"); sl.add("hnd0")
        lines.append(f"    (have hdisca (= (fp_disc slo (fp_app {TA} psx)) True) (rewrite-with (lemma fe_tr_disc) lr lhs ((inst e a)) ((steps ((rewrite (premise hdisc) lr lhs true ())) refl) (steps ((rewrite (premise hslo) lr lhs true ())) refl) (steps ((rewrite (premise hal) lr lhs true ())) refl) (steps ((rewrite (premise hnd0) lr lhs true ())) refl)) refl))"); sl.add("hdisca")
        lines.append(f"""    (have hget (= (mem_get {MAa} va) (mem_get (fp_mem mem0 (fbelow slo psx)) va))
      (chain
        (rewrite-with (lemma fp_below_b) lr lhs ((inst slo slo)) ((steps ((rewrite (premise hdisca) lr lhs true ())) refl) (steps ((rewrite (premise hl1) lr lhs true ())) refl)))
        (rewrite-with (lemma fe_tr_below) lr lhs ((inst e a)) ((steps ((rewrite (premise hslo) lr lhs true ())) refl) (steps ((rewrite (premise hnd0) lr lhs true ())) refl)))
        refl))"""); sl.add("hget")
        return lines
    return step_unary("fe_step_load", "(ILoad a)", "(a IExp)", "(list (XLoad8 RAX (AReg RAX)))", 1, 1,
        "(mem_get (fp_mem mem0 (fbelow slo psx)) va)", [("(le mlo va)","hl0"),("(lt va slo)","hl1")],
        "(mem_get (fp_mem mem0 (fbelow slo psx)) va)", ["(rewrite (premise hget) lr lhs true ())"], extra,
        ["(rewrite (premise hvv) rl rhs true ())"],
        tail_extra=[f"(compute lhs {STOPS_L})","(rewrite (premise hwlo) lr lhs true ())","(reduce lhs)",f"(compute lhs {STOPS_L})","(rewrite (premise hwhi) lr lhs true ())","(reduce lhs)"])

def step_lemmas_unary():
    return "\n".join([steps_trunc("U8"), steps_trunc("U64"), steps_shift("shl64"), steps_shift("shl32"), steps_shift("shr"), steps_rotr(), steps_load()])

# ---------------- the BINARY family: general IBin ops ----------------
# emission: ia ++ (spill ++ (ib ++ (reload ++ OP)))   — two IHs (a at tower c from RS;
# b at tower c-|ia|-3 from the post-spill file RS2 over psx2 = FWord ADDR va :: Ta ++ psx)

ADDRB = "(+ fp (* 8 (+ nl d)))"
TA_ = f"(fe_tr fp nl d a lc {IM} mlo slo)"
PSX2 = f"(Cons (FWord {ADDRB} va) (fp_app {TA_} psx))"
XM2 = f"(fp_mem mem0 {PSX2})"
IM2 = f"(fp_mem mem0 (fbelow slo {PSX2}))"
TB_ = f"(fe_tr fp nl (+ d 1) b lc {IM2} mlo slo)"
TBI = f"(fe_tr fp nl (+ d 1) b lc {IM} mlo slo)"
MB = f"(fp_mem mem0 (fp_app {TB_} {PSX2}))"
def RS2(): return f"(MkRegs va rcx (rdx_of (xo_regs {RUN('ia')})) rbx rbp rsi di r8 r9 {ADDRB} (r11_of (xo_regs {RUN('ia')})) r12 r13 dep fp)"
def RUNB(): return f"(xeval_seq (xt (- (- c (xil ia)) 3) g) m ib {RS2()} {XM2})"
MA_ = f"(fp_mem mem0 (fp_app {TA_} psx))"

def step_binary(name, e, binders, oplist, oplen, imp_value, imp_cases, op_need, op_kind, imp_unfolds_after, rax_final, rdx_final, r11_final, rhs_value_rewrites, extra_fn):
    """
    oplist: the op block as ixf_exp spells it; oplen: its ixf_oplen; op_need: the op block's tower need;
    op_kind: "bop64" | "bop32" | "div" | "rem" | "cmp" (selects the block lemma + the hrun choreography);
    rax_final/rdx_final/r11_final: the fields of hrun's RHS (R10 is va for div/rem/cmp, rax_final for bops);
    rhs_value_rewrites: rewrites applied to the RHS after hvv so imp's value matches rax_final;
    extra_fn(sl) -> extra have lines (op-specific facts: hle1 for div, hk for mods, …)
    """
    premises = ["p0","p1","p2","p3","p4","p5","p6","p7","p8","p9","p10","p11"]
    sl = Slots(premises)
    out=[]
    r10_final = rax_final if op_kind in ("bop64","bop32") else "va"
    out.append(f"""(claim {name}
  (goal
    ({binders} (ia (List XInstr)) (va Int) (ib (List XInstr)) (vb Int) {GOALVARS})
    ({common_premises(e)[1:]}
{ih_premises()}
     (= (ixf_exp nl (+ d 1) b) (Some ib))
     (= (iexp b lc {IM} mlo slo) (Some vb))
     (= {RUNB()} (fe_out vb {RS2()} {RUNB()} {MB})))
{conclusion(e)}
  (chain""")
    bop_unf = " (unfold ixf_bop rhs) (reduce rhs)" if op_kind in ("bop64","bop32") else ""
    out.append(f"""    (have hsome (= (Some (ix_app ia (ix_app (ixf_spill nl d) (ix_app ib (ix_app (ixf_reload10 nl d) {oplist}))))) (Some ie))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_exp rhs) (reduce rhs) (rewrite (premise 6) lr rhs true ()) (reduce rhs) (rewrite (premise 9) lr rhs true ()) (reduce rhs){bop_unf}) refl))
    (inject (premise hsome) (hie))
    (have hop (= (iop_val {op_kind_K(e)} {op_of(e)} va vb) (Some v))
      (steps ((rewrite (premise 1) rl rhs true ()) (unfold iexp rhs) (reduce rhs) (rewrite (premise 7) lr rhs true ()) (reduce rhs) (rewrite (premise 10) lr rhs true ()) (reduce rhs)) refl))""")
    sl.add("hsome"); sl.add("hie"); sl.add("hop")
    case_rw=[]
    for guard, nm in imp_cases:
        out.append(f"""    (case-on {guard} Bool
      ((case False
         (chain
           {cap(nm, f'(= {guard} False)')}
           (have hn (= None (Some v))
             (steps ((rewrite (premise hop) rl rhs true ()) (unfold iop_val rhs) (reduce rhs){''.join(' '+r for r in case_rw)} (rewrite (premise {nm}) lr rhs true ()) (reduce rhs)) refl))
           (absurd (premise hn))))
       (case True
         (chain
           {cap(nm, f'(= {guard} True)')}""")
        sl.add(nm); case_rw.append(f"(rewrite (premise {nm}) lr rhs true ()) (reduce rhs)")
    out.append(f"""    (have hv (= (Some {imp_value}) (Some v))
      (steps ((rewrite (premise hop) rl rhs true ()) (unfold iop_val rhs) (reduce rhs){''.join(' '+r for r in case_rw)}) refl))
    (inject (premise hv) (hvv))""")
    sl.add("hv"); sl.add("hvv")
    # context
    for nm in ["hdisc","hlocs","hxlo","hmlo","hslo","hal","hnl","hd0","hslo0","hhi"]:
        out.append(ctx_have(nm)); sl.add(nm)
    out.append("    (have hnn (= (le 0 (ilen lc)) True) (rewrite-with (lemma ilen_nonneg) lr lhs () () refl))"); sl.add("hnn")
    out.append("    (have hdepa (= (le 0 (ixf_dep a)) True) (rewrite-with (lemma ixf_dep_nonneg) lr lhs () () refl))"); sl.add("hdepa")
    out.append("    (have hdepb (= (le 0 (ixf_dep b)) True) (rewrite-with (lemma ixf_dep_nonneg) lr lhs () () refl))"); sl.add("hdepb")
    out.append(f"    (have hdepe (= (ixf_dep {e}) (+ 1 (imax2 (ixf_dep a) (ixf_dep b)))) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))"); sl.add("hdepe")
    out.append("    (have hmaxa (= (le (ixf_dep a) (imax2 (ixf_dep a) (ixf_dep b))) True) (rewrite-with (lemma imax2_ge_l) lr lhs ((inst b (ixf_dep b))) () refl))"); sl.add("hmaxa")
    out.append(f"    (have hxl (= (xmemlo_of m) mlo) (by arith {sl.cert2({'hxlo':-1},{'hxlo':1})}))"); sl.add("hxl")
    out.append(f"    (have hnd0 (= (le 0 (+ nl d)) True) (by arith {sl.cert({'hnl':1,'hnn':1,'hd0':1})}))"); sl.add("hnd0")
    out.append(f"    (have hfp0 (= (le 0 fp) True) (by arith {sl.cert({'hslo':1,'hslo0':1})}))"); sl.add("hfp0")
    out.append(f"    (have hsa (= (le slo {ADDRB}) True) (by arith {sl.cert({'hslo':1,'hnl':8,'hnn':8,'hd0':8})}))"); sl.add("hsa")
    out.append(f"    (have hwlo (= (le (xmemlo_of m) {ADDRB}) True) (steps ((rewrite (premise hxl) lr lhs true ())) (by arith {sl.cert({'hmlo':1,'hslo':1,'hnl':8,'hnn':8,'hd0':8})})))"); sl.add("hwlo")
    # ADDR + 8 <= hi:  P4: hi - fp - 8nl - 8d - 8*DEP(e) >= 0; hdepe: DEP(e) = 1 + MAX; hmaxa: MAX - DEPa >= 0; hdepa: DEPa >= 0
    out.append(f"    (have hwhi (= (le (+ {ADDRB} 8) (xmemhi_of m)) True) (by arith {sl.cert({'p4':1,'hdepe':8,'hmaxa':8,'hdepa':8})}))"); sl.add("hwhi")
    out.append(f"    (have hsm (= (lt (+ {ADDRB} 8) 18446744073709551616) True) (by arith {sl.cert({'hwhi':1,'hhi':1})}))"); sl.add("hsm")
    out.append(f"    (have hali (= (int_eq (mod (- {ADDRB} slo) 8) 0) True) (rewrite-with (lemma al_shift) lr lhs ((inst k (+ nl d))) ((steps ((rewrite (premise hal) lr lhs true ())) refl)) refl))"); sl.add("hali")
    out.append(f"    (have hlo8 (= (le (+ {ADDRB} 8) slo) False) (by arith {sl.cert({'hslo':1,'hnl':8,'hnn':8,'hd0':8})}))"); sl.add("hlo8")
    # bands
    out.append("    (have hcba (= (ixf_cb a) True) (rewrite-with (lemma cb_bin_a) lr lhs ((inst k "+op_kind_K(e)+") (inst o "+op_of(e)+") (inst b b)) ((steps ((rewrite (premise 3) lr lhs true ())) refl)) refl))"); sl.add("hcba")
    out.append(f"    (have hba (= (fe_inband va) True) (rewrite-with (lemma fe_band) lr lhs ((inst lc lc) (inst mem {IM}) (inst mlo mlo) (inst msz slo) (inst fp fp) (inst psx psx) (inst e a)) ((steps ((rewrite (premise 7) lr lhs true ())) refl) (steps ((rewrite (premise hlocs) lr lhs true ())) refl) (steps ((rewrite (premise hcba) lr lhs true ())) refl)) refl))"); sl.add("hba")
    out.append("    (have hal0 (= (le 0 va) True) (rewrite-with (lemma inband_lo) lr lhs () ((steps ((rewrite (premise hba) lr lhs true ())) refl)) refl))"); sl.add("hal0")
    out.append("    (have hah (= (lt va 18446744073709551616) True) (rewrite-with (lemma inband_hi) lr lhs () ((steps ((rewrite (premise hba) lr lhs true ())) refl)) refl))"); sl.add("hah")
    # patch-list facts: disc of Ta++psx, of psx2, of Tb'++psx2; the reload read
    out.append(f"    (have hdisca (= (fp_disc slo (fp_app {TA_} psx)) True) (rewrite-with (lemma fe_tr_disc) lr lhs ((inst e a)) ((steps ((rewrite (premise hdisc) lr lhs true ())) refl) (steps ((rewrite (premise hslo) lr lhs true ())) refl) (steps ((rewrite (premise hal) lr lhs true ())) refl) (steps ((rewrite (premise hnd0) lr lhs true ())) refl)) refl))"); sl.add("hdisca")
    out.append(f"    (have hdisc2 (= (fp_disc slo {PSX2}) True) (rewrite-with (lemma fp_disc_w_slot) lr lhs () ((steps ((rewrite (premise hlo8) lr lhs true ())) refl) (steps ((rewrite (premise hsa) lr lhs true ())) refl) (steps ((rewrite (premise hali) lr lhs true ())) refl) (steps ((rewrite (premise hdisca) lr lhs true ())) refl)) refl))"); sl.add("hdisc2")
    out.append(f"    (have hnd1 (= (le 0 (+ nl (+ d 1))) True) (by arith {sl.cert({'hnl':1,'hnn':1,'hd0':1})}))"); sl.add("hnd1")
    out.append(f"    (have hdiscb (= (fp_disc slo (fp_app {TB_} {PSX2})) True) (rewrite-with (lemma fe_tr_disc) lr lhs ((inst e b)) ((steps ((rewrite (premise hdisc2) lr lhs true ())) refl) (steps ((rewrite (premise hslo) lr lhs true ())) refl) (steps ((rewrite (premise hal) lr lhs true ())) refl) (steps ((rewrite (premise hnd1) lr lhs true ())) refl)) refl))"); sl.add("hdiscb")
    out.append(f"    (have hminb (= (fp_min {TB_} (+ fp (* 8 (+ nl (+ d 1))))) True) (rewrite-with (lemma fe_tr_min) lr lhs ((inst e b)) () refl))"); sl.add("hminb")
    out.append(f"    (have hlt1 (= (lt {ADDRB} (+ fp (* 8 (+ nl (+ d 1))))) True) (by arith {sl.cert({})}))"); sl.add("hlt1")
    out.append(f"    (have hwv (= (fp_wordv (fp_app {TB_} {PSX2}) {ADDRB}) (Some va)) (chain (rewrite-with (lemma fp_wordv_app_min) lr lhs ((inst lo (+ fp (* 8 (+ nl (+ d 1)))))) ((steps ((rewrite (premise hminb) lr lhs true ())) refl) (steps ((rewrite (premise hlt1) lr lhs true ())) refl))) (steps ((unfold fp_wordv lhs) (reduce lhs) (rewrite (lemma int_eq_refl) lr lhs true ()) (reduce lhs)) refl)))"); sl.add("hwv")
    out.append(f"    (have hread (= (load_le (xw8) {MB} {ADDRB}) va) (rewrite-with (lemma fp_read) lr lhs ((inst slo slo) (inst v va)) ((steps ((rewrite (premise hdiscb) lr lhs true ())) refl) (steps ((rewrite (premise hsa) lr lhs true ())) refl) (steps ((rewrite (premise hali) lr lhs true ())) refl) (steps ((rewrite (premise hwv) lr lhs true ())) refl) (steps ((rewrite (premise hal0) lr lhs true ())) refl) (steps ((rewrite (premise hah) lr lhs true ())) refl)) refl))"); sl.add("hread")
    out.append(f"    (have hfb (= (fbelow slo {PSX2}) (fbelow slo psx)) (chain (rewrite-with (lemma fbelow_w_hi) lr lhs () ((steps ((rewrite (premise hlo8) lr lhs true ())) refl))) (rewrite-with (lemma fe_tr_below) lr lhs ((inst e a)) ((steps ((rewrite (premise hslo) lr lhs true ())) refl) (steps ((rewrite (premise hnd0) lr lhs true ())) refl))) refl))"); sl.add("hfb")
    # lengths / towers
    out.append("    (have hlena (= (xil ia) (ixf_elen a)) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d d) (inst e a)) ((steps ((rewrite (premise 6) lr lhs true ())) refl)) refl))"); sl.add("hlena")
    out.append("    (have hlenb (= (xil ib) (ixf_elen b)) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d (+ d 1)) (inst e b)) ((steps ((rewrite (premise 9) lr lhs true ())) refl)) refl))"); sl.add("hlenb")
    out.append(f"    (have hcost (= (ixf_ecost {e}) (+ (+ (ixf_elen a) (+ 3 (+ (ixf_elen b) {3+oplen}))) 5)) (steps ((unfold ixf_ecost lhs) (unfold ixf_elen lhs) (reduce lhs) (unfold ixf_oplen lhs) (reduce lhs) (compute lhs (stop ixf_elen))) refl))"); sl.add("hcost")
    out.append("    (have hxs (= (xil (ixf_st (+ nl d))) 3) (steps ((compute lhs)) refl))"); sl.add("hxs")
    out.append("    (have hxr (= (xil (ixf_reload10 nl d)) 3) (steps ((compute lhs)) refl))"); sl.add("hxr")
    out.append("    (have hlb0 (= (le 0 (ixf_elen b)) True) (rewrite-with (lemma ixf_elen_nonneg) lr lhs () () refl))"); sl.add("hlb0")
    T = {'p5':1,'hlena':-1,'hlenb':-1,'hcost':1}
    T1 = {'p5':1,'hlena':-1,'hcost':1,'hlb0':1}
    out.append(f"    (have hle1 (= (le (xil ia) c) True) (by arith {sl.cert(T1)}))"); sl.add("hle1")
    out.append(f"    (have hsp4 (= (le 4 (- c (xil ia))) True) (by arith {sl.cert(T1)}))"); sl.add("hsp4")
    out.append(f"    (have hsp3 (= (le 3 (- c (xil ia))) True) (by arith {sl.cert(T1)}))"); sl.add("hsp3")
    out.append(f"    (have hle3 (= (le (xil ib) (- (- c (xil ia)) 3)) True) (by arith {sl.cert(T)}))"); sl.add("hle3")
    out.append(f"    (have hrl4 (= (le 4 (- (- (- c (xil ia)) 3) (xil ib))) True) (by arith {sl.cert(T)}))"); sl.add("hrl4")
    out.append(f"    (have hrl3 (= (le 3 (- (- (- c (xil ia)) 3) (xil ib))) True) (by arith {sl.cert(T)}))"); sl.add("hrl3")
    out.append(f"    (have hopn (= (le {op_need} (- (- (- (- c (xil ia)) 3) (xil ib)) 3)) True) (by arith {sl.cert(T)}))"); sl.add("hopn")
    for line in extra_fn(sl): out.append(line)
    acc_rs = "".join(f" (rewrite (lemma {f}_of_mk) lr lhs true ())" for f in ["rcx","rbx","rbp","rsi","rdi","r8","r9","r12","r13","r14","r15"])
    # the op block choreography
    if op_kind == "bop64":
        opblock = f"(rewrite-with (lemma fe_bop64_run) lr lhs () ((steps ((rewrite (premise hopn) lr lhs true ())) refl)))"
    elif op_kind == "bop32":
        opblock = f"(rewrite-with (lemma fe_bop32_run) lr lhs () ((steps ((rewrite (premise hopn) lr lhs true ())) refl)))"
    elif op_kind == "div":
        opblock = f"(rewrite-with (lemma fe_div_run) lr lhs () ((steps ((rewrite (premise hopn) lr lhs true ())) refl) (steps ((rewrite (premise hle) lr lhs true ())) refl) (steps ((rewrite (premise hal0) lr lhs true ())) refl) (steps ((rewrite (premise hah) lr lhs true ())) refl)))"
    elif op_kind == "rem":
        opblock = f"""(rewrite-with (lemma xseq_app) lr lhs () ((steps ((rewrite (premise hxd) lr lhs true ()) (rewrite (premise hrs4) lr lhs true ())) refl)))
        (steps ((rewrite (premise hxd) lr lhs true ())))
        (rewrite-with (lemma fe_div_run) lr lhs () ((steps ((rewrite (premise hopn) lr lhs true ())) refl) (steps ((rewrite (premise hle) lr lhs true ())) refl) (steps ((rewrite (premise hal0) lr lhs true ())) refl) (steps ((rewrite (premise hah) lr lhs true ())) refl)))
        (steps ((unfold xcont lhs) (reduce lhs)))
        (rewrite-with (lemma xt_peel2) lr lhs () ((steps ((rewrite (premise hrem2) lr lhs true ())) refl)))
        (steps ({' '.join(tail_steps(1))} (rewrite (lemma rdx_of_mk) lr lhs true ())))"""
    else:  # cmp
        opblock = f"(rewrite-with (lemma fe_cmp_run) lr lhs () ((steps ((rewrite (premise hopn) lr lhs true ())) refl)))"
    final_compute = f"(compute lhs {STOPS_L})" if op_kind in ("bop64","bop32","cmp") else ""
    if op_kind == "cmp":
        final_compute += " (rewrite (lemma r10_of_mk) lr lhs true ())"
    out.append(f"""    (have hrun (= {RUN('ie')} (Some (XNorm (MkRegs {rax_final} rcx {rdx_final} rbx rbp rsi di r8 r9 {r10_final} {r11_final} r12 r13 dep fp) {MB})))
      (chain
        (steps ((rewrite (premise hie) rl lhs true ())))
        (rewrite-with (lemma xseq_app) lr lhs () ((steps ((rewrite (premise hle1) lr lhs true ())) refl)))
        (steps ((rewrite (premise 8) lr lhs true ()) (unfold fe_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs){acc_rs} (unfold ixf_spill lhs)))
        (rewrite-with (lemma xseq_app) lr lhs () ((steps ((rewrite (premise hxs) lr lhs true ()) (rewrite (premise hsp3) lr lhs true ())) refl)))
        (steps ((rewrite (premise hxs) lr lhs true ())))
        (rewrite-with (lemma fe_st_run) lr lhs () ((steps ((rewrite (premise hsp4) lr lhs true ())) refl) (steps ((rewrite (premise hnd0) lr lhs true ())) refl) (steps ((rewrite (premise hfp0) lr lhs true ())) refl) (steps ((rewrite (premise hsm) lr lhs true ())) refl) (steps ((rewrite (premise hwlo) lr lhs true ())) refl) (steps ((rewrite (premise hwhi) lr lhs true ())) refl)))
        (steps ((rewrite (lemma fp_mem_cons_w) lr lhs true ()) (unfold xcont lhs) (reduce lhs)))
        (rewrite-with (lemma xseq_app) lr lhs () ((steps ((rewrite (premise hle3) lr lhs true ())) refl)))
        (steps ((rewrite (premise 11) lr lhs true ()) (unfold fe_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs){acc_rs}))
        (rewrite-with (lemma xseq_app) lr lhs () ((steps ((rewrite (premise hxr) lr lhs true ()) (rewrite (premise hrl3) lr lhs true ())) refl)))
        (steps ((rewrite (premise hxr) lr lhs true ())))
        (rewrite-with (lemma fe_rl_run) lr lhs () ((steps ((rewrite (premise hrl4) lr lhs true ())) refl) (steps ((rewrite (premise hnd0) lr lhs true ())) refl) (steps ((rewrite (premise hfp0) lr lhs true ())) refl) (steps ((rewrite (premise hsm) lr lhs true ())) refl) (steps ((rewrite (premise hwlo) lr lhs true ())) refl) (steps ((rewrite (premise hwhi) lr lhs true ())) refl)))
        (steps ((rewrite (premise hread) lr lhs true ()) (unfold xcont lhs) (reduce lhs)))
        {opblock}
        (steps ({final_compute}) refl)))""")
    sl.add("hrun")
    acc = "\n".join(f"       (rewrite (lemma {f}_of_mk) lr rhs true ())" for f in ACC14)
    vr = "\n".join("       "+r for r in rhs_value_rewrites)
    out.append(f"""    (have hc2 (= (fp_app (Cons (FWord {ADDRB} va) {TA_}) psx) {PSX2}) (steps ((unfold fp_app lhs) (reduce lhs)) refl))
    (have hte (= (fe_tr fp nl d {e} lc {IM} mlo slo) (fp_app {TBI} (Cons (FWord {ADDRB} va) {TA_})))
      (steps ((unfold fe_tr lhs) (reduce lhs) (rewrite (premise 7) lr lhs true ()) (reduce lhs)) refl))
    (steps
      ((rewrite (premise hrun) lr both true ())
       (rewrite (premise hvv) rl rhs true ())
{vr}
       (rewrite (lemma fe_out_of_norm) lr rhs true ())
{acc}
       (rewrite (premise hte) lr rhs true ())
       (rewrite (lemma fp_app_assoc) lr rhs true ())
       (rewrite (premise hc2) lr rhs true ())
       (rewrite (premise hfb) lr lhs true ()))
      refl)""")
    closers = ")" * (1 + 4*len(imp_cases))
    out.append(closers + ")")
    return "\n".join(out) + "\n"

def op_kind_K(e):
    return e.split()[1]
def op_of(e):
    return e.split()[2]

BOP64 = [("IAdd","XAdd","(mod (+ va vb) (ikmod U64))","(mod (+ va vb) 18446744073709551616)"),
         ("ISub","XSub","(mod (- va vb) (ikmod U64))","(mod (- va vb) 18446744073709551616)"),
         ("IMul","XMul","(mod (* va vb) (ikmod U64))","(mod (* va vb) 18446744073709551616)"),
         ("IAnd","XAnd","(band va vb)","(band va vb)"),
         ("IOr","XOr","(bor va vb)","(bor va vb)"),
         ("IXor","XXor","(bxor va vb)","(bxor va vb)")]
BOP32 = [("IAdd","XAdd","(mod (+ va vb) (ikmod U32))","(mod (+ va vb) 4294967296)"),
         ("ISub","XSub","(mod (- va vb) (ikmod U32))","(mod (- va vb) 4294967296)"),
         ("IMul","XMul","(mod (* va vb) (ikmod U32))","(mod (* va vb) 4294967296)")]

def steps_bop(kind):
    outs=[]
    table = BOP64 if kind=="U64" else BOP32
    instr = "XBin" if kind=="U64" else "XBin32"
    for op, xop, impv, xv in table:
        e=f"(IBin {kind} {op} a b)"
        needs_k = "mod" in impv
        def extra(sl, needs_k=needs_k, kind=kind):
            if needs_k:
                lit = "18446744073709551616" if kind=="U64" else "4294967296"
                sl.add("hk"); return [f"    (have hk (= (ikmod {kind}) {lit}) (steps ((compute lhs)) refl))"]
            return []
        rr = ["(rewrite (premise hk) lr rhs true ())"] if needs_k else []
        outs.append(step_binary(f"fe_step_{op.lower()[1:]}{kind[1:]}", e, "(a IExp) (b IExp)",
            f"(list ({instr} {xop} R10 (SReg RAX)) (XMovRR RAX R10))", 2, impv, [], 3, "bop64" if kind=="U64" else "bop32",
            [], xv, f"(rdx_of (xo_regs {RUNB()}))", f"(r11_of (xo_regs {RUNB()}))", rr, extra))
    return "\n".join(outs)

def steps_divrem(which):
    if which=="div":
        def extra(sl): return []
        return step_binary("fe_step_div", "(IBin k IDiv a b)", "(k IKind) (a IExp) (b IExp)", "(ixf_div)", 4,
            "(ediv va vb)", [("(le 1 vb)","hle")], 5, "div", [], "(ediv va vb)", "(mod va vb)", "vb", [], extra)
    def extra(sl):
        lines=["    (have hxd (= (xil (ixf_div)) 4) (steps ((compute lhs)) refl))"]; sl.add("hxd")
        lines.append(f"    (have hrs4 (= (le 4 (- (- (- (- c (xil ia)) 3) (xil ib)) 3)) True) (by arith {sl.cert({'p5':1,'hlena':-1,'hlenb':-1,'hcost':1})}))"); sl.add("hrs4")
        lines.append(f"    (have hrem2 (= (le 2 (- (- (- (- (- c (xil ia)) 3) (xil ib)) 3) 4)) True) (by arith {sl.cert({'p5':1,'hlena':-1,'hlenb':-1,'hcost':1})}))"); sl.add("hrem2")
        return lines
    return step_binary("fe_step_rem", "(IBin k IRem a b)", "(k IKind) (a IExp) (b IExp)", "(ix_app (ixf_div) (list (XMovRR RAX RDX)))", 5,
        "(mod va vb)", [("(le 1 vb)","hle")], 5, "rem", [], "(mod va vb)", "(mod va vb)", "vb", [], extra)

def steps_cmp():
    outs=[]
    for op, cd, prim in [("IEq","CEq","int_eq"),("ILt","CLtU","lt"),("ILe","CLeU","le")]:
        e=f"(IBin k {op} a b)"
        def extra(sl): return []
        outs.append(step_binary(f"fe_step_{op.lower()[1:]}", e, "(k IKind) (a IExp) (b IExp)", f"(ixf_cmp ({cd} R10 (SReg RAX)))", 3,
            f"(ib2i ({prim} va vb))", [], 8, "cmp", [], f"(if ({prim} va vb) 1 0)", f"(rdx_of (xo_regs {RUNB()}))", f"(if ({prim} va vb) 1 0)",
            ["(unfold ib2i rhs)","(reduce rhs)"], extra))
    return "\n".join(outs)

def step_lemmas_binary():
    return "\n".join([steps_bop("U64"), steps_bop("U32"), steps_divrem("div"), steps_divrem("rem"), steps_cmp()])


# ---------------- fe_sound: THE EXPRESSION LEMMA (the induction citing the step lemmas) ----------------

PINS6 = "(inst nl nl) (inst d d) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst v v)"
def PINSV(vv): return f"(inst nl nl) (inst d d) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst v {vv})"
def D(k): return f"(steps ((rewrite (premise {k}) lr lhs true ())) refl)"
P05 = " ".join(D(i) for i in range(6))
MA_a = f"(fp_mem mem0 (fp_app (fe_tr fp nl d a lc {IM} mlo slo) psx))"
IHA = f"(= {RUN('ia')} (fe_out va {RS} {RUN('ia')} {MA_a}))"

def respell(e_new, ctor_eqs):
    """haves hp0 hp1 hp3 hp4 hp5: the common premises re-spelled at e_new, via the captured ctor
    equations (rewritten RL first) then the premise"""
    rl = "".join(f" (rewrite (premise {h}) rl lhs true ())" for h in ctor_eqs)
    return f"""  (have hp0 (= (ixf_exp nl d {e_new}) (Some ie)) (steps ({rl[1:]} (rewrite (premise 0) lr lhs true ())) refl))
  (have hp1 (= (iexp {e_new} lc {IM} mlo slo) (Some v)) (steps ({rl[1:]} (rewrite (premise 1) lr lhs true ())) refl))
  (have hp3 (= (ixf_cb {e_new}) True) (steps ({rl[1:]} (rewrite (premise 3) lr lhs true ())) refl))
  (have hp4 (= (le (+ fp (* 8 (+ nl (+ d (ixf_dep {e_new}))))) (xmemhi_of m)) True) (steps ({rl[1:]} (rewrite (premise 4) lr lhs true ())) refl))
  (have hp5 (= (le (ixf_ecost {e_new}) c) True) (steps ({rl[1:]} (rewrite (premise 5) lr lhs true ())) refl))"""

def absurd_ie(hp, rewrites):
    items=[f"(rewrite (premise {hp}) rl rhs true ())","(unfold ixf_exp rhs)","(reduce rhs)"]
    for r in rewrites: items += [r, "(reduce rhs)"]
    return f"""(chain
  (have hn (= None (Some ie)) (steps ({' '.join(items)}) refl))
  (absurd (premise hn)))"""

def absurd_v(hp, rewrites):
    items=[f"(rewrite (premise {hp}) rl rhs true ())","(unfold iexp rhs)","(reduce rhs)"]
    for r in rewrites: items += [r, "(reduce rhs)"]
    return f"""(chain
  (have hn (= None (Some v)) (steps ({' '.join(items)}) refl))
  (absurd (premise hn)))"""

def unary_path(step, e, k_len, pins_extra, cb_have, dep_have, sl):
    """inside: hp0..hp5 respelled, ha/hva to be case-split here"""
    RA="(rewrite (premise ha) lr rhs true ())"
    RV="(rewrite (premise hva) lr rhs true ())"
    sl2 = Slots(sl.names + ["ha","hva","hcb","hdep","hmax","hp4a","hcost"])
    costrhs = f"(+ (+ (ixf_elen a) {k_len}) 5)" if k_len else "(+ (ixf_elen a) 5)"
    return f"""(case-on (ixf_exp nl d a) Option
  ((case None (chain {cap('ha','(= (ixf_exp nl d a) None)')} {absurd_ie('hp0',[RA])}))
   (case Some (ia)
     (chain
       {cap('ha','(= (ixf_exp nl d a) (Some ia))')}
       (case-on (iexp a lc {IM} mlo slo) Option
         ((case None (chain {cap('hva',f'(= (iexp a lc {IM} mlo slo) None)')} {absurd_v('hp1',[RV])}))
          (case Some (va)
            (chain
              {cap('hva',f'(= (iexp a lc {IM} mlo slo) (Some va))')}
              {cb_have}
              {dep_have}
              (have hmax (= (le (ixf_dep a) (imax2 (ixf_dep a) (ixf_dep (IConst cc)))) True) (rewrite-with (lemma imax2_ge_l) lr lhs ((inst b (ixf_dep (IConst cc)))) () refl))
              (have hp4a (= (le (+ fp (* 8 (+ nl (+ d (ixf_dep a))))) (xmemhi_of m)) True) (by arith {sl2.cert({'hp4':1,'hdep':8,'hmax':8})}))
              (have hcost (= (ixf_ecost {e}) {costrhs}) (steps ((unfold ixf_ecost lhs) (unfold ixf_elen lhs) (reduce lhs)) refl))
              (have hp5a (= (le (ixf_ecost a) c) True) (steps ((unfold ixf_ecost lhs)) (by arith {sl2.cert({'hp5':1,'hcost':1})})))
              (have hih {IHA} (rewrite-with (hyp ih) lr lhs ({PINSV('va')}) ({D('ha')} {D('hva')} {D('2')} {D('hcb')} {D('hp4a')} {D('hp5a')}) refl))
              (rewrite-with (lemma {step}) lr lhs ((inst a a) (inst ia ia) (inst va va) {pins_extra} {PINS6}) ({D('hp0')} {D('hp1')} {D('2')} {D('hp3')} {D('hp4')} {D('hp5')} {D('ha')} {D('hva')} {D('hih')}))
              refl))))))))"""

def unary_path_plain(step, e, k_len, pins_extra, cb_have, dep_have_text, sl, dep_is_sub=True):
    """for ITrunc/ILoad/IExt-like: ixf_dep e = ixf_dep a (no imax2)"""
    RA="(rewrite (premise ha) lr rhs true ())"
    RV="(rewrite (premise hva) lr rhs true ())"
    sl2 = Slots(sl.names + ["ha","hva","hcb","hdep","hcost"])
    costrhs = f"(+ (+ (ixf_elen a) {k_len}) 5)" if k_len else "(+ (ixf_elen a) 5)"
    return f"""(case-on (ixf_exp nl d a) Option
  ((case None (chain {cap('ha','(= (ixf_exp nl d a) None)')} {absurd_ie('hp0',[RA])}))
   (case Some (ia)
     (chain
       {cap('ha','(= (ixf_exp nl d a) (Some ia))')}
       (case-on (iexp a lc {IM} mlo slo) Option
         ((case None (chain {cap('hva',f'(= (iexp a lc {IM} mlo slo) None)')} {absurd_v('hp1',[RV])}))
          (case Some (va)
            (chain
              {cap('hva',f'(= (iexp a lc {IM} mlo slo) (Some va))')}
              {cb_have}
              {dep_have_text}
              (have hcost (= (ixf_ecost {e}) {costrhs}) (steps ((unfold ixf_ecost lhs) (unfold ixf_elen lhs) (reduce lhs)) refl))
              (have hp4a (= (le (+ fp (* 8 (+ nl (+ d (ixf_dep a))))) (xmemhi_of m)) True) (steps ((rewrite (premise hdep) rl lhs true ()) (rewrite (premise hp4) lr lhs true ())) refl))
              (have hp5a (= (le (ixf_ecost a) c) True) (steps ((unfold ixf_ecost lhs)) (by arith {Slots(sl2.names+['hp4a']).cert({'hp5':1,'hcost':1})})))
              (have hih {IHA} (rewrite-with (hyp ih) lr lhs ({PINSV('va')}) ({D('ha')} {D('hva')} {D('2')} {D('hcb')} {D('hp4a')} {D('hp5a')}) refl))
              (rewrite-with (lemma {step}) lr lhs ((inst a a) (inst ia ia) (inst va va) {pins_extra} {PINS6}) ({D('hp0')} {D('hp1')} {D('2')} {D('hp3')} {D('hp4')} {D('hp5')} {D('ha')} {D('hva')} {D('hih')}))
              refl))))))))"""

def bin_path(step, e, K, OP, pins_extra, sl):
    RA="(rewrite (premise ha) lr rhs true ())"; RB="(rewrite (premise hb) lr rhs true ())"
    RVA="(rewrite (premise hva) lr rhs true ())"; RVB="(rewrite (premise hvb) lr rhs true ())"
    names = list(sl.names) + ["ha","hb","hva","hvb"]
    s2 = Slots(names)
    lines=[]
    def add(line, nm):
        lines.append(line); s2.add(nm)
    for nm in ["hdisc","hlocs","hxlo","hmlo","hslo","hal","hnl","hd0","hslo0","hhi"]:
        add(ctx_have(nm), nm)
    add("              (have hnn (= (le 0 (ilen lc)) True) (rewrite-with (lemma ilen_nonneg) lr lhs () () refl))","hnn")
    add("              (have hdepa (= (le 0 (ixf_dep a)) True) (rewrite-with (lemma ixf_dep_nonneg) lr lhs () () refl))","hdepa")
    add("              (have hdepb (= (le 0 (ixf_dep b)) True) (rewrite-with (lemma ixf_dep_nonneg) lr lhs () () refl))","hdepb")
    add(f"              (have hdepe (= (ixf_dep {e}) (+ 1 (imax2 (ixf_dep a) (ixf_dep b)))) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))","hdepe")
    add("              (have hmaxa (= (le (ixf_dep a) (imax2 (ixf_dep a) (ixf_dep b))) True) (rewrite-with (lemma imax2_ge_l) lr lhs ((inst b (ixf_dep b))) () refl))","hmaxa")
    add("              (have hmaxb (= (le (ixf_dep b) (imax2 (ixf_dep a) (ixf_dep b))) True) (rewrite-with (lemma imax2_ge_r) lr lhs ((inst a (ixf_dep a))) () refl))","hmaxb")
    add(f"              (have hcba (= (ixf_cb a) True) (rewrite-with (lemma cb_bin_a) lr lhs ((inst k {K}) (inst o {OP}) (inst b b)) ({D('hp3')}) refl))","hcba")
    add(f"              (have hcbb (= (ixf_cb b) True) (rewrite-with (lemma cb_bin_b) lr lhs ((inst k {K}) (inst o {OP}) (inst a a)) ({D('hp3')}) refl))","hcbb")
    add(f"              (have hnd0 (= (le 0 (+ nl d)) True) (by arith {s2.cert({'hnl':1,'hnn':1,'hd0':1})}))","hnd0")
    add(f"              (have hsa (= (le slo {ADDRB}) True) (by arith {s2.cert({'hslo':1,'hnl':8,'hnn':8,'hd0':8})}))","hsa")
    add(f"              (have hali (= (int_eq (mod (- {ADDRB} slo) 8) 0) True) (rewrite-with (lemma al_shift) lr lhs ((inst k (+ nl d))) ({D('hal')}) refl))","hali")
    add(f"              (have hlo8 (= (le (+ {ADDRB} 8) slo) False) (by arith {s2.cert({'hslo':1,'hnl':8,'hnn':8,'hd0':8})}))","hlo8")
    add(f"              (have hdisca (= (fp_disc slo (fp_app {TA_} psx)) True) (rewrite-with (lemma fe_tr_disc) lr lhs ((inst e a)) ({D('hdisc')} {D('hslo')} {D('hal')} {D('hnd0')}) refl))","hdisca")
    add(f"              (have hdisc2 (= (fp_disc slo {PSX2}) True) (rewrite-with (lemma fp_disc_w_slot) lr lhs () ({D('hlo8')} {D('hsa')} {D('hali')} {D('hdisca')}) refl))","hdisc2")
    add(f"              (have hlocsa (= (fr_locs fp lc (fp_app {TA_} psx)) True) (rewrite-with (lemma fe_tr_locs) lr lhs ((inst e a)) ({D('hlocs')} {D('hnl')} {D('hd0')}) refl))","hlocsa")
    add(f"              (have hpast (= (le (+ fp (* 8 (ilen lc))) {ADDRB}) True) (by arith {s2.cert({'hnl':8,'hd0':8})}))","hpast")
    add(f"              (have hlocs2 (= (fr_locs fp lc {PSX2}) True) (rewrite-with (lemma fr_locs_skip) lr lhs () ({D('hlocsa')} {D('hpast')}) refl))","hlocs2")
    add(f"              (have hd1 (= (le 0 (+ d 1)) True) (by arith {s2.cert({'hd0':1})}))","hd1")
    add(f"              (have hctx2 (= (fe_ctx m mlo slo fp nl (+ d 1) lc {PSX2}) True) (rewrite-with (lemma ctx_intro) lr lhs () ({D('hdisc2')} {D('hlocs2')} {D('hxlo')} {D('hmlo')} {D('hslo')} {D('hal')} {D('hnl')} {D('hd1')} {D('hslo0')} {D('hhi')}) refl))","hctx2")
    add(f"              (have hfb (= (fbelow slo {PSX2}) (fbelow slo psx)) (chain (rewrite-with (lemma fbelow_w_hi) lr lhs () ({D('hlo8')})) (rewrite-with (lemma fe_tr_below) lr lhs ((inst e a)) ({D('hslo')} {D('hnd0')})) refl))","hfb")
    add(f"              (have hp4a (= (le (+ fp (* 8 (+ nl (+ d (ixf_dep a))))) (xmemhi_of m)) True) (by arith {s2.cert({'hp4':1,'hdepe':8,'hmaxa':8})}))","hp4a")
    add(f"              (have hp4b (= (le (+ fp (* 8 (+ nl (+ (+ d 1) (ixf_dep b))))) (xmemhi_of m)) True) (by arith {s2.cert({'hp4':1,'hdepe':8,'hmaxb':8})}))","hp4b")
    add("              (have hlena (= (xil ia) (ixf_elen a)) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d d) (inst e a)) ((steps ((rewrite (premise ha) lr lhs true ())) refl)) refl))","hlena")
    add("              (have hlb0 (= (le 0 (ixf_elen b)) True) (rewrite-with (lemma ixf_elen_nonneg) lr lhs () () refl))","hlb0")
    oplen = OPLEN[OP]
    add(f"              (have hcost (= (ixf_ecost {e}) (+ (+ (ixf_elen a) (+ 3 (+ (ixf_elen b) {3+oplen}))) 5)) (steps ((unfold ixf_ecost lhs) (unfold ixf_elen lhs) (reduce lhs) (unfold ixf_oplen lhs) (reduce lhs) (compute lhs (stop ixf_elen))) refl))","hcost")
    add(f"              (have hp5a (= (le (ixf_ecost a) c) True) (steps ((unfold ixf_ecost lhs)) (by arith {s2.cert({'hp5':1,'hcost':1,'hlb0':1})})))","hp5a")
    add(f"              (have hp5b (= (le (ixf_ecost b) (- (- c (xil ia)) 3)) True) (steps ((unfold ixf_ecost lhs)) (by arith {s2.cert({'hp5':1,'hcost':1,'hlena':-1})})))","hp5b")
    add(f"              (have hvb2 (= (iexp b lc (fp_mem mem0 (fbelow slo {PSX2})) mlo slo) (Some vb)) (steps ((rewrite (premise hfb) lr lhs true ()) (rewrite (premise hvb) lr lhs true ())) refl))","hvb2")
    add(f"              (have hiha {IHA} (rewrite-with (hyp ih) lr lhs ({PINSV('va')}) ({D('ha')} {D('hva')} {D('2')} {D('hcba')} {D('hp4a')} {D('hp5a')}) refl))","hiha")
    IHB = f"(= {RUNB()} (fe_out vb {RS2()} {RUNB()} {MB}))"
    add(f"              (have hihb {IHB} (rewrite-with (hyp ih1) lr lhs ((inst nl nl) (inst d (+ d 1)) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst v vb)) ({D('hb')} {D('hvb2')} {D('hctx2')} {D('hcbb')} {D('hp4b')} {D('hp5b')}) refl))","hihb")
    body = "\n".join(lines)
    return f"""(case-on (ixf_exp nl d a) Option
  ((case None (chain {cap('ha','(= (ixf_exp nl d a) None)')} {absurd_ie('hp0',[RA])}))
   (case Some (ia)
     (chain
       {cap('ha','(= (ixf_exp nl d a) (Some ia))')}
       (case-on (ixf_exp nl (+ d 1) b) Option
         ((case None (chain {cap('hb','(= (ixf_exp nl (+ d 1) b) None)')} {absurd_ie('hp0',[RA,RB])}))
          (case Some (ib)
            (chain
              {cap('hb','(= (ixf_exp nl (+ d 1) b) (Some ib))')}
              (case-on (iexp a lc {IM} mlo slo) Option
                ((case None (chain {cap('hva',f'(= (iexp a lc {IM} mlo slo) None)')} {absurd_v('hp1',[RVA])}))
                 (case Some (va)
                   (chain
                     {cap('hva',f'(= (iexp a lc {IM} mlo slo) (Some va))')}
                     (case-on (iexp b lc {IM} mlo slo) Option
                       ((case None (chain {cap('hvb',f'(= (iexp b lc {IM} mlo slo) None)')} {absurd_v('hp1',[RVA,RVB])}))
                        (case Some (vb)
                          (chain
                            {cap('hvb',f'(= (iexp b lc {IM} mlo slo) (Some vb))')}
{body}
                            (rewrite-with (lemma {step}) lr lhs ((inst a a) (inst b b) (inst ia ia) (inst va va) (inst ib ib) (inst vb vb) {pins_extra} {PINS6}) ({D('hp0')} {D('hp1')} {D('2')} {D('hp3')} {D('hp4')} {D('hp5')} {D('ha')} {D('hva')} {D('hiha')} {D('hb')} {D('hvb')} {D('hihb')}))
                            refl))))))))))))))))"""

OPLEN = {"IAdd":2,"ISub":2,"IMul":2,"IDiv":4,"IRem":5,"IAnd":2,"IOr":2,"IXor":2,"IEq":3,"ILt":3,"ILe":3}

def absurd_k_bin(hp):
    """ixf_exp (IBin K OP a b) = None: after the a/b sub-emissions the bop is refused"""
    RA="(rewrite (premise ha) lr rhs true ())"; RB="(rewrite (premise hb) lr rhs true ())"
    return f"""(case-on (ixf_exp nl d a) Option
  ((case None (chain {cap('ha','(= (ixf_exp nl d a) None)')} {absurd_ie(hp,[RA])}))
   (case Some (ia)
     (chain
       {cap('ha','(= (ixf_exp nl d a) (Some ia))')}
       (case-on (ixf_exp nl (+ d 1) b) Option
         ((case None (chain {cap('hb','(= (ixf_exp nl (+ d 1) b) None)')} {absurd_ie(hp,[RA,RB])}))
          (case Some (ib)
            (chain
              {cap('hb','(= (ixf_exp nl (+ d 1) b) (Some ib))')}
              {absurd_ie(hp,[RA,RB,'(unfold ixf_bop rhs)'])}))))))))"""

def fe_sound():
    base = Slots(["p0","p1","p2","p3","p4","p5"])
    # ---- IConst / ILoc / IExt
    iconst = f"(rewrite-with (lemma fe_step_const) lr lhs ((inst n n) {PINS6}) ({P05}) refl)"
    iloc = f"(rewrite-with (lemma fe_step_loc) lr lhs ((inst i i) {PINS6}) ({P05}) refl)"
    iext = f"""(chain
  (have hp0 (= (ixf_exp nl d a) (Some ie)) (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_exp rhs) (reduce rhs)) refl))
  (have hp1 (= (iexp a lc {IM} mlo slo) (Some v)) (steps ((rewrite (premise 1) rl rhs true ()) (unfold iexp rhs) (reduce rhs)) refl))
  (have hcb (= (ixf_cb a) True) (rewrite-with (lemma cb_ext) lr lhs ((inst k1 k1) (inst k2 k2)) ({D('3')}) refl))
  (have hdep (= (ixf_dep (IExt k1 k2 a)) (ixf_dep a)) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))
  (have hp4 (= (le (+ fp (* 8 (+ nl (+ d (ixf_dep a))))) (xmemhi_of m)) True) (steps ((rewrite (premise hdep) rl lhs true ()) (rewrite (premise 4) lr lhs true ())) refl))
  (have hcost (= (ixf_ecost (IExt k1 k2 a)) (ixf_ecost a)) (steps ((unfold ixf_ecost lhs) (unfold ixf_elen lhs) (reduce lhs) (unfold ixf_ecost rhs)) refl))
  (have hp5 (= (le (ixf_ecost a) c) True) (steps ((rewrite (premise hcost) rl lhs true ()) (rewrite (premise 5) lr lhs true ())) refl))
  (have hih (= {RUN('ie')} (fe_out v {RS} {RUN('ie')} {MA_a})) (rewrite-with (hyp ih) lr lhs ({PINS6}) ({D('hp0')} {D('hp1')} {D('2')} {D('hcb')} {D('hp4')} {D('hp5')}) refl))
  (rewrite-with (lemma fe_step_ext) lr lhs ((inst k1 k1) (inst k2 k2) (inst a a) {PINS6}) ({P05} {D('hih')}))
  refl)"""
    # ---- ITrunc
    def trunc_arm(kind, step, k_len):
        e=f"(ITrunc k1 {kind} a)"
        sl = Slots(base.names + ["hk","hp0","hp1","hp3","hp4","hp5"])
        cb = f"(have hcb (= (ixf_cb a) True) (rewrite-with (lemma cb_trunc) lr lhs ((inst k1 k1) (inst kt {kind})) ({D('hp3')}) refl))"
        dep = f"(have hdep (= (ixf_dep {e}) (ixf_dep a)) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))"
        return f"""(case {kind}
  (chain
    {cap('hk', f'(= kt {kind})')}
    (steps ((rewrite (premise hk) lr both true ())))
{respell(e, ['hk'])}
    {unary_path_plain(step, e, k_len, '(inst k1 k1)', cb, dep, sl)}))"""
    itrunc = f"""(case-on kt IKind
  ({trunc_arm('U8','fe_step_trunc8',1)}
   {trunc_arm('U32','fe_step_trunc32',1)}
   {trunc_arm('U64','fe_step_trunc64',0)}))"""
    # ---- ILoad
    sl = Slots(base.names + ["hp0","hp1","hp3","hp4","hp5"])
    iload = f"""(chain
{respell('(ILoad a)', [])}
  {unary_path_plain('fe_step_load', '(ILoad a)', 1, '', f"(have hcb (= (ixf_cb a) True) (rewrite-with (lemma cb_load) lr lhs () ({D('hp3')}) refl))", "(have hdep (= (ixf_dep (ILoad a)) (ixf_dep a)) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))", sl)})"""
    # ---- IRotr
    def rotr_absurd_k(kind):
        return f"""(case {kind}
  (chain
    {cap('hk', f'(= k {kind})')}
    (have hp0 (= (ixf_exp nl d (IRotr {kind} a cx)) (Some ie)) (steps ((rewrite (premise hk) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
    {absurd_ie('hp0',[])}))"""
    def rotr_c_absurd(ctor, binders):
        return f"""(case {ctor}{(' (' + binders + ')') if binders else ''}
  (chain
    {cap('hcc', f'(= cx ({ctor} {binders}))')}
    (have hp0 (= (ixf_exp nl d (IRotr U32 a ({ctor} {binders}))) (Some ie)) (steps ((rewrite (premise hcc) rl lhs true ()) (rewrite (premise hk) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
    {absurd_ie('hp0',[])}))"""
    others = "\n   ".join([rotr_c_absurd("ILoc","j"), rotr_c_absurd("IBin","k2 o2 a2 b2"), rotr_c_absurd("IRotr","k2 a2 c2"),
                          rotr_c_absurd("IExt","k2 k3 a2"), rotr_c_absurd("ITrunc","k2 k3 a2"), rotr_c_absurd("ILoad","a2")])
    e_rot = "(IRotr U32 a (IConst cc))"
    sl = Slots(base.names + ["hk","hcc","hp0","hp1","hp3","hp4","hp5"])
    cb_rot = f"(have hcb (= (ixf_cb a) True) (rewrite-with (lemma cb_rotr_a) lr lhs ((inst k U32) (inst c (IConst cc))) ({D('hp3')}) refl))"
    dep_rot = f"(have hdep (= (ixf_dep {e_rot}) (+ 1 (imax2 (ixf_dep a) (ixf_dep (IConst cc))))) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))"
    irotr = f"""(case-on k IKind
  ({rotr_absurd_k('U8')}
   (case U32
     (chain
       {cap('hk','(= k U32)')}
       (steps ((rewrite (premise hk) lr both true ())))
       (case-on cx IExp
         ((case IConst
            (cc)
            (chain
              {cap('hcc','(= cx (IConst cc))')}
              (steps ((rewrite (premise hcc) lr both true ())))
{respell(e_rot, ['hcc','hk'])}
              {unary_path('fe_step_rotr32', e_rot, 1, '(inst cc cc)', cb_rot, dep_rot, sl)}))
          {others}))))
   {rotr_absurd_k('U64')}))"""
    # ---- IBin
    def shift_absurd_b(op, ctor, binders):
        return f"""(case {ctor}{(' (' + binders + ')') if binders else ''}
  (chain
    {cap('hbc', f'(= b ({ctor} {binders}))')}
    (have hp0 (= (ixf_exp nl d (IBin k {op} a ({ctor} {binders}))) (Some ie)) (steps ((rewrite (premise hbc) rl lhs true ()) (rewrite (premise ho) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
    {absurd_ie('hp0',[])}))"""
    def shift_arm(op):
        oth = "\n   ".join([shift_absurd_b(op,"ILoc","j"), shift_absurd_b(op,"IBin","k2 o2 a2 b2"), shift_absurd_b(op,"IRotr","k2 a2 c2"),
                            shift_absurd_b(op,"IExt","k2 k3 a2"), shift_absurd_b(op,"ITrunc","k2 k3 a2"), shift_absurd_b(op,"ILoad","a2")])
        if op == "IShl":
            def kl(kind, step):
                e=f"(IBin {kind} IShl a (IConst cc))"
                sl = Slots(base.names + ["ho","hbc","hk","hp0","hp1","hp3","hp4","hp5"])
                cb = f"(have hcb (= (ixf_cb a) True) (rewrite-with (lemma cb_bin_a) lr lhs ((inst k {kind}) (inst o IShl) (inst b (IConst cc))) ({D('hp3')}) refl))"
                dep = f"(have hdep (= (ixf_dep {e}) (+ 1 (imax2 (ixf_dep a) (ixf_dep (IConst cc))))) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))"
                return f"""(case {kind}
  (chain
    {cap('hk', f'(= k {kind})')}
    (steps ((rewrite (premise hk) lr both true ())))
{respell(e, ['hk','hbc','ho'])}
    {unary_path(step, e, 1, '(inst cc cc)', cb, dep, sl)}))"""
            def k8():
                return f"""(case U8
  (chain
    {cap('hk','(= k U8)')}
    (have hp0 (= (ixf_exp nl d (IBin U8 IShl a (IConst cc))) (Some ie)) (steps ((rewrite (premise hk) rl lhs true ()) (rewrite (premise hbc) rl lhs true ()) (rewrite (premise ho) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
    (case-on (ixf_exp nl d a) Option
      ((case None (chain {cap('ha','(= (ixf_exp nl d a) None)')} {absurd_ie('hp0',['(rewrite (premise ha) lr rhs true ())'])}))
       (case Some (ia) (chain {cap('ha','(= (ixf_exp nl d a) (Some ia))')} {absurd_ie('hp0',['(rewrite (premise ha) lr rhs true ())'])}))))))"""
            const = f"""(case IConst
  (cc)
  (chain
    {cap('hbc','(= b (IConst cc))')}
    (steps ((rewrite (premise hbc) lr both true ())))
    (case-on k IKind
      ({k8()}
       {kl('U32','fe_step_shl32')}
       {kl('U64','fe_step_shl64')}))))"""
        else:
            e="(IBin k IShr a (IConst cc))"
            sl = Slots(base.names + ["ho","hbc","hp0","hp1","hp3","hp4","hp5"])
            cb = f"(have hcb (= (ixf_cb a) True) (rewrite-with (lemma cb_bin_a) lr lhs ((inst k k) (inst o IShr) (inst b (IConst cc))) ({D('hp3')}) refl))"
            dep = f"(have hdep (= (ixf_dep {e}) (+ 1 (imax2 (ixf_dep a) (ixf_dep (IConst cc))))) (steps ((unfold ixf_dep lhs) (reduce lhs)) refl))"
            const = f"""(case IConst
  (cc)
  (chain
    {cap('hbc','(= b (IConst cc))')}
    (steps ((rewrite (premise hbc) lr both true ())))
{respell(e, ['hbc','ho'])}
    {unary_path('fe_step_shr', e, 1, '(inst k k) (inst cc cc)', cb, dep, sl)}))"""
        return f"""(chain
  {cap('ho', f'(= o {op})')}
  (steps ((rewrite (premise ho) lr both true ())))
  (case-on b IExp
    ({const}
     {oth})))"""
    def general_arm(op):
        kinds = {"IAdd":["U32","U64"],"ISub":["U32","U64"],"IMul":["U32","U64"],"IAnd":["U64"],"IOr":["U64"],"IXor":["U64"]}.get(op)
        if kinds is None:  # kind-generic: div rem eq lt le
            e=f"(IBin k {op} a b)"
            step = {"IDiv":"fe_step_div","IRem":"fe_step_rem","IEq":"fe_step_eq","ILt":"fe_step_lt","ILe":"fe_step_le"}[op]
            sl = Slots(base.names + ["ho","hp0","hp1","hp3","hp4","hp5"])
            return f"""(chain
  {cap('ho', f'(= o {op})')}
  (steps ((rewrite (premise ho) lr both true ())))
{respell(e, ['ho'])}
  {bin_path(step, e, 'k', op, '(inst k k)', sl)})"""
        arms=[]
        for kind in ["U8","U32","U64"]:
            e=f"(IBin {kind} {op} a b)"
            if kind in kinds:
                step=f"fe_step_{op.lower()[1:]}{kind[1:]}"
                sl = Slots(base.names + ["ho","hk","hp0","hp1","hp3","hp4","hp5"])
                arms.append(f"""(case {kind}
  (chain
    {cap('hk', f'(= k {kind})')}
    (steps ((rewrite (premise hk) lr both true ())))
{respell(e, ['hk','ho'])}
    {bin_path(step, e, kind, op, '', sl)}))""")
            else:
                arms.append(f"""(case {kind}
  (chain
    {cap('hk', f'(= k {kind})')}
    (have hp0 (= (ixf_exp nl d {e}) (Some ie)) (steps ((rewrite (premise hk) rl lhs true ()) (rewrite (premise ho) rl lhs true ()) (rewrite (premise 0) lr lhs true ())) refl))
    {absurd_k_bin('hp0')}))""")
        return f"""(chain
  {cap('ho', f'(= o {op})')}
  (steps ((rewrite (premise ho) lr both true ())))
  (case-on k IKind
    ({chr(10).join('     '+a for a in arms)[5:]})))"""
    arms=[]
    for op in OPS:
        if op in ("IShl","IShr"): arms.append(f"(case {op} {shift_arm(op)})")
        else: arms.append(f"(case {op} {general_arm(op)})")
    ibin = "(case-on o IOp\n  (" + "\n   ".join(arms) + "))"
    global _sound_pieces
    _sound_pieces = dict(iconst=iconst, iloc=iloc, iext=iext, itrunc=itrunc, iload=iload, irotr=irotr, ibin=ibin, **{"arm_"+a.split()[1]: a for a in arms})
    return f""";; THE EXPRESSION LEMMA (generated by gen_fra.py fe_sound — REGENERATE, never hand-patch):
;; every accepted expression's emission simulates imp's value, at any tower at or
;; above its cost, leaving RAX = the value, the frame's locals intact, the spill
;; trace as the twin says, and the scratch as the run's own
(claim fe_sound
  (goal
    ((e IExp) {GOALVARS})
    ({common_premises('e')[1:]})
{conclusion('e')}
  (induct e
    ((case IConst (n) {iconst})
     (case ILoc (i) {iloc})
     (case IBin (k o a b) {ibin})
     (case IRotr (k a cx) {irotr})
     (case IExt (k1 k2 a) {iext})
     (case ITrunc (k1 kt a) {itrunc})
     (case ILoad (a) {iload}))))
"""


# ---------------- A-3: statement step lemmas ----------------
SGOALVARS = ("(nl Int) (own Int) (fail_ix Int) (is (List XInstr)) (lc (List Int)) (lc2 (List Int)) (mem2 Mem) (mem0 Mem) (psx (List FPatch)) "
             "(mlo Int) (slo Int) (c Int) (g Nat) (m XModule) (fp Int) (dep Int) (f Nat) (fs (List IpFn)) (dmax Int) "
             "(a0 Int) (rcx Int) (dx Int) (rbx Int) (rbp Int) (rsi Int) (di Int) (r8 Int) (r9 Int) (s10 Int) (s11 Int) (r12 Int) (r13 Int)")
def SRUN(is_): return f"(xeval_seq (xt c g) m {is_} {RS} {XM})"
def IMPST(s): return f"(ipstmt (S f) fs mlo slo dmax dep {s} lc {IM})"
def MTW(s): return f"(fp_mem mem0 (fp_app (ips_tr fp nl {s} lc {IM} mlo slo) psx))"
ACC15 = ["rax"]+ACC14

def stmt_set():
    e="(IpSet i e)"
    ADDR="(+ fp (* 8 i))"
    TE=f"(fe_tr fp nl 0 e lc {IM} mlo slo)"
    ME=f"(fp_mem mem0 (fp_app {TE} psx))"
    RUNE=RUN('ie')
    sl=Slots(["p0","p1","p2","p3","p4","p5"])
    L=[]
    def add(line,nm): L.append(line); sl.add(nm)
    add(f"""    (have hie (= (ixf_exp nl 0 e) (Some ie)) (steps ((rewrite (hyp 0) lr lhs true ())) refl))""","hie")  # placeholder replaced below
    # we structure with case-ons, so build the text directly
    sl=Slots(["p0","p1","p2","p3","p4","p5","p6","p7","hie","hsome","his","hv","hset","hn1","hn2","hlc","hmem"])
    def c(m,G=1): return sl.cert(m,G)
    lines=[]
    for nm in ["hdisc","hlocs","hxlo","hmlo","hslo","hal","hnl","hd0","hslo0","hhi"]:
        lines.append(ctx_have(nm, "0")); sl.add(nm)
    lines.append("    (have hnn (= (le 0 (ilen lc)) True) (rewrite-with (lemma ilen_nonneg) lr lhs () () refl))"); sl.add("hnn")
    lines.append("    (have hdep (= (le 0 (ixf_dep e)) True) (rewrite-with (lemma ixf_dep_nonneg) lr lhs () () refl))"); sl.add("hdep")
    lines.append("    (have hi0 (= (le 0 i) True) (rewrite-with (lemma ilset_lo) lr lhs ((inst ls lc) (inst v v) (inst ls2 lcs)) ((steps ((rewrite (premise hset) lr lhs true ())) refl)) refl))"); sl.add("hi0")
    lines.append("    (have hii (= (lt i (ilen lc)) True) (rewrite-with (lemma ilset_hi) lr lhs ((inst v v) (inst ls2 lcs)) ((steps ((rewrite (premise hset) lr lhs true ())) refl)) refl))"); sl.add("hii")
    lines.append(f"    (have hb (= (fe_inband v) True) (rewrite-with (lemma fe_band) lr lhs ((inst lc lc) (inst mem {IM}) (inst mlo mlo) (inst msz slo) (inst fp fp) (inst psx psx) (inst e e)) ((steps ((rewrite (premise hv) lr lhs true ())) refl) (steps ((rewrite (premise hlocs) lr lhs true ())) refl) (steps ((rewrite (premise 3) lr lhs true ())) refl)) refl))"); sl.add("hb")
    lines.append(f"    (have hxl (= (xmemlo_of m) mlo) (by arith {sl.cert2({'hxlo':-1},{'hxlo':1})}))"); sl.add("hxl")
    lines.append(f"    (have hfp0 (= (le 0 fp) True) (by arith {c({'hslo':1,'hslo0':1})}))"); sl.add("hfp0")
    lines.append(f"    (have hwlo (= (le (xmemlo_of m) {ADDR}) True) (steps ((rewrite (premise hxl) lr lhs true ())) (by arith {c({'hmlo':1,'hslo':1,'hi0':8})})))"); sl.add("hwlo")
    lines.append(f"    (have hwhi (= (le (+ {ADDR} 8) (xmemhi_of m)) True) (by arith {c({'p4':1,'hii':8,'hnl':8,'hdep':8})}))"); sl.add("hwhi")
    lines.append(f"    (have hsm (= (lt (+ {ADDR} 8) 18446744073709551616) True) (by arith {c({'hwhi':1,'hhi':1})}))"); sl.add("hsm")
    lines.append(f"    (have hp4 (= (le (+ fp (* 8 (+ nl (+ 0 (ixf_dep e))))) (xmemhi_of m)) True) (by arith {c({'p4':1})}))"); sl.add("hp4")
    lines.append(f"    (have hp5 (= (le (ixf_ecost e) c) True) (by arith {c({'p5':1})}))"); sl.add("hp5")
    lines.append(f"    (have hexp (= {RUNE} (fe_out v {RS} {RUNE} {ME})) (rewrite-with (lemma fe_sound) lr lhs ((inst e e) (inst nl nl) (inst d 0) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst v v)) ((steps ((rewrite (premise hie) lr lhs true ())) refl) (steps ((rewrite (premise hv) lr lhs true ())) refl) (steps ((rewrite (premise 2) lr lhs true ())) refl) (steps ((rewrite (premise 3) lr lhs true ())) refl) (steps ((rewrite (premise hp4) lr lhs true ())) refl) (steps ((rewrite (premise hp5) lr lhs true ())) refl)) refl))"); sl.add("hexp")
    lines.append("    (have hlen (= (xil ie) (ixf_elen e)) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d 0) (inst e e)) ((steps ((rewrite (premise hie) lr lhs true ())) refl)) refl))"); sl.add("hlen")
    lines.append("    (have hcost (= (ixf_ecost e) (+ (ixf_elen e) 5)) (steps ((unfold ixf_ecost lhs)) refl))"); sl.add("hcost")
    lines.append(f"    (have hle (= (le (xil ie) c) True) (by arith {c({'p5':1,'hlen':-1,'hcost':1})}))"); sl.add("hle")
    lines.append("    (have hxs (= (xil (ixf_st i)) 3) (steps ((compute lhs)) refl))"); sl.add("hxs")
    lines.append(f"    (have hst3 (= (le 3 (- c (xil ie))) True) (by arith {c({'p5':1,'hlen':-1,'hcost':1})}))"); sl.add("hst3")
    lines.append(f"    (have hst4 (= (le 4 (- c (xil ie))) True) (by arith {c({'p5':1,'hlen':-1,'hcost':1})}))"); sl.add("hst4")
    lines.append(f"    (have hnil (= (lt 0 (- (- c (xil ie)) 3)) True) (by arith {c({'p5':1,'hlen':-1,'hcost':1})}))"); sl.add("hnil")
    acc_rs = "".join(f" (rewrite (lemma {f}_of_mk) lr lhs true ())" for f in ["rcx","rbx","rbp","rsi","rdi","r8","r9","r12","r13","r14","r15"])
    RSF=f"(MkRegs v rcx (rdx_of (xo_regs {RUNE})) rbx rbp rsi di r8 r9 {ADDR} (r11_of (xo_regs {RUNE})) r12 r13 dep fp)"
    MF=f"(fp_mem mem0 (Cons (FWord {ADDR} v) (fp_app {TE} psx)))"
    lines.append(f"""    (have hrun (= {SRUN('is')} (Some (XNorm {RSF} {MF})))
      (chain
        (steps ((rewrite (premise his) rl lhs true ())))
        (rewrite-with (lemma xseq_app) lr lhs () ((steps ((rewrite (premise hle) lr lhs true ())) refl)))
        (steps ((rewrite (premise hexp) lr lhs true ()) (unfold fe_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs){acc_rs}))
        (rewrite-with (lemma fe_st_run) lr lhs () ((steps ((rewrite (premise hst4) lr lhs true ())) refl) (steps ((rewrite (premise hi0) lr lhs true ())) refl) (steps ((rewrite (premise hfp0) lr lhs true ())) refl) (steps ((rewrite (premise hsm) lr lhs true ())) refl) (steps ((rewrite (premise hwlo) lr lhs true ())) refl) (steps ((rewrite (premise hwhi) lr lhs true ())) refl)))
        (steps ((rewrite (lemma fp_mem_cons_w) lr lhs true ())) refl)))"""); sl.add("hrun")
    acc = "\n".join(f"       (rewrite (lemma {f}_of_mk) lr rhs true ())" for f in ACC15)
    body="\n".join(lines)
    return f"""(claim fs_step_set
  (goal
    ((i Int) (e IExp) (ie (List XInstr)) (v Int) {SGOALVARS})
    ((= (ixf_stmt nl own fail_ix {e}) (Some is))
     (= {IMPST(e)} (Some (IpNorm lc2 mem2)))
     (= (fe_ctx m mlo slo fp nl 0 lc psx) True)
     (= (ixf_cb e) True)
     (= (le (+ fp (* 8 (+ nl (ixf_dep e)))) (xmemhi_of m)) True)
     (= (le (+ (ixf_ecost e) 4) c) True)
     (= (ixf_exp nl 0 e) (Some ie))
     (= (iexp e lc {IM} mlo slo) (Some v)))
    (= {SRUN('is')} (fs_out {RS} {SRUN('is')} {MTW(e)})))
  (chain
    (have hie (= (ixf_exp nl 0 e) (Some ie)) (steps ((rewrite (premise 6) lr lhs true ())) refl))
    (have hsome (= (Some (ix_app ie (ixf_st i))) (Some is))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hsome) (his))
    (have hv (= (iexp e lc {IM} mlo slo) (Some v)) (steps ((rewrite (premise 7) lr lhs true ())) refl))
    (case-on (ilset lc i v) Option
      ((case None
         (chain
           (have hset (= (ilset lc i v) None) (steps ((rewrite (hyp 0) lr lhs true ())) refl))
           (have hn (= (Some IpTrap) (Some (IpNorm lc2 mem2)))
             (steps ((rewrite (premise 1) rl rhs true ()) (unfold ipstmt rhs) (reduce rhs) (rewrite (premise hv) lr rhs true ()) (reduce rhs) (rewrite (premise hset) lr rhs true ()) (reduce rhs)) refl))
           (inject (premise hn) (hx))
           (absurd (premise hx))))
       (case Some
         (lcs)
         (chain
           (have hset (= (ilset lc i v) (Some lcs)) (steps ((rewrite (hyp 0) lr lhs true ())) refl))
           (have hn1 (= (Some (IpNorm lcs {IM})) (Some (IpNorm lc2 mem2)))
             (steps ((rewrite (premise 1) rl rhs true ()) (unfold ipstmt rhs) (reduce rhs) (rewrite (premise hv) lr rhs true ()) (reduce rhs) (rewrite (premise hset) lr rhs true ()) (reduce rhs)) refl))
           (inject (premise hn1) (hn2))
           (inject (premise hn2) (hlc hmem))
{body}
           (steps
             ((rewrite (premise hrun) lr both true ())
              (rewrite (lemma fs_out_of_norm) lr rhs true ())
{acc}
              (unfold ips_tr rhs)
              (reduce rhs)
              (rewrite (premise hv) lr rhs true ())
              (reduce rhs)
              (unfold fp_app rhs)
              (reduce rhs))
             refl)))))))
"""

# ---------------- the statement step lemmas, MEMORY family (A-3 part 3) ----------------
# IpLoadW: ia ++ [XMem (XMLoad64 RAX (AReg RAX))] ++ ixf_st i — the word read below the
# cut agrees with imp's (fp_below_lw through the twin), then the slot store (fe_st_run).
# IpStore / IpStoreW: ia ++ spill(nl,0) ++ iv ++ reload10(nl,0) ++ [store] — the binary
# choreography (step_binary's) with fe_sound cited for both operands, then ONE byte /
# word patch below the cut (fe_stb_run / fe_stw_run; fp_mem_cons_b / _w). The spill
# slot is spelled (+ nl 0) by the translator and nl by ips_tr: add0 reconciles.

ACC_RS = "".join(f" (rewrite (lemma {f}_of_mk) lr lhs true ())" for f in ["rcx","rbx","rbp","rsi","rdi","r8","r9","r12","r13","r14","r15"])
def SD(h): return f"(steps ((rewrite (premise {h}) lr lhs true ())) refl)"
def RVR(h): return f"(rewrite (premise {h}) lr rhs true ())"
def CAP(name, eq): return f"(have {name} {eq} (steps ((rewrite (hyp 0) lr lhs true ())) refl))"

def absn_stmt(rws):
    """the imp engine (premise 1 = a normal outcome) returns a trap on this leg"""
    items=["(rewrite (premise 1) rl rhs true ())","(unfold ipstmt rhs)","(reduce rhs)"]
    for r in rws: items+=[r,"(reduce rhs)"]
    return f"""(chain
           (have hn (= (Some IpTrap) (Some (IpNorm lc2 mem2))) (steps ({' '.join(items)}) refl))
           (inject (premise hn) (hx))
           (absurd (premise hx)))"""

def ctx_lines(sl, names=("hdisc","hlocs","hxlo","hmlo","hslo","hal","hnl","hd0","hslo0","hhi")):
    out=[]
    for nm in names:
        out.append(ctx_have(nm, "0")); sl.add(nm)
    return out

def stmt_loadw():
    e="(IpLoadW i ae)"
    ADDR="(+ fp (* 8 i))"
    TE=f"(fe_tr fp nl 0 ae lc {IM} mlo slo)"
    ME=f"(fp_mem mem0 (fp_app {TE} psx))"
    RUNE=RUN('ie')
    VAL=f"(load_le (iw8) {IM} ad)"
    LDW="(XMem (XMLoad64 RAX (AReg RAX)))"
    sl=Slots(["p0","p1","p2","p3","p4","p5","p6","p7","hie","hcons","hsome","his","hv","hg0","hg1","hset","hn1","hn2","hlc","hmem"])
    def c(m,G=1): return sl.cert(m,G)
    L=[]
    def add(line,nm): L.append(line); sl.add(nm)
    L += ctx_lines(sl)
    add("    (have hnn (= (le 0 (ilen lc)) True) (rewrite-with (lemma ilen_nonneg) lr lhs () () refl))","hnn")
    add("    (have hdep (= (le 0 (ixf_dep ae)) True) (rewrite-with (lemma ixf_dep_nonneg) lr lhs () () refl))","hdep")
    add(f"    (have hi0 (= (le 0 i) True) (rewrite-with (lemma ilset_lo) lr lhs ((inst ls lc) (inst v {VAL}) (inst ls2 lcs)) ({SD('hset')}) refl))","hi0")
    add(f"    (have hii (= (lt i (ilen lc)) True) (rewrite-with (lemma ilset_hi) lr lhs ((inst v {VAL}) (inst ls2 lcs)) ({SD('hset')}) refl))","hii")
    add(f"    (have hxl (= (xmemlo_of m) mlo) (by arith {sl.cert2({'hxlo':-1},{'hxlo':1})}))","hxl")
    add(f"    (have hfp0 (= (le 0 fp) True) (by arith {c({'hslo':1,'hslo0':1})}))","hfp0")
    add(f"    (have hwlo (= (le (xmemlo_of m) {ADDR}) True) (steps ((rewrite (premise hxl) lr lhs true ())) (by arith {c({'hmlo':1,'hslo':1,'hi0':8})})))","hwlo")
    add(f"    (have hwhi (= (le (+ {ADDR} 8) (xmemhi_of m)) True) (by arith {c({'p4':1,'hii':8,'hnl':8,'hdep':8})}))","hwhi")
    add(f"    (have hsm (= (lt (+ {ADDR} 8) 18446744073709551616) True) (by arith {c({'hwhi':1,'hhi':1})}))","hsm")
    add(f"    (have hp4 (= (le (+ fp (* 8 (+ nl (+ 0 (ixf_dep ae))))) (xmemhi_of m)) True) (by arith {c({'p4':1})}))","hp4")
    add(f"    (have hp5 (= (le (ixf_ecost ae) c) True) (by arith {c({'p5':1})}))","hp5")
    add(f"    (have hexp (= {RUNE} (fe_out ad {RS} {RUNE} {ME})) (rewrite-with (lemma fe_sound) lr lhs ((inst e ae) (inst nl nl) (inst d 0) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst v ad)) ({SD('hie')} {SD('hv')} {SD(2)} {SD(3)} {SD('hp4')} {SD('hp5')}) refl))","hexp")
    add("    (have hlen (= (xil ie) (ixf_elen ae)) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d 0) (inst e ae)) ((steps ((rewrite (premise hie) lr lhs true ())) refl)) refl))","hlen")
    add("    (have hcost (= (ixf_ecost ae) (+ (ixf_elen ae) 5)) (steps ((unfold ixf_ecost lhs)) refl))","hcost")
    T={'p5':1,'hlen':-1,'hcost':1}
    add(f"    (have hle (= (le (xil ie) c) True) (by arith {c(T)}))","hle")
    add(f"    (have hx1 (= (xil (list {LDW})) 1) (steps ((compute lhs)) refl))","hx1")
    add(f"    (have hl1 (= (le 1 (- c (xil ie))) True) (by arith {c(T)}))","hl1")
    add(f"    (have hl2 (= (le 2 (- c (xil ie))) True) (by arith {c(T)}))","hl2")
    add(f"    (have hst4 (= (le 4 (- (- c (xil ie)) 1)) True) (by arith {c(T)}))","hst4")
    add(f"    (have hglo (= (le (xmemlo_of m) ad) True) (steps ((rewrite (premise hxl) lr lhs true ())) (by arith {c({'hg0':1})})))","hglo")
    add(f"    (have hghi (= (le (+ ad 8) (xmemhi_of m)) True) (by arith {c({'hg1':1,'hslo':1,'p4':1,'hnl':8,'hnn':8,'hdep':8})}))","hghi")
    add(f"    (have hnd0 (= (le 0 (+ nl 0)) True) (by arith {c({'hnl':1,'hnn':1})}))","hnd0")
    add(f"    (have hdisca (= (fp_disc slo (fp_app {TE} psx)) True) (rewrite-with (lemma fe_tr_disc) lr lhs ((inst e ae)) ({SD('hdisc')} {SD('hslo')} {SD('hal')} {SD('hnd0')}) refl))","hdisca")
    add("    (have hw8i (= (int_of_nat (xw8)) 8) (rewrite-with (lemma w8_int) lr lhs () () refl))","hw8i")
    add(f"    (have hbelow (= (le (+ ad (int_of_nat (xw8))) slo) True) (steps ((rewrite (premise hw8i) lr lhs true ())) (by arith {c({'hg1':1})})))","hbelow")
    add("    (have hiw (= (xw8) (iw8)) (steps ((compute both)) refl))","hiw")
    add(f"""    (have hld (= (load_le (xw8) {ME} ad) {VAL})
      (chain
        (rewrite-with (lemma fp_below_lw) lr lhs ((inst slo slo)) ({SD('hdisca')} {SD('hbelow')}))
        (rewrite-with (lemma fe_tr_below) lr lhs ((inst e ae)) ({SD('hslo')} {SD('hnd0')}))
        (steps ((rewrite (premise hiw) lr lhs true ())) refl)))""","hld")
    RSF=f"(MkRegs {VAL} rcx (rdx_of (xo_regs {RUNE})) rbx rbp rsi di r8 r9 {ADDR} (r11_of (xo_regs {RUNE})) r12 r13 dep fp)"
    MF=f"(fp_mem mem0 (Cons (FWord {ADDR} {VAL}) (fp_app {TE} psx)))"
    add(f"""    (have hrun (= {SRUN('is')} (Some (XNorm {RSF} {MF})))
      (chain
        (steps ((rewrite (premise his) rl lhs true ())))
        (rewrite-with (lemma xseq_app) lr lhs () ({SD('hle')}))
        (steps ((rewrite (premise hexp) lr lhs true ()) (unfold fe_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs){ACC_RS}))
        (rewrite-with (lemma xseq_app) lr lhs () ((steps ((rewrite (premise hx1) lr lhs true ()) (rewrite (premise hl1) lr lhs true ())) refl)))
        (steps ((rewrite (premise hx1) lr lhs true ())))
        (rewrite-with (lemma fe_ldw_run) lr lhs () ({SD('hl2')} {SD('hglo')} {SD('hghi')}))
        (steps ((rewrite (premise hld) lr lhs true ()) (unfold xcont lhs) (reduce lhs)))
        (rewrite-with (lemma fe_st_run) lr lhs () ({SD('hst4')} {SD('hi0')} {SD('hfp0')} {SD('hsm')} {SD('hwlo')} {SD('hwhi')}))
        (steps ((rewrite (lemma fp_mem_cons_w) lr lhs true ())) refl)))""","hrun")
    acc = "\n".join(f"       (rewrite (lemma {f}_of_mk) lr rhs true ())" for f in ACC15)
    body="\n".join(L)
    RG0=RVR('hg0'); RG1=RVR('hg1'); RHV=RVR('hv')
    return f"""(claim fs_step_loadw
  (goal
    ((i Int) (ae IExp) (ie (List XInstr)) (ad Int) {SGOALVARS})
    ((= (ixf_stmt nl own fail_ix {e}) (Some is))
     (= {IMPST(e)} (Some (IpNorm lc2 mem2)))
     (= (fe_ctx m mlo slo fp nl 0 lc psx) True)
     (= (ixf_cb ae) True)
     (= (le (+ fp (* 8 (+ nl (ixf_dep ae)))) (xmemhi_of m)) True)
     (= (le (+ (ixf_ecost ae) 4) c) True)
     (= (ixf_exp nl 0 ae) (Some ie))
     (= (iexp ae lc {IM} mlo slo) (Some ad)))
    (= {SRUN('is')} (fs_out {RS} {SRUN('is')} {MTW(e)})))
  (chain
    (have hie (= (ixf_exp nl 0 ae) (Some ie)) (steps ((rewrite (premise 6) lr lhs true ())) refl))
    (have hcons (= (ix_app (list {LDW}) (ixf_st i)) (Cons {LDW} (ixf_st i)))
      (steps ((unfold ix_app lhs) (reduce lhs) (unfold ix_app lhs) (reduce lhs)) refl))
    (have hsome (= (Some (ix_app ie (ix_app (list {LDW}) (ixf_st i)))) (Some is))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hie) lr rhs true ()) (reduce rhs) (rewrite (premise hcons) lr lhs true ())) refl))
    (inject (premise hsome) (his))
    (have hv (= (iexp ae lc {IM} mlo slo) (Some ad)) (steps ((rewrite (premise 7) lr lhs true ())) refl))
    (case-on (le mlo ad) Bool
      ((case False (chain {CAP('hg0','(= (le mlo ad) False)')} {absn_stmt([RHV,RG0])}))
       (case True
         (chain
           {CAP('hg0','(= (le mlo ad) True)')}
           (case-on (le (+ ad 8) slo) Bool
             ((case False (chain {CAP('hg1','(= (le (+ ad 8) slo) False)')} {absn_stmt([RHV,RG0,RG1])}))
              (case True
                (chain
                  {CAP('hg1','(= (le (+ ad 8) slo) True)')}
                  (case-on (ilset lc i {VAL}) Option
                    ((case None
                       (chain
                         {CAP('hset', f'(= (ilset lc i {VAL}) None)')}
                         {absn_stmt([RHV,RG0,RG1,RVR('hset')])}))
                     (case Some
                       (lcs)
                       (chain
                         {CAP('hset', f'(= (ilset lc i {VAL}) (Some lcs))')}
                         (have hn1 (= (Some (IpNorm lcs {IM})) (Some (IpNorm lc2 mem2)))
                           (steps ((rewrite (premise 1) rl rhs true ()) (unfold ipstmt rhs) (reduce rhs) {RHV} (reduce rhs) {RG0} (reduce rhs) {RG1} (reduce rhs) {RVR('hset')} (reduce rhs)) refl))
                         (inject (premise hn1) (hn2))
                         (inject (premise hn2) (hlc hmem))
{body}
                         (steps
                           ((rewrite (premise hrun) lr both true ())
                            (rewrite (lemma fs_out_of_norm) lr rhs true ())
{acc}
                            (unfold ips_tr rhs)
                            (reduce rhs)
                            (rewrite (premise hv) lr rhs true ())
                            (reduce rhs)
                            (unfold fp_app rhs)
                            (reduce rhs))
                           refl)))))))))))))))
"""

def stmt_store_like(word):
    name = "fs_step_storew" if word else "fs_step_store"
    e = "(IpStoreW ae ve)" if word else "(IpStore ae ve)"
    G1 = "(le (+ ad 8) slo)" if word else "(lt ad slo)"
    STI = "(XMem (XMStore64 (AReg R10) RAX))" if word else "(XStore8 (AReg R10) (SReg RAX))"
    BLK = "fe_stw_run" if word else "fe_stb_run"
    CONS = "fp_mem_cons_w" if word else "fp_mem_cons_b"
    PATCH = "(FWord ad vv)" if word else "(FByte ad vv)"
    MEM2 = f"(store_le (iw8) {IM} ad vv)" if word else f"(mem_set {IM} ad vv)"
    SPAN = "8" if word else "1"
    A = "(+ fp (* 8 nl))"
    A0 = "(+ fp (* 8 (+ nl 0)))"
    TA = f"(fe_tr fp nl 0 ae lc {IM} mlo slo)"
    PSX2 = f"(Cons (FWord {A} ad) (fp_app {TA} psx))"
    XM2 = f"(fp_mem mem0 {PSX2})"
    IM2 = f"(fp_mem mem0 (fbelow slo {PSX2}))"
    TB = f"(fe_tr fp nl 1 ve lc {IM2} mlo slo)"
    TBI = f"(fe_tr fp nl 1 ve lc {IM} mlo slo)"
    MB = f"(fp_mem mem0 (fp_app {TB} {PSX2}))"
    MA = f"(fp_mem mem0 (fp_app {TA} psx))"
    RUNA = RUN('ia')
    RS2 = f"(MkRegs ad rcx (rdx_of (xo_regs {RUNA})) rbx rbp rsi di r8 r9 {A} (r11_of (xo_regs {RUNA})) r12 r13 dep fp)"
    RUNV = f"(xeval_seq (xt (- (- c (xil ia)) 3) g) m iv {RS2} {XM2})"
    EM = f"(ix_app ia (ix_app (ixf_spill nl 0) (ix_app iv (ix_app (ixf_reload10 nl 0) (list {STI})))))"
    sl=Slots([f"p{i}" for i in range(11)]+["hia","hva","hiv","hvv","hsome","his","hg0","hg1","hn1","hn2","hlc","hmem"])
    def c(m,G=1): return sl.cert(m,G)
    L=[]
    def add(line,nm): L.append(line); sl.add(nm)
    L += ctx_lines(sl)
    add("    (have hnn (= (le 0 (ilen lc)) True) (rewrite-with (lemma ilen_nonneg) lr lhs () () refl))","hnn")
    add("    (have hdepa (= (le 0 (ixf_dep ae)) True) (rewrite-with (lemma ixf_dep_nonneg) lr lhs () () refl))","hdepa")
    add("    (have hdepv (= (le 0 (ixf_dep ve)) True) (rewrite-with (lemma ixf_dep_nonneg) lr lhs () () refl))","hdepv")
    add("    (have hmaxa (= (le (ixf_dep ae) (imax2 (ixf_dep ae) (ixf_dep ve))) True) (rewrite-with (lemma imax2_ge_l) lr lhs ((inst b (ixf_dep ve))) () refl))","hmaxa")
    add("    (have hmaxv (= (le (ixf_dep ve) (imax2 (ixf_dep ae) (ixf_dep ve))) True) (rewrite-with (lemma imax2_ge_r) lr lhs ((inst a (ixf_dep ae))) () refl))","hmaxv")
    add(f"    (have hxl (= (xmemlo_of m) mlo) (by arith {sl.cert2({'hxlo':-1},{'hxlo':1})}))","hxl")
    add(f"    (have hnl0 (= (le 0 nl) True) (by arith {c({'hnl':1,'hnn':1})}))","hnl0")
    add(f"    (have hnd0 (= (le 0 (+ nl 0)) True) (by arith {c({'hnl':1,'hnn':1})}))","hnd0")
    add(f"    (have hnd1 (= (le 0 (+ nl 1)) True) (by arith {c({'hnl':1,'hnn':1})}))","hnd1")
    add(f"    (have hd1 (= (le 0 1) True) (by arith {c({})}))","hd1")
    add(f"    (have hfp0 (= (le 0 fp) True) (by arith {c({'hslo':1,'hslo0':1})}))","hfp0")
    add(f"    (have hsa (= (le slo {A}) True) (by arith {c({'hslo':1,'hnl0':8})}))","hsa")
    add(f"    (have hwlo (= (le (xmemlo_of m) {A}) True) (steps ((rewrite (premise hxl) lr lhs true ())) (by arith {c({'hmlo':1,'hslo':1,'hnl0':8})})))","hwlo")
    add(f"    (have hwhi (= (le (+ {A} 8) (xmemhi_of m)) True) (by arith {c({'p5':1,'hmaxa':8,'hdepa':8})}))","hwhi")
    add(f"    (have hsm (= (lt (+ {A} 8) 18446744073709551616) True) (by arith {c({'hwhi':1,'hhi':1})}))","hsm")
    add(f"    (have hwlo0 (= (le (xmemlo_of m) {A0}) True) (by arith {c({'hwlo':1})}))","hwlo0")
    add(f"    (have hwhi0 (= (le (+ {A0} 8) (xmemhi_of m)) True) (by arith {c({'hwhi':1})}))","hwhi0")
    add(f"    (have hsm0 (= (lt (+ {A0} 8) 18446744073709551616) True) (by arith {c({'hsm':1})}))","hsm0")
    add(f"    (have hali (= (int_eq (mod (- {A} slo) 8) 0) True) (rewrite-with (lemma al_shift) lr lhs ((inst k nl)) ({SD('hal')}) refl))","hali")
    add(f"    (have hlo8 (= (le (+ {A} 8) slo) False) (by arith {c({'hslo':1,'hnl0':8})}))","hlo8")
    add(f"    (have hba (= (fe_inband ad) True) (rewrite-with (lemma fe_band) lr lhs ((inst lc lc) (inst mem {IM}) (inst mlo mlo) (inst msz slo) (inst fp fp) (inst psx psx) (inst e ae)) ({SD('hva')} {SD('hlocs')} {SD(3)}) refl))","hba")
    add(f"    (have hal0 (= (le 0 ad) True) (rewrite-with (lemma inband_lo) lr lhs () ({SD('hba')}) refl))","hal0")
    add(f"    (have hah (= (lt ad 18446744073709551616) True) (rewrite-with (lemma inband_hi) lr lhs () ({SD('hba')}) refl))","hah")
    add(f"    (have hdisca (= (fp_disc slo (fp_app {TA} psx)) True) (rewrite-with (lemma fe_tr_disc) lr lhs ((inst e ae)) ({SD('hdisc')} {SD('hslo')} {SD('hal')} {SD('hnd0')}) refl))","hdisca")
    add(f"    (have hdisc2 (= (fp_disc slo {PSX2}) True) (rewrite-with (lemma fp_disc_w_slot) lr lhs () ({SD('hlo8')} {SD('hsa')} {SD('hali')} {SD('hdisca')}) refl))","hdisc2")
    add(f"    (have hdiscb (= (fp_disc slo (fp_app {TB} {PSX2})) True) (rewrite-with (lemma fe_tr_disc) lr lhs ((inst e ve)) ({SD('hdisc2')} {SD('hslo')} {SD('hal')} {SD('hnd1')}) refl))","hdiscb")
    add(f"    (have hlocsa (= (fr_locs fp lc (fp_app {TA} psx)) True) (rewrite-with (lemma fe_tr_locs) lr lhs ((inst e ae)) ({SD('hlocs')} {SD('hnl')} {SD('hd0')}) refl))","hlocsa")
    add(f"    (have hpast (= (le (+ fp (* 8 (ilen lc))) {A}) True) (by arith {c({'hnl':8})}))","hpast")
    add(f"    (have hlocs2 (= (fr_locs fp lc {PSX2}) True) (rewrite-with (lemma fr_locs_skip) lr lhs () ({SD('hlocsa')} {SD('hpast')}) refl))","hlocs2")
    add(f"    (have hctx2 (= (fe_ctx m mlo slo fp nl 1 lc {PSX2}) True) (rewrite-with (lemma ctx_intro) lr lhs () ({SD('hdisc2')} {SD('hlocs2')} {SD('hxlo')} {SD('hmlo')} {SD('hslo')} {SD('hal')} {SD('hnl')} {SD('hd1')} {SD('hslo0')} {SD('hhi')}) refl))","hctx2")
    add(f"    (have hfb (= (fbelow slo {PSX2}) (fbelow slo psx)) (chain (rewrite-with (lemma fbelow_w_hi) lr lhs () ({SD('hlo8')})) (rewrite-with (lemma fe_tr_below) lr lhs ((inst e ae)) ({SD('hslo')} {SD('hnd0')})) refl))","hfb")
    add(f"    (have hp4a (= (le (+ fp (* 8 (+ nl (+ 0 (ixf_dep ae))))) (xmemhi_of m)) True) (by arith {c({'p5':1,'hmaxa':8})}))","hp4a")
    add(f"    (have hp4v (= (le (+ fp (* 8 (+ nl (+ 1 (ixf_dep ve))))) (xmemhi_of m)) True) (by arith {c({'p5':1,'hmaxv':8})}))","hp4v")
    add(f"    (have hlena (= (xil ia) (ixf_elen ae)) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d 0) (inst e ae)) ({SD('hia')}) refl))","hlena")
    add(f"    (have hlenv (= (xil iv) (ixf_elen ve)) (rewrite-with (lemma fe_len) lr lhs ((inst nl nl) (inst d 1) (inst e ve)) ({SD('hiv')}) refl))","hlenv")
    add("    (have hcosta (= (ixf_ecost ae) (+ (ixf_elen ae) 5)) (steps ((unfold ixf_ecost lhs)) refl))","hcosta")
    add("    (have hcostv (= (ixf_ecost ve) (+ (ixf_elen ve) 5)) (steps ((unfold ixf_ecost lhs)) refl))","hcostv")
    add("    (have hlv0 (= (le 0 (ixf_elen ve)) True) (rewrite-with (lemma ixf_elen_nonneg) lr lhs () () refl))","hlv0")
    T={'p6':1,'hlena':-1,'hlenv':-1,'hcosta':1,'hcostv':1}
    T1={'p6':1,'hlena':-1,'hcosta':1,'hcostv':1,'hlv0':1}
    add(f"    (have hp5a (= (le (ixf_ecost ae) c) True) (by arith {c({'p6':1,'hcostv':1,'hlv0':1})}))","hp5a")
    add(f"    (have hle1 (= (le (xil ia) c) True) (by arith {c(T1)}))","hle1")
    add(f"    (have hsp4 (= (le 4 (- c (xil ia))) True) (by arith {c(T1)}))","hsp4")
    add(f"    (have hsp3 (= (le 3 (- c (xil ia))) True) (by arith {c(T1)}))","hsp3")
    add(f"    (have hlev (= (le (xil iv) (- (- c (xil ia)) 3)) True) (by arith {c(T)}))","hlev")
    add(f"    (have hp5v (= (le (ixf_ecost ve) (- (- c (xil ia)) 3)) True) (by arith {c({'p6':1,'hlena':-1,'hcosta':1})}))","hp5v")
    add(f"    (have hrl4 (= (le 4 (- (- (- c (xil ia)) 3) (xil iv))) True) (by arith {c(T)}))","hrl4")
    add(f"    (have hrl3 (= (le 3 (- (- (- c (xil ia)) 3) (xil iv))) True) (by arith {c(T)}))","hrl3")
    add(f"    (have hst2 (= (le 2 (- (- (- (- c (xil ia)) 3) (xil iv)) 3)) True) (by arith {c(T)}))","hst2")
    add("    (have hxs (= (xil (ixf_st nl)) 3) (steps ((compute lhs)) refl))","hxs")
    add("    (have hxr (= (xil (ixf_reload10 nl 0)) 3) (steps ((compute lhs)) refl))","hxr")
    add(f"    (have hminv (= (fp_min {TB} (+ fp (* 8 (+ nl 1)))) True) (rewrite-with (lemma fe_tr_min) lr lhs ((inst e ve)) () refl))","hminv")
    add(f"    (have hlt1 (= (lt {A} (+ fp (* 8 (+ nl 1)))) True) (by arith {c({})}))","hlt1")
    add(f"    (have hwv (= (fp_wordv (fp_app {TB} {PSX2}) {A}) (Some ad)) (chain (rewrite-with (lemma fp_wordv_app_min) lr lhs ((inst lo (+ fp (* 8 (+ nl 1))))) ({SD('hminv')} {SD('hlt1')})) (steps ((unfold fp_wordv lhs) (reduce lhs) (rewrite (lemma int_eq_refl) lr lhs true ()) (reduce lhs)) refl)))","hwv")
    add(f"    (have hread (= (load_le (xw8) {MB} {A}) ad) (rewrite-with (lemma fp_read) lr lhs ((inst slo slo) (inst v ad)) ({SD('hdiscb')} {SD('hsa')} {SD('hali')} {SD('hwv')} {SD('hal0')} {SD('hah')}) refl))","hread")
    add(f"    (have hvv2 (= (iexp ve lc {IM2} mlo slo) (Some vv)) (steps ((rewrite (premise hfb) lr lhs true ()) (rewrite (premise hvv) lr lhs true ())) refl))","hvv2")
    add(f"    (have hexpa (= {RUNA} (fe_out ad {RS} {RUNA} {MA})) (rewrite-with (lemma fe_sound) lr lhs ((inst e ae) (inst nl nl) (inst d 0) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst v ad)) ({SD('hia')} {SD('hva')} {SD(2)} {SD(3)} {SD('hp4a')} {SD('hp5a')}) refl))","hexpa")
    add(f"    (have hexpv (= {RUNV} (fe_out vv {RS2} {RUNV} {MB})) (rewrite-with (lemma fe_sound) lr lhs ((inst e ve) (inst nl nl) (inst d 1) (inst lc lc) (inst mlo mlo) (inst slo slo) (inst v vv)) ({SD('hiv')} {SD('hvv2')} {SD('hctx2')} {SD(4)} {SD('hp4v')} {SD('hp5v')}) refl))","hexpv")
    add(f"    (have hglo (= (le (xmemlo_of m) ad) True) (steps ((rewrite (premise hxl) lr lhs true ())) (by arith {c({'hg0':1})})))","hglo")
    add(f"    (have hghi (= (le (+ ad {SPAN}) (xmemhi_of m)) True) (by arith {c({'hg1':1,'hslo':1,'p5':1,'hnl':8,'hnn':8,'hmaxa':8,'hdepa':8})}))","hghi")
    RSF=f"(MkRegs vv rcx (rdx_of (xo_regs {RUNV})) rbx rbp rsi di r8 r9 ad (r11_of (xo_regs {RUNV})) r12 r13 dep fp)"
    MF=f"(fp_mem mem0 (Cons {PATCH} (fp_app {TB} {PSX2})))"
    add(f"""    (have hrun (= {SRUN('is')} (Some (XNorm {RSF} {MF})))
      (chain
        (steps ((rewrite (premise his) rl lhs true ())))
        (rewrite-with (lemma xseq_app) lr lhs () ({SD('hle1')}))
        (steps ((rewrite (premise hexpa) lr lhs true ()) (unfold fe_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs){ACC_RS} (unfold ixf_spill lhs) (rewrite (lemma add0) lr lhs true ())))
        (rewrite-with (lemma xseq_app) lr lhs () ((steps ((rewrite (premise hxs) lr lhs true ()) (rewrite (premise hsp3) lr lhs true ())) refl)))
        (steps ((rewrite (premise hxs) lr lhs true ())))
        (rewrite-with (lemma fe_st_run) lr lhs () ({SD('hsp4')} {SD('hnl0')} {SD('hfp0')} {SD('hsm')} {SD('hwlo')} {SD('hwhi')}))
        (steps ((rewrite (lemma fp_mem_cons_w) lr lhs true ()) (unfold xcont lhs) (reduce lhs)))
        (rewrite-with (lemma xseq_app) lr lhs () ({SD('hlev')}))
        (steps ((rewrite (premise hexpv) lr lhs true ()) (unfold fe_out lhs) (reduce lhs) (unfold xcont lhs) (reduce lhs){ACC_RS}))
        (rewrite-with (lemma xseq_app) lr lhs () ((steps ((rewrite (premise hxr) lr lhs true ()) (rewrite (premise hrl3) lr lhs true ())) refl)))
        (steps ((rewrite (premise hxr) lr lhs true ())))
        (rewrite-with (lemma fe_rl_run) lr lhs () ({SD('hrl4')} {SD('hnd0')} {SD('hfp0')} {SD('hsm0')} {SD('hwlo0')} {SD('hwhi0')}))
        (steps ((rewrite (lemma add0) lr lhs true ()) (rewrite (premise hread) lr lhs true ()) (unfold xcont lhs) (reduce lhs)))
        (rewrite-with (lemma {BLK}) lr lhs () ({SD('hst2')} {SD('hglo')} {SD('hghi')}))
        (steps ((rewrite (lemma {CONS}) lr lhs true ())) refl)))""","hrun")
    add(f"    (have hc2 (= (fp_app (Cons (FWord {A} ad) {TA}) psx) {PSX2}) (steps ((unfold fp_app lhs) (reduce lhs)) refl))","hc2")
    add(f"""    (have htw (= (fp_app (ips_tr fp nl {e} lc {IM} mlo slo) psx) (Cons {PATCH} (fp_app {TBI} {PSX2})))
      (steps ((unfold ips_tr lhs) (reduce lhs) (rewrite (premise hva) lr lhs true ()) (reduce lhs) (rewrite (premise hvv) lr lhs true ()) (reduce lhs) (unfold fp_app lhs) (reduce lhs) (rewrite (lemma fp_app_assoc) lr lhs true ()) (rewrite (premise hc2) lr lhs true ())) refl))""","htw")
    acc = "\n".join(f"       (rewrite (lemma {f}_of_mk) lr rhs true ())" for f in ACC15)
    body="\n".join(L)
    RG0=RVR('hg0'); RG1=RVR('hg1'); RHA=RVR('hva'); RHV=RVR('hvv')
    return f"""(claim {name}
  (goal
    ((ae IExp) (ve IExp) (ia (List XInstr)) (iv (List XInstr)) (ad Int) (vv Int) {SGOALVARS})
    ((= (ixf_stmt nl own fail_ix {e}) (Some is))
     (= {IMPST(e)} (Some (IpNorm lc2 mem2)))
     (= (fe_ctx m mlo slo fp nl 0 lc psx) True)
     (= (ixf_cb ae) True)
     (= (ixf_cb ve) True)
     (= (le (+ fp (* 8 (+ nl (+ 1 (imax2 (ixf_dep ae) (ixf_dep ve)))))) (xmemhi_of m)) True)
     (= (le (+ (ixf_ecost ae) (+ (ixf_ecost ve) 8)) c) True)
     (= (ixf_exp nl 0 ae) (Some ia))
     (= (iexp ae lc {IM} mlo slo) (Some ad))
     (= (ixf_exp nl 1 ve) (Some iv))
     (= (iexp ve lc {IM} mlo slo) (Some vv)))
    (= {SRUN('is')} (fs_out {RS} {SRUN('is')} {MTW(e)})))
  (chain
    (have hia (= (ixf_exp nl 0 ae) (Some ia)) (steps ((rewrite (premise 7) lr lhs true ())) refl))
    (have hva (= (iexp ae lc {IM} mlo slo) (Some ad)) (steps ((rewrite (premise 8) lr lhs true ())) refl))
    (have hiv (= (ixf_exp nl 1 ve) (Some iv)) (steps ((rewrite (premise 9) lr lhs true ())) refl))
    (have hvv (= (iexp ve lc {IM} mlo slo) (Some vv)) (steps ((rewrite (premise 10) lr lhs true ())) refl))
    (have hsome (= (Some {EM}) (Some is))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_stmt rhs) (reduce rhs) (rewrite (premise hia) lr rhs true ()) (reduce rhs) (rewrite (premise hiv) lr rhs true ()) (reduce rhs)) refl))
    (inject (premise hsome) (his))
    (case-on (le mlo ad) Bool
      ((case False (chain {CAP('hg0','(= (le mlo ad) False)')} {absn_stmt([RHA,RG0])}))
       (case True
         (chain
           {CAP('hg0','(= (le mlo ad) True)')}
           (case-on {G1} Bool
             ((case False (chain {CAP('hg1', f'(= {G1} False)')} {absn_stmt([RHA,RG0,RG1])}))
              (case True
                (chain
                  {CAP('hg1', f'(= {G1} True)')}
                  (have hn1 (= (Some (IpNorm lc {MEM2})) (Some (IpNorm lc2 mem2)))
                    (steps ((rewrite (premise 1) rl rhs true ()) (unfold ipstmt rhs) (reduce rhs) {RHA} (reduce rhs) {RG0} (reduce rhs) {RG1} (reduce rhs) {RHV} (reduce rhs)) refl))
                  (inject (premise hn1) (hn2))
                  (inject (premise hn2) (hlc hmem))
{body}
                  (steps
                    ((rewrite (premise hrun) lr both true ())
                     (rewrite (lemma fs_out_of_norm) lr rhs true ())
{acc}
                     (rewrite (premise htw) lr rhs true ())
                     (rewrite (premise hfb) lr lhs true ()))
                    refl)))))))))))
"""

def stmt_store(): return stmt_store_like(False)
def stmt_storew(): return stmt_store_like(True)

def stmt_fail():
    return f"""(claim fs_step_fail
  (goal
    ((fam IFam) (nl Int) (own Int) (fail_ix Int) (is (List XInstr)) (c Int) (g Nat) (m XModule) (rs Regs) (xm Mem))
    ((= (ixf_stmt nl own fail_ix (IpFail fam)) (Some is))
     (= (xfunc_at (xfuncs_of m) fail_ix) (Some (MkXFunc 0 (list (XMovRI RAX 60) XSyscall))))
     (= (le 8 c) True))
    (= (xeval_seq (xt c g) m is rs xm) (Some XTrap)))
  (chain
    (have hsome (= (Some (Cons (XMovRI RDI (match fam (FOverflow 70) (FOom 71) (FStack 72))) (Cons (XCall fail_ix) Nil))) (Some is))
      (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_stmt rhs) (reduce rhs)) refl))
    (inject (premise hsome) (his))
    (rewrite-with (lemma xt_peel8) lr lhs () ((steps ((rewrite (premise 2) lr lhs true ())) refl)))
    (steps
      ((rewrite (premise his) rl lhs true ())
       (compute lhs (stop xt xfunc_at xfuncs_of))
       (rewrite (premise 1) lr lhs true ())
       (compute lhs (stop xt)))
      refl)))
"""

def stmt_unreach():
    ADDR = "18446744073709547520"
    RSL = "(MkRegs a0 rcx dx rbx rbp rsi di r8 r9 s10 s11 r12 r13 dep fp)"
    def arm(tv, extra):
        core = ["(rewrite (premise his) rl lhs true ())",
                "(compute lhs (stop xt xmemlo_of xmemhi_of wrap64))",
                "(rewrite (premise hw) lr lhs true ())",
                "(rewrite (premise hlo) lr lhs true ())",
                "(reduce lhs)",
                "(compute lhs (stop xt xmemlo_of xmemhi_of))"] + extra
        return ("(case " + tv + " (chain "
                + "(have hlo (= (le (xmemlo_of m) " + ADDR + ") " + tv + ") (steps ((rewrite (hyp 0) lr lhs true ())) refl)) "
                + "(steps (" + " ".join(core) + ") refl)))")
    true_extra = ["(rewrite (premise hghi) lr lhs true ())","(reduce lhs)","(compute lhs (stop xt xmemlo_of xmemhi_of))"]
    out = []
    out.append("(claim fs_step_unreach")
    out.append("  (goal")
    out.append("    ((nl Int) (own Int) (fail_ix Int) (is (List XInstr)) (c Int) (g Nat) (m XModule) (mlo Int) (xm Mem) (a0 Int) (rcx Int) (dx Int) (rbx Int) (rbp Int) (rsi Int) (di Int) (r8 Int) (r9 Int) (s10 Int) (s11 Int) (r12 Int) (r13 Int) (dep Int) (fp Int))")
    out.append("    ((= (ixf_stmt nl own fail_ix IpUnreach) (Some is)) (= (int_eq (xmemlo_of m) mlo) True) (= (le 0 mlo) True) (= (le (xmemhi_of m) 4294967296) True) (= (le 4 c) True))")
    out.append("    (= (xeval_seq (xt c g) m is " + RSL + " xm) (Some XTrap)))")
    out.append("  (chain")
    out.append("    (have hsome (= (Some (Cons (XMovRI RAX " + ADDR + ") (Cons (XMem (XMLoad64 RAX (AReg RAX))) Nil))) (Some is)) (steps ((rewrite (premise 0) rl rhs true ()) (unfold ixf_stmt rhs) (reduce rhs)) refl))")
    out.append("    (inject (premise hsome) (his))")
    out.append("    (have hghi (= (le 18446744073709547528 (xmemhi_of m)) False) (by arith (list 1 0 0 0 1 0 0 0)))")
    out.append("    (have hw (= (wrap64 " + ADDR + ") " + ADDR + ") (chain (steps ((unfold wrap64 lhs))) (rewrite-with (lemma wrap64_id) lr lhs () ((by arith (list)) (by arith (list)))) refl))")
    out.append("    (rewrite-with (lemma xt_peel4) lr lhs () ((steps ((rewrite (premise 4) lr lhs true ())) refl)))")
    out.append("    (case-on (le (xmemlo_of m) " + ADDR + ") Bool")
    out.append("      (" + arm("False", []) + " " + arm("True", true_extra) + "))))")
    return "\n".join(out) + "\n\n"
def step_lemmas_stmt():
    return "\n".join([stmt_set(), stmt_store(), stmt_loadw(), stmt_storew(), stmt_fail(), stmt_unreach()])

# ---------------- entry point (kept last: splice needs every emitter defined) ----------------
if __name__ == "__main__":
    arg = sys.argv[1] if len(sys.argv) > 1 else "fe_len"
    if arg == "splice":
        splice()
    else:
        sys.stdout.write(emit(BANNERS[arg][1]))
