# STORAGE.md — RECORDS (moved out of docs/STORAGE.md on 2026-09-02)

> **STATUS: RECORD.** Dated rung/slice records split out of [../STORAGE.md](../STORAGE.md) so the LAW ledger stays readable in one sitting. Section numbers are exactly as they were in docs/STORAGE.md; a citation `STORAGE.md §N` for a section listed here resolves to this file. Nothing here is normative unless the law ledger says so; any "NEXT" pointer is history.

## 8a. S2 OPENED (2026-08-02, user ruling after the S1 record +
retraction). Sub-slicing, recorded before build per the house
pattern:

- **S2a — the codec, standalone.** The image format (CANON.md §7
  identity-bytes style: tag byte per ctor, count-prefixed lists,
  sign+digits ints) + serializer/deserializer as a TOOL
  (tools/image/), no loader contact. Gate: corpus-wide ROUNDTRIP —
  every module text-load → serialize → deserialize → structural
  equality with the original, plus codec determinism. The core AST
  is small (Expr 9 ctors, Type 2, FnDef 1, Module 1) but the full
  covered set (claims/proof scripts, typedefs, externs, tries)
  gets inventoried before the schema freezes.
  **LOAD-PATH SURVEY FINDINGS (2026-08-02, recorded before the
  codec is written).** (i) The check path parses each file's bytes
  ~10–13 TIMES: stage A (build_module_d, reader.shard:3879) plus
  stage B's parse_decls/use_forms and seven more whole-source
  read_all sweeps in driver.shard. A raw-SExpr image (FOUR ctors +
  leaves) is CONTEXT-FREE — parsing depends on nothing accumulated
  — and replaces every one of those sweeps; it is the first codec
  layer. (ii) Per-file ELABORATION is context-dependent: the
  per-file Module depends on the accumulated resolve env and ctor
  set, so a Module-level image key must CHAIN over the import
  prefix — key(k) = H(engine, bytes(k), key(prefix(k))) — bytes
  alone would be unsound; this is the Merkle-by-reference shape
  CANON.md §7 anticipated. (iii) FnTrie is a pure derived index
  over the fn list (merge via trie_insert_many): REBUILT on load,
  never serialized. (iv) The encoder precedent is
  tools/canon/hash.shard's hx_ser_* family (tag-per-ctor,
  sign+digits ints, encode-only) — the codec mirrors it and adds
  the tree's first decoder. (v) Ctor budget: ~28 for
  SExpr+core-Module; ~42 adding the decl layer; ~91 with parsed
  proof scripts — proofs stay OUT of the v1 schema (they re-parse
  per-claim from cached SExpr, and per-claim parse is not the
  measured floor).
  **S2a RECORD — SExpr layer LANDED (2026-08-02, same-day).**
  tools/image/image.shard: format IMG1 (`"IMG1;" int(count) sx*`;
  tags 0=SInt 1=SSym 2=SStr 3=SList; ints sign+digits+`;`;
  count-prefixed lists; payload bytes raw). Encoder = reversed-
  stream accumulator, decoder = fuel-bounded (read_expr_go's
  measure idiom, the tree's first decoder); both obey the reader's
  own stack law (depth = nesting, never sibling count). Checks 14/0.
  GATE GREEN: 654/654 parseable tracked .shard files (sidecars
  included) roundtrip text-load → encode → decode → structural
  equality, AND re-encoding the decoded tree reproduces the bytes
  (determinism). The 655th file is pins/lang/parse_rejects.shard,
  the DELIBERATELY unparseable reject pin — no parse, no tree,
  outside the gate's domain by its own wording. Scale spot: the
  49k-line impgen_x86_out roundtrips in 8.5s interpreted (652KB
  image; S2b's warm number is the one that prices). Next in S2a:
  the core-Module ctor set on the same format, or fold that into
  S2b if the front door lands SExpr-first — decided when S2b opens.
- **S2b — the loader front door,** behind SHARD_IMAGES=1: the
  driver consults .shard-cache/images/<content-key>.img per import;
  miss or mismatch = text path. Gate: the image-vs-text
  DIFFERENTIAL over the corpus — identical verdicts and identical
  loaded structures.
  **S2b RECORD — LANDED in two slices (2026-08-02, same-day).**
  Slice 1 = PARSE-ONCE (8083473): SrcEntry grows a forms field
  (`(Option (List SExpr))`, invariant forms ≡ read_all(bytes)),
  parsed exactly once in the loader's visit_go; all ~13 whole-source
  sweeps (build_module_d/r, parse_decls, use-scope, bins/libs/
  own-externs/measure-clause/proof-seed/refined-return) consume the
  field via _mf/_f siblings; byte entry points keep their signatures
  for tool consumers. THE NUMBER: the S1 exhibit's cold check
  11.29s → 5.50s (2.05x). Slice 2 = THE FRONT DOOR (f2246dc): the
  codec moved to kernel/image.shard (tools/image stays as the gate
  driver citing it) + the IMG2 container (`"IMG2;" len;path len;src
  IMG1-payload`); visit_go consults
  .shard-cache/images/<mangled-path>.img via resolve_closure_i;
  bin/check translates SHARD_IMAGES=1 into a trailing +images argv
  token that check.shard strips anywhere (NO getenv extern, zero
  Rust/C changes). DESIGN STRENGTHENING vs this bullet's sketch: the
  filename is a mangled PATH, not a content key — the container
  embeds the source's path AND full bytes, and a hit requires
  byte-identity with what read_file just returned. Exact validation
  has no collision class at all and is CHEAPER in-shard than bignum
  hashing; the filename becomes non-load-bearing. Gate state: local
  battery green (cold populate; warm verdict BYTE-IDENTICAL to the
  text run; stale source detected + image rewritten; corrupted image
  healed byte-identically); the corpus differential is the
  variable-gated corpus-images CI job (CORPUS_IMAGES:1 — cold +
  warm passes, both FAIL-projections diffed vs the text baseline);
  the "identical loaded structures" half is discharged by S2a's
  654/654 roundtrip + the container's exact byte validation.
  Residual exposure: silent payload corruption on disk, bounded by
  cache-never-trust (corpus/CI never pass the flag).
  **THE HONEST NUMBER (S2c's opening fact): warm-image 8.28s vs
  parse-once text 5.50s on the exhibit — the SExpr-layer door
  LOSES.** Parse-once removed the 13x multiplier, and after it one
  parse is cheaper than the door's container read (~2.5x the source
  bytes as cons cells) + exact compare + decode. S2c's expectation
  re-sets accordingly: the default stays OFF at this layer; the
  profitable rung is the MODULE-LAYER image (payload replaces
  ELABORATION, not parse — survey finding (ii)'s chained keys),
  which reuses this container, door, and differential unchanged.
  S2's real measured win so far = slice 1's 2.05x.
- **S2c — the number + the default.** B1b-exhibit warm-image
  measurement, hit rate over a real session, then the default-on
  ruling. S2's schema freezes HERE, not before.
  **S2c MEASUREMENTS BANKED (2026-08-02; ruling PENDING user).**
  All against slice-1 parse-once text as the baseline, stamped
  engines, S1 cert bypassed: (i) exhibit all-hit warm (27-file
  closure, 34MB of source): text 5.50s vs image 8.28s (+50%);
  (ii) the REAL-SESSION profile — the S1 cert already skips
  unchanged targets, so images only ever fire on cert misses,
  i.e. one edited member + the rest hits: 26/27 hits with exactly
  the edited file rewritten (the mechanism is precise), text 5.69s
  vs image 7.96s (+40%); (iii) small closure (std/list, 4 files):
  19ms vs 21ms. Read amplification explains it: the closure's
  images total 38MB beside 34MB of source (the payload itself is
  SMALLER than source — comments drop — but the embedded
  exact-validation copy adds it back), so the warm path reads and
  cons-materializes 2.1x the bytes. The door loses on EVERY
  measured profile at the SExpr layer.
  **RULED (user, 2026-08-02): DEFAULT OFF.** The door stays opt-in
  behind SHARD_IMAGES=1, differential-guarded, kept as the
  validated container/door/differential infrastructure for the
  MODULE-LAYER image (the elaboration-replacing rung, survey
  finding (ii)). The user's stated flip condition: revisit
  default-on if check times start climbing prohibitively.
  REJECTED-because: retiring the door would save ~100 loader lines
  but rebuild tested infrastructure at the Module rung; default ON
  had no measured case. SCHEMA FROZEN (ratified at S2c): IMG1 +
  IMG2 exactly as landed — the magic strings are the version gate;
  any format change bumps the magic and old images self-invalidate
  to misses.
  **S2 CLOSED 2026-08-02.** Gate verdicts: the corpus-images
  differential GREEN (pipeline #278 — cold AND warm image corpus
  passes, FAIL-projections identical to the text baseline) and the
  text corpus GREEN on the stack (#279). S2's delivered win =
  slice 1's parse-once 2.05x on every check, unconditionally; the
  codec + door + differential stand ready for the Module layer.
  Next per §10 ordering / STREAM.md sequencing: B5; S3 arena stays
  maybe-never.
  (2026-08-22: B5 closed 2026-08-09; S3 = #33 parked; the Module-layer
  image = #34 parked; CERT.md §10's ordering retired by the reset.)

## 8. S1 RECORD (2026-08-02; landed same-day as ratification)

Mechanism (bin/check): key = engine stamp + sha256 over the
target's transitive import closure — file imports, directory-module
members recursively, and every member's `.auto.shard` sidecar
(sidecars are check inputs; the prove-regen flow must invalidate).
Only the stamp-fresh shard_check engine caches; EVAL-ladder runs
never do. Success-only writes (`0 failed` verified in the output);
unresolvable imports fail OPEN to a full check; certs live in
`.shard-cache/check/` (gitignored, F2 as ruled); corpus/CI never
consult the cache.

Numbers. The standing exhibit (sha256.xcomp, the 2.38B-call
154-line file): 11.29s cold → 0.103s warm — 109×, and the warm
cost is pure closure hashing (~60 files). Invalidation on the
three-edit benchmark is exact: an edit OUTSIDE the closure
(models/riscv) leaves the cert valid; an impl edit INSIDE the
closure (sha256.imp) forces the re-check; a SIDECAR edit
(sha256.stream.auto.shard) forces the re-check. §9 (d) at module
granularity: the re-checked set is exactly the dependency-forced
set. What S1 does not touch, measured: the cold check still pays
11.3s for 154 lines — the load floor, S2's charge, now the whole
residual by construction.

**F3 AMENDMENT — RETRACTED same-day (user yellow flag; the
correction is the record).** The first S1 landing claimed consumer
proofs compute imported fn bodies, concluding interface keying
would false-hit. That claim was WRONG — an inference from usage
patterns instead of a measurement, and the user's flag caught it.
Both probes are now on record: (1) in a consumer context
`(compute both)` on an opaque directory member (`pow2 3`) STICKS —
the body is invisible, the claim fails; (2) with the impl file
SYNTACTICALLY DESTROYED (`((((` appended to bits.shard) a
consumer's check is byte-identical — consumer-mode resolution
never reads member files at all; the surface (mod.req: sigs +
requirements, admitted at the boundary) is the entire consumer
input. The cases behind the wrong claim were FILE imports
(std/nat.shard, the sha sibling files), where the whole file IS
the surface by design — no discipline was ever pierced. NOTHING
BROKE; the design point is enforced and airtight. Consequence:
ruled F3 stands EXACTLY as ratified, stronger than github #7's
sketch dared claim — for a directory import the sound key is the
mod.req content (+ its transitive surface) alone; impl edits
cannot even cause consumer load failures. v1's walker is
CORRECTED to this (directory import → hash mod.req.shard +
recurse its imports; member files and their sidecars excluded);
file imports key on file content, which is correct and
unavoidable there. The retracted text's practical harm was
over-invalidation only — no wrong hit was ever possible.
