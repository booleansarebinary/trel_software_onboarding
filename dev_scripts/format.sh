#!/usr/bin/env bash
# Formats every Rust and C++ file in the repo, or checks them with --check.
#
#   ./dev_scripts/format.sh           # rewrite files in place
#   ./dev_scripts/format.sh --check   # report and exit non-zero, changing nothing
#
# Both formatters come from Bazel, so your results match CI's exactly.

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

CHECK_ONLY=0
if [ "${1:-}" = "--check" ]; then
    CHECK_ONLY=1
fi

# --- Rust -----------------------------------------------------------------
# rustfmt is also enforced by the build itself: `bazel build` runs the
# rustfmt_aspect from .bazelrc and fails on unformatted code. This target is
# how you FIX what the build complains about.
echo "==> rustfmt"
if [ "$CHECK_ONLY" -eq 1 ]; then
    # The build IS the check. Note that `bazel build //...` skips targets
    # tagged `manual`, which is deliberate - see rust_exercises/ex4.
    bazel build //... >/dev/null
    echo "    rust: formatted"
else
    bazel run @rules_rust//tools/rustfmt 2>/dev/null
    echo "    rust: formatted in place"
fi

# --- C++ ------------------------------------------------------------------
echo "==> clang-format"
bazel build //tools:clang_format >/dev/null 2>&1
CLANG_FORMAT="$(bazel info bazel-bin 2>/dev/null)/tools/clang-format"

# Built with a while-read loop rather than `mapfile`, because macOS still
# ships bash 3.2 and mapfile arrived in bash 4.
CPP_FILES=()
while IFS= read -r cpp_file; do
    CPP_FILES+=("$cpp_file")
done < <(find cpp_exercises -type f \( -name '*.cc' -o -name '*.h' \) | sort)

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
