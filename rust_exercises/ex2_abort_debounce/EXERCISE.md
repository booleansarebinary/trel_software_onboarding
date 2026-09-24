# Rust Exercise 2: Find the bug with a test

**About 45 minutes.**

**Goal:** the single most valuable habit in this repo. Write the test that
proves a bug exists, watch it fail, then fix the code.

This is a cut-down model of real ground software - the abort operator that
watches for redline conditions during a test and drives valves to a safe state.

## The setup

Run the tests. Three pass:

```bash
bazel test //rust_exercises/ex2_abort_debounce:tests
```

The code has a bug anyway. `min_cycles` is supposed to mean the abort condition
held for that many *consecutive* control cycles. The existing tests only ever
hold the condition continuously true or continuously false, so they never check
the word "consecutive" - and the implementation does not honor it.

This is the failure mode worth internalizing: **green tests are not evidence of
correct code. They are evidence about the cases someone thought to write.** The
happy paths here are covered. The feature does not work.

## Steps

**Commit 1 - the failing test.**

Write `test_run_issues_no_outputs_after_discontinuous_min_cycles` as described
in the `TODO(you)` block at the bottom of `src/lib.rs`. Run it. Read the
failure. Make sure you understand what the code did before you touch it.

Commit that, with the test failing.

**Commit 2 - the fix.**

Fix `AbortOperator::run`. It is a small change. Run the whole suite - all four
tests, not just yours.

Commit that.

**Then open one PR containing both commits.**

The two-commit order is the point of the exercise. It puts proof in the diff
that your test catches the bug. A test written after the fix passes on the
first run and demonstrates nothing at all - not to your reviewer, and not to
you.

## Done when

- `bazel test //rust_exercises/ex2_abort_debounce:tests` is green with 10
  passing cases: 7 from the three existing tests, 3 from your new one. Before
  your fix it should read 7 passed, 3 failed.
- Your PR has two commits in that order, and the description says what the bug
  was in one sentence.
- Reverting only your one-line fix makes only your new test fail.

## Why this one matters

The real ground software has this same abort operator, this same `min_cycles`
debounce, and a test with very nearly the name you just wrote - against a full
expression engine instead of two hardcoded comparisons. When you get access to
the main repo, that code will not be a mystery. That is the whole objective of
this onboarding.
