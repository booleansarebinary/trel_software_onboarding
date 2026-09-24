# Exercise 4: Make the tooling happy

**Goal:** learn what rustfmt and clippy actually enforce, and learn what the
`manual` tag does to a Bazel target.

The code in `src/lib.rs` works. Its tests pass. Everything else about it is
wrong.

## Why you have to ask for this target by name

Both targets in `BUILD.bazel` are tagged `manual`, which keeps them out of
wildcards like `//...`. So this package does not break `bazel build //...`
while you work on the other exercises:

```bash
# does not touch this package
bazel build //...

# does
bazel build //rust_exercises/ex4_lint_cleanup:channel_names
```

That second command fails. Good.

`manual` is a real tool with a real cost: a target CI never builds is a target
that quietly rots. Reach for it when a target genuinely cannot run in CI, not
when it is inconvenient.

## Steps

1. Build it and read the failure. It is a rustfmt diff, not a compile error.

2. Fix the formatting:

   ```bash
   bazel run @rules_rust//tools/rustfmt -- //rust_exercises/ex4_lint_cleanup:channel_names
   ```

3. Build again. Now you get clippy, roughly a dozen findings. Fix them **by
   hand, one at a time, reading each message.** Do not paste the file into an
   AI and ask it to clean it up; you will get working code and learn nothing,
   and the point of this exercise is that you recognize these patterns in your
   own code later.

   You will meet exactly six lints across 12 findings: `ptr_arg`, `len_zero`,
   `needless_return`, `needless_range_loop`, `needless_bool`, `collapsible_if`,
   and `single_match`. Every message links to an explanation. Read a few of
   them - `ptr_arg` in particular is worth understanding, because `&String`
   instead of `&str` is a habit people carry for years.

4. Now build the test target, which compiles the `mod tests` the library build
   skipped:

   ```bash
   bazel build //rust_exercises/ex4_lint_cleanup:tests
   ```

   One more lint appears: `bool_assert_comparison`, for `assert_eq!(x, true)`
   where `assert!(x)` says the same thing. Note the lesson in the mechanism -
   clippy only sees code that gets compiled, so lints in test code do not show
   up until something builds the test target. Same reason dead code hides.

5. The tests have problems no linter will catch: no blank-line structure
   separating setup from assertions, and one test named
   `test_is_analog_input` that checks two different behaviors. Fix both. This
   is the category of problem that only a reviewer catches, which is why we
   have reviewers.

6. **Remove the `manual` tags from both targets in `BUILD.bazel`.** Confirm the
   package now builds as part of the wildcard:

   ```bash
   bazel build //...
   bazel test //...
   ```

7. Open a PR.

## Done when

- `bazel build //...` and `bazel test //...` include this package and pass.
- No `tags = ["manual"]` left in this `BUILD.bazel`.
- You did not add a single `#[allow(...)]`. Silencing a linter is a decision
  that needs a reason in a comment, and none of these findings have one.

## The thing to take away

In Rust, formatting and linting are part of the build. You cannot forget them,
and neither can anyone else. C++ is not like that - see
`//cpp_exercises/README.md`.
