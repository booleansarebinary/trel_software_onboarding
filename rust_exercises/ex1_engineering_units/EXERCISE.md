# Rust Exercise 1: Build it, clean it, implement it, test it

**About 45 minutes.** This one walks you through everything you need for your
first PR: getting Bazel running, meeting the formatter and the linter, writing a
little Rust, and writing tests.

Try it without AI if you can - the errors in steps 1 and 2 are the whole point,
and they are much more useful when you read them yourself. Stuck? Ask us.

## 1. Try to build it. It fails on purpose.

```bash
bazel build //rust_exercises/ex1_engineering_units:engineering_units
```

The error is a **diff**, not a compile error. That is `rustfmt`. Our `.bazelrc`
wires the formatter into every build, so badly formatted Rust genuinely does not
compile here - which means nobody can forget to format. Fix it with the tool
rather than by hand:

```bash
./dev_scripts/format.sh
```

(If you are in VS Code and ran setup, formatting also happens when you hit save.)

## 2. Build again. Now the linter has opinions.

Three findings from `clippy`, all in `channel_label`. Clippy findings are hard
errors here, not warnings. Fix these by hand and actually read the messages -
these are patterns worth recognizing in your own code later:

- **`ptr_arg`** - `&String` where `&str` would do. The function only reads the
  string, so taking `&String` forces every caller to own a `String` first. Lots
  of people carry this habit for years; nice to drop it early.
- **`len_zero`** - `.len() == 0` should be `.is_empty()`.
- **`needless_return`** - the last expression in a Rust function *is* its return
  value, so `return` on that line is just noise.

Each message links to an explanation. Read at least one of them.

Please don't reach for `#[allow(...)]` here. Silencing a linter is sometimes the
right call, but it needs a reason in a comment, and none of these have one.

## 3. Read the code.

`src/lib.rs`, top to bottom, comments included. Read the existing tests twice -
they are the style we are asking you to match.

Then see where you stand:

```bash
bazel test //rust_exercises/ex1_engineering_units:tests
```

One test fails, inside `todo!()`. That is your next job.

## 4. Implement `to_counts`.

Its doc comment lists four behaviors - get all four. It returns `Option<u16>`
rather than `u16`; make sure you understand why before you start.

Delete the `let _ = (value, MIN_COUNTS, MAX_COUNTS);` line when you implement it.
It only exists so the unfinished stub compiles.

## 5. Write the tests.

The `TODO(you)` block at the bottom of the file lists them: one test per
remaining code path of `to_counts`, plus one for `channel_label` with a non-empty
prefix.

Then do the sabotage check the TODO describes - break your own code on purpose
and confirm your tests notice. If they don't, they aren't really testing
anything, and this is the fastest way to find that out.

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
- **A build error suggesting better code** is clippy. The message usually
  contains the fix.
- **You changed a file and Bazel says nothing changed.** You are probably
  building a different target than you think - check the label.
- **Your test will not re-run.** Bazel cached the result because nothing it knows
  about changed. Add `--nocache_test_results`.
