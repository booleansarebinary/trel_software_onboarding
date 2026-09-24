#!/usr/bin/env bash
# Runs clang-tidy over the C++ sources.
#
#   ./dev_scripts/lint_cpp.sh                 # everything under cpp_exercises
#   ./dev_scripts/lint_cpp.sh path/to/file.cc # just these files
#
# Why this needs a script at all: clang-tidy compiles the file to reason about
# it, so it needs the same flags the real build uses - the standard, the
# include paths, and on macOS the SDK sysroot, because the hermetic libc++
# cannot find the system headers on its own. A big repo solves this by
# generating a compile_commands.json. We are small enough to pass the flags by
# hand, and doing it explicitly here shows you what the problem actually is.
#
# clang-tidy is NOT part of `bazel build`. C++ hygiene is enforced by this
# script, by review, and by the cpp-format-tidy CI workflow. That is a real
# difference from Rust, where the build refuses unformatted or lint-dirty code.

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

bazel build //tools:clang_tidy >/dev/null 2>&1
CLANG_TIDY="$(bazel info bazel-bin 2>/dev/null)/tools/clang-tidy"

COMPILE_ARGS=(-std=c++20)

# Every exercise keeps its public headers in its own include/ directory, and
# `includes = ["include"]` in the BUILD file puts them on the compiler's path.
# Mirror that here.
while IFS= read -r include_dir; do
    COMPILE_ARGS+=("-I$include_dir")
done < <(find . -type d -name include -not -path './bazel-*' | sed 's|^\./||' | sort)

if [ "$(uname -s)" = "Darwin" ]; then
    if SDK_PATH="$(xcrun --show-sdk-path 2>/dev/null)" && [ -n "$SDK_PATH" ]; then
        COMPILE_ARGS+=("-isysroot" "$SDK_PATH")
    else
        echo "warning: no macOS SDK found. Run: xcode-select --install" >&2
    fi
fi

FILES=()
if [ "$#" -gt 0 ]; then
    FILES=("$@")
else
    # while-read rather than `mapfile`: macOS ships bash 3.2.
    while IFS= read -r cpp_file; do
        FILES+=("$cpp_file")
    done < <(find cpp_exercises -type f -name '*.cc' -not -path '*/tests/*' | sort)
fi

if [ "${#FILES[@]}" -eq 0 ]; then
    echo "no C++ sources to lint"
    exit 0
fi

echo "==> clang-tidy on ${#FILES[@]} file(s)"
STATUS=0
for file in "${FILES[@]}"; do
    if ! "$CLANG_TIDY" "$file" -- "${COMPILE_ARGS[@]}"; then
        STATUS=1
    fi
done

if [ "$STATUS" -ne 0 ]; then
    echo
    echo "clang-tidy found problems. Most have a suggested fix in the message."
    echo "To apply the mechanical ones: add --fix to the clang-tidy call above."
fi
exit "$STATUS"
