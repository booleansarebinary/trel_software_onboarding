# Exercise 1: Read the code, then test it

**Goal:** get Bazel working, learn what a test looks like here, and write your
first PR.

**You will not write any implementation code.** `EngineeringUnit` is finished.
This exercise is about reading.

## Steps

1. Build it, then test it:

   ```bash
   bazel build //rust_exercises/ex1_engineering_units:engineering_units
   bazel test //rust_exercises/ex1_engineering_units:tests
   ```

   Notice how long the build took the first time and how long it takes the
   second time. That difference is the Bazel cache, and it is most of the
   reason we put up with Bazel.

2. Read `src/lib.rs` top to bottom, including the comments. Read the two
   existing tests twice.

3. Write the four tests described in the `TODO(you)` block at the bottom of the
   file. Run them.

4. Do the sabotage check the TODO describes: break `to_counts` on purpose and
   confirm your tests catch it. If they do not, your tests are not testing.
   Undo the sabotage.

5. Format, then open a PR. The exact git commands are in the "Git And Pull
   Requests" section of the root README.

   ```bash
   ./dev_scripts/format.sh
   bazel test //rust_exercises/ex1_engineering_units:tests
   ```

## Done when

- Four new tests, one per code path of `to_counts`.
- Every test uses `#[rstest]`, is named
  `test_<function>_<behavior>[_when_<condition>]`, and has setup / call /
  assertions separated by blank lines.
- Nothing in `mod tests` is `pub`.
- You can say, out loud, why `to_counts` returns `Option<u16>` rather than
  `u16`.

## Things that will trip you up

- **`bazel build` fails and the error is a diff, not a compile error.** That is
  rustfmt. Run `./dev_scripts/format.sh`.
- **`bazel build` fails with a clippy suggestion.** Clippy findings are hard
  errors here, not warnings. Fix them; the message almost always contains the
  fix.
- **You changed a file and Bazel says nothing changed.** You are probably
  building a different target than you think. Check the label.
