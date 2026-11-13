#!/usr/bin/env bash
# v3/tools/shardfmt/test/shardfmt_test.sh — the formatter's gate (v3/LANGUAGE.md §8.7 slice 5.1
# rule 5; CANON.md §3). The binary is built here by route 1 (v3/build.sh over the program and
# its two views' directories) and then:
#
#   (1) THE SCENARIOS (test/scenarios/NAME.in, NAME.out): the layout's worked examples. The
#       binary's text for NAME.in is NAME.out byte for byte, exit 0, and NAME.out is its own
#       text (--check exits 0). The first nine are the old tree's requirements
#       (tools/shardfmt/mod.req's fmt_scenario_*), their texts unchanged; the rest pin the V3
#       forms. A `refuse_NAME` pair is a source that gets no text: exit 2, NAME.out the message.
#   (2) THE COMMAND LINE: the usage for no argument, two files, a flag after the file, three
#       arguments; a missing file; --check on a file that is not canonical — exit 1, the
#       canonical text written.
#   (3) THE DRIVER'S RUN: the program under `ev` (load.shard --run, on the engine under test)
#       over every scenario's source in one file — the binary's text, byte for byte.
#   (4) THE TREE: every tracked .shard file under v3/. None is refused by the gate (exit 3: the
#       printer's output read differently — a printer defect); the sources that do not read
#       are the pins broken on purpose and nothing else; each text is its own text on a second
#       pass; every comment is in the text as it stood in the file. How many files are
#       already canonical is printed, not judged: the tree is written toward the form until
#       the gate is turned on (CANON.md §8 stage 3).
#
#   (5) THE CONTRACT (shardfmt_bin.shard; landing 2): the load records the bin with both
#       requirements met — `fmt_gate` and `fmt_main_contract` —, the program's four effectful
#       functions threaded, and each requirement's assumptions are exactly the views'
#       parameters (the printer, the reader) and the bolts it uses: nothing else is assumed.
#
# The gate's refusal itself has no fixture: provoking it needs a printer that changes a
# reading. Exit code = the number of failed checks.
set -u
cd "$(dirname "$0")/../../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "shardfmt_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"
T=v3/tools/shardfmt
OUT=$(mktemp -d); trap 'rm -rf "$OUT" v3/bin/emit/shardfmt_test v3/bin/shardfmt_test.build.log' EXIT
fail=0; checks=0
bad() { echo "shardfmt_test: $*"; fail=$((fail+1)); }
FMT=$OUT/shardfmt_test

s=$(date +%s)
v3/build.sh $T/shardfmt.shard "$FMT" $T/print $T/reads > "$OUT/build.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the formatter did not build (exit $rc)"; grep -E 'EMIT|FAIL|REFUSE' "$OUT/build.txt" | head -3 | cut -c1-300; echo "shardfmt_test: $checks checks, $fail failed"; exit $fail; }
echo "shardfmt_test: the binary built in $(( $(date +%s) - s )) s ($(sed -n 's/^OK: [^ ]* (\(modules=[0-9]* declarations=[0-9]*\).*/\1/p' "$OUT/build.txt"))"

# ---- (1) the scenarios
n_sc=0; n_ref=0
for in in $T/test/scenarios/*.in; do
  name=$(basename "$in" .in); want=${in%.in}.out
  "$FMT" "$in" > "$OUT/got" 2> "$OUT/err"; rc=$?
  checks=$((checks+1))
  case $name in
    refuse_*)
      n_ref=$((n_ref+1))
      [ "$rc" = 2 ] && cmp -s "$OUT/got" "$want" && [ ! -s "$OUT/err" ] || { bad "scenario $name: exit $rc, expected 2 and the message"; head -2 "$OUT/got" | cut -c1-200; } ;;
    *)
      n_sc=$((n_sc+1))
      [ "$rc" = 0 ] && cmp -s "$OUT/got" "$want" && [ ! -s "$OUT/err" ] || { bad "scenario $name: exit $rc, or the text is not $name.out"; diff "$OUT/got" "$want" | head -6 | cut -c1-200; }
      checks=$((checks+1)); "$FMT" --check "$want" > "$OUT/again"; rc=$?
      [ "$rc" = 0 ] && cmp -s "$OUT/again" "$want" || bad "scenario $name: its text is not its own text (--check exit $rc)" ;;
  esac
done
echo "shardfmt_test: $n_sc scenarios laid out as pinned and fixed points, $n_ref refused as pinned"

# ---- (2) the command line
usage="usage: shardfmt [--check] FILE"
one=$T/test/scenarios/fit.in
u() { local l=$1; shift; checks=$((checks+1)); "$FMT" "$@" > "$OUT/got" 2>/dev/null; local rc=$?; [ "$rc" = 2 ] && [ "$(cat "$OUT/got")" = "$usage" ] || bad "$l: exit $rc, expected 2 and the usage"; }
u "no argument"
u "--check alone" --check
u "two files" "$one" "$one"
u "the flag after the file" "$one" --check
u "three arguments" --check "$one" "$one"
checks=$((checks+1)); "$FMT" "$OUT/no_such_file.shard" > "$OUT/got" 2>/dev/null; rc=$?
[ "$rc" = 2 ] && [ "$(cat "$OUT/got")" = "shardfmt: cannot read file" ] || bad "a missing file: exit $rc, expected 2"
checks=$((checks+1)); "$FMT" --check $T/test/scenarios/collapse.in > "$OUT/got" 2>/dev/null; rc=$?
[ "$rc" = 1 ] && cmp -s "$OUT/got" $T/test/scenarios/collapse.out || bad "--check on a file that is not canonical: exit $rc, expected 1 and the canonical text"
echo "shardfmt_test: the command line — the usage five ways, a missing file, --check's exit 1"

# ---- (3) the driver's run of the program against the binary
for in in $T/test/scenarios/*.in; do case $(basename "$in") in refuse_*) ;; *) cat "$in"; echo;; esac; done > "$OUT/all.shard"
s=$(date +%s)
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT --run tools.shardfmt.shardfmt.main $T/shardfmt.shard $T/print $T/reads -- "$OUT/all.shard" > "$OUT/driver" 2> "$OUT/driver.err"; rc=$?
"$FMT" "$OUT/all.shard" > "$OUT/native"; rn=$?
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$rn" = 0 ] || { bad "the scenarios in one file: the driver exit $rc, the binary exit $rn"; head -2 "$OUT/driver" | cut -c1-200; }
checks=$((checks+1)); [ -s "$OUT/native" ] && cmp -s "$OUT/driver" "$OUT/native" || { bad "the driver's text is not the binary's"; diff "$OUT/driver" "$OUT/native" | head -4 | cut -c1-200; }
echo "shardfmt_test: the program under ev in $(( $(date +%s) - s )) s — the binary's text over the scenarios' $(wc -l < "$OUT/all.shard") lines, byte for byte"

# ---- (4) the tree
cat > "$OUT/comments.py" <<'PY'
import sys
def comments(path):
    s = open(path, 'rb').read(); out = []; i = 0; n = len(s)
    while i < n:
        c = s[i]
        if c == 0x22:                       # a string: to its closing quote, escapes skipped
            i += 1
            while i < n and s[i] != 0x22:
                if s[i] == 0x5c: i += 1
                i += 1
            i += 1
        elif c == 0x3b:                     # a comment: to the end of its line
            j = s.find(b'\n', i); j = n if j < 0 else j
            out.append(s[i:j]); i = j
        else: i += 1
    return out
sys.exit(0 if comments(sys.argv[1]) == comments(sys.argv[2]) else 1)
PY
files=0; same=0; change=0; unread=0; gate=0; other=0; notfix=0; lost=0
while IFS= read -r f; do
  files=$((files+1))
  "$FMT" "$f" > "$OUT/a" 2>/dev/null; rc=$?
  case $rc in
    0) if cmp -s "$OUT/a" "$f"; then same=$((same+1)); else
         change=$((change+1))
         "$FMT" --check "$OUT/a" > /dev/null 2>&1 || { notfix=$((notfix+1)); echo "   not a fixed point: $f"; }
         python3 -I "$OUT/comments.py" "$f" "$OUT/a" || { lost=$((lost+1)); echo "   a comment changed: $f"; }
       fi ;;
    2) unread=$((unread+1)); echo "$f" >> "$OUT/unread" ;;
    3) gate=$((gate+1)); echo "   the gate refused: $f" ;;
    *) other=$((other+1)); echo "   exit $rc: $f" ;;
  esac
done < <(git ls-files 'v3/*.shard' | LC_ALL=C sort)   # the tracked sources: a test's scratch root beside this run is not the tree
checks=$((checks+1)); [ "$gate" = 0 ] || bad "the gate refused $gate files of the tree — a printer defect"
checks=$((checks+1)); [ "$other" = 0 ] || bad "$other files ended on an exit outside 0, 2 and 3"
checks=$((checks+1)); [ "$notfix" = 0 ] || bad "$notfix reformatted files are not fixed points"
checks=$((checks+1)); [ "$lost" = 0 ] || bad "$lost reformatted files changed a comment"
want_unread="v3/pins/loader/read_error/main.shard v3/pins/reader/refuse_unterminated.shard "
checks=$((checks+1)); [ "$(tr '\n' ' ' < "$OUT/unread" 2>/dev/null)" = "$want_unread" ] || { bad "the sources that do not read are not the two pins broken on purpose"; cat "$OUT/unread" 2>/dev/null | head -5; }
echo "shardfmt_test: the tree — $files files: $same canonical, $change that a reformat would change (each a fixed point after it, every comment kept), $unread that do not read (the pins broken on purpose), $gate refused by the gate"

# ---- (5) the contract
s=$(date +%s)
"$EVAL" direct v3/kernel/load.shard --root v3 $INIT $T/shardfmt_bin.shard $T/print $T/reads > "$OUT/bin.txt" 2>&1; rc=$?
checks=$((checks+1)); [ "$rc" = 0 ] || { bad "the contract's load failed (exit $rc)"; grep -E '^(READ-ERROR|LOAD-ERROR|REFUSE|UNMET|UNTHREADED|BAD-BIN|LOAD:)' "$OUT/bin.txt" | head -5 | cut -c1-300; }
B=tools.shardfmt.shardfmt_bin
checks=$((checks+1)); grep -qx "BIN $B.shardfmt entry tools.shardfmt.shardfmt.main externs 4 trusts 9 requires 2/2" "$OUT/bin.txt" || { bad "the bin's record is not BIN … externs 4 trusts 9 requires 2/2"; grep '^BIN' "$OUT/bin.txt" | cut -c1-200; }
checks=$((checks+1)); [ "$(grep -c '^\(UNMET\|UNTHREADED\|BAD-BIN\|PENDING\) ' "$OUT/bin.txt")" = 0 ] || { bad "the contract has failure or pending lines"; grep '^\(UNMET\|UNTHREADED\|BAD-BIN\|PENDING\) ' "$OUT/bin.txt" | head -3 | cut -c1-300; }
for f in refuse run_fmt usage main; do
  checks=$((checks+1)); grep -qx "THREAD tools.shardfmt.shardfmt.$f ok" "$OUT/bin.txt" || bad "shardfmt.$f is not THREAD ok"
done
closure() { sed -nE "s/^ACCEPT $1 .* axioms=([^ ]*) .*/\1/p" "$OUT/bin.txt" | tr ',' '\n' | LC_ALL=C sort | tr '\n' ' '; }
want_gate="tools.shardfmt.print.print tools.shardfmt.reads.reads_same "
want_main="tools.shardfmt.print.print tools.shardfmt.reads.read_error tools.shardfmt.reads.reads_same $B.exit_exit $B.exit_output $B.exit_reads $B.get_args_output $B.get_args_reads $B.read_file_output $B.read_file_reads $B.write_output $B.write_reads "
checks=$((checks+1)); [ "$(closure $B.fmt_gate)" = "$want_gate" ] || { bad "fmt_gate's assumptions are not the printer and the reader alone"; closure $B.fmt_gate; }
checks=$((checks+1)); [ "$(closure $B.fmt_main_contract)" = "$want_main" ] || { bad "fmt_main_contract's assumptions are not the views' three parameters and the nine bolts"; closure $B.fmt_main_contract; }
echo "shardfmt_test: the contract loaded in $(( $(date +%s) - s )) s — BIN $B.shardfmt requires 2/2, each requirement on the views' parameters and its bolts alone"

echo "shardfmt_test: $checks checks, $fail failed"
exit $fail
