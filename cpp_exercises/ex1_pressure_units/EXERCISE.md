# C++ Exercise 1: cc_library, cc_test, GoogleTest

**About 25 minutes.** Implement two functions and test them with GoogleTest.

```bash
bazel test //cpp_exercises/ex1_pressure_units:tests
```

Two tests already pass. They cover `psi_to_kpa`, which is done. The other two
functions are empty stubs.

[HINTS.md](HINTS.md) explains the C++ and GoogleTest this exercise uses.

## Steps

1. Read `include/pressure_units.h`. In C++ the header is what other code (and
   your tests) can see, so the expected behavior is written down there, not in
   the `.cc`.

2. Implement `kpa_to_psi` in `src/pressure_units.cc`. Reuse `KPA_PER_PSI`
   instead of adding a second constant. Two constants that have to agree are a
   bug waiting to happen.

3. Implement `is_within_tolerance`, including both edge cases the header calls
   out: a negative tolerance, and `expected == 0.0`.

4. Write the tests listed in the `TODO(you)` block in
   `tests/pressure_units_test.cc`.

5. Check formatting and lint, since the build doesn't do it for you:

   ```bash
   ./dev_scripts/format.sh --check cpp_exercises/ex1_pressure_units
   ./dev_scripts/lint_cpp.sh cpp_exercises/ex1_pressure_units/src/pressure_units.cc
   ```

6. Open a PR.

## Done when

- Both functions implemented and tested, edge cases included.
- Float comparisons use `EXPECT_DOUBLE_EQ` or `EXPECT_NEAR`, never `EXPECT_EQ`.
  `0.1 + 0.2 == 0.3` is false, and a test that relies on it is a coin flip.
- `EXPECT_*` by default. `ASSERT_*` only where carrying on would crash, like
  using a null pointer.
- clang-format and clang-tidy are clean on your files.
- The `TODO(you)` comments are gone. They were notes for you, and once the work
  is done they would only confuse the next reader.
- Your PR says whether a value exactly on the tolerance boundary counts as
  inside or outside, and why. Either is fine; we just want to see your reasoning.
