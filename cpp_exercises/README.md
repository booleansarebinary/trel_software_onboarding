# C++ Exercises

Two short exercises, about 45 minutes for the pair. Do the Rust ones first.

| # | Directory | About | What it teaches |
| --- | --- | --- | --- |
| 1 | `ex1_pressure_units/` | 25 min | `cc_library`, `cc_test`, GoogleTest, float assertions |
| 2 | `ex2_format_and_tidy/` | 20 min | A target that builds green and is still not mergeable |

Each exercise has a `HINTS.md` that explains the C++ it uses.

## Why there is C++ here at all

New TREL software is Rust, Python, or TypeScript. The C++ we do have lives in
**its own separate repository**.

So this isn't a C++ course. It's 45 minutes so that when you first open that
repo, the build and the tools aren't new too. If you end up writing C++
regularly, you'll learn it there.

## The one thing that is genuinely different

In Rust, formatting and linting are part of the build. In C++ they aren't.

| | Rust | C++ |
| --- | --- | --- |
| Formatter | `rustfmt`, run by `bazel build` | `clang-format`, a separate tool nothing runs for you |
| Linter | `clippy`, run by `bazel build`, findings are errors | `clang-tidy`, a separate tool nothing runs for you |
| Can you merge unformatted code? | No, the build refuses | Yes, unless review or CI stops you |
| Config | none checked in; rustfmt defaults | `.clang-format` and `.clang-tidy` at the repo root |

In Rust exercise 1, `bazel build` failed on a formatting diff. C++ won't do that
for you, so you run the tools yourself:

```bash
./dev_scripts/format.sh --check <folder>   # clang-format, reports and fails, changes nothing
./dev_scripts/format.sh <folder>           # clang-format, rewrites in place
./dev_scripts/lint_cpp.sh [file.cc]        # clang-tidy, on everything or just the files you name
```

The `C++ Format & Tidy` CI job also checks the C++ files your PR touches. And in
VS Code, C++ formats on save using the same clang-format, which is the easiest
way not to think about it.

## Where the compiler comes from

Bazel downloads an **LLVM toolchain** (version 20.1.7, pinned in `MODULE.bazel`).
A toolchain is everything needed to turn C++ into a running program: the compiler
(`clang`), the linker, and the standard library (`libc++`).

Downloading one instead of using your own compiler means everyone, including CI,
builds with the exact same one. No more "works on my machine."

`clang-format` and `clang-tidy` come in that same download. That's why the
scripts use `//tools:clang_format` and `//tools:clang_tidy` instead of anything
you installed yourself: different versions format the same file differently.

## Two things that will confuse you otherwise

**clang-tidy needs your compile flags.** It actually reads and understands the
code, so it needs to know the C++ version, where the headers are, and (on macOS)
where the system SDK is. Without those you get a hundred errors from inside
`<string>` and none about your code. `dev_scripts/lint_cpp.sh` passes all of that
for you.

**Tests are a separate file.** There's no `mod tests` like in Rust, and tests
can't see private members. You test through the public header. We use
GoogleTest.
