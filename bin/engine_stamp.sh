#!/usr/bin/env bash
# Content hash of everything the native engine's behavior depends on:
# the kernel sources it is compiled from plus the compiler chain itself.
# bin/rebuild.sh records this next to the binary; the test scripts compare
# against it to detect a stale binary.
set -u
cd "$(dirname "$0")/.."
# kernel/*.shard, MINUS the generated *.low.shard intermediates the chain writes
# into kernel/ (they match the glob but are derived from eval/check.shard +
# lower.shard, already hashed here; including them makes the stamp depend on
# which target was lowered last).
srcs=$(ls kernel/*.shard | grep -v '\.low\.shard$')
cat $srcs tools/lower/lower.shard tools/codegen/codegen.shard tools/codegen/rt.h \
  | sha256sum | cut -d' ' -f1
