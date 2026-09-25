# C++ Exercise 2: Green does not mean good

**About 20 minutes.** You'll see that C++ code can build and pass every test
and still not be ready to merge.

[HINTS.md](HINTS.md) walks through each fix with an example.

Start here:

```bash
bazel test //cpp_exercises/ex2_format_and_tidy:tests
```

It passes, with no complaints. Now run the tools Bazel doesn't run for you:

```bash
./dev_scripts/format.sh --check cpp_exercises/ex2_format_and_tidy
./dev_scripts/lint_cpp.sh cpp_exercises/ex2_format_and_tidy/src/valve_table.cc
```

Nine formatting violations and eight clang-tidy findings.

In Rust exercise 1, the same kind of mess made `bazel build` fail. In C++,
nothing stops it. **You and your reviewer are the safety net.**

## Steps

1. Fix the formatting with the tool, not by hand:

   ```bash
   ./dev_scripts/format.sh cpp_exercises/ex2_format_and_tidy
   ```

2. Fix the clang-tidy findings **by hand**. These are about the code, not its
   spacing, and each is worth understanding:

   - `modernize-use-using` - `typedef int ValveId;` should be
     `using ValveId = int;`
   - `readability-identifier-naming` - `FindByName` should be `find_by_name`,
     `CountNormallyOpen` should be `count_normally_open`, `Count` should be
     `count`, and the private member `entries` needs its
     trailing underscore. The naming rules live in `.clang-tidy`. They're
     arbitrary, but everyone uses the same ones, which is what matters.
   - `modernize-use-nullptr` - `NULL` is a C macro. Use `nullptr`.
   - `readability-braces-around-statements` - the braceless `if` body. This is
     roughly [how goto fail happened](https://www.imperialviolet.org/2014/02/22/applebug.html).
   - `performance-unnecessary-value-param` - `std::string name` copies the
     string on every call for no reason. Use `const std::string&`.

   While you're there: change `for (unsigned long i = 0; ...)` to a range-based
   `for`. clang-tidy won't flag it with our settings, but a reviewer would.

3. Re-run the tests. Renaming a public method means updating the test file too.

4. Open a PR.

## Done when

- `./dev_scripts/format.sh --check cpp_exercises/ex2_format_and_tidy` is clean.
- `./dev_scripts/lint_cpp.sh` is clean for this package.
- `bazel test //cpp_exercises/ex2_format_and_tidy:tests` still passes.
- No `// NOLINT` comments added. Silencing the linter needs a reason, and none
  of these have one.

## The point

Rust gives you a build that refuses bad code. C++ gives you tools you have to
remember to run. When you get to the C++ repo, find out on day one what its CI
checks and what it doesn't, and cover the gap yourself.
