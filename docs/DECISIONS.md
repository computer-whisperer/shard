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
- No `Int` measure (`measure_type`) and no lexicographic tuple; a rank is added into the measure
  (`(+ (+ s s) 1)` against `(+ s s)`) — kept until the arithmetic is the burden.
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

## Programs and the back end

- A program is read from the source and the types its binders state (`LAW.md` rule 24), so the
  Rust evaluator can come to read every program and the toolchain's own files can become ordinary
  ones; not type inference in the Rust evaluator — the elaborator, the erasure and K's type
  checker, about 19,400 of the toolchain's lines, inside the engine that is the authority; not
  a seed — the Rust evaluator running the erased form a previous toolchain wrote, so that the
  authority no longer runs the sources. Assent: "Let's adopt it and begin."; and once every
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
- The toolchain's text stays a list of bytes, `(List Int)`, and a function the checker reads
  converts the literals it uses as bytes — `(utf8 "…")`, a definition in `kernel/util.shard`
  that copies the list a `String` is represented by; 131 functions, eleven of them in K's
  directory. Not a conversion at every literal: the other 4,300 stand in functions nothing
  type-checks (`OPEN.md` §4), each use of a converted literal copies it, and a conversion
  there would be checked by nothing. Not the toolchain's text made `String`: every function
  over text would change.
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
- The lowering reads no measure; the binaries are engines, not proven artifacts — the bootstrap
  evaluator is the authority and no verdict stands on a binary's word.
- ★ World threading is checked on the erased program (`thread.shard`), through boxes and helper
  summaries; not read off variable names — a World boxed once and unwrapped twice is used twice.
- An unmet requirement, an unthreaded function, an extern off the roster or an untrusted axiom
  fails the load of a `bin`; not a warning. Requirements speak of the entry's observables only.
- `invoke` validates arguments against the entry's types (the checked door) and `ev` stays the
  assumed door; not one validated path — `ev`'s contract is over values, out-of-type ones included.
- The reader keeps no handle across a file's tactic steps: five links saved are ~6.5 ms of a
  1.7 s run, against a handle stale after every definition.

- ★ `Symbol` is a type of its own, stored by the engines as an interned id; not `String` — a
  `String` runs as the list of its bytes and a symbol as an id compared in one step, and one
  type has one form at run time; not a `Name` literal in place of symbols — K's `Name` is
  built of them. It has no type in the logic today. The direction for giving it one is an
  ordinary declaration over its bytes that every engine stores as declared (`LAW.md` rule
  26); not the interned id kept beside it as an engine's shortcut — rule 26 allows one, but
  it saves nothing on a native load, 5% of the Init replay and 1 to 10% of a load on the
  bootstrap (Measured), and it would need a test that the two forms agree. Assent: "Agreed
  with removing the engine's form of a symbol." On a scratch loader that reads the ordinary
  type, 6,682 of the toolchain's 9,570 functions are definitions with no source changed
  (Measured).
- A function the checker cannot define is a program with its reason, and one whose types do
  not check is refused (`LAW.md` rule 25, ruling 13); not a refusal of every function the
  checker cannot accept — a sketch would need its proofs before it ran; not a program for a
  function with a type error — nothing would then check a program's types.

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
- ★ A closed example at program size is a test on the binary; not a theorem by `rfl` — K's
  evaluation through a measured definition ends near depth 40 to 200 (see Measured).
- A long run is split into lemmas; not one theorem — K's budget (10^8 ticks) is per declaration.
- ★ Init is the library: a statement Init has is cited by Init's name and `v3/std` holds only
  what Init lacks; not a native copy of list and arithmetic lemmas.

## The checker K and the Init import

- K implements the pinned Lean kernel rule for rule (structure eta, nested inductives, literals,
  lazy delta, `eagerReduce`); not a smaller theory — the gate is all of `Init`, verdict for verdict.
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
  binary emits the loader's closure in 10 s where the bootstrap takes 108 (`LAW.md` ruling
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
  load would refuse, so the check is a load without the flag (`parity_test.sh` loads every
  toolchain root so), and the tie holds the build's program to the bootstrap reader's text.
  An ordinary root is never built `--unchecked`: its contract is what the check proves.
- The emit tie runs on every pipeline, in CI's `native` job; not a job started by hand, the
  owner's first choice ("I would make the trust path on CI something we trigger manually on
  occasion.") and reversed by them: "So long as it is not the limiting factor, you may add
  the --tie to the standard CI. I am reversing the decision to make it a separate CI task for
  while other actions take longer." CI builds every root on the bootstrap, so the tie there
  is the binary emitting each root again, seconds a root; if it becomes a pipeline's longest
  step it goes back to a job started by hand.
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
- Back end: the loader's own closure (9,182 declarations) loads, lowers and generates in
  108 s on the bootstrap as a build reads it (144 s read checked), `cc` 69 s, binary 5.6 MB;
  all 29 roots on the bootstrap 246 s at 24 jobs (the build's chain, below). It was 340 s
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
