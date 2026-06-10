#!/usr/bin/env bash
# Content hash of everything the native engine's behavior depends on:
# the kernel sources it is compiled from plus the compiler chain itself.
# bin/rebuild.sh records this next to the binary; the test scripts compare
# against it to detect a stale binary.
set -u
cd "$(dirname "$0")/.."
cat kernel/*.shard tools/lower/lower.shard tools/codegen/codegen.shard tools/codegen/rt.h \
  | sha256sum | cut -d' ' -f1
