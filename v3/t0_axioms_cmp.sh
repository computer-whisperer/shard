#!/usr/bin/env bash
# v3/t0_axioms_cmp.sh K_LOG ORACLE — T0's axiom-closure comparison (FOUNDATION
# §3.6: identical verdicts AND axiom closures). K_LOG holds the driver's `-a`
# output (`AX name: axiom …` per admitted constant); ORACLE is v3/axioms.lean's
# output for the same export. Both sides are normalized to one line per
# constant with the axioms sorted, joined by name; every K constant must be in
# the oracle with the same closure. Exit 0 = identical; the differences (up to
# 20) and the counts are printed either way.
set -u
klog=$1; oracle=$2
# POSIX awk only (CI's awk is mawk: no asort): the axioms are sorted by insertion.
# The field separator is a TAB — names may contain colons (`Array.term__[:_]::_]`).
T=$(printf '\t')
norm() { grep '^AX ' "$1" | awk -v T="$T" '{ name=$2; sub(/:$/,"",name); n=0; for (i=3;i<=NF;i++) a[n++]=$i
  for (i=1;i<n;i++) { v=a[i]; j=i-1; while (j>=0 && a[j] > v) { a[j+1]=a[j]; j-- } a[j+1]=v }
  s=""; for (i=0;i<n;i++) s=s " " a[i]; print name T s }' | LC_ALL=C sort -t "$T" -k1,1 | uniq; }
k=$(mktemp); o=$(mktemp); trap 'rm -f "$k" "$o"' EXIT
norm "$klog" > "$k"; norm "$oracle" > "$o"
nk=$(wc -l < "$k"); no=$(wc -l < "$o")
missing=$(LC_ALL=C join -t "$T" -v1 "$k" "$o")
differ=$(LC_ALL=C join -t "$T" "$k" "$o" | awk -F "$T" '$2 != $3')
nm=$(printf '%s' "$missing" | grep -c .); nd=$(printf '%s' "$differ" | grep -c .)
echo "axiom closures: K $nk constants, oracle $no; $nd differ, $nm missing from the oracle"
[ "$nd" -gt 0 ] && { echo "-- differ (K vs oracle):"; printf '%s\n' "$differ" | head -20; }
[ "$nm" -gt 0 ] && { echo "-- missing from the oracle:"; printf '%s\n' "$missing" | head -20; }
[ "$nk" -gt 0 ] && [ "$nd" -eq 0 ] && [ "$nm" -eq 0 ]
