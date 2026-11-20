#!/usr/bin/env bash
# v3/kernel/test/x86_test.sh — the x86-64 model (v3/models/x86), loaded, and held to the
# processor. The load: the model, its Linux boundary, the file writer and the runner, every
# function a definition, nothing refused or pending; `exit42_run` and `exit42_exits` — the
# model's run of the first program's file ends the process with status 42, from whatever the
# kernel leaves — accepted on no axiom but `propext`. The runs: the first program's file, written
# by the model's own writer, and the programs of v3/models/x86/test/*.hex, each run by the model
# from the command line and, on an x86-64 Linux machine, by the processor: where the model's run
# exits, the processor's exits with the same status. A file the model does not load, an
# instruction it does not have and a read of an unmapped address are `stuck` with the reason. On
# another machine the processor's runs are skipped, and said so. The assembler (asm.shard): its
# fact and the lemmas that open it accepted, a listing with a missing label and one with an
# immediate too wide refused, and its bytes for three listings the bytes assembled by hand. The
# demonstration (demo.shard): its theorems accepted, and its file run by the model and the
# processor. The trial of low-level code as shard (buffers.shard): its functions definitions,
# the laws of the model (laws.shard) and the contracts of two functions' code
# (buffers_facts.shard) accepted on Lean's axioms, and its file's run by the model and by the
# processor exiting with what the evaluator computes of the functions.
# Exit code = the number of checks that failed.
set -u
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "x86_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
# the scratch directory holds a file this script executes: beside the index, never under /tmp
mkdir -p v3/.cache
OUT=$(mktemp -d v3/.cache/x86_test.XXXXXX)
trap 'rm -rf "$OUT"' EXIT
fail=0; checks=0
bad() { echo "x86_test: $*"; fail=$((fail+1)); }
LOAD=("$EVAL" direct v3/kernel/load.shard --root v3 --init "$EXPORT" --init-index "$INDEX")
M=v3/models/x86

# ---- the load ---------------------------------------------------------------------------------
"${LOAD[@]}" $M/exit42.shard $M/run_main.shard $M/test/listings.shard $M/demo.shard > "$OUT/load.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the model did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/load.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/load.txt" || { bad "the summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/load.txt"; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE fn ' "$OUT/load.txt")" = 0 ] || { bad "a function is a program without a definition"; grep '^RUNNABLE fn ' "$OUT/load.txt" | head -5 | cut -c1-200; }
for f in x86.reg_get x86.reg_set x86.mapped x86.fetch x86.decode x86.exec x86.step linux.elf_load linux.run linux.run_file elf.elf_exec asm.encode asm.resolve asm.decodes asm.assemble; do
  checks=$((checks+1)); grep -q "^DEFINE models.x86.$f equations=." "$OUT/load.txt" || bad "models.x86.$f is not a definition with its equations"
done
# the theorems, on `propext` at most
for t in exit42_run exit42_exits; do
  checks=$((checks+1)); grep -q "^ACCEPT models.x86.exit42.$t " "$OUT/load.txt" || bad "models.x86.exit42.$t is not accepted"
  axs=$(grep "^ACCEPT models.x86.exit42.$t " "$OUT/load.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  checks=$((checks+1)); case "$axs" in ""|propext) ;; *) bad "models.x86.exit42.$t rests on $axs" ;; esac
done

# the assembler's fact, the lemmas that open it, and three listings decided by evaluation: on no
# axiom but `propext`
for t in asm.checked_some asm.assemble_decodes asm.same_decode_true asm.decodes_cons test.listings.refuses_missing_label test.listings.refuses_wide_immediate test.listings.assembles_three demo.demo_run demo.demo_exits; do
  checks=$((checks+1)); grep -q "^ACCEPT models.x86.$t " "$OUT/load.txt" || bad "models.x86.$t is not accepted"
  axs=$(grep "^ACCEPT models.x86.$t " "$OUT/load.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  checks=$((checks+1)); case "$axs" in ""|propext) ;; *) bad "models.x86.$t rests on $axs" ;; esac
done

# ---- the first program's file: written, run by the model, run by the processor ------------------
RUN=("${LOAD[@]}" --run models.x86.run_main.main $M/run_main.shard --)
"${LOAD[@]}" --run models.x86.exit42.main $M/exit42.shard -- "$OUT/exit42" > "$OUT/write.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$(stat -c %s "$OUT/exit42" 2>/dev/null)" = 132 ] || { bad "the writer did not write the 132 bytes (exit $rc)"; head -3 "$OUT/write.txt" | cut -c1-200; }
printf '\177ELF' > "$OUT/junk"
"${RUN[@]}" run 100 "$OUT/exit42" "$OUT/junk" > "$OUT/first.txt" 2>&1
checks=$((checks+1)); [ "$(sed -n 1p "$OUT/first.txt")" = "exit 42" ] || bad "the model's run of the file is not exit 42: $(sed -n 1p "$OUT/first.txt")"
checks=$((checks+1)); [ "$(sed -n 2p "$OUT/first.txt")" = "stuck: not a file the model loads" ] || bad "a file of four bytes is not refused by the model: $(sed -n 2p "$OUT/first.txt")"
# out of steps: two steps do not reach the system call
m=$("${RUN[@]}" run 2 "$OUT/exit42" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "out of steps at 4194434" ] || bad "two steps of the model do not stop at the system call's address: $m"
onchip=0; [ "$(uname -s)-$(uname -m)" = "Linux-x86_64" ] && onchip=1
# a file run by the processor: `exit N`, or `killed` where the shell reports a signal
chip() { local err rc; chmod +x "$1"; err=$( { bash -c '"$0"; exit $?' "$1"; } 2>&1 >/dev/null ); rc=$?; if [ -n "$err" ]; then echo killed; else echo "exit $rc"; fi; }
if [ $onchip = 1 ]; then
  checks=$((checks+1)); [ "$(chip "$OUT/exit42")" = "exit 42" ] || bad "the processor's run of the file is $(chip "$OUT/exit42"), the model's exit 42"
else
  echo "x86_test: not an x86-64 Linux machine: the processor's runs are skipped"
fi

# ---- the programs of test/*.hex -----------------------------------------------------------------
# A file is hexadecimal bytes with `;` comments. Where it holds `KK`, it is eight programs, KK
# running F8..FF. Its comments state `; statuses: N…` (one for each program: the model's line is
# `exit N`) or `; model: LINE` (the model's line), and may state `; processor: exit N` or
# `; processor: killed`. Each program is wrapped by the model's writer and run by the model;
# where the model's run exits, the processor's run of the same file must exit with that status.
hex2bin() { local h; for h in $(sed 's/;.*//' "$1" | tr -s ' \n' ' '); do printf "\\x${h/KK/$2}"; done; }
pairs=(); files=(); wants=(); chips=()
for f in $M/test/*.hex; do
  n=$(basename "$f" .hex)
  if sed 's/;.*//' "$f" | grep -q KK; then ks="F8 F9 FA FB FC FD FE FF"; else ks="--"; fi
  read -r -a st <<< "$(sed -n 's/^; statuses: //p' "$f")"
  ml=$(sed -n 's/^; model: //p' "$f"); cl=$(sed -n 's/^; processor: //p' "$f")
  j=0
  for k in $ks; do
    hex2bin "$f" "$k" > "$OUT/$n.$k.bin"
    pairs+=("$OUT/$n.$k.bin" "$OUT/$n.$k"); files+=("$OUT/$n.$k")
    if [ -n "$ml" ]; then wants+=("$ml"); elif [ -n "${st[$j]:-}" ]; then wants+=("exit ${st[$j]}"); else wants+=(""); fi
    chips+=("$cl"); j=$((j+1))
  done
done
"${RUN[@]}" wrap "${pairs[@]}" > "$OUT/wrap.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the programs were not wrapped (exit $rc)"; head -3 "$OUT/wrap.txt" | cut -c1-200; }
"${RUN[@]}" run 10000 "${files[@]}" > "$OUT/model.txt" 2>&1
checks=$((checks+1)); [ "$(wc -l < "$OUT/model.txt")" = "${#files[@]}" ] || { bad "the model printed $(wc -l < "$OUT/model.txt") lines for ${#files[@]} programs"; head -3 "$OUT/model.txt" | cut -c1-200; }
checks=$((checks+1)); [ "${#files[@]}" -ge 20 ] || bad "only ${#files[@]} programs were found under $M/test"
i=0
for e in "${files[@]}"; do
  m=$(sed -n "$((i+1))p" "$OUT/model.txt"); name=$(basename "$e")
  checks=$((checks+1)); [ -n "${wants[$i]}" ] || bad "$name states no expectation (statuses: or model:)"
  checks=$((checks+1)); [ -z "${wants[$i]}" ] || [ "$m" = "${wants[$i]}" ] || bad "$name: the model's run is '$m', the file states '${wants[$i]}'"
  if [ $onchip = 1 ]; then
    c=$(chip "$e")
    case "$m" in
      "exit "*) checks=$((checks+1)); [ "$c" = "$m" ] || bad "$name: the model's run is '$m', the processor's '$c'" ;;
    esac
    checks=$((checks+1)); [ -z "${chips[$i]}" ] || [ "$c" = "${chips[$i]}" ] || bad "$name: the processor's run is '$c', the file states '${chips[$i]}'"
  fi
  i=$((i+1))
done

# ---- the assembler against the hand ------------------------------------------------------------
# Three listings of test/listings.shard are the programs of stack.hex, high.hex and add.hex (at
# KK = F8): the assembler's bytes for each are the bytes assembled by hand.
"${LOAD[@]}" --run models.x86.test.listings.main $M/test/listings.shard -- stack "$OUT/stack.asm" high "$OUT/high.asm" add "$OUT/add.asm" > "$OUT/asm.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the listings were not assembled (exit $rc)"; head -3 "$OUT/asm.txt" | cut -c1-200; }
for n in stack high add; do
  k=--; [ $n = add ] && k=F8
  checks=$((checks+1)); cmp -s "$OUT/$n.asm" "$OUT/$n.$k.bin" || bad "$n: the assembler's bytes are not the bytes assembled by hand"
done

# ---- the demonstration (demo.shard) ---------------------------------------------------------------
# Three shard functions over machine words, a listing compiled by hand, the listing's file: its
# theorems are among those held above; here the file is written, and the model's run of it and
# the processor's exit with 20, the low byte of what the function computes.
"${LOAD[@]}" --run models.x86.demo.main $M/demo.shard -- "$OUT/demo" > "$OUT/demo.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ -s "$OUT/demo" ] || { bad "the demonstration's file was not written (exit $rc)"; head -3 "$OUT/demo.txt" | cut -c1-200; }
m=$("${RUN[@]}" run 100 "$OUT/demo" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "exit 20" ] || bad "the model's run of the demonstration is not exit 20: $m"
if [ $onchip = 1 ]; then
  checks=$((checks+1)); [ "$(chip "$OUT/demo")" = "exit 20" ] || bad "the processor's run of the demonstration is $(chip "$OUT/demo"), the model's exit 20"
fi

# ---- low-level code as shard: the trial (buffers.shard, laws.shard, buffers_facts.shard) ----------
# Three functions over buffers of machine words, an access taking the evidence that its index is
# in range, and a listing for them compiled by hand. Held here: every function is a definition;
# the laws of the model and the contracts of the code of `dot` and `addto` are accepted, on no
# axiom but Lean's three (a stored word read back cites Init's lemmas on division, which rest on
# `Classical.choice`); the evaluator's run of the functions, the model's run of
# the listing's file and the processor's give one status, 19.
"${LOAD[@]}" $M/buffers_facts.shard > "$OUT/buf.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the trial did not load (exit $rc)"; grep -E 'REFUSE|READ-ERROR|LOAD-ERROR|ERROR|POLICY' "$OUT/buf.txt" | head -5 | cut -c1-300; }
checks=$((checks+1)); grep -q '^LOAD: .* refused 0  pending 0 .* errors 0$' "$OUT/buf.txt" || { bad "the trial's summary is not 0 refused, 0 pending, 0 errors"; grep '^LOAD:' "$OUT/buf.txt"; }
checks=$((checks+1)); [ "$(grep -c '^RUNNABLE fn ' "$OUT/buf.txt")" = 0 ] || { bad "a function of the trial is a program without a definition"; grep '^RUNNABLE fn ' "$OUT/buf.txt" | head -5 | cut -c1-200; }
for f in buffers.word_at buffers.put_at buffers.dot_go buffers.dot buffers.addto_go buffers.addto buffers.both laws.steps; do
  checks=$((checks+1)); grep -q "^DEFINE models.x86.$f equations=." "$OUT/buf.txt" || bad "models.x86.$f is not a definition with its equations"
done
for t in laws.reaches_step laws.reaches_trans laws.step_at laws.do_alu_rr laws.do_cmp_rr laws.do_alu_ri laws.do_mov_rr laws.do_jcc_not laws.do_jcc_taken laws.do_jmp laws.do_load_sib laws.do_imul_sib laws.do_add_m_sib laws.do_push laws.do_pop laws.do_call laws.do_ret laws.mem_put64_other laws.le64_put64_word laws.le64_same laws.fetch_same laws.code_in_same buffers_facts.holds_upd buffers_facts.holds_same buffers_facts.dot_loop buffers_facts.dot_contract buffers_facts.dot_contract_same buffers_facts.add_loop buffers_facts.addto_contract; do
  checks=$((checks+1)); grep -q "^ACCEPT models.x86.$t " "$OUT/buf.txt" || bad "models.x86.$t is not accepted"
  axs=$(grep "^ACCEPT models.x86.$t " "$OUT/buf.txt" | sed -E 's/.*axioms=([^ ]*).*/\1/')
  other=$(echo "$axs" | tr ',' '\n' | grep -v -x -e propext -e Quot.sound -e Classical.choice | tr '\n' ' ')
  checks=$((checks+1)); [ -z "${other// /}" ] || bad "models.x86.$t rests on $other"
done
"${LOAD[@]}" --run models.x86.buffers.main $M/buffers.shard -- "$OUT/buffers" > "$OUT/bufrun.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 19 ] && [ -s "$OUT/buffers" ] || { bad "the trial's file was not written, or the evaluator's status of the functions is not 19 (exit $rc)"; tail -3 "$OUT/bufrun.txt" | cut -c1-200; }
m=$("${RUN[@]}" run 1000 "$OUT/buffers" 2>&1 | tail -1)
checks=$((checks+1)); [ "$m" = "exit 19" ] || bad "the model's run of the trial's file is not exit 19: $m"
if [ $onchip = 1 ]; then
  checks=$((checks+1)); [ "$(chip "$OUT/buffers")" = "exit 19" ] || bad "the processor's run of the trial's file is $(chip "$OUT/buffers"), the model's exit 19"
fi

echo "x86_test: $checks checks, $fail failed"
exit $fail
