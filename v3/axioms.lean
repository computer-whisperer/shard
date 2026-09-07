/-
v3/axioms.lean — the T0 axiom-closure oracle (FOUNDATION §3.6: "identical
verdicts and axiom closures"). Run by v3/export.sh in the lean4export
checkout (`lake env lean v3/axioms.lean`) against the pinned toolchain; it
prints one line per constant of `Init`'s environment:

    AX <name>: <axiom> <axiom> …        (axioms sorted by Name.lt)

The closure is the transitive reachability of CollectAxioms' relation
(src/Lean/Util/CollectAxioms.lean at v4.33.1): an axiom contributes itself
and its type's constants; a definition, theorem or opaque its type's and
value's; a quotient nothing; a constructor and a recursor their type's; an
inductive its type's and its constructors. Computed here as a least
fixpoint over that relation (Kleene iteration in module order), NOT by
calling Lean's `collectAxioms`: that function memoizes with a sentinel that
makes an inductive's cached closure depend on whether it or one of its
constructors was visited first (an order-dependent cache, records §8), and
T0 compares by rule, not by tool name. K's side is v3/kernel/axioms.shard.
-/
import Lean
open Lean

def deps (env : Environment) (c : Name) : Array Name :=
  match env.find? c with
  | some (.axiomInfo v)  => v.type.getUsedConstants
  | some (.defnInfo v)   => v.type.getUsedConstants ++ v.value.getUsedConstants
  | some (.thmInfo v)    => v.type.getUsedConstants ++ v.value.getUsedConstants
  | some (.opaqueInfo v) => v.type.getUsedConstants ++ v.value.getUsedConstants
  | some (.quotInfo _)   => #[]
  | some (.ctorInfo v)   => v.type.getUsedConstants
  | some (.recInfo v)    => v.type.getUsedConstants
  | some (.inductInfo v) => v.type.getUsedConstants ++ v.ctors.toArray
  | none                 => #[]

def isAxiom (env : Environment) (c : Name) : Bool :=
  match env.find? c with
  | some (.axiomInfo _) => true
  | _ => false

def insertName (s : Array Name) (a : Name) : Array Name :=
  if s.contains a then s else s.push a

def main : IO Unit := do
  initSearchPath (← findSysroot)
  let env ← importModules #[{ module := `Init }] {} (trustLevel := 0)
  let env := env.setExporting false
  -- module order is import order: a constant's dependencies precede it
  -- except inside one inductive block (types ↔ constructors)
  let mut names : Array Name := #[]
  for md in env.header.moduleData do
    for n in md.constNames do
      names := names.push n
  let mut depsOf : Std.HashMap Name (Array Name) := {}
  let mut cl : Std.HashMap Name (Array Name) := {}
  for n in names do
    depsOf := depsOf.insert n (deps env n)
    cl := cl.insert n (if isAxiom env n then #[n] else #[])
  let mut changed := true
  let mut passes := 0
  while changed do
    changed := false
    passes := passes + 1
    for n in names do
      let mut s := cl[n]!
      let before := s.size
      for d in depsOf[n]! do
        if let some ds := cl[d]? then
          for a in ds do s := insertName s a
      if s.size != before then
        cl := cl.insert n s
        changed := true
  IO.eprintln s!"axioms: {names.size} constants, {passes} passes"
  let out ← IO.getStdout
  for n in names do
    let axs := (cl[n]!.qsort Name.lt).map (·.toString (escape := false))
    out.putStrLn s!"AX {n.toString (escape := false)}: {" ".intercalate axs.toList}"
