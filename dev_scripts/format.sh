#!/usr/bin/env bash
# Formats Rust and C++ files, or checks them with --check.
#
#   ./dev_scripts/format.sh                   # rewrite every file in the repo
#   ./dev_scripts/format.sh --check           # report and exit non-zero, changing nothing
#   ./dev_scripts/format.sh cpp_exercises/ex1_pressure_units
#   ./dev_scripts/format.sh --check rust_exercises/ex1_engineering_units
#
# Pass one or more directories to touch only those. Do this when you are working
# on an exercise: some exercises are badly formatted on purpose, and formatting
# the whole repo would quietly do another exercise's first step for you.
#
# Both formatters come from Bazel, so your results match CI's exactly.

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

CHECK_ONLY=0
if [ "${1:-}" = "--check" ]; then
    CHECK_ONLY=1
    shift
fi

# Remaining arguments are directories. Strip trailing slashes so tab completion
# works, and turn each into a Bazel target pattern for the Rust half.
DIRS=()
RUST_TARGETS=()
for dir in "$@"; do
    dir="${dir%/}"
    if [ ! -d "$dir" ]; then
        echo "format.sh: not a directory: $dir" >&2
        exit 1
    fi
    DIRS+=("$dir")
    RUST_TARGETS+=("//$dir/...")
done
if [ "${#DIRS[@]}" -eq 0 ]; then
    DIRS=(.)
    RUST_TARGETS=(//...)
fi

# --- Rust -----------------------------------------------------------------
# rustfmt is also enforced by the build itself: `bazel build` runs the
# rustfmt_aspect from .bazelrc and fails on unformatted code. This target is
# how you FIX what the build complains about.
echo "==> rustfmt"
if [ "$CHECK_ONLY" -eq 1 ]; then
    # The build IS the check. Note that `bazel build //...` skips targets
    # tagged `manual`, which is deliberate - see rust_exercises/ex4.
    bazel build "${RUST_TARGETS[@]}" >/dev/null
    echo "    rust: formatted"
else
    bazel run @rules_rust//tools/rustfmt -- "${RUST_TARGETS[@]}" 2>/dev/null
    echo "    rust: formatted in place"
fi

# --- C++ ------------------------------------------------------------------
echo "==> clang-format"
bazel build //tools:clang_format >/dev/null 2>&1
CLANG_FORMAT="$(bazel info bazel-bin 2>/dev/null)/tools/clang-format"

# Built with a while-read loop rather than `mapfile`, because macOS still
# ships bash 3.2 and mapfile arrived in bash 4. The bazel-* symlinks point into
# Bazel's output tree, which holds third-party headers we must not touch.
CPP_FILES=()
while IFS= read -r cpp_file; do
    CPP_FILES+=("$cpp_file")
done < <(find "${DIRS[@]}" -name 'bazel-*' -prune -o \
    -type f \( -name '*.cc' -o -name '*.h' \) -print | sort)

if [ "${#CPP_FILES[@]}" -eq 0 ]; then
    echo "    c++: no files found"
    exit 0
fi

if [ "$CHECK_ONLY" -eq 1 ]; then
    "$CLANG_FORMAT" --dry-run --Werror "${CPP_FILES[@]}"
    echo "    c++: formatted"
else
    "$CLANG_FORMAT" -i "${CPP_FILES[@]}"
    echo "    c++: formatted ${#CPP_FILES[@]} file(s) in place"
fi
