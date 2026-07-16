# PIO — the RP2350 state-machine ISA model, and PIO-search

> Status legend: **[BUILT]** landed and exercised by the corpus · **[DECIDED]**
> ratified, not yet built · **[FUTURE]** anticipated, deliberately deferred.

See also: `ISA.md` (models are ordinary libraries; the trust-leaf story this
instantiates), `SEARCH.md` (the typed-scope search engine this model feeds;
`typed_x86_calculator` is the pattern PIO-search follows), and the external
provenance repo `~/workspace/mlx-pio` (the "twin spec"
`docs/evaluator-spec.md`, the Rust/vendored-emulator reference pair, and the
101 certified differential vectors this arc inherits as its reality leg).

Drafted 2026-07-16 on the `PIO` fork (based on search-arc). Ratified scope:
model + vector gate + search objective, per the survey discussion of the same
date.

---

## 1. Why this model exists

PIO is the programmable-I/O coprocessor on RP2040/RP2350: per state machine,
a 32-slot × 16-bit instruction memory, nine instructions, two scratch
registers, two shift registers with saturating counters, two 4-deep FIFOs, a
fractional clock divider, and pin latches. Programs are reactive waveform
transformers.

The model's primary purpose is **search**: PIO programs are tiny (≤ 32
words), the useful ones are 2–10 instructions, and the objective space
(waveform specs) is crisp — exactly the shape the typed-grammar search engine
eats. The flagship objective is reproducing `~/workspace/mlx-pio`'s
superoptimizer result (a 10BASE-T1S DME/biphase encoder) with a kernel-checked
correctness proof attached, the way `typed_x86_calculator.shard` reproduced
the mlx86 calculator. Nothing in the engine names PIO (the S4a/S5 pins in
SEARCH.md); the model is an ordinary library per ISA.md — zero kernel
changes, zero axioms.

## 2. The denotation — a total cycle-trace transformer [DECIDED]

Unlike wasm/x86, PIO needs no big-step call denotation and no fuel `Option`:

```
pio_run : Cfg -> St -> (List stimulus) -> (List trace-word)
```

One input sample per system-clock cycle in, one observation word out; the
input trace's length IS the budget, and it is physically meaningful (a
cycle). The function is **total by construction** — there is no trap outcome;
invalid instructions are unrepresentable (§4). This dodges the structural
problem that made unstructured ISAs unsuitable as first models (ISA.md §2):
the semantic object here is the small-step machine itself, observed per
cycle, which is also exactly what a waveform spec constrains.

Observation format (inherited from the vector corpus so the reality leg needs
no adapter): per cycle, bit j = level of capture-pin j, bit 16+j = its
direction bit.

## 3. The v1 fragment [DECIDED]

Fixed by census over the 101-vector corpus (all measurements 2026-07-16, via
the scratch twin; see §5):

**In:** all nine instructions — JMP (all 8 conditions), WAIT (GPIO / PIN /
IRQ / JMPPIN with idx ≤ 3), IN (PINS/X/Y/NULL/ISR/OSR), OUT
(PINS/X/Y/NULL/PINDIRS/ISR), PUSH/PULL (all IfFull/IfEmpty/Block combos), MOV
(ops none/invert/bit-reverse; dsts PINS/X/Y/PINDIRS/ISR/OSR; srcs
PINS/X/Y/NULL/STATUS/ISR/OSR), IRQ set/clear/wait (this-block + REL), SET
(PINS/X/Y/PINDIRS). Delay + side-set including the optional `side_en` enable
bit and `side_pindir`. Wrap. Autopush/autopull (datasheet §3.5.4 shapes,
including background refill and the PULL-on-full-OSR no-op). The fractional
clock divider (heavily exercised: 40 distinct frac values). Pin mapping (in /
out / set / side-set bases+counts), `in_count` masking, STATUS sel/level.
FIFO depths as config (join = a depth assignment).

**Out (named growth, [FUTURE]):** the EXEC family (`OUT EXEC`, `MOV EXEC`,
forced `SMx_INSTR` — no `pending_exec` state in v1; the corpus generator
never emits these), `OUT PC` / `MOV PC`, MOV↔RXFIFO (RP2350 `FJOIN_RX_PUT/GET`),
cross-block IRQ targets (see quiet-neighbor below), multi-SM / the 2-SM
product machine, GPIOBASE >32-pin windows, streaming TX-FIFO refill in the
driver (corpus inputs are always ≤ 4 words, preload-only), `SM_RESTART`,
input-synchronizer modeling.

**Chip target:** RP2350 (superset; also mlx-pio's and Raven-Firmware's chip).
The v1 fragment minus JMPPIN/MOV-PINDIRS/IRQ-modes is RP2040-clean; a
RP2040 mode is not modeled separately.

**Quiet-neighbor IRQ [DECIDED]:** RP2350 IRQ/WAIT-IRQ IdxMode 01/11
(PREV/NEXT) target another PIO block's flags. v1 models a single block in an
otherwise-idle system: remote set/clear is locally invisible; remote wait
reads 0 (wait-for-1 parks). This is hardware-true for single-block-active
deployments and corpus-compatible (27 vectors execute such words; zero trace
divergence). The assumption is named here so multi-block growth knows what to
revisit.

## 4. Representation [DECIDED]

- **`PInstr` is a typed ADT with per-field enums** (`JmpCond`, `WaitSrc`,
  `InSrc`, `OutDst`, `MovDst/MovOp/MovSrc`, `SetDst`) — illegal instructions
  are unrepresentable, and every field is a typed zone for the search
  grammar. This is the deliberate inversion of mlx-pio's shard attempt
  (raw 16-bit words decoded by bit-twiddling per cycle), which would collapse
  typed search back to blind genome enumeration.
- Side-set/delay ride each instruction as **decoded fields** (`side`,
  `delay`), not the packed 5-bit word field. Encode/decode are
  config-indexed (the split depends on `side_count`/`side_en`, as in real
  assembly, where `.side_set` is a directive).
- **`encode : Cfg -> PInstr -> Int`** (16-bit word) and
  **`decode : Cfg -> Int -> Option PInstr`**, with round-trip theorems.
  `decode` is the fragment gate: reserved encodings and fenced features
  return `None`, so the model never guesses at undefined behavior — the
  wasm-model posture ("refuse invalid code") transposed to a total machine.
- Config and state are `(record …)` consumers; no getter/setter boilerplate.
- Values are bare `Int` in range, masked at operation boundaries (the wasm
  model's discipline; the interpreter must REDUCE in consumer proofs).

## 5. Semantics authority and the divergence register [DECIDED]

**The model speaks datasheet truth; the inherited vectors speak emulator
truth; where they disagree, the datasheet wins and the disagreement is
pinned here.** Sources, in trust order: RP2040/RP2350 datasheets (primary;
local markdown under Raven-Firmware), mlx-pio's `evaluator-spec.md` +
`shard_pio/emulator.shard` (the oracle the vectors certify), bench silicon
([FUTURE] — the eventual authority, via a Raven-Firmware capture harness).

A scratch Python twin (session artifact, 2026-07-16) implemented both
semantics and replayed the corpus. Oracle-exact mode: **101/101** (the
oracle's semantics are fully understood). Datasheet-true mode + quiet
neighbor + off-fragment stop: **85 full / 13 clean-prefix / 3 excluded**.

Divergences of the oracle (= vendored `picoem` emulator) from the datasheet,
all verified in primary text, with corpus attribution:

| # | delta | datasheet says | oracle does | corpus impact |
|---|-------|----------------|-------------|---------------|
| 1 | `JMP X--`/`Y--` at 0 | always decrements (wraps to 2³²−1); branch tests pre-value (§3.4.2 note) | decrements only when taken | **2 vectors diverge** (sideset_12, stim_18) |
| 2 | `PULL IfEmpty` / `PUSH IfFull` | no-op unless shift count ≥ threshold; fullness handled after | gates on FIFO fullness instead; `PULL IfEmpty` on empty TX does an X-copy even with Block | **1 vector diverges** (sideset_10) |
| 3 | `MOV ISR/OSR` | clears the respective shift counter (§3.2.3.3) | leaves counters untouched | 0 (unexercised discriminably) |
| 4 | `OUT ISR, n` | sets input shift counter := n | leaves it | 0 |
| 5 | `WAIT` src 3 | RP2350 JMPPIN (idx ≤ 3); idx > 3 reserved | no-op for all idx | 13 vectors execute **reserved** idx>3 words → prefix-gated; valid JMPPIN is never exercised |
| 6 | autopush shape | shift first, push at threshold, **stall the IN** on full RX (§3.5.4.1) | defers the stall to the next IN's pre-check | 0 trace divergence (events fire ×7) |
| 7 | autopull shape | OUT at threshold refills-and-**stalls**; background refill on non-OUT cycles; PULL no-ops on full OSR (§3.5.4.2) | refills-and-executes same cycle; no background refill; no PULL no-op | 0 in corpus; **DME discriminates it (P4)**: the first OUT finds an empty OSR and the refill bubble shifts every edge **+1 cycle** vs the oracle (twin-measured, both corpora — a constant phase shift, identical edge structure) |
| 8 | `OUT PINS/PINDIRS` window | writes the full `out_count` window, zero-padded | clips to min(out_count, bitcount) | 0 (out_count ≡ 1 in corpus) |
| 9 | IRQ index modes | RP2350 IdxMode [4:3]: this/PREV/REL/NEXT | RP2040-style bit-4 rel; bit 3 silently dropped | 0 under quiet-neighbor |

Deltas 6–7 are datasheet-derived but corpus-unadjudicated; bench
certification is their eventual authority. Deltas 1–2 are candidate **bugs
in mlx-pio's evaluator and vendored emulator** (picoem's `exec_jmp` comment
misattributes its behavior to the datasheet); to be reported upstream.

## 6. The vector gate (P2) [DECIDED]

The 101 vectors land as committed generated shard data (regen pin, canon
contract: committed file byte-identical to regeneration from
`shard_vectors.jsonl`). The gate classifies, it does not average:

- **85 vectors: exact full-trace agreement**, cycle for cycle.
- **13 vectors: exact prefix agreement** up to a pinned stop cycle where the
  program executes a reserved encoding (`decode = None`); the stop cycle and
  reason are part of the pinned expectation.
- **3 vectors: excluded**, each naming its delta row above. If mlx-pio fixes
  deltas 1–2 and regenerates, these rejoin.

Any drift from this classification is a FAIL. The driver contract
(TX preload, output-pin dirs, stimulus last-value-holds, capture words) is
`evaluator-spec.md` §9, implemented in the runner, not the model.

## 7. Rungs

- **P1 [LANDED 2026-07-16, e6f5e26 + 6237e77]** — `models/pio/pio.shard`
  (types + machine + `pio_run`) and `models/pio/encode.shard` (config-indexed
  word encode/decode), with ground smoke pins (square wave, side-set+delay,
  clock divider, JMP X-- wrap, PULL/OUT, family round trips, fenced decodes).
- **P2 [LANDED 2026-07-16, 70d8cf1]** — the vector gate of §6, corpus-pinned:
  `examples/pio_vecgate.shard` `vec_gate_85_13_3`, one claim over all 101
  vectors (~8s), plus the generated-data regen pin in `run_corpus.sh`.
- **P3 [LANDED 2026-07-16]** — the first search objective:
  `tools/search/tasks/typed_pio_square.shard` on the `typed_x86_calculator`
  pattern — routed `TgScopeEnv` zones over the bare-item PIO ctor scope
  (`MkPIns`; `PSet`/`PJmp`; four SET destinations; int atoms for value,
  target, delay; side-set pinned `None`), observer = `wave_bits ∘ pio_run`
  over a twelve-cycle all-zero battery. Census: TOTAL 400, FOUND 2, BEST =
  WITNESS = rank 61 (`set pins,1 [1] / set pins,0 [1]`, the datasheet wave);
  the second solution is the `set pindirs` GAUGE TWIN — toggling the
  direction against the all-ones reset latch reads as the same pad wave,
  because an undriven unstimulated pad composes to 0. The G4 half,
  `tools/search/gen/pio_square_refinement.shard`, proves the winner's trace
  equals the period-4 waveform spec for ALL cycle counts (`sq_refines`; §8
  records the measured proof shape) plus ground battery/spec/gauge-twin
  pins. Both files are corpus targets; the engine run is a corpus pin
  (expected census in the pin's comment).
- **P4 [LANDED 2026-07-16]** — the DME reproduction:
  `tools/search/tasks/typed_pio_dme.shard` transplants mlx-pio's locked
  benchmark — the reference is `dme_spec_ref` (pio_superopt's 8-instruction
  spec-shaped compression seed: 16-cycle cell, autopull threshold 5), the
  train corpus is `dme_corpus` under the locked 278-cycle window, and the
  golden is the reference's own trace on THIS model (their `dme_golden`
  pattern; datasheet-true — every edge sits +1 cycle after their emulator's,
  the row-7 refill-bubble measurement above). On the calculator4 composition
  pattern the eight slot ROLES are fixed (consume / toggle / drive / branch /
  skip / toggle / drive / drive) and the search owns the timing and wiring:
  drive delays, branch condition + target, skip target + balance delay,
  drive polarity. Census: TOTAL 4,608, FOUND 2, KILLED 4,606 (531 regions,
  ~2m40s): the reference (WITNESS, rank 854) and the `jmp 6 [0]` gauge twin
  (BEST, rank 834) — the bit-0 path re-drives the held level through slot 6
  instead of idling a delay cycle, cycle-identical on any data. The G4 half,
  `tools/search/gen/pio_dme_refinement.shard` (~0.3s), kernel-checks the
  reproduction gates: exact 278-cycle replay on the train battery AND on the
  held-out `dme_validation_corpus` (mlx-pio's `dme_validate` both-zero gate)
  for BOTH survivors, the 340-cycle drain, the exact drained-FIFO park state
  (via `pio_final`, the run-final companion added to the model at this
  rung), and `dme_park_hold`: the park is a fixed point — the pin holds its
  level for every stimulus budget. Deferred as named growth: the certifier
  oracle (phase-free DP waveform alignment) is not transplanted, and
  window-composition lemmas (`pio_run` over appended stimuli) would
  formalize the golden-plus-constant-tail extension.
- **P5 [DECIDED 2026-07-16]** — "mine our own DME without the line-by-line
  template": port the search branch's transition toolkit (whole-orbit
  transition mining; checked observer/spine/guard profiles) to PIO and
  re-derive DME with the template freed.
  - **P5a [LANDED 2026-07-16]** — the mining census:
    `tools/search/tasks/pio_transition_mining.shard`, straight-line
    0–3-instruction windows over the DME fragment (OUT X,1 / MOV Y,(!)Y /
    MOV PINS,(!)Y, delays 0–1, no jumps — control flow is composition glue),
    observed from four seed states as exact trace-plus-`pio_final` keys (the
    PSOff self-truncation makes a window's cycle count observable). Census
    (~1m20s): TOTAL 1,111 / BEHAVIORS 333 / EXCESS 778 / COLLIDED-BUCKETS
    208 / UNIQUE-SCHEMAS 54 / VALIDATED-PROPOSALS 20, AUTHENTICATED 0 (no
    PIO window theorems exist yet). The mined families are PIO-native gauge,
    several with a NONLINEAR `MovOp` metavariable held across the window:
    same-value re-drives absorbing into delays, delay redistribution across
    pin-silent windows, MOV Y,Y as a pure hold, and double-toggle collapses.
  - **P5b [LANDED 2026-07-16]** — the checked-pressure half:
    `tools/search/tasks/pio_transition_window.shard` on the `xtw` pattern.
    The probe is a task-local straight-line projection (`PTW`, the six
    observed fields with the pin/pindir latches held PRE-MASKED as bits —
    spine proofs are infeasible over `pio_run` directly: the program is
    inside the config): `ptw_step` mirrors the model's
    `out_shift`/`exec_mov` arms by direct constructor matching, `ptw_wave`
    emits 1+delay copies of the post-execute pad bit per instruction,
    `ptw_fin` composes states. Because the latch is pre-masked, every law
    in the mined re-drive family is an identity of ONE SHARED STUCK TERM
    (`ptw_op x0 y` parks identically on both sides), so the four checked
    laws are premise-free, hold for arbitrary Int state fields, need no
    case split, and quantify the `MovOp` nonlinearly — one theorem covers
    invert/none/reverse where the x86 adapter needed a word invariant plus
    guards. Landed laws (each: wave/fin tail lemmas by single-ctor case-on
    + compute-both, wave/fin context lemmas by prefix induction, observe by
    seeds induction, spine claim over `search_probe`): `ptw_redrive_spine`
    (pins[0] pins[0] ⇒ pins[1], the 2-window core that fires at the most
    positions) and the mined drive-absorb trio `ptw_absorb_{a,b,c}_spine`.
    Model tie-back: `ptw_model_absorb_a` re-proves the flagship on the REAL
    `pio_run`/`pio_final` at every census seed for every `MovOp` (case-on +
    ground compute), and `ptw_projection_pin_a` pins projection ≡ model
    observer on the flagship windows. Results: audit run (36s) SPINE RULES
    4 / RAW 1111 / ENUMERATIVE AGREEMENT OK / ACCEPTED 1067 / CONSTRAINED
    44 (exactly the hand-counted union: 40 re-drive + 4 additional
    3-window absorbs); mining replay against the projection probe
    reproduces P5a's census BYTE-IDENTICAL (1111/333/208/54/20) — the
    strongest adequacy evidence — and now reports AUTHENTICATED 3 (the
    trio cites its spine theorems by name; worklist 20→17). Boundary
    finding (refined post-landing; PIO_REQUEST.md filed in the search
    worktree): the mined mov-y collapse family (pins-y-y ⇒ pins-pins,
    nonlinear `x0:MovOp`) is not state-universal as ONE law — MOInvert
    needs y-is-32-bit-word — but it DECOMPOSES: the MONone instance is a
    capturable premise-free hold law today, and the MOInvert instance is
    capturable via the xtw word-invariant pattern (premise discharged at
    the concrete seeds inside the observe lemma, preserved through
    arbitrary prefixes — every fragment write keeps y a word; the final
    spine claim is premise-free), i.e. PIO-side proof labor, no engine
    change. The GENUINE engine gap is arithmetic relations between int
    metavariables: the whole delay-redistribution gauge is one schema
    under cycle-count conservation (`pins[d1] pins[d2] ⇒ pins[d1+d2+1]`),
    which TrsGuard cannot express — hence four literal-delay laws here,
    multiplying with the delay alphabet at P5c. First engine exercise of
    int literals and polymorphic `None` inside checked window patterns —
    both capture fine.
  - **P5c-1 [LANDED 2026-07-16]** — the free-synthesis surface + the first
    scaling measurement: `tools/search/tasks/pio_dme_free.shard` — the P4
    benchmark with the role skeleton removed (all lengths 0–9, free
    per-slot alphabet, **wrap PARKED at 31**: P4's wrap_top=7 leaked the
    loop structure; jumps are the only control, a program that runs off
    its code dies against the golden's length). Because the engine's
    witness gate is fatal-on-fail, the census is TWO-SIDED: the grammar
    spans all lengths ≤ 9 with a passing witness, so "every solution
    decodes to k slots" carries the < k impossibility inside the
    affirmative run (standalone FOUND-0 ladders need the absence-gate ask
    in PIO_REQUEST.md). Two NEW kernel-checked encoders landed as ground
    pins: `search_witness` = the 9-slot jumps-only reference
    (delay-redistribution across the loop edge: slot 7 [4]→[3] + explicit
    `jmp 0` — the jump cycle replaces the displaced delay cycle) and
    `pdf_ref8` = an EIGHT-slot jumps-only encoder (fold the 0-path
    through the re-drive slot: `jmp 6` skips only the toggle, the second
    drive slot and the skip landing collapse) — one instruction less
    control hardware than mlx-pio's wrap-assisted eight. MEASURED
    boundary: the full-free alphabet (23 ops × delays 0–7 × 9-way jmp
    targets, ~4×10²⁰ raw) drove the lazy engine past 64 GB of region
    frontier and was killed — the golden constrains EDGES, not interior
    filler, so every pin-silent instruction composition whose delays sum
    right survives each constant-bit run, and regions never re-merge.
    The filler gauge cannot be spine-quotiented here (jump-ful probe), so
    the P5c-2 scope DECLARES it away: a delay-normal-form wrapper
    grammar (delays only on drives and jumps) via a task-local candidate
    ADT + decoder, since zone routing cannot couple an op to its own
    delay domain through a shared MkPIns head.
  - **P5c-2 [NEXT]** — the delay-normal-form deep run on the rebased
    engine (search-arc landed the affine-guard + ground-arithmetic-fold +
    theorem-first features from PIO_REQUEST.md): wrapper-ADT grammar
    (~54/slot), watchdogged; solution set = the gauge-quotiented encoder
    census, minimality read from the solution lengths. Affine note: the
    kernel-checked delay-conservation law over a cycle-emitting
    projection is FALSE at negative Ints and ℤ-affine equality cannot
    express 0 ≤ d — the follow-up ask is an inequality guard or ℕ-typed
    guard variables; graded impossibility censuses below the witness
    threshold stay gated on the absence gate.
- **[FUTURE]** — bench certification (hardware-captured vectors adjudicating
  deltas 5–7 and the excluded 3); the 2-SM product machine (mlx-pio's
  statable-but-unproven flagship `2-SM-pair ≡ 1-SM-TX` becomes provable
  here); EXEC family; MOVRX; streaming driver leg.

## 8. Proof posture (expectations, to be measured at P3)

Piece theorems are trace equations: `∀ n stim, pio_run cfg st0 (stim n) =
spec n`. Cycle recursion is structural on the stimulus list — no fuel
apparatus at all, one ι-step per cycle. The snake/wasm lessons transfer:
specs must mirror the interpreter's residue; statement literals ride
`(inline …)` sugar; per-cycle cost expected O(1) via `compute`. The measured
question (ISA.md §7's analog) is whether 200-cycle ground traces and
symbolic-length loop inductions stay cheap; the go/no-go response is model
re-factoring, never heroic proofs.

**Measured at P3** (`pio_square_refinement.shard`): the expectations hold,
comfortably. A fired cycle is one ground step lemma closed by `(compute
both)` — the recursive `pio_run` tail stays folded on the free stimulus
tail, so both sides meet on the same literal machine state; the winner's
period is four such lemmas plus one for the reset state's entry cycle. The
symbolic theorem (`sq_phase`) is a wf-induct on the Int stimulus budget with
a four-way phase split: per phase, one step-lemma rewrite, one stop-set
fenced compute, and the IH at the next phase (measure obligations = the
scan_free_sound pair). The whole artifact — five step lemmas, the induction,
the refinement, three ground batteries — checks in ~0.2s; the P2 gate's 101
certified traces (up to ~260 cycles each) compute in ~8s. One idiom is
load-bearing: computing `wave_bits` against a stuck `pio_run` tail parks as
an OPENED MATCH RESIDUE that folded spellings cannot cite, so observation
peeling rides a distribution lemma (`wave_bits_cons`) with `wave_bits` in
the compute fence — the imp-arc's stuck-scrutinee lesson, replayed at the
first opportunity.
