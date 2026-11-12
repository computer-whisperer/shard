#!/usr/bin/env bash
# bin/ci_docs_only.sh — did the push this pipeline runs for change documentation only?
#
#   bin/ci_docs_only.sh               exit 0: the push changed files and every one of them ends
#                                     in `.md` — the gate jobs step aside (.gitlab-ci.yml:
#                                     corpus, v3, native, suite_tie); exit 1: anything else, and
#                                     every doubt
#   bin/ci_docs_only.sh --self-test   the cases at the foot of this file on a scratch repository
#                                     (the engine job runs it first, on the runner's own git);
#                                     exit = the cases that failed
#
# The push is $CI_COMMIT_BEFORE_SHA..$CI_COMMIT_SHA — the branch's head before the push and
# after it — so a code commit pushed together with the commit that records it is one push and is
# gated; a rename is read as a removal and an addition. What makes stepping aside sound: no gate
# reads a `.md` file (checked 2026-11-11 over run_corpus.sh, the v3 scripts and the shard sources;
# a gate that starts to read one must name that file here as code).
#
# The answer is "no" whenever it cannot be had: not a push pipeline (a manual, API or scheduled
# run — `glab ci run -b main` is how a full run is forced on a documentation head); no previous
# head (a new branch); a previous head the clone lacks (GitLab fetches 20 commits); a diff that
# fails; a push that changed no file. And the caller's line is
#   if bin/ci_docs_only.sh; then …; exit 0; fi
# so a script that is missing or broken is a "no" as well: nothing here can stop a gate from
# running, only let one step aside.
set -u
zero=0000000000000000000000000000000000000000

# docs_only REPO BEFORE HEAD — 0 when BEFORE..HEAD changed at least one file and only `.md` files
docs_only() {
  local repo=$1 before=$2 head=$3 n=0 f list
  case $before in ''|"$zero") echo "docs-only: no — the push has no previous head"; return 1;; esac
  [ -n "$head" ] || { echo "docs-only: no — no commit named"; return 1; }
  git -C "$repo" cat-file -e "$before^{commit}" 2>/dev/null \
    || { echo "docs-only: no — the previous head ${before:0:12} is not in this clone"; return 1; }
  list=$(mktemp) || return 1
  git -C "$repo" diff --no-renames --name-only -z "$before" "$head" > "$list" 2>/dev/null \
    || { rm -f "$list"; echo "docs-only: no — the diff ${before:0:12}..${head:0:12} failed"; return 1; }
  while IFS= read -r -d '' f; do
    n=$((n+1))
    case $f in
      *.md) ;;
      *) rm -f "$list"; echo "docs-only: no — the push changed $f"; return 1;;
    esac
  done < "$list"
  rm -f "$list"
  [ "$n" -gt 0 ] || { echo "docs-only: no — the push changed no file"; return 1; }
  echo "docs-only: yes — the push ${before:0:12}..${head:0:12} changed $n file(s), every one a .md"
  return 0
}

self_test() {
  local d fails=0 c0 c1 c2 c3 c4 c5
  d=$(mktemp -d) || return 1
  g() { git -C "$d" -c user.name=ci -c user.email=ci@invalid -c commit.gpgsign=false "$@" > /dev/null 2>&1; }
  rev() { git -C "$d" rev-parse HEAD; }
  expect() {   # WANT LABEL BEFORE HEAD — WANT 0 (documentation only) or 1
    local got=0
    docs_only "$d" "$3" "$4" > /dev/null 2>&1 || got=1
    [ "$got" = "$1" ] || { echo "ci_docs_only self-test: FAIL: $2 (want $1, got $got)"; fails=$((fails+1)); }
  }
  g init -q; mkdir -p "$d/docs"
  echo a > "$d/a.shard"; echo r > "$d/README.md"; g add a.shard README.md; g commit -q -m c0; c0=$(rev)
  echo r2 >> "$d/README.md"; echo n > "$d/docs/N.md"; g add README.md docs/N.md; g commit -q -m c1; c1=$(rev)
  echo a2 >> "$d/a.shard"; g add a.shard; g commit -q -m c2; c2=$(rev)
  g mv a.shard a.md; g commit -q -m c3; c3=$(rev)
  g rm -q docs/N.md; echo x > "$d/notes.md"; g add notes.md; g commit -q -m c4; c4=$(rev)
  echo t > "$d/a.md.txt"; g add a.md.txt; g commit -q -m c5; c5=$(rev)
  [ -n "$c5" ] && [ "$c0" != "$c5" ] || { echo "ci_docs_only self-test: FAIL: the scratch repository was not built"; rm -rf "$d"; return 1; }
  expect 0 "a push of documentation alone" "$c0" "$c1"
  expect 1 "a push of code" "$c1" "$c2"
  expect 1 "documentation and code in one push" "$c0" "$c2"
  expect 1 "a file renamed from code to .md" "$c2" "$c3"
  expect 0 "documentation removed and added" "$c3" "$c4"
  expect 1 "a name that contains .md and does not end in it" "$c4" "$c5"
  expect 1 "a push that changed no file" "$c4" "$c4"
  expect 1 "no previous head (zeros)" "$zero" "$c4"
  expect 1 "no previous head (empty)" "" "$c4"
  expect 1 "a previous head the clone lacks" 1234567890123456789012345678901234567890 "$c4"
  rm -rf "$d"
  echo "ci_docs_only self-test: 10 cases, $fails failed"
  return "$fails"
}

case ${1:-} in
  --self-test) self_test; exit $?;;
  '') ;;
  *) echo "bin/ci_docs_only.sh [--self-test]"; exit 2;;
esac
[ "${CI_PIPELINE_SOURCE:-}" = push ] \
  || { echo "docs-only: no — not a push pipeline (source: ${CI_PIPELINE_SOURCE:-none})"; exit 1; }
cd "$(dirname "$0")/.."
docs_only . "${CI_COMMIT_BEFORE_SHA:-}" "${CI_COMMIT_SHA:-}"
