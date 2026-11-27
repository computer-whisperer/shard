#!/usr/bin/env bash
# Reader parity over the toolchain: for every toolchain entrypoint — the four drivers, the
# search's driver (v3/search/find.shard), the calc harness, every test — the V3 loader's program (`load.shard --unchecked --dump`,
# kernel/dump.shard) and the Rust bootstrap's reading of the same closure (`eval dump`,
# rust_bootstrap/src/plain/) printed as one canonical text, compared byte for byte. The
# bootstrap reads the toolchain's sources to run them — it is what every other check here
# runs on — and the V3 loader's agreement over those sources is what keeps its reader honest.
# plain_test.sh is the same comparison over ordinary closures.
#
# THE READING. The loader reads each closure --unchecked, a program alone for every `fn`: the
# reading a build compiles (v3/build.sh) and every test runs. The load without the flag — the
# check of the toolchain's sources, and the same comparison for the program it yields — is
# kernel/test/toolchain_check.sh, on the loader's binary: under the bootstrap it is the
# loader interpreted, running the checker over the sources it is written in.
#
# THE TABLE. The toolchain's sources stand on Init's List, Option, Bool and Prod, and the
# bootstrap, which has no K, reads them by a table of facts about Init's constants kept in the
# tree (v3/kernel/init.facts; `eval` takes it where no --facts is named). The loader prints
# that table from K for each closure it loads here (`load.shard --facts`), and the union over
# the closures must be the file, line for line: a fact the file states and K does not, or one
# it lacks, fails the gate. After a change of the toolchain's citations of Init, or of the
# pinned export, the line this test prints says how to write the file again.
#
# The text is a PROJECTION (dump.shard's header): names print as
# their last component and a constructor, a call and an extern print alike,
# so the tie is evidence only where the projection is injective — each
# closure is checked here for one declaration per short name among the
# heads (fn, extern, sig), among the types, among the constructors, and no
# constructor named like a head. A closure that fails the check fails the
# gate before its dumps are compared: byte agreement over an ambiguous
# name would say nothing about which declaration either reader resolved.
#
# The bootstrap links every view it loads to its implementation; the loader links a directory
# it is given as a root, and checks it there. So K's directory is named to the loader beside
# each consumer of K's view, and the two programs hold K's implementation under the view's
# sigs alike.
cd "$(dirname "$0")/../../.."
EVAL=${EVAL:-./rust_bootstrap/target/release/eval}
read EXPORT INDEX < <(v3/kernel/test/init_index.sh) || { echo "parity_test: no Init index (v3/export.sh, kernel/test/init_index.sh)"; exit 1; }
INIT="--init $EXPORT --init-index $INDEX"
TABLE=v3/kernel/init.facts
d=$(mktemp -d); f=$d/k.facts; trap 'rm -rf "$d"' EXIT
JOBS=${V3_PARITY_JOBS:-8}
s=$(date +%s)
# one I ENTRY: the two texts of one closure compared; $d/I.res holds `ok DECLS` or the failure's
# lines, $d/I.facts the loader's table for the closure. The closures are independent, and the
# loader's load of a toolchain root is a minute or more on the bootstrap: JOBS at a time
one() {
  local i=$1 entry=$2 a=$d/$1.a b=$d/$1.b dups kroot
  "$EVAL" dump "$entry" > "$a" 2>&1 || { { echo "parity_test: eval dump failed on $entry"; head -3 "$a"; } > "$d/$i.res"; return; }
  dups=$(awk '
    /^(fn|extern|sig) / { h[$2]++ }
    /^type / { t[$2]++; d=0; tok=""; want=0
      for (i=1;i<=length($0);i++) { c=substr($0,i,1)
        if (c=="(") { d++; if (d==1) { want=1; tok="" } }
        else if (c==")") { if (d==1 && want && tok!="") { k[tok]++; want=0 }; d-- }
        else if (c==" ") { if (d==1 && want && tok!="") { k[tok]++; want=0 } }
        else if (want) tok=tok c } }
    END { for (n in h) if (h[n]>1) printf "head %s x%d; ", n, h[n]
          for (n in t) if (t[n]>1) printf "type %s x%d; ", n, t[n]
          for (n in k) if (k[n]>1) printf "ctor %s x%d; ", n, k[n]
          for (n in k) if (n in h) printf "ctor and head %s; ", n }' "$a")
  [ -z "$dups" ] || { echo "parity_test: $entry: the projection is not injective — $dups" > "$d/$i.res"; return; }
  # K's directory as a second root for K's consumers: the consumer sees the view, the directory's
  # own check links the implementation (env_empty is its). A test inside the directory imports
  # the private files and no view; a closure without K gets no extra root
  if grep -q '^fn env_empty ' "$a" && [ "${entry#v3/kernel/k/}" = "$entry" ]; then kroot=v3/kernel/k; else kroot=""; fi
  "$EVAL" direct v3/kernel/load.shard --root v3 $INIT --unchecked --facts "$d/$i.facts" --dump "$entry" $kroot > "$b" 2>&1 || { { echo "parity_test: load --dump failed on $entry"; head -3 "$b"; } > "$d/$i.res"; return; }
  if cmp -s "$a" "$b"; then echo "ok $(wc -l < "$a")" > "$d/$i.res"
  else { echo "parity_test: $entry differs"; diff "$a" "$b" | head -6; } > "$d/$i.res"; fi
  rm -f "$a" "$b"
}
export -f one; export EVAL INIT d
entries=(v3/kernel/load.shard v3/kernel/t0.shard v3/kernel/prove.shard v3/kernel/initindex.shard v3/search/find.shard v3/kernel/test/calc_harness.shard v3/kernel/test/*_test.shard v3/kernel/k/test/*_test.shard)
for i in "${!entries[@]}"; do echo "$i ${entries[$i]}"; done | xargs -P "$JOBS" -L 1 bash -c 'one "$0" "$1"'
n=${#entries[@]}; decls=0; failed=0; all=$d/all.facts; : > "$all"
for i in "${!entries[@]}"; do
  if read -r ok k < "$d/$i.res" && [ "$ok" = ok ]; then decls=$((decls + k)); cat "$d/$i.facts" >> "$all"
  else cat "$d/$i.res" 2>/dev/null || echo "parity_test: ${entries[$i]}: no result"; failed=$((failed+1)); fi
done
LC_ALL=C sort -u "$all" > "$f"
if [ "$failed" -eq 0 ] && ! cmp -s "$f" "$TABLE"; then
  echo "parity_test: $TABLE is not what K states of Init for the toolchain's closures ($(wc -l < "$TABLE") lines, K's $(wc -l < "$f")):"
  diff "$TABLE" "$f" | head -6
  if [ -n "${V3_FACTS_OUT:-}" ]; then cp "$f" "$V3_FACTS_OUT"; echo "parity_test: K's table written to $V3_FACTS_OUT"; else echo "parity_test: run with V3_FACTS_OUT=FILE to have K's table written, review the difference, and copy it over $TABLE"; fi
  failed=$((failed+1))
fi
e=$(date +%s)
echo "parity_test: $n closures, $decls declarations, $failed differ, in $((e-s)) s"
[ "$failed" -eq 0 ] && echo "parity_test: byte-identical, every closure's projection injective; the table of Init's facts is K's ($(wc -l < "$TABLE") lines)"
exit $failed
