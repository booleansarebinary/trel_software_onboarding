# C++ Exercise 2: Green does not mean good

**Goal:** see for yourself that a C++ target can pass its build and its tests
while being unmergeable.

Start here:

```bash
bazel test //cpp_exercises/ex2_format_and_tidy:tests
```

It passes. Nothing is tagged `manual`, nothing is skipped, and Bazel has no
complaint. Now run the tools Bazel does not run:

```bash
./dev_scripts/format.sh --check
./dev_scripts/lint_cpp.sh cpp_exercises/ex2_format_and_tidy/src/valve_table.cc
```

Nine formatting violations and eight clang-tidy findings.

Compare that to Rust exercise 4, where the equivalent mess makes `bazel build`
fail outright. Same class of problem, completely different safety net. **In C++,
you and your reviewer are the safety net.**

## Steps

1. Fix the formatting. You may use the tool - formatting is mechanical and
   arguing about it by hand is a waste of everyone's time:

   ```bash
   ./dev_scripts/format.sh
   ```

2. Fix the clang-tidy findings **by hand**. Unlike formatting, these are
   semantic, and each one is worth understanding:

   - `modernize-use-using` - `typedef int ValveId;` should be `using ValveId = int;`
   - `readability-identifier-naming` - `FindByName` should be `find_by_name`,
     `Count` should be `count`, the private member `entries` needs its
     trailing underscore. Naming rules live in `.clang-tidy`; they are
     arbitrary but shared, which is the only property that matters.
   - `modernize-use-nullptr` - `NULL` is a C macro. Use `nullptr`.
   - `readability-braces-around-statements` - the braceless `if` body. This is
     [how goto fail happened](https://www.imperialviolet.org/2014/02/22/applebug.html).
   - `performance-unnecessary-value-param` - `std::string name` by value copies
     the string on every call for no reason. `const std::string&`.

   While you are in there: `for (unsigned long i = 0; ...)` indexing a vector
   should be a range-based `for`. clang-tidy will not tell you that with the
   checks we enable. Your reviewer will.

3. Re-run the tests. Renaming a public method means updating the test file too.
   That is the test suite doing its job.

4. Open a PR.

## Done when

- `./dev_scripts/format.sh --check` is clean.
- `./dev_scripts/lint_cpp.sh` is clean for this package.
- `bazel test //cpp_exercises/ex2_format_and_tidy:tests` still passes.
- You added no `// NOLINT` comments. Suppressing a linter needs a reason in a
  comment, and none of these have one.

## The point

Rust hands you a build that refuses bad code. C++ hands you tools you have to
remember to run. When you get to the C++ repo, find out on day one what its CI
checks and what it does not - and assume the gap is your responsibility.
