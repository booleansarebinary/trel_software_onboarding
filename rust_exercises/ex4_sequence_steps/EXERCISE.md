# Rust Exercise 4: Step through a sequence

**About 35 minutes. Optional extra practice** - do exercises 1, 2, and 3 first.

A hotfire is mostly a script: open this valve, wait, open those, wait, shut
everything. The ground software runs that script as a *sequence* of steps,
moving forward one control cycle at a time. Here you'll write the function that
moves it forward.

This is a state machine, and in Rust a state machine is usually an enum whose
variants carry data. The logic is short. The point is getting comfortable with
the patterns around it.

[HINTS.md](HINTS.md) explains enums with data, `let ... else`, and what the
borrow checker will and won't let you do here.

## 1. Read the code.

```bash
bazel test //rust_exercises/ex4_sequence_steps:tests
```

One test fails inside `todo!()`. Read `src/lib.rs` top to bottom, especially
the timeline at the top. `start` and `abort` are finished. Read them, since
`tick` works the same way.

## 2. Implement `tick`.

Its doc comment lists the rules. Work through the timeline at the top of the
file by hand before you start: what should `state` be after each cycle?

## 3. Write the tests.

The `TODO(you)` block at the bottom of the file lists them. Then do the sabotage
check it describes.

## 4. Open the PR.

```bash
bazel test //rust_exercises/ex4_sequence_steps:tests
```

## Done when

- All tests pass, including your four new ones.
- A step's commands come out on its first cycle only, and a test proves it.
- Nothing happens after `Aborted`, and a test proves it.
- Every test uses `#[rstest]`, follows the naming rule, and has setup / call /
  assertions separated by blank lines.
- The `TODO(you)` comments and the `let _ = ...` line are gone.

## Why this one matters

The real sequence operator is bigger, but it has the same bones: a list of
steps, a current position, and a `tick` called every control cycle. Off-by-one
errors in "how long have we been in this step" are exactly the kind of bug that
holds a valve open one cycle too long.
