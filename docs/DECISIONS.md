# Decisions, with what was rejected

Each entry is a design choice in the system as it stands where an alternative was considered and
turned down: "X is done this way; not Y, because Z". Read the section for an area before proposing a
change in it: the road you are about to suggest may be here. A star (★) marks an entry whose
rejected alternative is what a newcomer, or someone who knows Lean 4, would try first.

An entry is a reason, not a prohibition. If its reason no longer holds, say so and reopen it; a
design choice the owner approved by bare assent is a checkpoint (`LAW.md` §5). Numbers quoted in
entries are explained under "Measured" at the end.

## Surface syntax

- ★ A name is scoped by writing it dotted (`(fn List.sum …)`); not `namespace`/`section`/`variable`
  blocks (refused, `reserved_form`) — identity is module path + declared name, nothing more.
- ★ `let` is sequential in terms and programs; not a parallel `let` beside `let*` — a census found
  30,611 `let` groups, 162 with two or more names, none citing an earlier binder: no source differs.
- ★ Type parameters are written `((T Type) …)` binders and universes `NAME.{u}`; not auto-bound
  names or a `universe` line — one binder rule for every form, nothing bound by spelling.
- ★ What the inputs leave open is refused with a pointer (`unsolved_implicit`, `unsolved_universe`,
  `instance_needed`); not defaulted — the one default is a numeral's `Nat`. Unification first-order.
- ★ Between data types the only inserted coercion is `Nat` to `Int` (`Int.ofNat`); not coercion
  classes — any other mismatch is `type_mismatch` naming both types, and `Int.toNat` is written.
  (Between `Bool` and propositions there are two more: `LANGUAGE.md`, "Terms".)
- ★ Instances come from a fixed table (`sp_instance`: Init's at `Nat`, `Int`, `Bool`, `List`); not
  instance search — `instance` and `class` are refused, an unsolved one is `instance_needed`.
- A class method under Init's instance (`HAdd.hAdd` at `Nat`) is read as its function by a table
  (`sp_canon`); not by transparency — `rw` and `simp_only` match by structure and need one spelling.
- An operator's type is the expected type, else its first non-numeral operand's; not the first
  operand's alone — `(- 1 2)` at `Int` would be a `Nat` subtraction, coerced.
- ★ `"…"` is a `String` in every file; not list sugar chosen by the expected type — one
  meaning per literal — and not a byte list in the toolchain's own files by their module path.
- ★ Numeral rows of a `match` are an `if` chain on `=`, the last row `_` or a name; not `Nat.succ`
  patterns through the matcher — `Nat` and `Int` run as integers and have no constructor cells.
- ★ Notation, macros, `syntax`, `elab`, attributes, `set_option` are refused (`lean_form`); not an
  extensible surface — it is fixed with the elaborator; lemma sets are passed by name.
- ★ `lemma`, `example`, `private`, `noncomputable`, `partial`, `unsafe`, `open`, `termination_by`,
  `deriving`, `implemented_by` are refused, the V3 spelling in the message; one form per concept.
- ★ A plain `if` gives its branches no proof; `(dif h C T F)` names it. Only around a self-call of
  a recursive `fn` is `if` compiled dependent, so that a decrease obligation sees the branch fact.
- `if` tests a two-constructor type's second constructor, by the type's declaration; not the cell's
  tag — on a view's opaque type (`private_if`) the tag would leak the hidden constructor order.
  (The evaluator and a native binary both do: `v3/pins/emit/if_second`.)
- ★ A specification is a `def` into `Prop`, a comparison a proposition, a length or address a `Nat`;
  not `Bool` functions with `(= (le a b) True)` over `Int` — Init's lemmas and `arith` then apply.

## Files, modules and views

- Identity is module path + declared name, with a hash of the checked declaration; not the source
  text — layout, comments and form order change nothing; moving a file changes the identity.
- ★ Init's declarations keep their bare names; a native file cannot add to Init's namespace —
  `List.foo` in `std/list.shard` is `std.list.List.foo`; an Init constant's very name: `name_taken`.
- `import` opens nothing and `use PREFIX` opens; one mechanism for modules and namespaces — both
  are name prefixes. A citation under two opened prefixes is an error naming both, never a choice.
- ★ A citation that two visible names answer is refused naming both (`ambiguous_name`), a file's
  own name against a name of Init under an opened namespace included; not precedence for the
  file's own module, over Init or over anything — an upgrade of Init can then refuse a file that
  did not change, and the file is renamed or opens the namespace by name (`LAW.md` §6, 11).
- A `type` opens its constructors for its own file; `(use M)` opens none, `(use M.T)` does — a
  prefix opens only what stands directly under it. An unused `use` is a lint (`UNUSED`), no refusal.
- ★ A hash is a fingerprint, never an authority: it is affine in a literal, so a preimage is one
  subtraction — pins compare declarations structurally, keys only report.
- ★ A consumer of a directory module holds only its view: each `sig fn`, `sig type`, `requirement`
  is a parameter K cannot unfold; not the bodies or `eq_N` — a missing fact is exported, not pried.
- Opacity is by not loading the body into the consumer's environment; not by a name gate — so no
  step (`rfl`, `decide`, `unfold`, `simp_only`) can see across a view, by construction.
- The implementation is checked in a fork with its declarations substituted at the signatures; not
  two same-named declarations with a lookup preference — K holds one declaration per name.
- A file in a directory that has a view is private (`private_module`) and carries the directory's
  path; not a facade of wrappers — a wrapper can lend a function the view's identity, never a type.
- K's seal is the directory `kernel/k` behind one view; not a view over the environment type —
  exporting the operations that build environments lets a forged one in through the API.
- An `axiom` is admitted only under `(trusts NAME)` in its own file and is a declared parameter
  every dependent is conditional on; not a load-wide trust list.
- ★ `sorry` admits nothing: the theorem is `PENDING` and citing it is refused
  (`pending_obligation`); not an axiom that lets dependents check.

## Definitions and recursion

- ★ A `fn` is elaborated once; its program is the erasure and its K value the compilation of that
  one term; not a second, program-side type system — the two would have to be tied by a test.
- A `fn` whose signature has an L reading must elaborate, else it is refused; not a fall-back to
  "program only" — a fallback may weaken a guarantee (`RUNNABLE why=…`), never reinterpret.
- ★ Structural recursion is Lean's `brecOn` over `below`, generated under Lean's names; not the
  bare recursor — a self-call at any depth, under a nested `match` or a `let`, is found by type.
- ★ Other recursion needs `(measure E)`, `E : Nat`, and is `WellFounded.fix`; each call's decrease
  is an obligation `f.dec_N` closed by `(fulfills f.dec_N …)`; not discharged by a default tactic.
- No `Int` measure (`measure_type`); a rank is added into the measure (`(+ (+ s s) 1)` against
  `(+ s s)`). A lexicographic tuple was held off "until the arithmetic is the burden"; it came
  when groups had no single number at all — a fuel at one call, a list at another (below, "A
  measure may have several parts").
- ★ An equation says nothing about termination: `f n = f (n+1)` holds of a constant function and
  proves by `rfl` while the body loops — so a body under an open measure is `PENDING`, not complete.
- ★ Functions that call each other are found as a group when the last is read; no `mutual` form —
  forward references already resolve, and a second surface would state the same fact twice.
- ★ A group is one fixpoint over a type generated for it (a constructor per member); not Lean's
  `PSum` of tuples. The reason recorded was `PSum`'s position in the export under a horizon;
  every file now sees `PSum`, so this stands as built and is open.
- ★ Not the structural route for groups: it owes no proofs but needs `below` for nested types and
  refuses a call that hands its argument on unchanged — five of the formatter's seven renderers.
- ★ Every member of a group states a `Nat` measure; `(measure (struct x))` there is `group_measure`;
  not read as "the size of x" — an author who wants the size derives it.
- ★ A value's size is derived on request, `(derive T size)`, as `def`s with no program; not made
  for every type like Lean's `sizeOf` — a type that asks for nothing adds no records.
- ★ A function proofs will run is written structurally with its `match` leading (structural fuel
  `FZ`/`FS`); not on a `Nat` measure — its one unconditional equation unfolds at every inner call.
- ★ A forward reference parks the function and retries it after each definition; not an ordering
  rule — what is still parked at the file's end is `RUNNABLE` with the obstacle named.
- ★ A function-typed parameter makes a `fn` a template: one definition, one program per instance,
  keyed by the function named and its captured count; not closures or dictionaries at run time.
- ★ A law-bearing interface is a `structure` with function-typed fields passed statically (a
  package); not a type class — its laws are erased, its fields static arguments, no dictionary.
- ★ Init's functions get programs by `realize` (a supplied body with equations K proves, or the
  erased view of a non-recursive body); not from the kernel term — `brecOn` hides the recursion.
- Realization equations are `NAME.realize_N`; not Lean's `NAME.eq_N` — the export declares `eq_N`
  for some constants only, and a native theorem of that name collides.
- A `fn` cannot take an admitted constant's name, nor a `def` a program-only `fn`'s (`name_taken`,
  `program_named`) — else a body calls one thing while its equations speak of another.
- Operations that differ at an edge are distinct entries (`Nat.sub` saturating, `Int.tdiv` zero
  at a zero divisor, the kernel's `/` stuck there); not one entry with a mode.
- ★ An `extern` is an opaque constant (a type, no equations), so its callers still get definitions;
  effects are bolt axioms over observables that a `bin` trusts; not an IO monad.
- Deriving writes ordinary `fn` source the loader reads, found at use through a table keyed on
  the closed type; not trusted generated terms and not instance search.
- The int map's lookup and insert (`intmap.shard` `im_find`, `im_insert`) state the recursion
  they have, `(measure (struct m))` — a child of the map at each call; not `(measure k)`, the
  key: an `Int` measure gives a function no definition (`measure_type`), and the key is not
  what ends the walk. Assent: "Good. Let's change the two measure lines and see where that
  gets us." Both are definitions by it.
- A toolchain function that recurses on an integer states a `Nat` measure, `(measure
  (Int.natAbs n))`, and the proof of each descent stands beside it, over a fact of
  `kernel/descent.shard`; not the obligations left pending — every theorem resting on such a
  function would list them, and under the measure `im_split` stated, its key, the one it
  owed is false at -1, which a shift leaves -1; not all 83 such functions at once — first
  the three under which the most others stood (`util.shard` `show_nat_go`, `unify.shard`
  `inst_level_go`, `intmap.shard` `im_split`), to learn what a proof costs. Assent, of the
  three ways put to them (leave pending, prove all, pilot three): "C works -- pilot the three
  largest first. If that works, probably the answer is to fork out the bulk changes to opus
  subagents."
- Chosen in building the pilot, not put to the owner one by one. `im_split` takes over from
  `im_insert` the test that the two keys are the same, and its measure is the two keys'
  distances from 0 together: keys that differ are not both -1, so one comes nearer at each
  call — the function ends at every pair of integers and returns what it returned; not a
  guard that turns a negative key away — the map as written finds and parts negative keys,
  and a guard would change what it returns for them. The facts are a toolchain file of
  theorems alone, which `util.shard` and `intmap.shard` import, so it is in K's closure; not
  in `util.shard` — the int map imports nothing else. A measured function's obligation is
  stated over its type parameters first: over the data binders alone it left a type
  parameter free, K refused the statement, and a measured function with a type parameter
  had no definition (`definition_refused`); `im_split` is the toolchain's one such.
  One committed sidecar is made again with it (`examples/tactic/use_split.auto.shard`): a
  sidecar's key takes in the descent obligations of every module in its file's closure,
  proven or not, and that file imports `kernel/sexpr.shard` (`OPEN.md` §1).
- The other integer measures the checker gets to were restated the same way, by three
  agents on disjoint files under one rule — no function's body changes, and the program
  text of the loader's closure stays byte for byte what it was, which it did —: 102
  functions more, 105 in all with 146 obligations, none pending. Assent to that plan: "Good.
  Proceed with the bulk when the pilot lands." Where the plain form did not fit, the measure
  was chosen so that the obligation is true of the function as written; not the function
  changed to fit a measure. A counter tested against 0 is measured one higher. A walk down
  lists is measured by Init's `List.length`, a `Nat`; not by the sources' own `len` — a fact
  about it cannot stand in `descent.shard`, which `util.shard` imports. Two functions that
  call each other, one passing the counter on, take measures weighted so that the pass is
  paid for (`level.shard` `is_geq_core`, `is_geq_offsets`). Seven were left for the entries
  below: `k/inductive.shard` `fresh_univ_name`, whose counter nothing in it bounded, so what
  it would have owed was false; `kw.shard` `w_norm_ty` and `w_norm_tys` and four of
  `k/axioms.shard`, whose descent is by a counter and then by a list's length. One proof of the
  146 as delivered cited `Classical.byContradiction` to turn a `Bool` test into its
  comparison, and two functions listed `Classical.choice` by it; it is a case split on the
  comparison's decision now, and no declaration of the closure lists more than `propext`.
  `init.facts` takes one line, `List.length`: a measure is a part of its `fn`, and the
  table holds what the sources' forms name.
- Two functions that call each other over a term are measured by the term's derived size:
  `(derive Expr size)` beside the type, `expr.shard` `expr_eq` at one above twice its term's
  size and `expr_eq_core` at twice, the 12 descents proven beside them; not the two written
  as one structural function — no body changes, and the program of the loader's closure is
  byte for byte what it was. Assent, of the two ways put to them (this group as a pilot, or
  the types that name each other first): "Good. Let's continue with the expr_eq group of
  functions."
- Chosen in building it, not put to the owner one by one. The Rust reader passes over a
  derivation of `size` written alone: its functions are definitions with no program, and the
  reader named the form unread, which failed its reading of every toolchain root once
  `expr.shard` held one — the first derivation in a toolchain file; one that writes
  functions is unread as before. The loader's reading of a build still makes the size's
  definitions; not passed over there as a `theorem` is — under a second, and no table takes
  a line for it. Four counters the checker came to behind `expr_eq` are restated by the
  forms of the integer measures (`k/tc.shard` `mk_binding_fold`, `k/add.shard`
  `ref_closure_ok`, `k/inductive.shard` `ivia_go`, `tactic.shard` `pat_fits`). Fourteen
  more it came to are left programs: eight in groups whose descent is by a counter and then
  by a list's length, as six were before (`inhabit.shard` `inhabit`, `matcher.shard`
  `mt_compile` and `mt_pat_term`), and the six of `spelling.shard` `sp_instance_go`, a
  group that hands its counter round, which ranks in the measures would pay for. The
  loader's search for a group is left as it is, and the checked load is slower by it
  (Measured; `OPEN.md` §1): ending a walk at the first function read that calls a program
  outside the waiting ones was tried on a scratch loader, gave the same records and took 4 s
  of 107 off, and is not in.
- The search for a group keeps how each reading of a waiting function ended — whole, stopped
  at a name the environment lacks, or not read — under the callee-variables it was read with,
  and takes it again while it holds. The search itself is as it was: its walks, its fuels,
  the waits it records; and what a callee is — waiting, a program, nothing yet — it asks anew
  each time, so a function that comes to wait ends nothing kept. A reading is forgotten when a
  function its form names is read and loaded, when a waiting function it names is, and every
  one at a form that is no function, a `use`, a `trusts`, an import, a fetch from Init and a
  file's end. Not the search made less often, or from the newest waiting function alone:
  which of its looks are idle hangs on its fuel and on the waits it records, and nothing
  showed the same groups would be found. It rests on one thing no load checks: a function's
  definition puts constants under its own name alone, so it changes the reading of a form
  only if the form names it (`OPEN.md` §4). A reading wrongly kept can leave a group without
  its definition; it cannot define one wrongly — a group found is read again in one state
  and K checks it. Assent, of the three ways put to them after `expr_eq`'s group doubled the
  toolchain's check (this, the remaining groups first, that landing taken back): "Let's go
  ahead with trying to fix the group search".
- A requirement key is made by the walk it always was, over names that carry their keys and
  declarations whose parts are kept. For each declaration a walk reaches, its term of the
  sum and the constants it cites, each with its key in the environment, are made once in a
  load (`reader.shard` `ReqNode`, `ReqMemo`, `req_node`); a step of the walk is then a
  lookup and hashes no name. The names, their order and the fuel spent are what they were,
  and whether a name is the load's own is asked of the scope at every step as before: the
  table that answers grows. The memo is one field with K's memo of axiom closures
  (`loader.shard` `Memos`), so it is kept, advanced and emptied where that one is, and on
  the same ground: an entry is a function of the declaration as the environment holds it.
  The reader's own key, at an `auto`, starts from no memo and keeps none — its environment
  holds matchers the load has not admitted. Not the keys of generated equations left out of
  the records: no stored proof is matched against one and it would take nearly all the
  cost, but a record would then say less than `LANGUAGE.md` gives it. Left as they are: the
  file's parts of the key, made again for each theorem, and the repeats among the constants
  a declaration cites. Assent, of the three ways put to them once the keys were measured
  (this, the equations' keys left out, nothing): "Let's proceed with 1".
- A `type` is declared with the types below it that it names. At a `type` whose field names a
  type K does not hold, the loader gathers the types below it in the file that it needs, as
  far as they are read in the scope it is read in — up to the next `use`, `trusts` or import
  — and admits them where it stands: of those not yet admitted, the one that needs the
  fewest with those it needs, as one declaration, again and again. Types that name each other
  are so a block of K's, and a type named above its form is admitted before the one naming
  it. Their records stand there; the program's declaration of each stands where its form
  does, so a program's text is what it was. Each type of a block has its acceptance, its
  `casesOn` — the other types' motives constant, as a nested type's auxiliary ones are — and
  its constructors' injectivity. Not a form written around the types of a group: functions
  that call each other are found without one, and the bootstrap's reader, the formatter and
  the classifier would each have to learn it. Not the sources rewritten to avoid the cycles,
  an arm as a pair inside its term: the language would still not take such types, and the
  loader's state would be a cycle still. Not admitted when the last type of a group is read,
  as functions that call each other are defined: a record's accessors stand between it and
  the next type, and would have no type to be read over. Types that name each other at
  different numbers of parameters are no block and stay programs with the reason
  (`block_parameters`), as does a type naming one below a `use` (`e_only_type`): neither
  loaded as a type of the logic before, and a refusal would take the program away. A
  derivation over a type of a block is refused (`derive_shape`), its size excepted (below).
  Assent, of the
  three ways put to them (this, a form around the group, the sources rewritten): "Agreed
  with updating the loader to handle this (option 1). Proceed."; and after it was measured,
  with the next entry's change in it: "Good, we can land this as built. Proceed with the
  remaining todo items."
- A function waiting for one declared below it is read again once that one has a
  declaration, and not before. After every function that loads the waiting ones are gone
  through as they were; one whose callee still has no declaration is left without a reading,
  on the ground that the reading would end at that callee as it did (`OPEN.md` §4). A
  refusal such a function would have met sooner is met when its callee comes, or at the
  file's end, where every waiting function is read. Not asked for, and put to them with its
  measurements: with the toolchain's types in the logic the checker reads a thousand
  functions more, in files that call forward throughout, and the loader's closure loaded in
  76 s for 36; with this, 49 s. Assent: "Good, we can land this as built. Proceed with the
  remaining todo items."
- Types that name each other have a size, and the functions that walk them are measured by
  it. `(derive T size)` on a type of a block gives each type of the block its size from its
  own recursor and each auxiliary type its from the recursors K names after the block's
  first type (`T.rec_N`), whichever type the form names; they are read off the recursor's
  telescope as a nested type's are, and every other capability stays refused on a block
  (`derive_shape`). One group is restated by them, as a pilot: `prog.shard` `calls_acc`,
  `calls_all` and `calls_arms`, each measured by the size of what it walks, with the 16
  proofs beside them in the fixed lines the toolchain's other descents use (`(rw h) (rw
  SIZE.eq_K) (arith)`). Not structural recursion for a group: of the 113 groups among the
  programs, 46 descend on one parameter per member. Assent, of the three ways put to them once the recursion the
  programs lack was read off the call graph (the groups whose descent is plain restated in
  bulk with the proofs left to the engine; the recursion no measure fits read first, with
  nothing built; this — a block's size and one group, then that reading): "Let's proceed
  with C".
- The toolchain's plainly recursive functions state a measure, and their descents are proven
  in the sources, beside the functions. Where every recursive call is on a part of a
  parameter or a counter less one, and the function was a program because it is a member of
  a group, walks a type that holds lists of itself, matches two constructors deep or counts
  on an integer, it states a `Nat` — a derived size, a list's length, `Int.natAbs` of a
  counter, a rank where a member hands its argument on whole — and each obligation has its
  `fulfills` in the fixed lines (`LANGUAGE.md`, the toolchain's own sources). No body
  changes: the loader's program text is byte for byte the one before, which each batch of
  the work was held to. Not the proofs left to the engine in sidecars beside toolchain
  sources: a toolchain module that gains an obligation stales the stored proofs of every
  file above it (`OPEN.md` §1), and the engine does not find a share of these (`OPEN.md`
  §3). A function behind a numeral under a constructor pattern takes a change of its body
  first (below). A fact about a helper that returns a part of its argument is
  proven beside the `fulfills` that cites it, by induction where the helper recurses
  (`classify.shard` `live_arm_lt`). Two stated measures did not descend at their guard and
  are corrected: `template.shard` `inst_equations` and `define.shard` `ctor_locals`. Assent,
  of the three ways put to them (this; the kinds not yet tried piloted first; K's checker
  discussed first) and to the question where the proofs live: "The bulk's proofs can live
  in the sources, and the budget parameter in the eraser's environment walks sound
  reasonable. For K's checker, option 1 sounds reasonable for now. Let's proceed."
- A measure may have several parts, compared part by part. `(measure A B …)` is the pair
  `(Prod.mk A (Prod.mk B …))`, each part a `Nat`; the definer reads the measure's type and
  builds the order from it — Init's `Prod.Lex` over `Nat.lt`, well-founded by `Prod.lex`
  (`define.shard` `measure_order`) —, an obligation is a `Prod.Lex` statement, and a measure
  of one part is built and stated as before. Not one number per function with ranks: a
  descent by a fuel at one call and by a list at another, at the same fuel, has no single
  number — what is met at a lower fuel is computed and can be any size (K's checker's nine
  walks at one depth; `kw.shard` `w_norm_ty`, the first group written so). Not every walk
  spending the fuel: K's checker would give up on a long telescope or argument list that
  the C++ kernel it follows accepts. Not the checker's entry points handed to the walks as
  function values, the form of the Lean kernel written in Lean (`Lean4Lean`, where each
  entry is a field of a record indexed by the fuel): these sources pass no functions. Not
  an order of the author's choosing: `Nat` parts in sequence are what the sources' descents
  are. The cost: K does not evaluate through a recursive call of such a function, and a
  proof cites its equations (`OPEN.md` §3). Assent, of the three ways put to them for K's
  checker once its 60 functions were read call by call (this, with the checker restated in
  five places; the checker restated alone, its walks spending depth; the checker left a
  program): "For K's checker, option 1 sounds reasonable for now."
- K's checker is a definition of the logic. Its 60 functions that call each other
  (`k/tc.shard`, `infer` to `defeq_step7`) are one group, each measured in three parts: the
  depth, `(Int.natAbs d)`; a rank, the longest chain of calls below the function that stay
  at its depth (0 to 12); and what a walk or a loop of its own descends by — a term's size,
  a list's length, the fields left up to an index, the heartbeats left. The 189 descents
  stand at the end of the file, 169 of them by the rank alone. The code is restated in four
  places for it. The step of the depth stands where the depth is tested (`infer`,
  `whnf_core_c`), so that the guard's result is the step's ground (`rec_guard_ok`); every
  call sees the depth it saw. `reduce_nat` normalizes an argument one level deeper, past a
  test of the depth: not the argument at the depth it has with a fourth part for the
  heartbeat — a nested accelerator is a nesting like any other, the kernel written in Lean
  counts it so, and the term's own nesting was what bounded it. A loop goes round only when
  the heartbeat count advanced, under the limit, since it was entered (`loop_advanced`), and
  is out of heartbeats otherwise: not a count of rounds handed down, that kernel's form — a
  round that spent no heartbeat can only repeat itself, so the test refuses nothing that
  ended, and no signature changes. And `proj_walk_fields` answers `invalid_projection` at an
  index it has passed: it tested the index by equality alone, a negative one walked the
  constructor's type to its end for the same answer, and its obligation was false as it
  stood — the reading of the checker had taken the counter for a descent. Under the checker,
  `level.shard`'s `normalize` and the order it sorts by are definitions with no body
  changed: two facts about the helpers they recurse through and a weight written as a `def`.
  Assent as for the measure above; then, asked what stood in the way next:
  "Let's continue with the next chunk. What do you see as the next blocker?"
- No toolchain function matches a numeral under a constructor. The 21 that did — 29 rows: a
  byte at the head of a list, a constructor's field count, a de Bruijn index, a matcher's
  count of scrutinees — bind the field and compare it with `if`, or ask `starts_with_lit`
  where the row was the whole of a test off the hot path. The logic has no form for such a
  row, each of the 21 was a program nothing read (`no_l_identity`), and 425 programs stood
  above them. Not the definer taught to read the row: a change of the language for every
  file, made in the elaborator, whose own group already elaborates forms it built and is the
  furthest from a definition; and the rewrite is the one the reference gives for a `def` or
  a statement. Two helpers came of it, each for a branch the rewrite would have written
  twice (`elab.shard` `el_pat_var`, `list_mismatch`). The owner, of the two ways: "We can
  restate the 21 patterns. Proceed with that when practical".
- K's walks down a telescope and its three searches are definitions, and with them all of
  K's directory but the reader of an export's lines, a definition by a fact about the JSON
  parser (below). Each
  of the nine walks of the inductive-type checker (`k/inductive.shard`) spends a heartbeat
  at every binder it takes and calls itself only under a test that the count advanced, below
  the limit, since the walk was entered; the heartbeats left are the measure, and each of
  the 16 obligations is the test's own hypothesis (`a_advanced_lt`). The term gives none: a
  walk goes on with a body it substituted into, or with what `whnf` made of that, and seven
  of the nine had stated a structural descent on it. The heartbeat is the guard of the
  checker's three sites taken at a depth that passes (`a_step` over `rec_guard`): not a
  count of the walks' own, so the state's check has no new primitive and a walk draws on
  the budget the block's other checks draw on. The test refuses nothing that ended — every pass adds a heartbeat and
  no step takes one back —, and a block spends one more heartbeat for each binder walked.
  Not a count of binders handed down: `whnf` can unfold a result into more of them, and
  nothing in the declaration states how many. The searches got bounds their code tests. The
  fresh universe name is looked for among one more candidate than there are names taken,
  which always holds a free one; past them it answers nothing and the block is out of
  heartbeats, not a taken name. The auxiliary name of a nested occurrence and the worklist
  of auxiliary types stop at 100,000 (`nested_max`), out of heartbeats: the pin's two loops
  have no bound, and the worklist's measure had named that number without the code testing
  it. `k/refgen.shard`'s table of names recurses on the prefix a row binds, not on a
  helper's answer. One branch of `cit_binders` that tested the parameter count twice for one
  answer tests it once. The design was put with the checker's; then: "Good work. Let's
  continue with K's other walks while CI runs."
- A descent through a helper is a fact about the helper, proven once beside it. A walk of a
  term by its spine goes on with the head `get_app_fn` returned and the arguments
  `get_app_args` returned; `expr.shard` weighs a list of terms at each term's size and one
  (`exprs_weight`, a `def`) and proves that the head and the arguments together weigh what
  the term does, and ten units descend by what follows from it — the head smaller under an
  application, the arguments lighter than the term, an argument smaller than it. Not an
  induction in each of the ten, and not a fuel handed to each walk: the code was right as
  it stood, and what was missing was a statement about two functions. Two shapes had no
  measure and were restated. A function handed a head and its arguments apart
  (`tactic.shard` `tc_show_app`) showed the head by a call back to the printer, which no
  measure pays for when the list may be empty: it takes the constant's name now, and the
  head that is no constant is shown where the term is known to be an application. And a
  function that asked whether the argument list was empty matches on the term
  (`derive.shard` `dv_show_ty`, `dv_ty_s`): a row `(App _ _ _)` says what a variable row
  after `nil` does not. A merge sort's two piles are dealt by one function (`util.shard`
  `halves`), with the fact that each pile of a list of two or more is shorter, and the four
  copies of it are gone; a sort's last row is written out, `(cons _ (cons _ _))`. Not
  another sort — bottom-up, or by a count of elements, whose descents are plainer —: either
  orders equal keys otherwise, and the Init index and a dump are compared byte for byte.
  The unifier's twelve functions, which waited on the walks alone, are measured as K's
  checker is — the depth, a rank, a list's length — with no code changed: the two that test
  the depth and the walk of an argument list state it, and the nine that pass it on to a
  function that steps it state the stepped depth. A function stands below the type its
  signature names; four that stood above it were programs nothing read. The readers that go
  on from what a helper left of their input are measured by the length of the input, on the
  fact that the helper leaves no more than it was given (`initload.shard` `od_block_lines`
  over `take_line`, `sexpr.shard` `read_string` over `take_hex`, `json.shard`
  `parse_string` over `parse_hex4`): with them the whole s-expression reader is a
  definition. The way was the one assented for the bulk (above:
  "The bulk's proofs can live in the sources"); then, with the walks landed: "Good. Let's
  push on to the next chunk while CI runs."
- The JSON parser's three functions take a fuel beside the input, and the fuel is the input.
  A value, an array and an object go on from the rest a call of their own returned, and
  nothing they are given grows smaller at such a call. `json_parse` hands its input down a
  second time; each of the three answers `JErr` where the fuel is empty, a value's call of
  its array or object and an array's or an object's call of itself for the next member pass
  the fuel less a byte, and the measure is its length (twice it, and one more on an array
  and an object, which call a value at the fuel they have). The input left is never longer
  than the fuel, so the fuel is empty only where the input is, which is no value, and no
  answer changes. That was argued and tested when the fuel was built, and is proven since
  (the next entry): the parser before and after agree
  on every sequence of up to six of eleven tokens — 1,948,717 inputs, 1,509,842 of them
  refused —, and the Init replay and the Init index are what they were. Not a count —
  twice the input's length, one less at each call, as it was proposed and first built —:
  the entry would measure its input at every call, and K's reader of an export's lines
  parses each record off the whole rest of a chunk of 20,000 lines. Not the parser left a
  program: the index's builder and verifier and the load on demand waited on it, 21
  functions with its own three. K's reader of an export's lines did not come with them, as the
  proposal had said it would: it goes on from the rest the parser returned, and descends by
  a fact about the parser (the next entry). The owner, of a fuel or the
  parser left a program: "Agreed with giving it fuel -- proceed with A."
- K's reader of an export's lines is a definition by a fact proven of the JSON parser; not by
  a fuel of its own. `process_lines` states the length of its input as its measure, and its
  one obligation is `json_parse_lt`: a parsed value leaves less input than it was given. No
  body changed. The fact is proven in `json.shard` function by function — the blanks, a
  number, a string, a literal (`util.shard`), then a value, an array and an object together
  by induction on the fuel — with each call standing in the target: a result is measured by
  `jres_room`, the length of what it left and one more, 0 for an error, and a block takes
  its function's branches by `cases` on each scrutinee and `reduce`. By the same means the
  fuel's own claim is proven, `json_parse_fuel`: the parse is the same at every fuel as long
  as the input or longer. Not a fuel in `process_lines`: a body of K's directory changed,
  with a row never taken, on a fact not proven. The cost chosen: some 700 lines of proof
  that follow the parser's bodies and are typed again where a body changes. The owner, of
  the fuel, the proof or the reader left a program: "Let's proceed with proving the fact
  that the parsed value leaves less input. This seems like a reasonable point to stress
  test the proof system itself, unless you see a good reason not to."
- The eraser's readings of a type, and the evaluator's of the types an entry states, take a
  depth from their caller and are definitions by it. A type is read into the types it names
  — a type's arguments, an inductive's fields while its eligibility is examined
  (`erase.shard` `l_to_etype`, `ind_eligible` and their three walks), a package's fields
  (`binder_role`, `package_type`, `package_of`, `package_fields`) —, and the next term is
  one K computed or one the environment holds: nothing the function is given grows smaller.
  A caller hands down `(budget_depth)`, 8,192, as callers hand a telescope's walk its fuel;
  `l_to_etype` and `package_type` answer none where it is spent and read on at one less;
  the measure is the depth, then a rank, then a walk's own list or fuel. Not a measure over
  the names not yet seen, which the guard of the first reading suggests: it takes facts
  about the environment's table that are nowhere stated, and the second reading had no such
  guard. Not the two left programs: 316 programs stood above them. The cost chosen: a type
  that names types deeper than the depth is answered as no program type, or no package,
  with no record that the depth ran out — as a constructor of more than 8,192 fields
  already was. In `ev.shard` the depth of the door's reading (`ty_checked`, four functions)
  is the count of the program's declarations and is never spent — an inductive is opened
  while it is not among the seen, and those open are among them —, so no answer changes;
  that is argued, not proven (`OPEN.md` §4). Found on the way: the package reading had no
  guard, and a structure with a function-typed field and a field of its own type, as a
  function's parameter, sent the loader round without end; with the depth the reading ends
  in the time of any load and the function is a program, as over every type that is neither
  a package nor a program type (pin `package_recursive`). Not also a test of the
  inductive's own flag for recursion: the depth gives the same answer. The owner, when the
  walks were put to them with K's checker (above): "the budget parameter in the eraser's
  environment walks sound reasonable", and to go on: "let's continue with the eraser's
  budget". The value handed down, the answer where it is spent and the evaluator's count
  are the builder's, not asked apart.
- The classifier's reader of a written type and of a body is a definition by a fact proven
  of the helper it goes on from; no body changed. A type's reader and a call's hand the
  arguments written to `written_data`, which keeps those that run, and read what it kept:
  a list that is no part of what they were given. The fact is that it is no larger —
  `written_data_room`, by induction on the binders' flags with the call in the target, a
  result measured by `wr_room`, its list's size and 0 for a refusal; `written_data_le` and
  `type_args_le` state it of a result known by its equation —, and with `last_of_lt` it is
  all the reader's 39 functions cite: the two of a type are measured by the size of what
  they hold, the 37 of a body by eight times that size and a rank. Not a fuel handed down
  the reader: a body of 39 functions changed, with a row never taken, on a claim not proven.
  Not the reader left a program: 127 programs stood above it, 65 on it alone. The owner, of
  the six units left and my lean to this one: "Let's continue with the classifier's reader".
- The erasure of a term is a definition by its fuel, and the function that tests a fuel is
  the one that steps it. `erasure.shard`'s 39 functions hand a fuel round. `erase` tested
  it, and 19 other functions handed on one less, at 31 calls, without a test: nothing such a
  function knows of its fuel says that one less is nearer a bound, so no measure stated
  the step. `static_shape` went round on a lambda applied, at one less, with no test
  anywhere on the loop, and through `package_field_shape` on a term it built, at the fuel
  it had. Now `erase` and `static_shape`, the two functions the walk comes back to, test the
  fuel and hand on one less where they go round — `erase` at its three dispatches,
  `static_shape` at its own call and at `package_field_shape` —, and every other function
  hands on the fuel it was given. A function's measure is the fuel, a rank above every
  function it hands the fuel to, and the list it walks. Not the bodies kept under a measure
  staggered between the tester and the rest, as the definer's projection has it:
  `erase_cond` enters `erase` at its own fuel and calls members at it, and `static_shape`
  tests under `erase` at one fuel, which leaves no room for the half steps. Not a size of
  the term for the arguments erased at the caller's fuel: it takes facts about
  `get_app_args`, `drop` and `append`, and the terms the erasure builds must be stepped all
  the same. The cost chosen: what the fuel counts changed — every entry of `erase` costs
  one, where an argument erased through `erase_all` cost none and a lambda's body under a
  static argument two —, and `static_shape` refuses `erased_in_runtime` where the fuel is
  spent, where it read on. The budget is the 8,192 it was, and the deepest body of the
  loader's closure needs the same before and after (Measured). Found on the way: the
  loader's search for a group read a waiting function again for 16 callees at most, and
  `erase_const_at` names 17 members, so the group was never found and each member was
  recorded as waiting on another; the search has the bound of the load's own loop, 1,024
  (pin `mutual_wide`). The owner, of the five units left and my lean to this one, which I
  had described as "three one-token body changes, then the recipe": "Good. Let's proceed
  with the erasure unit". That description was wrong — it counted the three terms built at
  the caller's fuel and not the 31 steps without a test. Which function steps the fuel, the
  refusal in `static_shape` and the search's bound are the builder's, built and then put
  to the owner, who took them: "This works -- we can ratify the fuel discipline. Let's
  proceed with the next units".
- The evaluator's machine is a definition with no body changed, and the join-point pass
  with one row made a function. `ev.shard`'s `ev_call` already tested the fuel and stepped
  it; at one fuel the machine either opens a term into a frame and its first part or
  closes a frame, so its second part is twice the term's size and a weight of the
  continuation's frames (`kont_weight`), and its step into the arm a match picked descends
  by a fact proven of `pick_arm`. Not a fuel spent at every step: the machine's answers
  would change with it, and `ev.shard` is what "run" means (rule 21). `lower.shard`'s
  `lw_jp_named`, `lw_jp_lifted` and `lw_jp_tail` handed one term round, the last on a row
  that matches no shape; the row the first and the last ended on is `lw_jp_app`, which both
  call. Not the rows of `lw_jp_tail` written out one a constructor with a rank that reads
  the term's shape: seven rows and a measure with a case in it, for what one function
  says. The pass before and the pass after compile the loader to one C text. The owner's
  word for both is the one above, "the next units", of a list that named them with the
  loader's re-entry and the elaborator; the weight and the function are the builder's.
  Left where they were, each with a choice that is the owner's and was put to them: the
  elaborator (a depth its callers hand down, as the eraser's, or the size of its source
  with a weight that pays for the three forms it builds), the loader's re-entry (a bound
  on a derivation generating a derivation) and `run_loop` (a bound on a run's externs, a
  fact about the machine's halt, or a program). The owner, of the three and of the
  definer's request below: "On your decisions: 1: depth handed down, 2: do it with the
  elaborator, 3: leave it as a program, 4: do it before the elaborator, and do your
  research first, but mirroring lean's approach is a decent prior." So the elaborator
  takes a depth its callers hand down, the loader's re-entry is bounded with it, and
  `run_loop` stays a program.
- A function defined by a measure takes evidence about its parameters, and the recursion's
  parameters are packed as Lean packs them where that is needed and as they were where it
  is not. The definer nested the data parameters' types in `PProd` as written, so a type
  that names an earlier parameter (`(hk (<= k n))`) left the tuple's type open — the
  obligation over it was no closed statement (`fvar_in_type`) —, and a proposition among
  the parameters did not fit a pair built at levels 1, 1 (`app_type_mismatch`). Now a pair
  is `PSigma` over its first component where a later type names that component, `PProd`
  where none does, and a proposition stands at level 0. Not `PSigma` at every pair, which
  is Lean's whole packing: the term of every function defined by a measure would change,
  and with it the obligations of 47 proofs in 11 files that spell `(proj PProd …)`, for
  no function that takes it today; where no type names a parameter the tuple is the term
  it was, held on the loader's closure (Measured). Not the evidence moved out of the tuple
  into the result's type (`∀ evidence, RET`): it is not how Lean packs, the prior the
  owner named, and it was not built to compare. Left as it was and written in `OPEN.md`
  §1: a group of functions that call each other takes no evidence. Asked for by the
  `compiler` branch for its buffer loops; the owner's word is the fourth above. Which
  pairs are dependent and the levels are the builder's, on that prior. The owner, asked
  whether the group's gap is to be queued: "We can wait on the group-with-evidence point
  until it's actually needed."
- The elaborator is a definition by a depth its callers hand down, and `elab` alone tests
  it. Its 34 functions took no measure on the source: `elab_list` elaborates the
  constructor chain it built from a list literal, `elab_match_lit` the `if`s it built from
  numeral rows, `elab_type` a head it read as a type. Every member takes the depth last;
  `elab`, which every cycle of the group passes through but a walk of a list at one depth,
  tests it and hands on one less at its 14 dispatches, and the other members hand on what
  they were given — the discipline ratified with the erasure —, so a member is measured by
  the depth, a rank above every member it calls at that depth, and the list it walks.
  `elab_op_at`'s call of itself for `>` and `>=` was the one cycle outside `elab`: the part
  that call reaches is `elab_op_go`, which both call. Callers pass `(budget_depth)`, K's
  8,192, and not a budget of the elaborator's own: K already refused a term nested deeper,
  so one number bounds a term, and the elaborator's refusal (`term_depth`) comes first and
  says what to do. Not a wrapper that keeps `elab`'s three parameters for the 36 callers
  outside the group: the eraser's callers pass the depth where they call, and a caller
  that holds one can hand on its own. The cost chosen: every caller states the depth, and
  a list literal costs a level for each element — 8,190 at most, as K held it (Measured).
  The loader's re-entry is bounded the same way: `load_derive` tests a depth and hands on
  one less, a file's forms are loaded at `(derive_depth)`, 1, and a `derive` among the
  forms a derivation generated is refused `derive_generated`. No capability generates one,
  so the refusal has no pin, and the bound states what the code did; its six functions
  state their measures and wait on the loader's units below them. The owner's rulings are
  the first and the second above, and then, of the plan: "Proceed with the elaborator".
  Which function tests, the budget's being K's, the refusal's name and text, `elab_op_go`
  and the re-entry's depth of 1 are the builder's.
- The proof steps' loops, the producer's ladder, the loader's reading of a `fn` and the
  loader's driver are definitions. Steps moved to the function that tests, by the
  discipline: `reduce_loop` tests the budget and hands `reduce_step` one less, which handed
  it back less one untested; `eng_solve` tests the depth and hands `eng_induct` one less,
  where `eng_induct_on` stepped it three calls on; `settle_missed` tests its fuel before
  anything else — every row of it answered the same at a spent fuel — and a fact says that
  a reading to be made again was given a fuel of at least 1 (`settle_missed_ok`), which
  `flush_fetching`, `load_decl_fetching` and the implementation's two loops descend by as
  the checker's loops do by `rec_guard_ok`. `load_fn_with` and `load_template` each test
  the fuel they step, and state both fuels and a rank. In the reading of a module with its
  imports the depth is stepped by `load_file_module`, which tests it, where `load_forms`
  stepped it untested: a file's forms are read one below the depth its import passed, the
  two callers of `load_forms` outside the group hand it one less, and the depth at which
  `load_file_module` refuses is what it was. `base_loaded`, which steps into the base
  module's source, tests before it (`(< fuel -1)`): the last file asks for the base at -1,
  so the test turns away only a base that asks for itself, which went round without end.
  The group's eleven are measured by the depth plus 2, a rank and the list walked; the
  re-entry's six by their depth, a rank and a list; the eight loops by their counter.
  `probe_reach` matches on `probe_fn` called at a fuel of 1,024, and its `Nat` measure was
  held back while K's check of its equation did not end; it stands since K takes a stuck
  major apart as written (below, "The checker K and the Init import"). Not the budget
  handed to `probe_reach` as a parameter, so that it is a variable there: that hid the cost
  at one site and left it for the next function that writes a budget as a numeral. The
  owner's words are the ones for the units, and of the cost when it was put to them:
  "Find and fix the blowup problem."; what was built is the builder's.

## Programs and the back end

- A program is read from the source and the types its binders state (`LAW.md` rule 24), so the
  Rust evaluator can come to read every program and the toolchain's own files can become ordinary
  ones; not type inference in the Rust evaluator — the elaborator, the erasure and K's type
  checker, about 19,400 of the toolchain's lines, inside the bootstrap; not
  a seed — the Rust evaluator running the erased form a previous toolchain wrote, so that the
  bootstrap no longer runs the sources. Assent: "Let's adopt it and begin."; and once every
  program the tree writes read the same both ways, "ratify it".
- The classifier writes the entries the erasure writes by sharing its tables (`prog.shard`:
  `realization_of`, `realization_expr`), and types a body before it writes a row's expression;
  not a second copy of the erasure's cases.
- Which written arguments run is read off the head's declared signature — K's constant by the
  roles the erasure reads (`erase.shard` `tele_of`), or the binders of the `fn` being read — with
  `@` naming every binder; not a table of Init's names kept beside the registry.
- A call that leaves a runtime argument implicit is refused by the classifier
  (`unwritten_argument`) and the source writes it: `Byte.decEq` in `std/derive.shard` writes
  `(@BitVec.toNat 8 …)`; not the classifier inferring it, which is unification. Follows rule 24.
- The classifier passes a written proof over only in a function the typed route has read; where
  its reading is the only one a written proof is `unchecked_proof`; not passing proofs over
  everywhere — a body the logic never read would then take a `Fin` on a proof nothing checked
  (pin `dif_scope`).
- A build's load (`--unchecked`) passes a `theorem` and a `fulfills` over; not checking them
  in every build — each of the 29 roots would prove again what its closure states, the
  loader's own on the bootstrap —; not the load error a `fulfills` was there, the function
  it names being a program that owes nothing; not refused as a proof written inside a `fn`
  is — no value of the program rests on one, and the check of the same file is the load
  without the flag (`toolchain_check.sh`); not the proofs kept in files outside every
  build's closure — a function and the proof that it ends would be edited apart. Nor does
  such a load ask the export for a constant that only a passed-over form cites: the table
  the bootstrap reads Init by (`kernel/init.facts`, made under the flag) would else take a
  line for each lemma a proof names, and every new proof would send it to be made again.
  Chosen in building the pilot of the integer measures (Definitions and recursion).
- A function that is only a program builds and reads the registry's rows by the table the
  erasure reads; not the classifier refusing every row's constructor so that the two routes
  could not disagree on a representation (pin `row_e_first` stated that).
- A lambda is read where it is written and lifted where its instance is taken: between the two
  the body as read holds it as a node of the term type (`ELam` in `prog.shard`), which no
  program holds; not a second term type for the classifier's passes, and not a result
  channel through every reading function.
- A template's call is typed in the elaborator's order (`elab.shard` `elab_args`): its bare
  variables, the type expected of the call, its other arguments left to right, its numerals.
  A lambda's result type is what that order fixes for its parameter's result, else its body's
  own static type; not a result written in the source, which the typed route does not ask
  for. Left open by the owner: looked at again when the Rust reader reads templates.
- Numeral rows and a `let` of several bindings are written by the classifier as the
  elaborator reads them — tests in row order, a let in a let; not the erasure folding its
  tests back into a match.
- A function that is only a program passes a lambda, a constant of the registry or a package:
  the classifier lifts and names as the typed route does, and the instance's program stays
  the erasure of the template's elaborated body; not such a function refused for want of a
  reading in the logic, as it was.
- An instance is named by the classifier and read by the erasure alone; not the classifier
  reading a template's source at an instance's shapes, which nothing needs until something
  must run a template's instance without the elaborator (`OPEN.md` §4).
- The Rust evaluator reads a closure by the classifier's reading written again in Rust,
  function by function, and takes what that reading asks K about a constant of Init from a
  table the loader prints from K (`facts.shard`, `load --facts`); not K or the export read in
  Rust, and not the roles written by an author in the source. For an ordinary closure the
  table is made when a test runs.
- For the toolchain's own closure the table is a file of the tree (`v3/kernel/init.facts`),
  printed by the loader from K, which the bootstrap believes (`LAW.md` §4) and
  `parity_test.sh` has K print again and compares; not the same facts as constants in the
  Rust reader, written by hand where the file is printed. The owner chose the file.
- The Rust evaluator has that one reader, for the toolchain's sources and for any program; not
  a second, flat one for the toolchain (one name per declaration, a citation resolved by its
  suffix), which let a toolchain file be something no ordinary file is. The reader that reads K
  is 3,600 lines where the flat one was 900 — the trust base's engine grew by what reads names,
  modules and views — and `parity_test.sh` holds it to the loader's program on every toolchain
  root. The direction is `LAW.md` §6, 10.
- The Rust reader links every view it loads to its implementation; not only a directory named
  as a root, the loader's rule, which is where the implementation is checked: the Rust side
  checks nothing and a run needs the link. A comparison with the loader names the directories
  to it.
- The runtime's seven cells — a comparison's `True` and `False`, the `Cons` and `Nil` of a
  symbol's bytes, the host's `Some`, `None` and `Pair` — are the constructors of Init's
  `Bool`, `List`, `Option` and `Prod`, by identity, on all three engines; not a prelude of
  the toolchain's own beside Init's types, and not a constructor so named in whatever type
  declares it — a program's own `(Pair …)` is its own, and the back end refused such a
  constructor at another place in its type. A native binary still finds the World's cell by
  the name `World` (`OPEN.md` §4).
- The toolchain writes Init's constructors under Lean's names, opened by name in each file
  (`(use Init.List cons nil)`); not the namespaces opened whole — its own `append`, `take`
  and `drop` would then meet `List`'s and be refused — and not its old capitalised names
  kept (`LAW.md` rule 17).
- A string literal is a `String` in the toolchain's sources as in every file, and no rule
  reads a file by its module path; not a byte list under `kernel…`. The owner: "We need a
  path to dropping this, but we can defer the cutover if that makes more sense for
  ordering.", and then "Let's continue with the string-literal cutover."
- The toolchain's text stays a list of bytes, `(List Int)`, and a literal used as bytes is
  converted — `(utf8 "…")`, a definition in `kernel/util.shard` that copies the list a
  `String` is represented by — at every such literal, 3,600 of them; not only in a function
  the checker reads, as it was while the checker read 131: it now reads the types of five
  functions in six, and "convert where checked" would follow each function's status as it
  changes. The copies cost a fifth of the loader's emit on the bootstrap (Measured). Not the
  toolchain's text made `String`: every function over text would change.
- The table of Init's facts names the structure of every projection function it lists
  (`facts.shard`); not the projection's line alone — the Rust reader could not read
  `Array.toList` or `ByteArray.data` without the structure's line, the conversion's own body
  among them.
- A constructor prints under its type's name in the text the two readers are compared by
  (`Prod.mk`, `Tok.TPlus`); not by its last component — Init has several named `mk`, and
  the comparison is evidence only where the text tells declarations apart.
- Two toolchain functions whose `(measure (struct x))` passed no subterm state `(measure (len
  x))`, an `Int`, and stay programs; not a `Nat` measure with its proofs, which nothing in
  the tree needs of them.
- `eval` takes its package root as `--root DIR` and reads `v3` where none is named, as
  `v3/build.sh` does; not the root inferred from the file's path, and not a root required on
  every one of the suite's hundred invocations.
- `load --facts` prints, with the load's own table, that of each implementation it linked; not
  the main line's alone: the fork that checks an implementation is dropped with what it fetched
  of Init, and a closure with a directory module would be read by half a table.
- The Rust reader refuses nothing: exhaustiveness, a view's privacy, the match of an
  implementation against its view, a proof's reader stay the loader's, where the same files
  are loaded; not the language's refusals stated twice.
- Three functions of Init the library ran by their derived view have their bodies written out
  — `BitVec.toNat`, `UInt8.toNat`, `Option.getD` (`std/bytes.shard`, `std/list.shard`): K
  proves each and the programs are the same text; not the library put back on the view with
  the view's program handed to the Rust reader in the table, a program no source writes, and
  not the form retired. `(realize NAME (view))` stays in the language, and the Rust reader does
  not read it.
- `ev` is pure: an extern is `EvStuck extern` and only the driver's run performs it; fuel is spent
  at function entry; not effects inside the evaluator.
- A primitive answers a value, a failed guard, or a resource exhausted, decided before the work;
  not after building the result — a shift of zero by 10^12 is otherwise 16 billion steps.
- ★ Bytes are lists in a program — `UInt8` an integer, `ByteArray` its list, `String` its bytes,
  `Fin`/`BitVec`/`Array` their one runtime field; not packed buffers: no new kind of runtime value.
- ★ `ev` is a machine with an explicit continuation, at most six parameters per function; not host
  recursion — compiled, a wider function cannot tail-jump: 2.5 GB of stack against 12 MB.
- ★ A binary is the loader's own program lowered (`lower.shard`) and written as C (`cgen.shard`)
  in the loader's process; not source printed for a second front end — 222 s to build, was 2,826.
- The lowered form is still a `Prog`, so `ev` runs it and the lowering is tested without the C
  generator (`load.shard --lowered`).
- C names are the identity mangled injectively (`_` as `__`, other bytes `_xHH`); not short
  names — two identities never collide and no word is reserved.
- The lowering keeps `Nat.beq`, `Nat.ble` and the bitwise five under their own names; not the
  kernel's primitives — they answer Init's `Bool`, and `ev` sticks (`no_arm`) where C does not.
- The lowering reads no measure; the binaries are engines, not proven artifacts — each is
  compared with the Rust evaluator on every test, and the evaluator written in shard is the
  canonical one (`LAW.md` rule 21).
- ★ World threading is checked on the erased program (`thread.shard`), through boxes and helper
  summaries; not read off variable names — a World boxed once and unwrapped twice is used twice.
- An unmet requirement, an unthreaded function, an extern off the roster or an untrusted axiom
  fails the load of a `bin`; not a warning. Requirements speak of the entry's observables only.
- `invoke` validates arguments against the entry's types (the checked door) and `ev` stays the
  assumed door; not one validated path — `ev`'s contract is over values, out-of-type ones included.
- The reader keeps no handle across a file's tactic steps: five links saved are ~6.5 ms of a
  1.7 s run, against a handle stale after every definition.

- ★ `Symbol` is an ordinary type, `(type Symbol (MkSym (List Int)))` in `kernel/sym.shard` —
  the bytes of a text —, with ordinary operations (`sym`, `sym_eq`, `chars_of_sym`,
  `sym_of_chars`), and no reader or engine knows it (`LAW.md` rule 26); not a built-in stored
  as an interned id, which it was: it had no type in the logic, and nine tenths of the
  toolchain's functions were out of the checker's reach for it. Not the interned id kept
  beside the declaration as an engine's shortcut — rule 26 allows one, but it saved nothing on
  a native load, 5% of the Init replay and 1 to 10% of a load on the bootstrap (Measured), and
  it would need a test that the two forms agree. Assent: "Agreed with removing the engine's
  form of a symbol." Not a `String` for its field — a `String` made from bytes needs a proof
  that they are UTF-8; not a `Name` literal in place of symbols — K's `Name` is built of them.
  A symbol is written as an ordinary call, `(sym "x")`; not `(quote x)` kept as syntax — a
  form of the readers for one type is what rule 26 removes, and a call needs nothing of them.
  Assent: "(sym "x") sounds reasonable". `(quote x)` and the mark `'x` are refused with the
  spelling to write. The whole change, its four steps put together: "Good. Proceed with the
  symbol change (full 1-4 scope)".
- A function the checker cannot define is a program with its reason, and one whose types do
  not check is refused (`LAW.md` rule 25, ruling 13); not a refusal of every function the
  checker cannot accept — a sketch would need its proofs before it ran; not a program for a
  function with a type error — nothing would then check a program's types.
- The bit-level five (`band bor bxor bshl bshr`) have a meaning in the logic (`LAW.md`
  ruling 16) as operators that take it from an operand's type, as `+` does — Init's
  `Nat.land` and its kin at `Nat`, `Int.shiftLeft` and `Int.shiftRight` at `Int`, and for
  `and`, `or`, `xor` at `Int`, which Init lacks, three definitions of the system's own that
  every file sees — and run as the engine operations they were, by rows of the engines'
  fixed table; not a rewrite of the toolchain's functions over division or over Init's
  `Nat` operations — the operations are wanted in ordinary code; not functions a file
  imports — two spellings; not a form in source that declares a definition's engine
  operation — no second consumer yet. Assent: "Yes, operators by type with the fixed
  table, measure first"; and after the measurement, of the two points put to them — a row
  of the registry speaking for a declaration that is not Init's, and a base of the
  system's own that every file sees — with the smaller consequences: "The two points are
  approved, as well as the smaller consequences -- proceed."
- Chosen in building it, not put to the owner one by one. The base is TEXT inside the
  loader (`kernel/base.shard`), read as the module `shard.int` when a file's forms first
  use one of its operators; not a file the loader finds — a load's root is the package it
  loads (each pin is its own), the loader reads no file outside it, and a text of the
  loader's is in every binary's stamp already; not read at every load — on the bootstrap
  each load would fetch and check what the three definitions cite of Init. No file's module
  may be `shard` (`reserved_module`). The three definitions are by cases on an integer's
  two constructors over Init's `Nat.land`, `Nat.lor`, `Nat.xor`, and rest on `propext`
  because Init's own three do: what the owner was told would rest on no axiom does not. An
  expected type of `Nat` or `Int` decides a bit-level operator as it decides an arithmetic
  one, so `(bshl 1 k)` where an `Int` is expected is the integer's; a shift's count is a
  `Nat` at either type, and an `Int` there is a type error (three functions of the
  toolchain convert theirs with `Int.natAbs`). The engines' `bshl` and `bshr` shift by any
  count that is a natural number, as Init's do, where they were stuck past 63; a negative
  count is still stuck. The bare `ediv` and `tmod` stay in the engines' table for the back
  end's own use and are refused in a source with the spelling to write — `/`, which is
  `Int.ediv` at `Int`, and `Int.tmod` (21 and 2 uses rewritten): at a zero divisor those
  are total where the bare ones were stuck. Init's two shifts at `Int` written by name are
  read as the bare operators, by all three readers.

## Proofs and automation

- ★ A proof block is a certificate: a step is a function of its node and the goal, replay searches
  nothing; not tactics that read an ambient lemma set — an entry carries its rules.
- ★ `auto` delegates: the build replays the block in the machine-owned sidecar `FILE.auto.shard`
  and only `prove` searches; not search at build — a missing or stale entry is `PENDING`.
- A sidecar entry is keyed `(req S D P E)`: statement, cited declarations' identities, policy,
  environment; not the statement's hash alone — a cited body can change under its name.
- ★ A new tactic is an ordinary `fn` over text (the goal in as one form, steps out), named by
  `(auto (tactic M.FN))` and replayed before acceptance; not a metaprogram over `Expr`.
- ★ `simp_only` rewrites by the lemmas listed (256 rewrites) and `reduce` runs functions by their
  equations (2,000 rewrites, 64 unfoldings); not one `simp` over a database.
- `(reduce (using FACT…))` takes its facts in order and finds none itself; it holds the functions
  the facts speak of until their arguments are rewritten; an unused fact is refused.
- ★ `rw` matches its left side by structure, never up to K's conversion; `(occ K EXPECT)` guards
  which occurrence — a moved source fails at the step, not later.
- A rewrite's motive goes through K's checking inference; not infer-only — under a dependency
  (`Fin.mk v h`) the motive is ill-typed and K refuses the whole proof, naming no step.
- ★ `arith` is a Farkas certificate through `Lean.Omega`'s lemmas, K's conversion doing the
  sums; not reflection and not `omega`: no case split, `≠` no row, `Nat` subtraction an atom.
- `reduce` lets K's whnf fire on closed subterms only and waits at an open discriminant; not
  everywhere — K turns `a + 8` into eight successors; a forced tag test ran ten minutes, 69 GB.
- `reduce` opens a `let` by hand where K's whnf ran past it, the body at the value in one
  step; not the `let` left standing — over a case analysis K's whnf goes on into the row and
  its `if` on an open condition, a stuck eliminator that reads as no progress, and every
  case step under the `let` then waited on its variable for good: a function whose body
  opens with a `let` could be unfolded and taken no further (pin `reduce_let`). Found and
  built while the JSON parser's facts were typed, which need it, and put to the owner after:
  "Keep the reduce change".
- `reduce` leaves the calls of the functions a step names as written, `(reduce (keep NAME…))`,
  as if each were a variable: no equation of the function at such a call, no step of K's on a
  closed term it stands in, a case step waiting where it stands in the discriminant; its
  arguments reduced, a rule of `using` rewriting it. Not a variable written in the theorem for
  the argument the equation would match on, its shape a hypothesis (the statement then carries
  what only the proof wants, and every caller passes an `Eq.refl`); not the call unfolded and
  folded again by its equation reversed (that holds where one equation fired once). The owner:
  "build the folded-call option". Its shape is the builder's: named per step and by
  function, not a form of its own over a block of steps (a seventeenth form) nor the call's
  own term; `keep` before `using`; a name no call of which the run met refused
  (`reduce_keep_unused`), as a rule that rewrote nothing is — so a keep cannot be written on
  every step of a block, only where the call stands. Put to the owner, per step or a block's
  form: "The per-step shape can stay for now" (the order and the refusal were not asked
  apart).
- ★ A closed example at program size is a test on the binary; not a theorem by `rfl` — K's
  evaluation through a measured definition ends near depth 40 to 200 (see Measured).
- A long run is split into lemmas; not one theorem — K's budget (10^8 ticks) is per declaration.
- ★ Init is the library: a statement Init has is cited by Init's name and `v3/std` holds only
  what Init lacks; not a native copy of list and arithmetic lemmas.

## The checker K and the Init import

- K implements the pinned Lean kernel rule for rule (structure eta, nested inductives, literals,
  lazy delta, `eagerReduce`); not a smaller theory — the gate is all of `Init`, verdict for verdict.
- ★ Where a structure's recursor meets a major premise that reduces to no constructor, K takes
  the structure apart as the projections of the major as it was written; not, as the pinned
  kernel does, as the projections of what reduction made of it (`k/tc.shard`
  `to_cnstr_when_structure`). The two terms are convertible and the same rule fires, so the
  rules are Lean's and the Init replay holds the verdicts to Lean's as before; what is given
  up is the pin's order at this one line, and with it a term's cost being Lean's there.
  Lean's choice loses the call: a function's call that reduction unfolded and left stuck is
  replaced, on both sides of a comparison, by its unfolding, and where the two sides spell an
  argument differently the unfoldings are compared part by part. A function defined by a
  measure and by itself reads its parameters out of the tuple of its arguments
  (`define.shard` `pj_wf_go`), so the two sides of its unfolding equation do spell them
  differently; with the matched call's result a pair, the callee measured and its fuel a
  numeral, the comparison went down the callee's recursion a level for each unit of the
  fuel, each level reducing an accessibility proof — the square of the fuel and more
  (Measured). Kept as written, two calls are compared by their arguments. Not the definer
  giving such a function the shape a group has — the arguments taken apart by their
  recursor, as Lean's own definitions by well-founded recursion are, which spells a
  parameter alike on both sides and is free of the cost today (Measured): K would stay the
  pin, but 278 obligations of 174 functions would be restated over the parameters by name, a
  group takes no evidence yet (`OPEN.md` §1), and it closes this source of two spellings and
  not the next. Not the budget handed down as a variable (above, "Definitions and
  recursion"). The owner, of the cost put to them with the three: "Find and fix the blowup
  problem." — which of the three is the builder's, and a checkpoint.
- ★ A theorem's body is delta-reducible in K; not opaque to unfolding — the pinned kernel's
  `has_value` is "definition or theorem", and Init checks under that rule. `opaque` never unfolds.
- ★ Native reduction (`Lean.reduceBool`), `unsafe`, `partial` and mutual definition blocks are
  refused as unsupported; not admitted under an extra axiom.
- Axiom closures are a least fixpoint with the memo outside the environment; not Lean's
  order-dependent `collectAxioms` memo — the oracle `v3/axioms.lean` computes the same relation.
- ★ K's raw entry rebuilds every submitted node and believes no cached id, hash, range or flag;
  not the caller's annotations — two literals wearing one id make `0 = 1` a theorem.
- The rebuild makes each distinct structure once, by structural hash; not from the submitted ids
  (62% of nodes carry none, and the load is slower) and not modulo binder names.
- ★ One environment, one counter: K has no operation over two environments; not a checked
  transfer from a pre-checked Init — ids are unique per counter; a transfer admits `(p : Prop) → p`.
- K's sources are held to linear use of the checker state by a gate (`load.shard --state`); the
  layers above reuse states by design and are safe only through the rebuild.
- K's view finds an entry at a key the caller holds (`env_find_at`) beside `env_find`, so that
  resolution hashes a citation's bytes once for all its candidates; not a hash the engines keep
  for each symbol — one more name every engine would know (`LAW.md` rule 26) — and not an index
  of what a file sees by a name's last component, the larger change while Init's names are
  fetched on demand. The bucket is searched by name: a key that is not the name's finds nothing.
  The owner saw the function: "commit and push with env_find_at as shown".
- ★ `Init` loads on demand: the export is indexed once and a module's cited closure read by range
  and checked; not the whole export per load, not a cached or serialized environment.
- No receipt that lets a load skip Init's typing judgments: measured, it saves a tenth to a
  fifth — the walk and ingestion are the cost — and only a full replay could write it.
- Every file sees all of Init; not a horizon by position in the export (`(import Init NAME)`)
  — a horizon is raised lemma by lemma, and what the toolchain builds on by name was not held
  to it, so a file's records depended on the other roots of a load.
- No line says that a file sees Init; not an `(import Init)` line that some files write and
  others do not — there were two languages by it, and the owner asked "why does the file
  need marking at all?". The line is refused with the instruction to delete it (`bad_import`);
  not read as nothing, which keeps a spelling that means nothing in the sources.
- A load given no export loads no file (`init_unavailable`); not a file read without Init
  where it cites none — that reading was the second language.
- What a file is given of Init is its own demand; not whatever the process loaded — records must
  not depend on load order. A name missed is fetched and the declaration read again. A
  `realize`'s head is registered after the file's demand is in, for the same reason: before
  it, its target resolves only where the process holds it already.

## Build and gates

- Every suite entrypoint runs on the bootstrap and on the binaries and the outputs are compared
  byte for byte; on a difference the back end is fixed and rebuilt, the binary never patched.
- ★ The shim `v3/bin/eval` exits 70 on a missing or stale binary; not a silent fallback to the
  bootstrap — a missing binary must not count as native coverage.
- ★ The loader's binary emits every root but its own (`v3/build.sh --all`): the bootstrap
  builds the loader, the loader's binary the other 28; not the bootstrap for every emit — a
  rebuild after a toolchain edit waited on 29 interpreted emits, and on the dev box the
  binary emits the loader's closure in 10 s where the bootstrap takes 128 (`LAW.md` ruling
  14). Not the previous loader's binary as the seed of the next, which would make the loader's
  own rebuild 80 s for 177: a stale binary would be run on purpose, an edit of the compiler
  would need a second round, and a change of the language the toolchain is written in could
  not be read by the old binary. What a binary-emitted root rests on is checked
  (`v3/build.sh --tie`): the C text of each root under the other engine, byte for byte.
- A build reads a toolchain root `--unchecked`: every `fn` a program alone; not the checked
  load on every build — with `Symbol` an ordinary type the checked load of the loader's
  closure is 41.6 minutes on the bootstrap for 4.4 (Measured), and the switch already takes
  the emit from 144 s to 108. Assent: "We can do that in any case, and add a runtime switch
  for the compiler to enable or disable checking." A build then compiles a function a checked
  load would refuse, so the check is a load without the flag (`toolchain_check.sh` loads
  every toolchain root so), and `parity_test.sh` holds the build's program to the bootstrap
  reader's text.
  An ordinary root is never built `--unchecked`: its contract is what the check proves.
- The check of the toolchain's own sources — every root loaded checked, nothing refused, its
  program the bootstrap reader's text — runs on the loader's binary (`toolchain_check.sh`,
  CI's `native` job); not on the bootstrap, where it is the loader interpreted running the
  checker over the sources it is written in: with `Symbol` an ordinary type that load is
  41.6 minutes for the loader's closure alone (Measured). It was `parity_test.sh` on both
  engines; that test now compares the reading of programs alone, and so does every test
  that runs or dumps a toolchain root (`route2`, `lower`, `k_clients`, `state`, the native
  tie's three). The two engines are compared on a part, the eight roots under 400
  declarations (`LAW.md` rule 21, as for the replay): two more, K's own tests of its checker,
  are half a minute each on the binary and many times that on the bootstrap. Step 1 of the `Symbol` change as put to
  the owner, who answered the four steps together: "Good. Proceed with the symbol change
  (full 1-4 scope)".
- The emit tie runs on every pipeline, in CI's `native` job; not a job started by hand, the
  owner's first choice ("I would make the trust path on CI something we trigger manually on
  occasion.") and reversed by them: "So long as it is not the limiting factor, you may add
  the --tie to the standard CI. I am reversing the decision to make it a separate CI task for
  while other actions take longer." CI builds every root on the bootstrap, so the tie there
  is the binary emitting each root again, seconds a root; if it becomes a pipeline's longest
  step it goes back to a job started by hand.
- The C text is a translation unit a file, the files compiled side by side and linked
  (`v3/build.sh`, THE COMPILE); not one unit for the whole program — `cc -O2` on the loader's
  text was 66 s in one process, the larger part of what an edit of the toolchain waits for
  after the emit. The owner: "we can try parallelizing the C compilation to speed things
  up". A function and a symbol's variable are then external names of the program, and the
  runtime is compiled once, in `main.c`'s unit, its short paths inline in every unit; the
  binary runs as fast (Measured). Under `--all` the compiles of all the roots in flight share
  one count (`V3_CC_JOBS`, by default the machine's processors); not a count for each root,
  which would be that many compiles for each of the roots in flight.
- A binary's stamp covers its sources, the toolchain's, the runtime, the flags and the
  implementation directories behind its views; not the emit's source list alone, which names views.
- Files a tool writes (`*.auto.shard`, `k/accel_pins.shard`) are outside the format gate: their
  tests compare regenerated text byte for byte.
- The loader pins run in one process on one Init base, re-rooted per case; not Init loaded again
  for each case — 33 to 42 s against 783 s.
- What a gate runs that is independent runs side by side: the byte-tie's workloads
  (`V3_TIE_JOBS`), 24 of the suite's entrypoints at a time on CI, the Init replay in a job of
  its own; not one after another — the tie was 59 of a pipeline's 88 minutes with one core of
  64 busy. What is compared is unchanged, and a log's order does not depend on the count. The
  owner: "go ahead with the two CI changes".
- Before a push the dev box runs the checks a change can move, and the pipeline runs the
  whole gate after it; not the whole gate on the dev box first — 40 minutes before every
  push, most of it over what the change cannot reach. For a measure or a proof beside a
  toolchain function, the loader's program text against the last build's and the checked
  load; with a `def`, a `derive` or a new citation of Init, the table of Init's facts
  (`parity_test.sh`); for a changed body, the build and the tests of its area, and for K the
  Init replay (`LAW.md` ruling 17; which checks count for which change is this file's reading
  of it).
- The kernel's sources (`v3/kernel`) are changed in the main worktree, and the `compiler`
  branch asks there for what it needs of them; not edited on both branches. The owner, to
  that branch's session as it relayed them: "pass a message to the agent working in the
  main branch on the toolchain fix. Let's treat that agent as the main kernel editor"; and
  here: "I figured it would be best to concentrate kernel changes here".

## Layout

- ★ The formatter is not trusted: `fmt` returns text only if the loader's own reader reads it as
  it read the source; not a second reader written for the formatter.
- The formatter is an ordinary program over Init's types with a proven `bin` contract; not
  kernel-scope code — that has no L meaning, so nothing could be stated about it.
- `shardfmt` reads one file and writes standard output; in-place rewriting is `v3/fmt.sh --fix`;
  not a mode of the binary — it would add an observable and its bolts to the proven contract.
- ★ Layout breaks at structure, 100 columns the backstop: a head keeps the arguments that fit, a
  `match` or an else-`if` chain never prints flat; not "80 columns, one argument a line" (+78%).
- Not a wider line alone either: it puts a whole nested `match` on one line, the worst shape to
  edit. The measures are edit locality, lines read and predictable rules — the readers are agents.

## Measured

Figures that still calibrate the decisions above. On the development machine (24 cores, 125 GB)
unless said. "Native" is the compiled loader; "bootstrap" is the Rust evaluator. Several are single
measurements; measure again before building on one.

- Init export: 6,490,422 lines, 57,977 declarations. Full replay by K: native 380–435 s at ~29 GB
  peak; bootstrap 4,591 s at 8.0 GB (one theorem of it takes about 25 minutes).
- K's nodes with ids, cached hashes and memo tables: export chunks 0–4 check in 7.8–18.6 s each,
  flat in the DAG size; as plain trees 13 / 28 / 48 / — / 118 s.
- K's limits per declaration: 10^8 ticks, depth 8,192. One evaluator-run proof as a single theorem
  elaborated in 365 s native and was refused `exhausted`.
- Init on demand: a module's cited closure is under 1% of the export; the index builds in 36 s and
  verifies in 36 s native; its rows are 62 / 218 / 9 bytes.
- Skipping Init's typing judgments (the removed receipt): calc 9 s against 10 s, `std/bytes` 20 s
  against 25 s. Over 3,523 declarations: parse 9.2 s, tables 13.1 s, check 47.2 s.
- Init for one file: a file with one theorem loads in 1.7 s native (the toolchain's fixed names
  and what they depend on, about 1,400 declarations); `calc_bin` in 2.9 s, `calc_spec` in 2.5 s.
- Compiled `ev`, functions over six parameters: 2.5 GB of stack on K's `nested_test`; else 12 MB.
- K evaluating a closed call through a measured definition by `rfl` (a probe, not in the tree):
  depth 40 in 2.4 s, 200 in 7.8 s, 1,000 over 120 s, 4,000 `unify_exhausted`.
- Proof-layer budgets: `reduce` 2,000 rewrites and 64 unfoldings; `simp_only` 256 rewrites; `arith`
  512 derived rows; templates 1,024 instances per load. The longest real run is 906 rewrites.
- `(reduce (using …))` on the model proofs: `len.shard` 56,452 to 25,921 bytes, `app.shard` 181,204
  to 55,387; their closures check slower, 29.3 s against 23.6 and 34.5 against 30.9.
- A 906-rewrite proof is 4.05 million nodes as a tree, 50 thousand distinct. Each structure built
  once at K's raw entry: the model root 33.2 to 26.8 s, peak 8.3 to 3.3 GB; 11.2 million node
  visits remain. Sharing read from submitted ids instead: 62% of nodes carry none; 17.0 to 23.6 s.
- A case step forced at an open discriminant: ten minutes and 69 GB; waiting at it: 15 s.
- Largest roots, native load: the model root (`models/imp`, 896 accepted) 28–31 s; the std root
  (ten files) ~6 s, ~92 s on the bootstrap; the formatter's contract ~3 s.
- Linking a program: native 4.4 ms at 1,000 functions, 24 ms at 5,000 (linear), an invocation
  0.02 ms; bootstrap 33 ms and 0.1 ms. One tactic-library step: 1.3 ms link, 2.5 ms run.
- Back end: the loader's own closure (9,174 declarations) loads, lowers and generates in
  128 s on the bootstrap as a build reads it, its C text compiles in 8 s
  as 32 units side by side (the compile of the C text, below), binary 7.2 MB;
  all 29 roots on the bootstrap were 246 s at 24 jobs before the literals were converted
  (the build's chain, below; not measured since). It was 340 s
  and 460–550 s before resolution hashed a citation's bytes once (above), and
  34 s and 146–217 s while the toolchain stood on a prelude of its own: the loader checks
  some 290 of its own functions as definitions, and Init's fixed kit, on every load of a
  toolchain root — 11 s for its own closure natively, where it took 4 s.
- Init's fixed kit, which every load reads once in a process: 2 s natively, 26 s on the
  bootstrap, for a file of one function.
- The toolchain's string literals: 5,175, of which 841 stand in the 131 functions the checker
  reads and are converted (`utf8`). Over the 29 roots 372 functions are definitions, 305
  before those were read; a load of a one-function file takes the same 2 s and 26 s.
- The two readings (`load --both`) over what the examples, the library, the formatter and the
  model write — 473 functions, the 65 lambdas they lift, 12 supplied `realize` bodies: 550
  programs, each one text both ways. Over the pins' 341: 340, and one the classifier does not
  read (a package's field at its constant). Of the 473 functions alone, as the classifier came
  to read more: 341 the same, 10 different and 122 not read; 370, none and 103 once it took
  an operator's entry from its operands; 402, none and 71 once it read what is written and
  erased; 473 once it read a function as an argument. One function of the 473 left a runtime
  argument to the elaborator (`Byte.decEq`: `BitVec.toNat`'s width, an implicit binder of
  type `Nat`). No load of the 545 roots changed with the last step.
- The Rust evaluator's reader over the same 59 roots, against the loader's program (the
  canonical text, the formatter's two directories linked on both sides; 773 lines: 705
  functions, 62 types, 6 externs): 501 lines the same, none different, 272 functions not read
  — 80 instances, 65 lifted lambdas and the 76 functions that call a template; the 43 functions
  derivations write and 8 that call one. 51 of the 59 roots read whole, the model's 174
  declarations and the formatter's binary, 173, among them. calc's closure, 59 declarations:
  read and run over five inputs in 4 ms, where the loader's load of it takes 39 s on the
  bootstrap. Over the 29 toolchain roots, 146,322 declarations, its text is the loader's; it
  reads the loader's own closure, 9,161 declarations by the table of Init's facts, in 0.29 s.
- Suite: 52 entrypoints, eight at a time: 1,061 s on the bootstrap (7,725 s of entrypoint
  time), 212 s on the binaries (1,265 s); it was 740–870 s and 135–150 s while the toolchain
  stood on a prelude of its own, and 1,014 s and 204 s while a file that cited nothing of
  Init loaded without it. The parity test loads its 29 closures eight at a time, 655 s on
  the bootstrap and 86 s natively: one after another they took 64 minutes on the bootstrap.
  Loader pins on one shared Init base, each case loaded twice: 21 s native, about 190 s on
  the bootstrap. The native tie: 1,336 s, 858 s before every load read Init.
- Formatter binary: 591 sources in 3.8 s, a 26 MB file in 9.2 s. Reformat at 100 columns: 46,669
  to 65,525 lines over the gated tree, +40% (80 columns and one argument a line: +78%).
- The engine closes 54 of calc's 100 claims unaided.
- `Symbol` as an ordinary type, on a scratch loader (2026-11-16; the loader built by the
  present one, a function the checker refuses kept as a program with the refusal, and a typed
  stand-in for every function without a definition so that each is judged on its own). Over
  the 29 toolchain roots, 9,570 functions, of which 4,792 are the generated rows of
  `k/accel_pins.shard`: 372 definitions as the tree stands; 6,682 with `Symbol` ordinary and
  no source changed; 6,896 with two types moved above their first use; 7,371 with three more
  moved and the three arm types of the program representations written as pairs (133
  sites). Types in programs only: 293, 97, 66, 19. Of the 4,778 written functions 3,283 are
  then definitions. The 2,199 left: a string literal used as bytes 1,361 (704 of them
  generated rows); behind the loader's state 290; an integer measure 189; a group without a
  measure on every member 173; a bare bit or division primitive 47; a recursion with no clause
  34; a `match` whose type must be written 33; a stated structural recursion that is not one
  20 (9 in K: `open_pis` recurses on an instantiated body); a callee still without a definition
  31; others 21. By file, definitions of functions: `prog` 40 of 49, `lower` 20 of 60, `cgen`
  33 of 84, `erasure` 103 of 151, `ev` 95 of 140. Time, native: the two loads that cover the
  roots 1,803 s and 2,098 s, where the present loader takes seconds; in five profiles K is at
  most 0.2% of it and the elaborator 0.7%, the rest the list of functions waiting on a later
  declaration and the runtime's symbol table (`OPEN.md` §1). With the generated rows put in an
  order where none cites a later one, and nothing else changed, the first load's K root takes
  53 s for 1,754 s and the second load 125 s and 6.3 GB, where the present loader takes 16 s
  on the same roots; a profile of that whole load: allocation 33%, name hashing
  (`name.shard` `mix`, a call a byte) 27%, maps 10%, the symbol table and a symbol's bytes
  8%, K's own functions and the elaborator under 2% each, the waiting list nothing. The
  present loader's 17 s on those roots divide the same way: allocation 33%, name hashing 21%,
  the symbol table 10%, maps 8% — the table was then searched one symbol after another; found
  by a hash of the text (`rt.h`, `rt_intern`) the same load takes 13.4 s for 16.7 s, its output
  the same; with a citation's bytes hashed once for all the names it may stand for
  (`scope.shard` `scope_keyed`, K's `env_find_at`) 9.4 s, and under the Rust evaluator the load
  of `elab.shard` with K's directory 67 s for 99 s. Of the 9.4 s: allocation and collection
  48%, each file's demand of Init 28% (its candidates rendered and compared as text, the index
  searched over lists of bytes), the environment's probes 11%, the primitive table built again
  at each lookup 6%. Nothing was run with the ordinary `Symbol`, and the Rust reader was not
  changed.
- A symbol as the engines' interned id against a symbol as an ordinary cell holding its bytes
  (2026-11-16, scratch trees; same sources, same inputs, the outputs identical in every pair).
  Native, a loader that has the checker read every toolchain function, over 24 roots and K's
  directory: 136.8 s and 6.29 GB interned, 134.7 s and 5.51 GB ordinary. Native, K's replay
  of the whole Init export: 484 s of CPU and 26.6 GB interned, 510 s and 27.7 GB ordinary. On
  the bootstrap, the toolchain's sources rewritten to an ordinary `Symbol` and the Rust reader
  made to forget the built-in: `loader_test` 449 s to 468 s, `loader_pins_test` 210 to 230,
  five more tests that link the loader 1 to 6% slower, the loader over the std root 116 to
  123; `ev_test`, small and all symbols, 4 s to 8 s. Of 705 million dispatches of one loader
  run, 3 million are the three symbol primitives.
- The build's chain (2026-11-17, dev box). The loader's own closure, 9,182 declarations with
  K's directory: emitted on the bootstrap in 144 s as a checked load and 108 s `--unchecked`
  (two runs side by side); by the loader's binary in 10 s, the C text — 34 files, 366,000
  lines — byte for byte the bootstrap's; `cc -O2` on it 69 s, one process over 31 parts, and
  the binary byte for byte the one built from the bootstrap's text. `v3/build.sh --all`, 24
  roots at a time: 29 binaries in 288 s — the loader's emit 107 s and its `cc` 67 s, then an
  emit of 11 to 32 s and a `cc` of up to 81 s for each other root. `v3/build.sh --tie` on
  that build, the bootstrap emitting 28 roots again and the binary the loader's: 174 s, 29
  the same. With every root on the bootstrap (`--all --bootstrap`, what CI builds) 246 s,
  and the tie after it, the binary emitting each root again, 61 s. At 8 roots at a time, the
  script's default: the chain 336 s and 19 minutes of CPU, every root on the bootstrap 397 s
  and 47 minutes — the chain waits for the loader before it starts the rest, so with a core
  free for every root the bootstrap's build is the shorter. On CI (#733, read from the
  `native` job's log): the build 591 s where it was 703 (#729), the tie 81 s, the job 1,766 s
  for 1,783; the pipeline's longest job is `v3`, 1,845 s. The checked and the unchecked reading of the loader's closure gave C text in
  another order (88 of 9,177 functions: a checked function that cites a later one is settled
  later) and, order set aside, two functions in another shape: `kernel.util.is_ws` and
  `is_digit` passed a comparison to `bool_or` and `bool_and`, declared below them, and the
  reading of programs alone does not make such a comparison a `Bool` (`OPEN.md` §1). The
  two helpers now stand above their callers, and the three texts — the bootstrap reader's,
  the checked load's, the unchecked load's — are one for every root.
- The compile of the C text (2026-11-17, dev box, gcc 16.2, 24 processors). The loader's
  text, 367,000 lines: `cc -O2` on it as one translation unit 66 s; as 32 units 73 s of CPU
  one after another, 11.7 s at 8 side by side, 7.4 s at 24, the largest unit 4.9 s alone. The
  binary of the units against the binary of the one unit, on the checked load of the
  loader's closure, six runs each: 7.75 s and 7.71 s of CPU, the outputs identical; 6.4 MB
  for 5.6. The loader's own rebuild is then 117 s for 174 — the emit 110, the compile 7 —
  and `v3/build.sh --all` 206 s at 8 roots at a time and 207 s at 24, for 336 s and 288 s.
  With the allocator's reuse of a freed cell inline in every unit as well, the same load was
  5% shorter in three runs and the binary 8.2 MB; not built. On CI (#736, read from the
  jobs' logs): the build of the 29 binaries 428 s where it was 591 (#733) — the loader's emit
  383 s, its compile 40 s —, the `native` job 1,610 s for 1,766 and `replay` 1,737 s for
  1,781; the pipeline's longest job is `v3`, 1,864 s, which builds nothing of the inventory.
- The ordinary `Symbol` as landed (2026-11-18, dev box). The loader's closure, 8,834
  functions: the checker holds 2,438 as definitions where it held 320, reads the types of
  4,963 more, and does not read 1,433. The first checked load of the rewritten sources
  refused 24 functions at their own text and 854 as their callers: a `match`, a list
  literal, a comparison or a negative numeral bound by a `let` that stated no type (17), a
  constructor with its type's name in such a `let` (4), Expr's constructor `Sort` read as
  the form, a helper of K's view used at another type, and one function with more
  program-only callees than the loader read again for (the bound was 16; it is 1,024). The
  checked load of that closure on the loader's binary: 63 s alone, 8 s before; 299 s while
  K's 4,800 generated rows stood in an order where a row cited later ones. All 29 roots
  loaded checked, 12 at a time, with the eight small ones on the bootstrap too
  (`toolchain_check.sh`): 226 s, 73 s before. The loader's emit on the bootstrap 127 to
  130 s for 107 to 110 — the 3,600 literal conversions —, `v3/build.sh --all` 238 s for 206.
  The emit tie 184 s; the native tie 314 s for 216; the suite 255 s on the binaries for 230
  and 982 s on the bootstrap for 905. The table of Init's facts the bootstrap reads the
  toolchain by is 36 lines for 155: a quoted name was a symbol of the source and counted as
  a citation of Init. The Init index built by the bootstrap from the rewritten driver is
  byte for byte the one it replaced. On CI (#739 against #738, read from the jobs' logs):
  each of the three long jobs built the Init index again on the bootstrap, the driver's
  text having changed, 1,548 s in `native` and within 15 s of that in the other two; that
  set aside, `native` 2,255 s for 1,707, `v3` about 2,180 s for 1,830 and `replay` about
  1,850 s for 1,786, so the pipeline's longest job is `native` and a pipeline is 38 minutes
  for 31. In `native`: the build 521 s for 439 (the loader's emit
  469 s for 395), the emit tie 51 s for 43, the toolchain's check 329 s for 146 at 24 roots
  at a time, the native tie 758 s for 485, the suite on the binaries 445 s for 441. The
  native tie is as long as its longest workload, the loader pins' harness on the bootstrap:
  700 s for 446, and 56 s for 37 on its binary; the tie's other workloads are within 10%
  of what they were. In `v3` the suite's longest entrypoint, the load of the imp model, is
  2,017 s for 1,680. Where the harness spends it, by the runtime's counters on its binary
  (dev box, 17 s, 3,231 million calls): `int_list_eq` 209 million, the first of all;
  `byte_ints`, a literal's conversion, 153; `chars_of_sym` 121; `sym_eq` 48; `utf8` 27;
  `sym` 16 — a symbol is compared byte by byte, and a `(sym "x")` is built at every
  evaluation.
- What stands between the toolchain's functions and a definition (2026-11-18, the checked
  load of the loader's closure, each function without one followed through the callee its
  record names to the function the chain starts at). Of the 8,834 functions 4,792 are the
  generated rows of `k/accel_pins.shard` and 4,041 are written. Written: 1,450 definitions,
  1,203 type-checked without one, 1,388 not read; K's own 630 are 259, 371 and none, the
  3,411 outside K 1,191, 832 and 1,388. K's `whnf`, `infer`, `is_def_eq`, `env_find` and
  every constructor of a term (`mk_app`, `mk_const`, …) are among those without one; what K
  has as definitions is accessors, small predicates and the names of constants. Where the
  2,591 written ones without a definition start: 1,388 at a signature that names one of 95
  types the checker does not read (52 `bad_binder`, 43 `bad_shape`: two types declared below
  their first use, `Gen` in `elab.shard` and `Req` in `reader.shard`, and four groups of
  types that name each other); 905 at one of 41 functions that use a
  primitive with no form in the logic (`band`, `bshr`, `ediv` on an `Int`) — `name.mix`,
  the hash of a name, 276; `intmap.im_find` 270; `util.show_nat_go` 206; `json.parse_int`
  37 —; 280 at a recursion the checker does not take as written (`group_measure` 138,
  `measure_type` 120, the `realize_` reasons 21). All 3,804 generated rows without a
  definition start at `name.mix`. A chain names the first callee without a definition, so a
  function counted at one start may meet another behind it. With each function judged on
  its own — a stand-in constant for every callee without a definition, the two types moved
  and the literals converted — the scratch loader gave 3,283 of 4,778 written functions a
  definition, 69%.
- The two types moved above their first use (2026-11-18, the same load): 331 functions more
  are read — written, 1,525 definitions, 1,459 type-checked without one, 1,057 not read; 64
  types are programs only. The checker refused eleven of the functions it then read, ten
  for a `match` in a place that gives it no type (bound by a `let` with no type stated, a
  scrutinee, a condition, an argument of `append`) and one for a constructor that has its
  type's name; they state the type now. The 64 types stand behind four groups that name
  each other, which no order of declarations reads: `prog.shard` `ETerm` and `EArm` (43
  types behind them), `lower.shard` `LwE` and `LwArm` (13), `ev.shard` `LTerm` and `LArm`
  (10), and the loader's `Load`, `Module` and `ForkPt`. The checked load of the loader's
  closure is 116 s for 68, and all 29 roots, 12 at a time, 380 s for 226.
- The bit-level operators with a meaning, on a scratch loader (2026-11-18; nothing of it is
  in the tree). The scratch: `and`, `or`, `xor` on `Int` as three definitions over
  `Nat.land`, `Nat.lor`, `Nat.xor` by the operands' signs; `band bor bxor bshl bshr ediv`
  read by the elaborator as operators — those three at `Int`, `Int.shiftRight`,
  `Int.shiftLeft`, `Int.ediv` —; five rows that run them as the engine operations they are.
  The checked load of the loader's closure: 6,498 definitions for 2,513 — all 4,792
  generated rows for 988, and of the written functions 1,706 for 1,525 (K's own 306 for
  259); the name hash and every constructor of a term are definitions. The load takes 114.9
  s for 114.7: no longer. The program it yields is the Rust reader's text at every use of
  the five bit operators; the bare `ediv` comes out as `Int.ediv`, in nine functions. What
  the scratch met: a row of the registry speaks for a constant of Init alone
  (`erasure.shard`, `erase_const`), so the three definitions were first accepted and still
  left `band` a program; Init's shifts take a `Nat` count, and three functions pass an
  `Int` they computed; the definitions as written rest on `propext`, by their `if` on a
  comparison of integers, and 8,187 accepted declarations with them. With the int map's
  lookup and insert stating the measure they have — the map, a child at each call — 6,568
  definitions, written 1,776 (K's 335). What the rest then starts at: 1,057 the groups of
  types that name each other; 731 an integer measure (`util.show_nat_go` 273,
  `unify.inst_level_go` 169, `intmap.im_split` 101 — which does not end on a negative key
  as written); 344 a group of functions that call each other (`expr.expr_eq` 227); 78 a
  numeral under a constructor pattern.
- The bit-level operators as landed (2026-11-18, dev box). The checked load of the loader's
  closure, 8,853 functions: 6,512 definitions for 2,513 — every generated row of
  `k/accel_pins.shard`, and of the 4,061 written functions 1,720 for 1,525 (K's own 630:
  306 for 259) —, 1,261 type-checked without one, 1,080 not read; in 116 s for 115. 8,062
  of its accepted declarations list `propext`. What the functions without a definition
  start at: 1,059 the groups of types that name each other; 947 an integer measure — 490
  of them the int map's lookup, which recurses on a child of the map and states `(measure
  k)`, 233 `util.show_nat_go`; 208 a group of functions that call each other; 76 a numeral
  under a constructor pattern. The three readings of every toolchain root — the Rust
  reader's, the loader's of programs alone, the checked load's — are one text; one use of
  `/` read otherwise in the loader's own, its first operand a function declared below
  (`OPEN.md` §1), and the two constants stand above it now. All 29 roots loaded checked,
  12 at a time: 402 s for 380. `v3/build.sh --all` 267 s, the emit tie 314 s, the
  native tie 321 s, the suite 234 s on the binaries and 1,021 s on the bootstrap. On CI
  for the type move before it (#743 against #739): `native` 2,419 s for 2,255 — the
  toolchain's check 505 s for 329 —, `v3` 2,203 s, `replay` 1,851 s: a pipeline is 40
  minutes. For this landing (#745): `native` 2,469 s — the build 529 s, the toolchain's
  check 508 s, the native tie 789 s —, `v3` 2,231 s, `replay` 1,860 s.
- The int map's lookup and insert on the map's structure (2026-11-18, dev box, a loader built
  from the tree). The checked load of the loader's closure: 6,578 definitions for 6,512 — of
  the written functions 1,786 for 1,720 (K's own 335 for 306) —, 1,195 type-checked without
  one for 1,261, the same 1,080 not read; in 116 s, as before. The lookup was the first
  function without a definition under 490 functions, and 66 gained one: the others call a
  second such function. What the functions without a definition start at now: 1,059 the
  groups of types that name each other; 733 an integer measure — `util.show_nat_go` 273,
  `unify.inst_level_go` 169, `intmap.im_split` 101 —; 347 a group of functions that call
  each other (`expr.expr_eq` 227); 78 a numeral under a constructor pattern
  (`json.parse_int` 38). The three readings of every toolchain root are one text.
- The pilot of three integer measures (2026-11-18, dev box, a loader built from the tree).
  The checked load of the loader's closure, 8,854 functions: 6,786 definitions for 6,578 —
  of the written functions 1,994 for 1,786 (K's own 376 for 335) —, 987 type-checked without
  one, 1,081 not read, no obligation pending; in 77 s for 116. The same loader on the
  sources before takes the 116 s: a checked function is read again for each callee that is
  a program only (`OPEN.md` §1), and `show_nat`, the int map's insert and `inst_level` are
  callees of hundreds. What it took: eight facts (43 lines); six proofs — one step after the
  introductions for the two counters' and the digit's obligations, a case split and two
  citations for the two keys'; the two changes of the loader (a build passes proofs over, an
  obligation is stated over type parameters). What the functions without a definition start
  at now: 1,060 the groups of types that name each other; 425 an integer measure, which 96
  functions state (`unify.inst_go` 170); 427 a group of functions that call each other, 91
  functions (`expr.expr_eq` 244); 82 a numeral under a constructor pattern. 9,022 accepted
  declarations list `propext`. All 29 roots loaded checked, 12 at a time: 258 s for 402; the
  suite 229 s on the binaries and 983 s on the bootstrap, for 234 and 1,021. On CI for the
  int map's two lines before it (#746): `native` 2,469 s, `v3` 2,192 s, `replay` 1,871 s.
- The bulk of the integer measures (2026-11-18, dev box, the pilot's loader). The checked
  load of the loader's closure, 8,855 functions: 7,011 definitions for 6,786 — of the
  written functions 2,219 for 1,994 (K's own 403 for 376) —, 762 type-checked without one,
  1,082 not read, no obligation pending; in 66 s for 77. The 96 functions restated were the
  first without a definition under 425 others, and 225 functions gained one: most of the
  others stand under `expr.expr_eq` now, with 349. What the functions without a definition
  start at: 1,061 the groups of types that name each other; 569 a group of functions that
  call each other, 93 functions; 88 a numeral under a constructor pattern; 74 a recursion
  the checker does not take — on no part of the structural argument, or with no measure —;
  16 an integer measure, the seven left. 116 functions more state an integer measure where
  the checker does not get to it: 57 behind a callee that is a program, 52 behind a
  signature it does not read. 9,756 accepted declarations list `propext`. The three agents
  took 17, 21 and 29 minutes and 152, 104 and 202 tool calls. An edit of `index.shard`,
  proofs alone, sends the Init index to be made again by its stamp: 11 minutes on the
  bootstrap here. All 29 roots loaded checked, 12 at a time: 235 s for 258; the suite 225 s
  on the binaries and 920 s on the bootstrap. On CI for the pilot (#747): `native` 2,447 s —
  the toolchain's check 363 s for 508 —, `v3` 2,199 s, `replay` 1,868 s. On CI for the bulk
  (#748): `native` 4,068 s, `v3` 3,770 s, `replay` 3,456 s, each with the Init index made
  again first, 1,582 s on the runner; `native` without it 2,486 s — the build 533 s, the
  toolchain's check 335 s, the byte-tie 967 s for 897 and for 789 before the pilot. The
  byte-tie waits for the loader pins' harness on the bootstrap: 899 s there for 833, and
  343 s for 333 on the dev box, where it did not grow. So a pipeline is as long as it was:
  what the toolchain's check gave back the byte-tie took on the runner.
- `expr_eq`'s group a definition (2026-11-18), on the binaries: of the loader's 8,855
  functions 7,122 are definitions for 7,011; of the 4,063 written, 2,330 for 2,219; of K's
  own 630, 460 for 403. A first count had 349 functions at or behind the group; 111 took a
  definition, 93 by the group alone and 18 more by four counters it uncovered. 165
  obligations of 111 functions, none pending; 10,297 accepted declarations list `propext`,
  none `Classical.choice`. What the 1,733 without a definition start at: 1,061 the types
  that name each other; 247 a group of functions that call each other with a structural
  clause or none, 95 functions in 43 groups; 195 six groups the loader reports as each
  member waiting on the next, five of them over an integer depth or fuel — the unifier's
  `assign` 83, K's `infer` 50, `whnf_core` 38 and `is_def_eq` 15, `sexpr.shard` `read_expr`
  5, `record.shard` `rewrite` 4; 89 a numeral under a constructor pattern; 83 a recursion
  the checker does not take; 53 an integer measure, 21 functions. The checked load of the
  loader's closure is slower by it, 107 s for 71: K's directory 78 s for 31, `k/tc.shard`
  alone 63 s for 18 — every count of the reader and the elaborator there some 3.5 times
  what it was, 53,122 readings of a function for 17,754, nearly all of them the group
  search's (`probe_fn`, 51,366 calls for 15,824); a build's reading of the same closure, 8 s,
  is not changed. All 29 roots loaded checked, 12 at a time: 467 s for 235. The byte-tie
  563 s for 363: it waits for the loader pins' harness on the bootstrap, 537 s for 343 (on
  the binary 25 s for 19) — three of its pins load K's view, and with it `expr.shard`,
  checked; how much of the 194 s is theirs is not measured. The suite 237 s on the binaries
  and 974 s on the bootstrap, for 225 and 920. On CI (#750): `native` 3,107 s for 2,486 —
  the toolchain's check 524 s for 335, the byte-tie 1,386 s for 967, its harness on the
  bootstrap 1,296 s for 899 —, `v3` 2,126 s, `replay` 1,892 s: a pipeline is 52 minutes
  for 41.
- The search for a group keeping its readings (2026-11-18), on the binaries: the checked
  load of the loader's closure 44 s for 107 (71 before `expr_eq`'s group was a definition),
  `k/tc.shard` 6.6 s for 59; all 29 roots loaded checked, 12 at a time, 151 s for 467 (235).
  The loader before it and the loader with it, on the tree before it, record for record and
  exit for exit: the 29 toolchain roots, 4,171 s of load for 1,355; and each of the 542
  source files of the pins, the examples, the library and the tools loaded as a root — 317
  load, 225 fail, alike; the 11 files of the imp model were not among them. Each of four
  rules that forget a reading has a pin that fails on a loader built without it
  (`mutual_forward_after`, `mutual_callee_settles`, `mutual_type_between`,
  `mutual_use_between`); the rules at a `trusts`, an import, a fetch
  from Init, a file's end and in a directory's implementation have none — each drops every
  reading kept. The loader's closure with it: 8,878 functions, 7,134 definitions, 12 of the
  23 the change adds among them; 44 groups without a measure, the pair that collects the
  names a form cites one of them. The gate: the byte-tie 534 s for 563 — still not the 363 s
  it was: it waits for the loader pins' harness on the bootstrap, 510 s for 537 (343 before
  the group), which this does not touch; a checked load of `expr.shard` on the binary is
  4.6 s for 3.4 by the group's definition and its proofs, and three pins load it. The
  suite 225 s on the binaries, 946 s on the bootstrap. On CI (#752): `native` 2,840 s for
  3,107 (2,486 before the group) — the toolchain's check 280 s for 524 (335), the byte-tie
  1,362 s for 1,386 (967), its harness on the bootstrap 1,274 s for 1,296 (899) —, `v3`
  2,114 s, `replay` 1,866 s: a pipeline is 48 minutes for 52, and 41 before the group.
- Where the byte-tie's time since `expr_eq`'s group went (2026-11-18), on the bootstrap. The
  loader pins' harness on the tree before the group and on today's, side by side, each
  line's time taken: 362 s and 522 s, and every case as long as it was but the three that
  load K's view (`k_client_reach`, `k_client_admit`, `k_client_transfer`) — 33 s each
  before, 60 s now, each loaded twice: 163 s of the 160. Such a load is 14 modules, K's
  directory not among them; it accepts 1,482 declarations for 1,172, all 310 new ones in
  `expr.shard`, 229 of them theorems: 182 the equations of `expr_eq_core` (121, one for each
  pair of constructors) and `ing_is` (61). One of the three loaded alone by `load.shard`,
  30 s of each figure the evaluator's start: 65 s with `expr.shard` as it was, 65 s with the
  size derived and nothing measured by it, 89 s with the group measured and its proofs
  absent, 93 s with them. With no key made for an accepted theorem, on a scratch loader:
  60 s before the group and 68 s with it, where the same batch had 68 s and 96 s — of the
  28 s a load, 21 are the keys of the new theorems, 8 their definitions, their checks and
  their proofs. The count of calls says the same: 825 million more with the group, some
  two thirds of them the hashing of names from their bytes; `req_walk` takes 1,034,000
  steps for 309,000 and reaches a declaration some 50,000 times for 22,000. A walk that hashes a name
  once a step and asks what it has seen first, on a scratch loader: the same records, 85 s
  for 96. On the binary, the loader's own closure loaded checked: 47.5 s, and 33.3 s with
  no key made, 11,469 of them. Nothing is changed in the tree.
- The keys by a walk that keeps each declaration's part (2026-11-18). The loader before it
  and the loader with it, on the tree before it, record for record and exit for exit: the
  29 toolchain roots, and each of the 557 other source files loaded as a root, the imp
  model's 11 among them — 332 load, 225 fail, alike. Three loads side by side, each way:
  one load of K's view on the bootstrap 61 s for 86, and 57 s with no key made — the keys
  4 s for 28; the loader's own closure checked on its binary 38.2 s for 49.3, and 35.1 s
  with no key made — 3 s for 14. What they still cost there is lookups at a key, the
  comparison of names in a bucket, and the hashing that is left (a statement's own
  constants, a file's parameters). `loader_test.shard` holds the key of `main.three_rfl`
  beside that of `a.two_rfl`, a walk that meets in the memo what another file's walk left;
  a loader whose memo answers a wrong part fails both. The gate: all 29 roots loaded
  checked, 12 at a time, 128 s for 151; the byte-tie 383 s for 534, its harness of the
  loader's pins 362 s on the bootstrap for 510 and 20 s on the binary for 24 — 363 s and
  343 s before `expr_eq`'s group was a definition; the suite 231 s on the binaries and
  979 s on the bootstrap, for 225 and 946. The loader's closure with it: 8,892 functions,
  7,147 definitions — 14 of the 15 functions the change adds, `req_deps` gone —, loaded
  checked in 36 s alone for 44. On CI (#755): `native` 2,426 s for 2,840 (2,486 before the
  group) — the toolchain's check 218 s for 280 (335), the byte-tie 1,016 s for 1,362 (967),
  its harness on the bootstrap 944 s for 1,274 (899) —, `v3` 2,189 s, `replay` 1,846 s: a
  pipeline is 41 minutes for 48, as it was before the group.
- The toolchain's types that name each other, in the logic (2026-11-18). Of the 64 types of
  the loader's closure that were programs only, four groups name each other — `prog.shard`
  `ETerm` and `EArm`, `ev.shard` `LTerm` and `LArm`, `lower.shard` `LwE` and `LwArm`,
  `loader.shard` `Module`, `Load` and `ForkPt` — and the other 55 only name one of them; of
  the 1,070 functions unread for such a type, 996 rest on the first group. K took the shape
  before anything was built: a block of two with one under `List`, declared by hand (now a
  case of `nested_test`). With the groups admitted where their first type stands, no source
  form of a type moved, and the closure is 8,940 functions for 8,892: 7,641 definitions for
  7,147 (written, 2,849 for 2,355), 1,265 type-checked programs for 654, 34 unread for
  1,091, no type a program only. Of the 1,070: 460 definitions, 599 type-checked programs —
  540 for a callee that is a program, 26 in a group without a measure, 16 at an integer
  measure —, 6 unread still, each a function above the type its signature names, 5 replaced.
  Their first reading met 16 matches with no type from their position and 7 constructors
  written as their type's name in a `let`, each rewritten, and one error: `reader.shard`
  `read_decl_autos` gave an empty list for the producers of a `def`, an `abbrev` and an
  `opaque`. One comparison bound at `Bool` read as another program than the bootstrap's
  until written `(if c true false)`; the toolchain's check found it, in ten roots. The
  checked load of the loader's closure on its binary: 76 s with every waiting function read
  again after each function that loads, where it was 36 s before the types — the definer's
  reading of a function called 68,484 times for 24,249, the loading of one 45,017 for 16,932
  —, and 49 s with a waiting function left unread while its callee has no declaration; 5 s
  of the 49 are the keys, 12,628 of them. The loader with that and the loader without, on
  the tree as it is, record for record and exit for exit: the 29 toolchain roots and each of
  the 557 other source files. The loader before the types and the loader with them, on the
  tree before: 551 of the 557 other files alike; the 6 others are pins that made a
  program-only type by naming a type declared below it (`type_e_only`, `impl_e_only`,
  `route_sig_callee`'s view and root, the readings' two fixtures), which now name one below a
  `use`. Each of the seven new pins fails on the loader before the types. The gate: all 29
  roots loaded checked, 12 at a time, 166 s for 128; the emit tie 320 s for 290; the
  byte-tie 402 s for 383, its harness of the loader's pins 380 s on the bootstrap for 362
  and 22 s on the binary for 20; the suite 241 s on the binaries and 1,048 s on the
  bootstrap, for 231 and 979. On CI (#757): `native` 2,463 s for 2,426 — the build of the 29
  binaries 553 s for 523, the toolchain's check 261 s for 218, the byte-tie 990 s for 1,016,
  its harness on the bootstrap 916 s for 944 —, `v3` 2,160 s, `replay` 1,835 s: a pipeline is
  42 minutes for 41.
- A size for types that name each other, and the recursion the programs lack (2026-11-19).
  Read off the call graph of the loader's closure (8,940 functions, 1,299 of them programs)
  by the shape of each recursive call: 455 programs do not recurse and wait on a callee, 13
  have another reason, 205 recurse as the checker takes it and wait on a callee, and 626
  recurse and lack something, in 220 units — 164 (297 functions, some 770 calls) where every
  call is on a part of a parameter or a counter less one, 13 (64) by a counter and then a
  list, 43 (265) where that reading saw no descent. Of the 113 groups among the programs, 46
  descend on one parameter per member. With `(derive ETerm size)` and the three walkers
  `calls_acc`, `calls_all`, `calls_arms` measured, 16 proofs: 8,944 functions, 7,653
  definitions for 7,641, 1,291 programs for 1,299 — the three, `calls_of`, and four of the
  loader's that waited on it alone; four functions the change adds —, 181 obligations of 114
  functions for 165 of 111. The checked load of the loader's closure, the sources before
  against the sources after on one development loader, twice: 57 and 54 s against 58 and
  52 s. The engine on the 16: 15 unaided, in 3.5 s with the load; the entries it wrote for
  the rows with a wildcard did not read back — the binder's name, `_ wildcard`, stood in
  `intro` as two —, which `engine.shard` `binder_name` now names by position. The 43 units
  read one by one are in `OPEN.md`; of its first kind, the classifier's `c_type` and its
  eight, with the four pairs under them, restated in a scratch tree: all 17 defined, 25
  programs of `classify.shard`'s closure fewer, 40 obligations, the engine 33 of them — six
  of the seven left close by the fixed lines once the two hypotheses of a row under two
  matches have different names, which the engine gives the same; the seventh needs a lemma
  about `live_arm`. The gate: all 29 roots loaded checked, 12 at a time, 154 s for 166; the
  emit tie 361 s for 320 and the byte-tie 430 s for 402, both with scratch loads running
  beside them; the suite 269 s on the binaries and 1,027 s on the bootstrap, for 241 and
  1,048. On CI (#760): `native` 2,487 s for 2,463 — the build of the 29 binaries 548 s for
  553, the toolchain's check 269 s for 261, the byte-tie 1,007 s for 990, its harness on the
  bootstrap 934 s for 916 —, `v3` 2,149 s, `replay` 1,850 s: a pipeline is 42 minutes for 42.
- The plain recursion restated, and a measure in several parts (2026-11-19). Read off the
  call graph at the commit before (8,944 functions, 1,291 of them programs): 120 units (257
  functions) where every recursive call is plain and nothing under the unit lacks anything,
  and 49 more that stand above a unit of another kind. Restated bottom-up by file in nine
  batches, each a set of files in a worktree of its own, held to the loader's program text
  byte for byte and to a checked load with nothing pending: 281 functions and 741
  obligations in 36 files, 16 of the functions in the six groups measured in several parts.
  Now 8,958 functions — 14 the definer's change adds —, 8,133 definitions for 7,653, 825
  programs for 1,291, 922 obligations of 394 functions for 181 of 114; in K's own directory
  151 programs for 170. The definer's change alone, over the loader's closure on one
  development loader: no record of another module differs but the wording of one refusal,
  in 38 records. One stored proof file stood on modules that gained obligations and was
  written again (`v3/examples/tactic/use_split.auto.shard`: the policy part of six keys).
  The gate: the 29 binaries built in 258 s; all 29 roots loaded checked, 12 at a time, 200 s
  for 154; the emit tie 317 s and the byte-tie 419 s, for 361 and 430 with scratch loads
  beside them and 320 and 402 before that; the suite 242 s on the binaries and 993 s on the
  bootstrap, for 269 and 1,027. The batches' workers: nine, each 139 to 219 thousand tokens
  and 9 to 23 minutes, none restarted for its work; the first six were started twice, their
  worktrees having been made at the pushed commit and not at the one their brief named. On
  CI (#762): 44.8 minutes, the `native` job 44.1, `v3` 35.4, `replay` 31.1.
- The level functions and K's checker restated (2026-11-19). Read off the call graph, 588 of
  the 825 programs stood above `level.shard`'s three; defined, they took 10 functions out of
  the programs, and K's 60 then stated a reason of their own (an integer measure). By what
  the rest lacked, the programs standing above each kind and those it alone kept: a numeral
  under a constructor pattern 425 and 72 (21 functions); a fact about a helper 414 and 148
  (64); a walk of the environment with no bound 316 and 79 (13); K's checker 149 and 73
  (60); K's other walks and searches 84 and 8 (13); the elaborator's group 182 and none; the
  erasure's 114 and none. With the checker: 8,960 functions, 8,218 definitions for 8,133,
  742 programs for 825, 1,120 obligations of 457 functions for 922 of 394; in K's own
  directory 78 programs for 151. The checker's 189 obligations were stated by the loader at
  the first load with the measures on, and its proofs checked at the first load with them
  written; K's directory loads checked in 26 s for 18 on the development loader. The gate on
  the dev box: the 29 binaries built in 281 s; the Init replay 413 s, 57,977 accepted and
  none exhausted under the checker's new tests; all 29 roots loaded checked in 255 s for
  200; the byte-tie 484 s; the suite 250 s on the binaries and 1,138 s on the bootstrap.
  One step failed and was mended: the table of Init's facts lacked `Nat` and its two
  constructors, which the weight's `def` cites (`parity_test.sh`, 399 s on the bootstrap).
  On CI (#763): 47.9 minutes, the `native` job 47.9, `v3` 35.9, `replay` 31.2.
- The numeral rows restated (2026-11-19). 742 programs before; 686 with the 21 functions'
  bodies changed — 56 out, the 21 and those that waited on them alone —, and no function
  refused for the row any more; 672 with the six units that opened to the forms already in
  use (14 obligations, no body changed). With them 8,962 functions, 8,290 definitions, 1,134
  obligations of 464 functions. Checked on the dev box before the push, by ruling 17: the
  29 binaries built in 279 s, all roots loaded checked in 422 s beside the suite on the
  binaries, 386 s, 52 entrypoints; then for the six units the loader's program text against
  that build's, byte for byte. On CI (#764): 77.2 minutes for the pipeline before's 47.9 —
  `native` 76.1, `v3` 62.1, `replay` 57.7; the `v3` job's build of the Init index took 27
  of them. The cause was not found: the runner is a pod of the cluster, not the dev box,
  and the next pipeline ran beside this one only for its last 25 minutes.
- K's walks and searches (2026-11-19). 672 programs before, 616 after: the 13 functions and
  those that waited on them alone. Now 8,965 functions, 8,349 definitions, 1,153 obligations
  of 476 functions; of the functions of K's directory one is a program, for 57. Checked on
  the dev box before the push, by ruling 17: the 29 binaries built in 296 s; then side by
  side the Init replay, 661 s for 413 alone — 57,977 accepted and none exhausted with a
  heartbeat spent at each binder, the bootstrap and the binary byte for byte on its first
  100,000 lines —, all 29 roots loaded checked in 477 s, and the suite on the binaries, 354
  s, 52 entrypoints. The state's check judges 322 functions of K with the twelve primitives
  it had, and the generator of the reference rows prints the committed table byte for byte
  (`t0 --pins` over the export's first 36 chunks, 105 s on the binary). The first build
  failed 11 of the 29 binaries on the user's quota of `/tmp`, where the compiler writes its
  temporary files; with `TMPDIR` on the home disk it built. On CI (#765): `native` 48.6
  minutes, `v3` 61.9, `replay` 27.4 at its second run — the first ended in 28 s at the
  checkout, the runner's pod not resolving the GitLab host, and the job was run again.
- The helper facts (2026-11-19). 616 programs before, 471 after: 29 units (a function, or a
  group) and those that waited on them alone. They opened in three rounds, each read off a
  load of the whole closure: the fourteen units of the facts and the two readers of a
  proposition, 543 programs left; the unifier's twelve and nine functions with an integer
  counter that had stood directly above, 479; one more counter and two functions above
  their type, 471. Now 8,963 functions, 8,492 definitions, 1,319 obligations of 522
  functions; three functions are not read, for 13. Checked on the dev box before the push,
  by ruling 17: the state's check; the 29 binaries built in 264 s; then side by side the
  Init replay, 661 s, 57,977 accepted and none exhausted, all 29 roots loaded checked in
  500 s for 477, and the suite on the binaries, 337 s, 52 entrypoints. The Init index as
  the tree's sources write it under the bootstrap is the one the binary built before the
  change writes, 19,208,769 bytes, byte for byte. Then the three readers and the
  s-expression reader's four functions and its loop over a file, which they opened: 461
  programs, 8,502 definitions, 1,340 obligations of 530 functions; one row restated
  (`od_block_lines`, `(cons _ _)` for `_`), no other body changed. Checked the same way:
  the build 263 s, the replay 663 s, the roots checked in 506 s, the suite 345 s — where
  the tactic library's test failed, its `solve` a definition now and the example's sidecar
  stale by the library's fingerprint, and passed with the expectation and the sidecar
  renewed. On CI: #766 82.3 minutes (`native` 81.0, `v3` 62.4, `replay` 57.3); #767, which
  started while it ran, 64.5 (`native` 54.2, `v3` 63.9, `replay` 31.3).
- The JSON parser's fuel (2026-11-19). 461 programs before, 440 after: the parser's three
  functions, the two walks of the index's builder and the walk of the load on demand, which
  stated a counter (`ix_scan`, `ix_lines`, `od_walk`), and the 15 that waited on those. Now
  8,964 functions, 8,524 definitions, 1,349 obligations of 536 functions, and no function
  the checker does not read: the type a run ends as stands above the first signature that
  names it (`ev.shard` `RunRes`), and the three that were unread wait on the evaluator's
  machine. Checked on the dev box before the push, by ruling 17: the state's check; the 29
  binaries built in 270 s; the Init replay alone, 424 s for 413, 57,977 accepted and none
  exhausted; then side by side all 29 roots loaded checked in 529 s and the suite on the
  binaries, 371 s, 52 entrypoints. The Init index as the new binary writes it is the one
  the tree's sources wrote before the change, 19,208,769 bytes, byte for byte. On CI: #768
  80.6 minutes (`native` 79.9, `v3` 61.9, `replay` 57.6), the replay itself 1,442 s for
  #767's 1,498. Of the `native` and the `replay` job 26 minutes each are the Init index
  written again before the binaries are built, a source it is stamped by changed; #767
  found it in the cache.
- The JSON parser's facts (2026-11-20). 27 theorems and 3 definitions, 716 lines with their
  comments: 678 in `json.shard`, which had 432, and 38 in `util.shard`; the obligation of
  K's reader is 5 lines. A checked load of `json.shard` alone 3.0 s before, 5.3 s after.
  Now 8,965 functions, 8,526 definitions, 439 programs, none in K's directory, 1,350
  obligations of 537 functions. The program text is the one before but for `reduce`'s
  search for a step and the one function it gained (`tactic.shard` `find_redex`,
  `zeta_once`). Checked on the dev box before the push, by ruling 17: the state's check;
  the 29 binaries built in 260 s; then side by side all 29 roots loaded checked, 148,356
  declarations in 503 s for 529, and the suite on the binaries, 341 s, 52 entrypoints. The
  Init replay was not run: the parser's program and that of K's reader are what they were.
  On CI: #770 65.9 minutes (`native` 65.2, `v3` 40.6, `replay` 31.2), both jobs finding the
  Init index in the cache; the replay itself 1,458 s, 57,977 accepted and none exhausted;
  all 29 roots loaded checked in 499 s; the byte-tie 2,167 s for 82 checks.
- `reduce`'s kept calls (2026-11-20). Nine functions more in `tactic.shard` (3,366 lines for
  3,274), each a definition: 8,974 functions, 8,535 definitions, 439 programs. In `json.shard`
  the four step theorems that took a variable for the fuel and its shape as a hypothesis state
  the fuel as it is, and the three rewrites that folded a literal's call again are gone: 15
  keeps, 8 of the value's call and 7 of the literal's, 1,082 lines for 1,110, its checked load
  alone 5.4 s for 5.3. The program text is the one before but for the step elaborator's
  functions the option touches and the engine's printer of a step. Checked on the dev box
  before the push, by ruling 17: the state's check; the 29 binaries built in 277 s, the Init
  index the one before byte for byte; then side by side all 29 roots loaded checked, 148,464
  declarations in 510 s for 503, and the suite on the binaries, 383 s, 52 entrypoints, the
  loader's pins 350 directories. The Init replay was not run: nothing of K's directory or of
  a program K runs changed. On CI: #772 66.3 minutes (`native` 65.1, `v3` 39.4, `replay`
  31.4).
- The eraser's depth (2026-11-20). 113 programs became definitions: 8,974 functions, 8,648
  definitions, 326 programs — 281 for a function they call, 45 in four groups (the
  elaborator's 34, the evaluator's machine 4, `lw_jp` 5, the classifier's reader of a
  written type 2). The 13 functions that take the depth state 25 obligations; above them
  29 functions that stated an integer, a structural clause in a group, or nothing were
  restated by the fixed lines with no body changed — the definer's projection of a body
  (`define.shard` `pj_term`, 12 functions, 35 obligations) and the split of its equations
  (`realize.shard`, 4), the classifier's pattern readers (4), the erasure's patterns and a
  constructor's fields (3), `facts_closure`, and five of `erase.shard` —: 88 obligations in
  all, 1,438 of 579 functions now, each accepted as first typed. The program text differs
  in 34 functions, those that take a depth or hand one down. The file that never ended
  loads in 2.3 s, the time of the same file without the field of its own type. Checked on
  the dev box before the push, by ruling 17: the state's check; the 29 binaries built in
  271 s, the Init index the one before byte for byte; then side by side all 29 roots loaded
  checked, 148,464 declarations in 520 s for 510, and the suite on the binaries, 393 s
  for 383, 52 entrypoints, the loader's pins 351 directories. The Init replay was not run:
  nothing of K's directory changed. On CI: #773 66.4 minutes (`native` 65.6, `v3` 40.2,
  `replay` 30.9).
- The classifier's reader (2026-11-20). 65 programs became definitions: 8,974 functions,
  8,713 definitions, 261 programs — 218 for a function they call, 43 in three groups (the
  elaborator's 34, the evaluator's machine 4, `lw_jp` 5) —, 1,535 obligations of 618
  functions. Eight theorems and a `def` beside `written_data`, some 100 lines; 97
  obligations of the reader's 39 functions, 94 of them the body's: 48 by `(arith)` alone,
  43 by the row's equation, `(reduce)` and `(arith)`, 3 by a fact (`written_data_le` twice,
  `last_of_lt`), each accepted as first typed. The fact itself was refused twice before it
  stood: its function's equations are per innermost row, and `reduce` computed the closed
  symbols its conditions name before `cases` could name them (`LANGUAGE.md`, "A fact about
  what a function returns"). `classify.shard` is 3,994 lines for 3,740. No program changed: the
  loader's program text is the one before byte for byte and so is its binary. Checked on
  the dev box before the push, by ruling 17: the 29 binaries built in 269 s, the Init index
  the one before; the loader's closure loaded checked, 58 modules with none pending; the
  suite on the binaries, 377 s for 393, 52 entrypoints. The checked load of all 29 roots
  was started beside the suite and stopped unfinished when the machine ran short of memory;
  it was left to CI. The state's check and the Init replay were not run: nothing K runs
  changed. On CI: #774 70.7 minutes (`native` 70.0, `v3` 39.1, `replay` 30.8), the checked
  load of all 29 roots in it.
- The erasure (2026-11-20). 58 programs became definitions: 8,974 functions, 8,771
  definitions, 203 programs — 160 for a function they call, 43 in three groups (the
  elaborator's 34, the evaluator's machine 4, `lw_jp` 5) —, 1,655 obligations of 658
  functions. The group's 117: 6 a step of the fuel, 14 a shorter list, 97 a lower rank at
  the same fuel, each accepted as first typed. Above it the loader's `build_demands`
  stated an integer and states its fuel and its list (3 obligations); seven functions of
  `loader.shard` waited on it alone. What the fuel counts was measured by lowering the
  budget in a scratch loader and loading the loader's closure checked: at 128 levels one
  function is refused (`k/accel_pins.shard` `accel_ref_closure`, 20 nested `if`s around a
  list of 158 names), at 192 none — before the change and after it, of 8,192. The closure's
  program text differs in 24 functions: 23 of `erasure.shard`, and the search's
  `probe_reach`. The checked load of the loader's closure takes 253 s for 192, timed side
  by side: the 58 definitions, not the search, which timed 254 s at a bound of 20 and
  256 s at 1,024. `erasure.shard` is 2,685 lines for 2,183. Checked on the dev box before
  the push, by ruling 17: the 29 binaries built in 272 s, the Init index the one before;
  the loader's closure loaded checked, 58 modules with none pending; the state's check, 36
  checks; the suite on the binaries, 265 s, 52 entrypoints, the loader's pins 352
  directories. The checked load of all 29 roots was not run on the dev box and is CI's: it
  was stopped there for memory at the step before, and was not started again unasked. The
  Init replay was not run: nothing of K's directory changed.
  On CI: #778 `native` 81.8 minutes for #774's 70.0, the checked load of all 29 roots in
  it; `v3` 44.7.
- The machine, the join-point pass and the loader's account (2026-11-20). 17 programs
  became definitions: 8,984 functions, 8,798 definitions, 186 programs — 151 for a function
  they call, the elaborator's 34, `run_loop` —, 1,697 obligations of 668 functions. The
  machine's 23 obligations and the pass's 19 were accepted as first typed; the fact about
  `pick_arm` at the second try (a definition over a constructor is cited by an equation of
  its own, `arm_room_at`: `reduce` does not open it). The pass before and after compiles
  the loader to one C text, 35 files, 25,084,934 bytes. Where a checked load's time goes
  was profiled on `erasure.shard`'s closure, its 39 functions as programs (37 s) and as
  definitions (54 s): the calls double, 9.1 to 18.7 thousand million, down one chain —
  `mem_name`, `name_eq`, `sym_eq`, `int_list_eq` — under `discharge_axs` (9.7% of the load
  as programs, 24.9% as definitions) and `params_of` (1.0%, 6.3%); K's checking is 22% and
  16.5%, 0.6 s more. With the discharged obligations and the parameters found by name,
  side by side on one tree: the loader's closure loads checked in 122 s for 256 s and
  `erasure.shard`'s in 37 s for 51 s, the records of the two loads the same bytes (47,232
  lines); `loader.shard`'s in 92 s for 241 s. The index was built unasked, on the profile,
  and reported. Checked on the dev box before the push, by ruling 17: the 29 binaries
  built in 281 s, the Init index the one before; the loader's closure loaded checked, 58
  modules with none pending; the suite on the binaries, 268 s, 52 entrypoints —
  `parity_test.sh` had failed on the build before, K's table of Init's facts 45 lines for
  the file's 42: a proof of the machine's names `band`, and the scan for the base's
  operators read the proofs a build passes over; the scan reads what the load reads, and
  the table is the 42 —; the state's check, 36 checks, one commit before the last. The
  checked load of all 29 roots was not run on the dev box and is CI's. The Init replay was
  not run: nothing of K's directory changed.
  On CI: #780 `native` 66.7 minutes for #778's 81.8, the checked load of all 29 roots in
  it; `v3` 41.8, `replay` 32.0 (#778's `replay` 29.0).
- Evidence among a measured function's parameters (2026-11-20). Probed before and after
  with one form each: a measure and `(hk (<= k n))` beside `k` and `n` was
  `definition_refused` (`fvar_in_type`), the same with the evidence between the data, and
  a proposition that names no parameter `definition_refused` (`app_type_mismatch`); each
  is a definition now, and so is a function of two parts. The loader's closure loaded
  checked before and after: of 47,301 records, the 45,660 outside `define.shard`'s module
  are the same with the requirement key left out — every hash the one it was —, and the
  key differs in 1,663, where `define.shard` is seen. `define.shard` is 2,977 lines for
  2,929. Checked on the dev box before the push, by ruling 17: the 29 binaries built in
  416 s, the Init index the one before; the loader's closure loaded checked, 58 modules
  with none pending; the suite on the binaries, 317 s, 52 entrypoints, the loader's pins
  353 directories. The checked load of all 29 roots was not run on the dev box and is CI's.
  The state's check and the Init replay were not run: nothing K runs changed. On CI: #782
  `native` 71.6 minutes, `v3` 45.8, `replay` 28.0, the checked load of all 29 roots in it.
- The elaborator (2026-11-21). 87 programs became definitions: 8,990 functions, 8,891
  definitions, 99 programs — 90 for a function they call, 8 for the integer they state as
  a measure, `run_loop` —, 1,797 obligations of 703 functions. The group's 100: 14 a step
  of the depth, 8 a shorter list, 78 a lower rank at the same depth, each accepted as first
  typed. What the depth counts was measured with scratch loaders whose test was lowered,
  the loader's closure loaded checked: at 128 levels one function is refused
  (`k/accel_pins.shard` `accel_ref_closure`, the one the erasure's fuel met), at 256 and at
  512 none, of 8,192. A list literal, by the loader before the change and by the one after:
  each reads 8,190 elements and refuses 8,191 — K before (`exhausted_rec_depth_at_infer`),
  the elaborator now (`term_depth`). The closure's program text differs in 79 functions: 77
  changed (the group's, the loader's six, their callers), `elab_op_go` and `derive_depth`
  new. `elab.shard` is 2,596 lines for 2,188. Checked on the dev box before the push, by
  ruling 17: the 29 binaries built in 278 s, the Init index the one before; the loader's
  closure loaded checked, 58 modules with none pending; the suite on the binaries, 268 s,
  52 entrypoints, of which one failed — the formatter's gate on `elab.shard`, which was not
  formatted after its proofs were placed; formatted, and that entrypoint run again, 69
  checks, 0 failed; the loader's pins 354 directories. The checked load of all 29 roots
  was not run on the dev box and is CI's. The state's check and the Init replay were not
  run: nothing of K's directory changed.
- The driver's lower units (2026-11-21). 54 programs became definitions: 8,990 functions,
  8,945 definitions, 45 programs — 43 for a function they call, `probe_reach` for its
  integer, `run_loop` —, 1,826 obligations of 722 functions. The 29 proofs (the proof
  steps' 10, the producer's 9, the loader's 10) were each accepted as first typed. The
  closure's program text differs in 5 functions. The cost of a closed fuel under a
  constructor match (`OPEN.md` §1), on a file of 24 lines, the load's time less the 2.5 s
  it takes at a fuel of 1 to 16: 0.1 s at 32, 0.6 at 64, 2.9 at 128, 13.5 at 256; with a
  result bound by a row that names no constructor, with the call handed to a function,
  and from a caller that recurses structurally or not at all, 256 costs nothing. The
  loader's own case was found by a load that ran ten minutes to 45 GB, then by a stack,
  then by a loader whose K gives up at 4 million heartbeats, which names the declaration:
  `probe_reach.eq_1`. `verify.sh`'s readings of the loader's closure take 370 s for 309.
  Checked on the dev box before the push, by ruling 17: the 29 binaries built in 292 s,
  the Init index the one before; the loader's closure loaded checked, 58 modules with none
  pending; the suite on the binaries, 270 s, 52 entrypoints, none failed. The checked load
  of all 29 roots was not run on the dev box and is CI's. The state's check and the Init
  replay were not run: nothing of K's directory changed.
- Where the cost of a closed fuel comes from (2026-11-21, `OPEN.md` §1; nothing in the tree
  changed). The file of 24 lines at a fuel of 256, a load's seconds, 2.2 of them Init: 15.4
  as it stands; 2.3 with the fuel a variable; 2.2 with the callee and the caller returning
  the three-constructor type and no pair; 14.1 with the pair matched by one row and its
  first part matched inside; 2.2 with the call naming only variables the match bound; 14.1
  with it naming another parameter; 15.1 with the callee's fuel a `Nat`; 2.3 with the
  caller one of a group of two; 14.9 with a callee that does not stop on a variable, which
  K runs to its end. The stack nine seconds in: K's comparison 195 deep — 91 rounds of an
  application's arguments and a binder's body under the comparison of two projections —
  over 54 recursors, each waiting on its major premise. A loader built from the tree's
  sources with one change to K, three lines — the pair is taken apart as the projections of
  the call as it stood, not of its unfolding — loads the file in 2.3 s at 256 and at 1,024,
  and the loader's own source with `probe_reach` measured by `Int.natAbs` in 130 s at
  7.3 GB with the equation accepted; the callee that does not stop still costs 14.1.
  Of the measured functions outside a group, 174 take two parameters or more, with 278
  obligations, read off the call graph; 279 descent proofs open by naming the tuple `p`.
- Pipeline #785 (the elaborator, `bd2f2d88`): `native` 4,451 s, `v3` 2,527, `replay` 1,823,
  the pipeline 6,432 — `native` was 4,293 on #782. Pipeline #787 (the driver's lower units,
  `748ccaa0`): `native` 4,440, `v3` 2,438, `replay` 1,836, the pipeline 6,949. Its `native`
  job, 74 minutes, from the job's log: the build of the 29 binaries 514 s, the emit tie 53,
  all 29 roots loaded checked 641, the byte-tie 42 minutes, the suite on the binaries 9.
- K's structure eta on the major as written, and the loader's driver (2026-11-21). The
  change to K is three lines; `to_cnstr_when_structure`'s three descents name one more
  local. `tc_test` has 105 cases for 101: the four on the test structure's recursor, of
  which two fail on K as it was. At a fuel of 1,024 the 24-line file was stopped by a guard
  at 24.4 GB after 155 s on K as it was, and loads in 2.3 s. Checked on the dev box before
  the push, by ruling 17: the 29 binaries built in 269 s; the state's check, 36 checks;
  the suite on the binaries, 286 s, 52 entrypoints, none failed; the Init replay on
  compiled K — 57,977 accepted, none rejected or exhausted, the axiom closures the
  oracle's, the verdict line the pinned one, all 20 accelerators pinned —, 551 s and
  29 GiB beside two other loads (420 s is the box's time alone). Then the driver: 37
  programs became definitions — 8,990 functions, 8,982 definitions, 8 programs, `run_loop`
  and the seven above it —, 1,866 obligations of 747 functions. The 40 proofs (2 for
  `probe_reach`, 38 for the 24 functions above it) were each accepted as first typed.
  `loader.shard` alone loads checked in 190 s at 8.2 GB, for 133 s with `probe_reach` only
  and 151 s before either; it is 5,647 lines for 5,507. The closure's program text differs
  in 7 functions: `to_cnstr_when_structure` and `inductive_reduce_rec`, and the five whose
  import depth moved (`base_loaded`, `load_file_module`, `load_forms`, `impl_replay`,
  `impl_form`). After the driver: the 29 binaries built in 263 s, the suite 248 s, 52
  entrypoints, none failed; the loader's closure loaded checked, 58 modules with none
  pending. The checked load of all 29 roots was not run on the dev box and is CI's. On CI,
  #792 (`5f9375ce`): `native` 4,245 s — the build 488, all 29 roots loaded checked in 885 for
  641 on #787, 148,624 declarations and none failed, the byte-tie 36 minutes —, `v3` 2,521,
  `replay` 1,805 with the replay itself 1,412 s and 57,977 accepted; the pipeline 6,215.
- A pipeline's 88 minutes before the gate's work ran side by side (#725, read from the jobs'
  logs, whose lines carry their time): the `native` job 87 — the build of the 29 binaries 18
  (the C text of each produced under the bootstrap: 10,083 s summed, the compiler 2,711 s), the
  byte-tie 59, its workloads one after another, the suite on the binaries 7; the `v3` job 70 —
  the suite on the bootstrap 43 at 8 entrypoints at a time (18,816 s summed; the longest, one
  load of the imp model, 31), the Init replay 21. The runner takes 2 to 3 times the dev box's
  time for each step. One start of the loader on a file of 14 lines is 30 s on the bootstrap
  (64 s on the runner) and 2 to 4 s native, all of it Init's part: the declarations the file
  needs fetched from the export's text and checked. On the dev box the byte-tie takes 200 s
  with 8 workloads at a time, where it took 1,350 s. With the work side by side and
  resolution's change (#728): `v3` 31 minutes — the suite 27.5, which is its longest
  entrypoint's time —, `replay` 29, `native` 27 (the build 10 with 6,399 s of C text summed,
  the byte-tie 8, the suite on the binaries 7). The three start together; in #728 itself
  `native` waited 29 minutes for a place behind an older pipeline, which made it 57. The next
  pipeline, #729, on an idle runner: 31 minutes, its three gate jobs 30, 29 and 30.
