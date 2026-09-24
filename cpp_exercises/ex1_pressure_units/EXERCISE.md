# C++ Exercise 1: cc_library, cc_test, GoogleTest

**Goal:** implement two functions and test them with GoogleTest.

```bash
bazel test //cpp_exercises/ex1_pressure_units:tests
```

Two tests pass already. They cover `psi_to_kpa`, which is done. The other two
functions are stubs.

## Steps

1. Read `include/pressure_units.h`. In C++ the header is the contract: it is
   what callers see and what your tests can reach. Note that the behavior
   decisions are documented there, not in the `.cc`.

2. Implement `kpa_to_psi` in `src/pressure_units.cc`. Reuse `KPA_PER_PSI` - do
   not introduce a second constant. Two constants that have to agree are one
   constant and one future bug.

3. Implement `is_within_tolerance`, including both edge cases the header calls
   out: a negative tolerance, and `expected == 0.0`.

4. Write the tests listed in the `TODO(you)` block in
   `tests/pressure_units_test.cc`.

5. Check formatting and linting, because nothing in the build will do it for
   you:

   ```bash
   ./dev_scripts/format.sh --check
   ./dev_scripts/lint_cpp.sh cpp_exercises/ex1_pressure_units/src/pressure_units.cc
   ```

6. Open a PR.

## Done when

- Both functions implemented and tested, including the edge cases.
- Float comparisons use `EXPECT_DOUBLE_EQ` or `EXPECT_NEAR`, never `EXPECT_EQ`.
  `0.1 + 0.2 == 0.3` is false, and a test that depends on it is a coin flip.
- `EXPECT_*` where the test can usefully continue, `ASSERT_*` only where
  continuing would dereference a null pointer or similar.
- clang-format and clang-tidy are clean on your files.
- Your PR says whether you made the tolerance boundary inclusive or exclusive,
  and why.
