# Dev Scripts

Small scripts so you don't have to memorize tool invocations.

| Script | What it does |
| --- | --- |
| `setup.sh` | Run once after cloning. Checks where the repo lives, checks for bazelisk, downloads the toolchains, runs a baseline build and test. |
| `check_storage.sh` | Verifies this checkout is on local encrypted storage and not in a cloud-synced folder. Safe to run any time. |
| `format.sh` | Formats all Rust and C++. `--check` reports without rewriting. |
| `lint_cpp.sh` | Runs clang-tidy over the C++ sources with the right compile flags. |

## Do I need to run `format.sh` every time?

Short answer: no for Rust, yes for C++ - and you can make it automatic for both.

It helps to separate two different jobs:

- **Checking** - noticing that something is wrong.
- **Fixing** - making it right.

For **Rust**, checking is already automatic. `.bazelrc` attaches the `rustfmt`
and `clippy` aspects to every build, so `bazel build` *is* the lint - it fails
and shows you the diff. You cannot forget, because the build won't let you.
`format.sh` is just the fixer you run when it complains, so you don't fix
whitespace by hand.

For **C++**, nothing in the build checks anything. `clang-format` and
`clang-tidy` are separate programs, and Bazel does not invoke them. So there you
do need to run `format.sh` and `lint_cpp.sh` yourself, or wait for the
`C++ Format & Tidy` CI job to tell you. `cpp_exercises/README.md` explains why.

**To stop thinking about it:** open the repo in VS Code. `.vscode/settings.json`
turns on format-on-save for both languages, pointing at the same pinned
`clang-format` the build and CI use. Run `setup.sh` once first, since that is what
downloads the binary.

## Also worth knowing

**Formatting is not the same as linting.** A formatter only moves whitespace
around - the meaning of your code is identical afterward, which is why it is safe
to let a tool rewrite your file. A linter makes claims about the code itself
("this `&String` should be `&str`"), and applying those is a judgment call. That
is why `format.sh` rewrites files and clippy findings are left for you.

**These scripts target bash 3.2**, because that is what macOS still ships. No
`mapfile`, no associative arrays. If you add to them, keep that constraint or test
on a Mac.

The main repo's equivalent directory does considerably more - git submodules, a
faster linker, `rust-project.json` generation for rust-analyzer, pnpm installs,
and VS Code tasks that run some of it automatically.
