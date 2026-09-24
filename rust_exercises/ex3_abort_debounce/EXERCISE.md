# Exercise 3: Find the bug with a test

**Goal:** the single most valuable habit in this repo. Write the test that
proves a bug exists, watch it fail, then fix the code.

This is a cut-down model of real ground software:
`//ground_software/hardware_control/src/aborts/abort_operator.rs` in trel3.

## The setup

Run the tests. Three pass:

```bash
bazel test //rust_exercises/ex3_abort_debounce:tests
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

- `bazel test //rust_exercises/ex3_abort_debounce:tests` is green with 10
  passing cases: 7 from the three existing tests, 3 from your new one. Before
  your fix it should read 7 passed, 3 failed.
- Your PR has two commits in that order, and the description says what the bug
  was in one sentence.
- Reverting only your one-line fix makes only your new test fail.

## Then go read the real thing

In trel3, open
`ground_software/hardware_control/src/aborts/abort_operator.rs` and find
`test_abort_config_does_not_issue_response_after_discontinuous_min_cycles`.
Same test, against the real expression engine. You now understand production
flight-adjacent code, which is the entire objective of this onboarding.
