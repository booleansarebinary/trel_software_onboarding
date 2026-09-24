# Rust Exercise 1: Build it, clean it, implement it, test it

**About 45 minutes.** Everything you need for your first PR: get Bazel working,
meet the formatter and the linter, write a little Rust, and write tests.

## 1. Try to build it. It fails.

```bash
bazel build //rust_exercises/ex1_engineering_units:engineering_units
```

The error is a **diff**, not a compile error. That is `rustfmt`: our `.bazelrc`
wires the formatter into every build, so badly formatted Rust does not compile
here. Fix it with the formatter rather than by hand:

```bash
./dev_scripts/format.sh
```

## 2. Build again. Now the linter complains.

Three findings from `clippy`, all in `channel_label`. Clippy findings are hard
errors in this repo, not warnings. Fix them **by hand**, reading each message -
the point is that you recognize these patterns in your own code later:

- **`ptr_arg`** - `&String` where `&str` would do. The function only reads the
  string, so taking `&String` forces every caller to own a `String` first. This
  is a habit people carry for years; break it now.
- **`len_zero`** - `.len() == 0` should be `.is_empty()`.
- **`needless_return`** - the last expression in a Rust function is its return
  value. `return` on that line is noise.

Each message links to an explanation. Read at least one of them.

Do not add `#[allow(...)]`. Silencing a linter needs a reason in a comment, and
none of these have one.

## 3. Read the code.

`src/lib.rs`, top to bottom, comments included. Read the three existing tests
twice - they are the style you are held to.

Then build and run the tests to see where you stand:

```bash
bazel test //rust_exercises/ex1_engineering_units:tests
```

## 4. Implement `to_counts`.

Its doc comment lists four behaviors. Get all four. `to_counts` returns
`Option<u16>` rather than `u16` - make sure you understand why before you start.

Delete the `let _ = (value, MIN_COUNTS, MAX_COUNTS);` line when you do.

## 5. Write the tests.

The `TODO(you)` block at the bottom of the file lists them: one test per code
path of `to_counts`, plus one for `channel_label` with a non-empty prefix.

Then do the sabotage check the TODO describes - break your own code on purpose
and confirm your tests notice. If they do not, they are decoration.

## 6. Open the PR.

```bash
./dev_scripts/format.sh
bazel test //rust_exercises/ex1_engineering_units:tests
```

Then follow the loop in the "Git And Pull Requests" section of the root README.

## Done when

- `bazel build //...` and `bazel test //...` pass for this package.
- `to_counts` handles all four cases, with one test per case.
- Every test uses `#[rstest]`, is named
  `test_<function>_<behavior>[_when_<condition>]`, and has setup / call /
  assertions separated by blank lines.
- Nothing in `mod tests` is `pub`.
- No `#[allow(...)]` anywhere.

## Things that trip people up

- **A build error that is a diff** is the formatter. `./dev_scripts/format.sh`.
- **A build error suggesting better code** is clippy. Fix it; the message
  usually contains the fix.
- **You changed a file and Bazel says nothing changed.** You are probably
  building a different target than you think. Check the label.
- **Your test will not re-run.** Bazel cached the result. Add
  `--nocache_test_results`.
