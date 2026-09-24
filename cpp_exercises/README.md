# C++ Exercises

## Why there is C++ here at all

New TREL software is Rust, Python, or TypeScript. C++ is not on that list, and
the C++ that exists lives in **its own separate repository**.

You are doing these two exercises so that the first time you open that repo, the
mechanics are not new. Two short exercises, not a language course. If you end up
writing C++ regularly, you will learn it there, on real code.

## The one thing that is genuinely different: hygiene is not in the build

This is the difference worth carrying with you.

| | Rust | C++ |
| --- | --- | --- |
| Formatter | `rustfmt`, run by `bazel build` via a `.bazelrc` aspect | `clang-format`, a separate tool nothing runs for you |
| Linter | `clippy`, run by `bazel build`, findings are hard errors | `clang-tidy`, a separate tool nothing runs for you |
| Can you merge unformatted code? | No. The build refuses. | Yes, unless review or CI stops you. |
| Config | none checked in; rustfmt defaults | `.clang-format` and `.clang-tidy` at the repo root |

In Rust, `bazel build` failing on a formatting diff is not an inconvenience, it
is the feature - you saw that in Rust exercise 1. In C++ you get no such help:

```bash
./dev_scripts/format.sh --check   # clang-format, reports and fails
./dev_scripts/format.sh           # clang-format, rewrites in place
./dev_scripts/lint_cpp.sh         # clang-tidy
```

Plus the `C++ Format & Tidy` CI workflow, which checks the C++ files your PR
changed.

### Administrative details that will confuse you otherwise

**The compiler is not yours.** `MODULE.bazel` downloads LLVM 20.1.7 and Bazel
builds with that. Your system clang is irrelevant, and so is your system
clang-format - which matters, because clang-format 18 and clang-format 20
disagree about real code. `//tools:clang_format` and `//tools:clang_tidy` hand
you the pinned binaries out of that same toolchain, so you, your reviewer, and
CI produce identical output.

**clang-tidy needs to know your compile flags.** It parses and semantically
analyzes the file, so it needs the standard, the include paths, and on macOS the
SDK sysroot - the hermetic libc++ cannot find the system headers by itself. Run
it without those and you get a hundred errors from inside `<string>` and none
about your code. `dev_scripts/lint_cpp.sh` assembles the flags for you. Large
codebases solve this by generating a `compile_commands.json`; we are small
enough to pass them explicitly, and doing it in the open shows you what is
actually going on.

**Bazel 8 wants explicit rule loads.** `load("@rules_cc//cc:defs.bzl",
"cc_library", "cc_test")` at the top of the `BUILD.bazel`. Older files elsewhere
may still rely on the legacy native globals.

**Tests are a separate translation unit.** No `mod tests`, no access to private
members. You test through the public header, which is a decent argument for
keeping headers small. We use GoogleTest.

## The exercises

| # | Directory | About | What it teaches |
| --- | --- | --- | --- |
| 1 | `ex1_pressure_units/` | 25 min | `cc_library`, `cc_test`, GoogleTest, float assertions |
| 2 | `ex2_format_and_tidy/` | 20 min | A target that builds green and is still unacceptable |

Do both Rust exercises first. The testing habits transfer; the C++ mechanics do
not transfer backwards.
