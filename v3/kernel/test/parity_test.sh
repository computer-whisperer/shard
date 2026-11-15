#!/usr/bin/env bash
# Reader parity over the toolchain: for every toolchain entrypoint — the four drivers, the
# calc harness, every test — the V3 loader's program (`load.shard --dump`, kernel/dump.shard)
# and the Rust bootstrap's reading of the same closure (`eval dump`, rust_bootstrap/src/
# plain/) printed as one canonical text, compared byte for byte. The bootstrap reads the
# toolchain's sources to run them — it is what every other check here runs on — and the V3
# loader's agreement over those sources is what keeps its reader honest. plain_test.sh is the
# same comparison over ordinary closures, which see Init.
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
a=$(mktemp); b=$(mktemp); trap 'rm -f "$a" "$b"' EXIT
n=0; decls=0; failed=0
s=$(date +%s)
for entry in v3/kernel/load.shard v3/kernel/t0.shard v3/kernel/prove.shard v3/kernel/initindex.shard v3/kernel/test/calc_harness.shard v3/kernel/test/*_test.shard v3/kernel/k/test/*_test.shard; do
  n=$((n+1))
  "$EVAL" dump "$entry" > "$a" 2>&1 || { echo "parity_test: eval dump failed on $entry"; head -3 "$a"; failed=$((failed+1)); continue; }
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
  [ -z "$dups" ] || { echo "parity_test: $entry: the projection is not injective — $dups"; failed=$((failed+1)); continue; }
  # K's directory as a second root for K's consumers: the consumer sees the view, the directory's
  # own check links the implementation (env_empty is its). A test inside the directory imports
  # the private files and no view; a closure without K gets no extra root
  if grep -q '^fn env_empty ' "$a" && [ "${entry#v3/kernel/k/}" = "$entry" ]; then kroot=v3/kernel/k; else kroot=""; fi
  "$EVAL" direct v3/kernel/load.shard --root v3 --dump "$entry" $kroot > "$b" 2>&1 || { echo "parity_test: load --dump failed on $entry"; head -3 "$b"; failed=$((failed+1)); continue; }
  if cmp -s "$a" "$b"; then
    decls=$((decls + $(wc -l < "$a")))
  else
    echo "parity_test: $entry differs"; diff "$a" "$b" | head -6; failed=$((failed+1))
  fi
done
e=$(date +%s)
echo "parity_test: $n closures, $decls declarations, $failed differ, in $((e-s)) s"
[ "$failed" -eq 0 ] && echo "parity_test: byte-identical, every closure's projection injective"
exit $failed
