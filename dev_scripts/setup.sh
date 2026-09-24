#!/usr/bin/env bash
# One-time setup for this repo. Idempotent - run it again whenever something
# feels off.
#
# The main repo has a bigger version of this that also handles git submodules,
# the mold linker, and more.

set -euo pipefail

cd "$(dirname "${BASH_SOURCE[0]}")/.."

echo "=== 1/4: where this code lives ==="
./dev_scripts/check_storage.sh
echo

echo "=== 2/4: bazel ==="
if ! command -v bazel >/dev/null 2>&1; then
    cat <<'MSG'
bazel not found.

Install bazelisk, not bazel. Bazelisk reads .bazelversion and runs the exact
Bazel that this repo expects, which is how everyone stays on one version.

  macOS:  brew install bazelisk
  Linux:  see https://github.com/bazelbuild/bazelisk/releases

MSG
    exit 1
fi
echo "  bazel $(bazel --version 2>/dev/null | awk '{print $2}') (pinned: $(cat .bazelversion))"
echo

echo "=== 3/4: fetching toolchains ==="
echo "  First run downloads a Rust toolchain and a full LLVM toolchain."
echo "  Expect several minutes and a few GB. This is the price of a build that"
echo "  behaves the same on your laptop and in CI."
bazel build //tools:clang_format //tools:clang_tidy >/dev/null
echo "  toolchains ready"
echo

echo "=== 4/4: baseline ==="
echo "  Building and testing everything. Exercises you have not done yet are"
echo "  EXPECTED to fail - that is the assignment, not a broken setup."
echo "  Rust exercise 1 fails to BUILD on purpose; that is its first step."
echo
bazel test //... || true
echo
cat <<'MSG'
Setup done.

Next: read README.md end to end, then start with
rust_exercises/ex1_engineering_units/EXERCISE.md.
MSG
