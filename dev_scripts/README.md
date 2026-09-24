# Dev Scripts

Small scripts so you do not have to memorize tool invocations.

| Script | What it does |
| --- | --- |
| `setup.sh` | Run once after cloning. Checks storage rules, checks for bazelisk, fetches toolchains, runs a baseline build and test. |
| `check_storage.sh` | Verifies this checkout is on local encrypted storage and not in a cloud-synced folder. Safe to run any time. |
| `format.sh` | Formats all Rust and C++. `--check` reports without rewriting. |
| `lint_cpp.sh` | Runs clang-tidy over C++ sources with the right compile flags. |

trel3's equivalent directory does considerably more - git submodules, the mold
linker, `rust-project.json` generation for rust-analyzer, pnpm installs, and VS
Code tasks that run some of it automatically. Read
`//dev_scripts/README.md` over there when you get to it.

## Notes

**Rust needs no lint script.** `.bazelrc` attaches the rustfmt and clippy
aspects to every build, so `bazel build` *is* the lint. `format.sh` is how you
fix what it complains about.

**C++ needs both.** Neither clang-format nor clang-tidy is part of the C++
build. See `//cpp_exercises/README.md`.

**These scripts target bash 3.2**, because that is what macOS ships. No
`mapfile`, no associative arrays. If you add to them, keep that constraint or
test on a Mac.
