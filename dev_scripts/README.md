# Dev Scripts

Small scripts so you don't have to memorize tool commands.

| Script | What it does |
| --- | --- |
| `setup.sh` | Run once after cloning. Checks where the repo lives, checks for bazelisk, downloads the toolchains, and runs a first build and test. |
| `check_storage.sh` | Checks this repo is on an encrypted local disk and not in a cloud-synced folder. Safe to run any time. |
| `format.sh` | Formats all Rust and C++, or only the folders you pass it. `--check` reports without changing anything. |
| `lint_cpp.sh` | Runs clang-tidy on C++ files with the right compile flags. |

## Do I need to run `format.sh` every time?

No for Rust, yes for C++. And VS Code can do both for you.

For **Rust**, `bazel build` already checks formatting and lint, and fails with a
diff if something's off. You can't forget. `format.sh` is just how you fix it
without editing whitespace by hand.

For **C++**, nothing in the build checks anything. Run `format.sh` and
`lint_cpp.sh` yourself, or wait for the `C++ Format & Tidy` CI job to tell you.
`cpp_exercises/README.md` explains why.

**To stop thinking about it:** open the repo in VS Code. It formats both
languages on save, using the same clang-format as CI. Run `setup.sh` once first,
since that's what downloads it.

## Also worth knowing

**Formatting isn't linting.** A formatter only moves whitespace, so your code
means exactly the same thing afterward. That's why it's safe to let a tool do it.
A linter suggests changes to the code itself ("this `&String` should be
`&str`"), and those need a human to decide. So `format.sh` rewrites files, and
clippy findings are left for you.

**These scripts must work on bash 3.2**, because that's what macOS still ships.
No `mapfile`, no associative arrays. If you edit them, test on a Mac.

The main repo's version of this folder does a lot more: git submodules, a faster
linker, rust-analyzer setup, pnpm installs, and VS Code tasks.
