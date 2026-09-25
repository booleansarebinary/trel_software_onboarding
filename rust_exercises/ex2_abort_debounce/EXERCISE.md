# Rust Exercise 2: Find the bug with a test

**About 45 minutes.** This is the important one. The habit: write a test that
proves the bug exists, watch it fail, *then* fix the code.

The code is a cut-down version of real ground software: the abort operator,
which watches for dangerous conditions during a test and puts the valves in a
safe state.

[HINTS.md](HINTS.md) explains the Rust and git this exercise uses, and has a few
nudges toward the bug if you get stuck.

## The setup

Run the tests. All three existing tests pass:

```bash
bazel test //rust_exercises/ex2_abort_debounce:tests
```

The code has a bug anyway.

`min_cycles` should mean the abort condition was true for that many
*consecutive* control cycles. Sensors are noisy, and one bad reading shouldn't
vent the stand. But the existing tests only ever keep the condition always true
or always false, so nothing tests "consecutive" - and the code gets it wrong.

The lesson: **passing tests don't prove code is correct. They only prove the
cases someone thought to test.**

## Steps

**Commit 1 - the failing test.**

Write `test_run_issues_no_outputs_after_discontinuous_min_cycles`, as described
in the `TODO(you)` block at the bottom of `src/lib.rs`. Run it, read the failure,
and make sure you understand what went wrong before changing the code.

Commit that, with the test failing.

**Commit 2 - the fix.**

Fix `AbortOperator::run`. It's a small change. Then run all four tests, not
just yours.

Commit that.

**Then open one PR containing both commits.**

The order matters. Committing the test first proves, right in the PR, that your
test catches the bug. A test written after the fix passes on its first run, so it
proves nothing.

## Done when

- `bazel test //rust_exercises/ex2_abort_debounce:tests` passes with 10 cases:
  7 from the three existing tests and 3 from yours. Before your fix, it should
  say 7 passed, 3 failed.
- Your PR has two commits in that order, and the description says what the bug
  was in a sentence.
- Reverting just your fix makes only your new test fail.
- The `TODO(you)` comment is gone. It was a note for you, and once the work is
  done it would only confuse the next reader.

## Why this one matters

The real ground software has this same abort operator, the same `min_cycles`
logic, and a test with almost the name you just wrote. When you get access to the
main repo, that code won't be a mystery. That's the point of this onboarding.
