# Rust Exercise 5: Run the operators

**About 50 minutes. Optional extra practice** - the hardest of the five. Do 1-4 first.

The ground software doesn't have one giant function that knows about sequences
and aborts and everything else. It has a list of *operators*, each doing one
job, and a control loop that runs every operator once per cycle.

Two rules keep that safe, and this exercise is about both:

1. **Safety operators run last**, so when a sequence and an abort set the same
   valve in the same cycle, the abort wins.
2. **One operator failing doesn't stop the rest.** If the sequence hits an
   error, the abort still runs that cycle.

You'll write three pieces: putting operators in the right order, an abort
operator, and the loop that runs them all. The new Rust here is *traits*: how
one `Vec` can hold different kinds of operators.

[HINTS.md](HINTS.md) explains traits, `Box<dyn Trait>`, and the rest.

## 1. Read the code.

```bash
bazel test //rust_exercises/ex5_operator_loop:tests
```

Two tests fail inside `todo!()`, both in `add`. Read `src/lib.rs` top to
bottom. `ScheduledOutputs` is a finished operator, and your model for step 3.

## 2. Implement `ControlLoop::add`.

Keep the operators in run order: `Normal` before `Safety`, and otherwise in the
order they were added. The `Priority` enum and HINTS.md's section on ordering
enums are the big clues. When this works, the first test passes.

## 3. Implement `ThresholdAbort::run`.

Its doc comment lists four rules. Pay attention to the third: once tripped, it
stays tripped.

## 4. Implement `ControlLoop::run_cycle`.

Run every operator in order, keep going past errors, collect them, and move the
cycle counter forward.

## 5. Write the tests.

The `TODO(you)` block at the bottom of the file lists them. Test 5 needs an
operator that always fails, which you'll write yourself, right in the tests
module. Then do the sabotage check.

## 6. Open the PR.

```bash
bazel test //rust_exercises/ex5_operator_loop:tests
```

## Done when

- All tests pass, including your five new ones.
- Tests prove the abort overrides the sequence, and still runs when another
  operator fails.
- The abort stays tripped after the reading drops, and a test proves it.
- Every test uses `#[rstest]`, follows the naming rule, and has setup / call /
  assertions separated by blank lines.
- The `TODO(you)` comments and the `let _ = ...` lines are gone.

## Why this one matters

This is the shape of the real control loop. When you open the main repo, you'll
find a trait that operators implement, a list of them, and a loop that runs
them every cycle - and the reason the abort operator gets the last word.
