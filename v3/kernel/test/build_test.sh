#!/usr/bin/env bash
# v3/kernel/test/build_test.sh — what v3/build.sh says of a build it cannot stamp. One small
# program of the toolchain's profile is built through a stand-in for the C compiler, a shell
# script first on PATH that does one thing as told and then runs the real one:
#   1. an ordinary build                      -> a binary, and the stamp `--stamp` gives now
#   2. the compiler fails                     -> `FAIL: NAME: cc:` with the compiler's first
#                                                error line, and the path of the log
#   3. a source edited during the build       -> the build fails naming the file; no stamp
#   4. the build repeated                     -> a binary of the edited source, stamped
#   5. a file outside the closure edited      -> the build passes: only the stamp's files count
#   6. a source touched, its content the same -> the build passes
# The edit of step 3 happens between the emit and the stamp, where a stamp taken from the
# sources as they are then would mark the edited source as built. Exit = the checks that failed.
set -u
cd "$(dirname "$0")/../../.."
REAL_CC=$(command -v cc) || { echo "build_test: no C compiler"; exit 1; }
SCRATCH=$(mktemp -d -p v3 build_test_XXXXXX)
NAME=$(basename "$SCRATCH")
trap 'rm -rf "$SCRATCH" "v3/bin/$NAME.build.log" "v3/bin/emit/$NAME"' EXIT
fail=0; checks=0
bad() { echo "build_test: $*"; fail=$((fail+1)); }

ROOT=$SCRATCH/p; SRC=$ROOT/kernel/main.shard; OTHER=$ROOT/kernel/other.shard; OUT=$SCRATCH/$NAME
mkdir -p "$ROOT/kernel" "$SCRATCH/stub"
cat > "$SRC" <<'EOF'
(type (List T) (Nil) (Cons T (List T)))
(type Bool (False) (True))
(type World (World Int))
(extern write_line ((bytes (List Int)) (w World)) World)
(fn main ((w World)) World (write_line "version one" w))
EOF
echo ";; a file under the root that the program does not import" > "$OTHER"
cat > "$SCRATCH/stub/cc" <<'EOF'
#!/usr/bin/env bash
# the stand-in compiler. BUILD_TEST_MODE: fail — a compiler's output with its first error early
# and notes after it, and no binary; edit — a line appended to BUILD_TEST_FILE; touch — its
# time changed and nothing else. Then, unless failing, the real compiler on the same arguments
case ${BUILD_TEST_MODE:-} in
  fail)
    echo "main.c:1:1: warning: a warning before the error"
    echo "main.c:3:5: error: the first error"
    for i in 4 5 6 7 8; do echo "main.c:$i:1: note: a note after it"; done
    echo "main.c:9:9: error: a later error"
    exit 1;;
  edit) sed -i 's/version one/version two/' "$BUILD_TEST_FILE"; echo ";; edited" >> "$BUILD_TEST_FILE";;
  touch) touch "$BUILD_TEST_FILE";;
esac
exec "$BUILD_TEST_CC" "$@"
EOF
chmod +x "$SCRATCH/stub/cc"
# build MODE FILE: one build through the stand-in; its output in $SCRATCH/build.txt, its status in rc
build() {
  PATH=$SCRATCH/stub:$PATH BUILD_TEST_CC=$REAL_CC BUILD_TEST_MODE=$1 BUILD_TEST_FILE=${2:-} \
    v3/build.sh "$SRC" "$OUT" --root "$ROOT" > "$SCRATCH/build.txt" 2>&1; rc=$?
}
current() { [ -s "$OUT.stamp" ] && [ "$(cat "$OUT.stamp")" = "$(v3/build.sh --stamp "$NAME")" ]; }

# 1. an ordinary build
build none
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$("$OUT" 2>&1)" = "version one" ] || { bad "an ordinary build did not make the binary (exit $rc)"; head -3 "$SCRATCH/build.txt" | cut -c1-300; }
checks=$((checks+1)); current || bad "an ordinary build's stamp is not the one --stamp gives"

# 2. the compiler fails: its first error and the log's path on the console
build fail
checks=$((checks+1)); [ "$rc" != 0 ] || bad "a failed compile passed"
checks=$((checks+1)); grep -qxF "FAIL: $NAME: cc: main.c:3:5: error: the first error" "$SCRATCH/build.txt" || { bad "a failed compile does not show the compiler's first error"; head -4 "$SCRATCH/build.txt" | cut -c1-300; }
checks=$((checks+1)); grep -qF "v3/bin/$NAME.build.log" "$SCRATCH/build.txt" || bad "a failed compile does not name the log"
checks=$((checks+1)); grep -qF "a later error" "v3/bin/$NAME.build.log" || bad "the log does not hold the compiler's whole output"

# 3. a source edited between the emit and the stamp
build edit "$SRC"
checks=$((checks+1)); [ "$rc" != 0 ] || bad "a build whose source was edited while it ran passed"
checks=$((checks+1)); grep -q "^FAIL: $NAME: changed during the build: .*kernel/main.shard" "$SCRATCH/build.txt" || { bad "the failure does not name the edited source"; head -3 "$SCRATCH/build.txt" | cut -c1-300; }
checks=$((checks+1)); [ ! -e "$OUT.stamp" ] || bad "a stamp stands after a build whose source was edited while it ran"

# 4. the build repeated: the edited source built and stamped
build none
checks=$((checks+1)); [ "$rc" = 0 ] && [ "$("$OUT" 2>&1)" = "version two" ] && current || { bad "the repeated build did not make a stamped binary of the edited source (exit $rc)"; head -3 "$SCRATCH/build.txt" | cut -c1-300; }

# 5. a file under the root and outside the closure edited during the build
build edit "$OTHER"
checks=$((checks+1)); [ "$rc" = 0 ] && current || { bad "an edit of a file outside the closure failed the build (exit $rc)"; head -3 "$SCRATCH/build.txt" | cut -c1-300; }

# 6. a source touched during the build, its content unchanged
build touch "$SRC"
checks=$((checks+1)); [ "$rc" = 0 ] && current || { bad "a source touched and not changed failed the build (exit $rc)"; head -3 "$SCRATCH/build.txt" | cut -c1-300; }

echo "build_test: $checks checks, $fail failed"
exit $fail
