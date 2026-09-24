# C++ Exercises

Two short exercises, about 45 minutes for the pair. Do the Rust ones first - the
testing habits carry forward, the C++ mechanics do not carry backward.

| # | Directory | About | What it teaches |
| --- | --- | --- | --- |
| 1 | `ex1_pressure_units/` | 25 min | `cc_library`, `cc_test`, GoogleTest, float assertions |
| 2 | `ex2_format_and_tidy/` | 20 min | A target that builds green and is still not mergeable |

## Why there is C++ here at all

New TREL software is Rust, Python, or TypeScript. C++ is not on that list, and the
C++ we do have lives in **its own separate repository**.

So this is not a C++ course. It is 45 minutes so that the first time you open that
repo, the build and the tooling are not also new. If you end up writing C++
regularly you will learn it there, on real code.

## The one thing that is genuinely different

In Rust, formatting and linting are part of the build. In C++ they are not, and
that is the difference worth carrying with you.

| | Rust | C++ |
| --- | --- | --- |
| Formatter | `rustfmt`, run by `bazel build` | `clang-format`, a separate tool nothing runs for you |
| Linter | `clippy`, run by `bazel build`, findings are errors | `clang-tidy`, a separate tool nothing runs for you |
| Can you merge unformatted code? | No, the build refuses | Yes, unless review or CI stops you |
| Config | none checked in; rustfmt defaults | `.clang-format` and `.clang-tidy` at the repo root |

You saw this in Rust exercise 1: `bazel build` failed on a formatting diff, and
that was the feature. C++ gives you no such help, so here you run the tools:

```bash
./dev_scripts/format.sh --check   # clang-format, reports and fails, changes nothing
./dev_scripts/format.sh           # clang-format, rewrites in place
./dev_scripts/lint_cpp.sh         # clang-tidy
```

Plus the `C++ Format & Tidy` CI workflow, which checks the C++ files your PR
touched. And if you work in VS Code, `.vscode/settings.json` formats C++ on save
using the same pinned clang-format, which is the easiest way not to think about
it.

## Where the compiler comes from

Bazel downloads an **LLVM toolchain** - version 20.1.7, pinned in `MODULE.bazel`.
A toolchain is the whole set of programs needed to turn C++ source into a running
binary: the compiler (`clang`), the linker, and the standard library that gets
linked in (`libc++`).

The point of downloading one instead of using your system compiler is that
"works on my machine" stops being possible. Your Mac's Apple clang, a teammate's
Ubuntu gcc, and the CI runner would otherwise be three different compilers
producing three different binaries from the same source.

`clang-format` and `clang-tidy` ship inside that same bundle, which is why
`//tools:clang_format` and `//tools:clang_tidy` exist and why the scripts use
them instead of anything you installed yourself. Different major versions of
clang-format format the same file differently, so pinning the formatter keeps
formatting-only diffs out of PRs.

## Two things that will confuse you otherwise

**clang-tidy needs to know your compile flags.** It parses and semantically
analyzes the file, so it needs the language standard, the include paths, and on
macOS the SDK sysroot - the pinned libc++ cannot find your system headers on its
own. Run it without those and you get a hundred errors from inside `<string>` and
none about your actual code. `dev_scripts/lint_cpp.sh` assembles the flags for
you. Big codebases solve this by generating a `compile_commands.json`; we are
small enough to pass them explicitly, and doing it in the open shows you what the
problem actually is.

**Tests are a separate translation unit.** No `mod tests` like Rust, and no
access to private members - you test through the public header, which is a decent
argument for keeping headers small. We use GoogleTest.
