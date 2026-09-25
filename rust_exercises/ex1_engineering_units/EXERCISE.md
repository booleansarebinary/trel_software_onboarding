# Rust Exercise 1: Build it, clean it, implement it, test it

**About 45 minutes.** This walks you through everything for your first PR:
running Bazel, the formatter and linter, a little Rust, and tests.

Try it without AI if you can. The errors in steps 1 and 2 are the point, and
they're more useful when you read them yourself. Stuck? Ask us.

New to Rust? [HINTS.md](HINTS.md) explains every piece of syntax this exercise
uses - `mut`, `Some`, `as u16`, `.clamp()`, `push_str`, the leading `_`, and more.

## 1. Try to build it. It fails on purpose.

```bash
bazel build //rust_exercises/ex1_engineering_units:engineering_units
```

The error is a **diff**, not a compile error. That's `rustfmt`, the formatter.
It runs on every build here, so badly formatted Rust won't build and nobody can
forget to format. Fix it with the tool, not by hand:

```bash
./dev_scripts/format.sh rust_exercises/ex1_engineering_units
```

The folder at the end keeps it to this exercise. Without it, `format.sh` would
also fix C++ exercise 2, which is messy on purpose for later.

(In VS Code, after running setup, files also format when you save.)

## 2. Build again. Now the linter has opinions.

Three errors from `clippy`, the linter, all in `channel_label`. Here they're
errors, not warnings. Fix them by hand, and read the messages - each one tells
you the fix:

- **`ptr_arg`** - `&String` where `&str` would do. The function only reads the
  string, so `&str` lets callers pass any text, not just a `String`.
- **`len_zero`** - `.len() == 0` should be `.is_empty()`.
- **`needless_return`** - the last line of a Rust function *is* its return
  value, so `return` there isn't needed.

Each message links to an explanation. Read at least one.

Please don't use `#[allow(...)]` to silence these. That's sometimes fine, but it
needs a reason in a comment, and none of these have one.

## 3. Read the code.

`src/lib.rs`, top to bottom, comments included. Read the existing tests
closely - your tests should look like them.

Then see where you stand:

```bash
bazel test //rust_exercises/ex1_engineering_units:tests
```

One test fails, inside `todo!()`. That's your next job.

## 4. Implement `to_counts`.

Its doc comment lists four behaviors - get all four. It returns `Option<u16>`,
not `u16`. Make sure you understand why before you start.

Delete the `let _ = (value, MIN_COUNTS, MAX_COUNTS);` line when you do. It's only
there so the unfinished stub compiles.

## 5. Write the tests.

The `TODO(you)` block at the bottom of the file lists them: one test per
remaining code path of `to_counts`, plus one for `channel_label` with a non-empty
prefix.

Then do the sabotage check the TODO describes: break your code on purpose and
make sure a test fails. If none do, your tests aren't really checking anything.

## 6. Open the PR.

```bash
./dev_scripts/format.sh rust_exercises/ex1_engineering_units
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
- The `TODO(you)` comments are gone. They were notes for you, and once the work
  is done they would only confuse the next reader.

## Things that trip people up

- **A build error that is a diff** is the formatter.
  `./dev_scripts/format.sh rust_exercises/ex1_engineering_units`.
- **A build error suggesting better code** is clippy. The message usually
  contains the fix.
- **You changed a file and Bazel says nothing changed.** You're probably
  building a different target than you think. Check the label.
- **Your test won't re-run.** Bazel saved the last result because nothing
  changed. Add `--nocache_test_results`.
