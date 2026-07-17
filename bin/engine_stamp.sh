#!/usr/bin/env bash
# Content hash of everything the native engine's behavior depends on:
# the kernel sources it is compiled from plus the compiler chain itself.
# bin/rebuild.sh records this next to the binary; the test scripts compare
# against it to detect a stale binary.
set -u
# The hash depends on file ORDER, and both bash's glob expansion and ls
# sort by LC_COLLATE — en_US.UTF-8 orders underscore names differently
# than the C locale, so the same tree stamped differently on this box vs
# the CI container (measured 2026-07-17: 851590118bef vs 87b0a7d42966).
# Pin the locale so the stamp is a function of content alone.
export LC_ALL=C
cd "$(dirname "$0")/.."
# kernel/*.shard, MINUS the generated *.low.shard intermediates the chain writes
# into kernel/ (they match the glob but are derived from eval/check.shard +
# lower.shard, already hashed here; including them makes the stamp depend on
# which target was lowered last).
srcs=$(ls kernel/*.shard | grep -v '\.low\.shard$')
cat $srcs tools/lower/lower.shard tools/codegen/codegen.shard tools/codegen/rt.h \
  | sha256sum | cut -d' ' -f1
