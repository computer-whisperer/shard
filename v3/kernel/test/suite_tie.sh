#!/usr/bin/env bash
# v3/kernel/test/suite_tie.sh A B — the suite's two runs compared entrypoint by entrypoint
# (v3/LANGUAGE.md §8.6 slice 4.1 rule 4). A and B are V3_TEST_OUT directories of `bash v3/test.sh`
# (DIR/NAME.out, .rc, .secs; DIR/engine, DIR/wall), one on the bootstrap — the authority — and one
# on the binaries: the same entrypoints on both sides, each with the same exit status and the same
# output under THE NORMALIZATION, in full: a count of seconds (`N s`), a store's size (`N kB`), a
# scratch path's generated name (mktemp's /tmp/tmp.XXXXXXXXXX, v3/NAME_XXXXXX,
# v3/.cache/init.index.XXXXXX). A verdict, a count of checks or declarations, an assumption
# closure, a fingerprint, a record are never normalized. The seconds of both sides are printed per
# entrypoint and summed — the native run's time is the measurement (rule 7). Exit = the entrypoints
# that differ. Not an entrypoint of v3/test.sh (it compares two of its runs); CI's suite_tie job runs
# it over the v3 and native jobs' kept outputs, and native_tie_test.sh tests it on a fabricated pair.
set -u
a=${1:?suite_tie.sh A B (two V3_TEST_OUT directories)}; b=${2:?suite_tie.sh A B}
norm() { sed -E 's/\b[0-9]+ s\b/N s/g; s/\b[0-9]+ kB\b/N kB/g; s#/tmp/tmp\.[A-Za-z0-9]{10}#/tmp/tmp.X#g; s#\bv3/([a-z_]+)_[A-Za-z0-9]{6}\b#v3/\1_X#g; s#init\.index\.[A-Za-z0-9]{6}#init.index.X#g' "$1"; }
ea=$(cat "$a/engine" 2>/dev/null || echo A); eb=$(cat "$b/engine" 2>/dev/null || echo B)
n=0; differ=0; ta=0; tb=0
for f in "$a"/*.out; do
  [ -e "$f" ] || { echo "suite_tie: no entrypoint outputs under $a"; exit 1; }
  name=$(basename "$f" .out); n=$((n+1))
  if [ ! -e "$b/$name.out" ]; then echo "   $name: only on the $ea side"; differ=$((differ+1)); continue; fi
  ra=$(cat "$a/$name.rc"); rb=$(cat "$b/$name.rc"); sa=$(cat "$a/$name.secs"); sb=$(cat "$b/$name.secs"); ta=$((ta+sa)); tb=$((tb+sb))
  if [ "$ra" != "$rb" ]; then echo "   $name: exit $ra on the $ea, $rb on the $eb"; differ=$((differ+1)); continue; fi
  if cmp -s <(norm "$f") <(norm "$b/$name.out"); then echo "   $name: tie, exit $ra ($ea $sa s, $eb $sb s)"
  else echo "   $name: the output differs ($ea $sa s, $eb $sb s)"; diff <(norm "$f") <(norm "$b/$name.out") | head -4 | cut -c1-200; differ=$((differ+1)); fi
done
for f in "$b"/*.out; do name=$(basename "$f" .out); [ -e "$a/$name.out" ] || { echo "   $name: only on the $eb side"; differ=$((differ+1)); }; done
echo "suite_tie: $n entrypoints, $differ differ; the $ea $ta s of entrypoint time (wall $(cat "$a/wall" 2>/dev/null || echo ?) s), the $eb $tb s (wall $(cat "$b/wall" 2>/dev/null || echo ?) s)"
[ "$differ" -eq 0 ] && echo "suite_tie: byte-identical under both engines on every entrypoint"
exit $differ
