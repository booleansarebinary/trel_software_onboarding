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
echo "  First run downloads a Rust toolchain and a full LLVM toolchain: the"
echo "  compilers, linker, and standard library this repo builds with, pinned to"
echo "  one version so everyone gets identical builds."
echo "  Expect several minutes and a few GB. Later runs take seconds."
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

Next: finish reading README.md, then start on
rust_exercises/ex1_engineering_units/EXERCISE.md.

Tip: if you use VS Code, open this folder in it now. The .vscode/settings.json in
this repo formats Rust and C++ on save using the same pinned tools the build and
CI use, so formatting stops being something you have to remember.

Anything confusing or broken here is worth telling us about - the docs are only
good because people said what did not make sense.
MSG
