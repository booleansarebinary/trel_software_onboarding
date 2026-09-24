# Rust Exercises

Two exercises, about 45 minutes each. Each one ends in a pull request.

| # | Directory | What you do | What it teaches |
| --- | --- | --- | --- |
| 1 | `ex1_engineering_units/` | Fix what the tooling rejects, implement a function, test it | Bazel, rustfmt, clippy, `Option`, house test style, your first PR |
| 2 | `ex2_abort_debounce/` | Write a test that exposes a real bug, then fix it | The habit that matters most; enums, `Result`, rstest fixtures |

**Do them in order.** The second assumes the first.

## Why Rust

Rust is where safety-critical and high-performance TREL code goes: flight
software, ground software, the scopium firmware. If you have written C or C++,
the pitch is memory safety without a garbage collector. If you have written
Python or Java, the surprise is the borrow checker - the compiler tracks who
owns each value and refuses to build code that could have a use-after-free or a
data race.

Fighting the borrow checker is normal for the first week. It is not a hazing
ritual; it is a class of bug being caught at compile time instead of during a
hotfire. If you want background reading,
[the Rust Book](https://doc.rust-lang.org/book/) chapters 4, 6, and 9 cover
ownership, enums and pattern matching, and error handling - about an hour, and it
will save you three.

## Conventions you are expected to follow

These are the conventions used in the main repo, and they are enforced in review.

- **`#[rstest]` on every test, never `#[test]`.** Every Rust test in TREL uses
  [rstest](https://crates.io/crates/rstest).
- **Naming:** `test_<function>`, or `test_<function>_<behavior>`, or
  `test_<function>_<behavior>_when_<condition>`. The failure name should tell a
  reviewer what broke without opening the file.
- **Three blocks, one blank line between each:** setup, the call under test,
  assertions. Required in review, not suggested.
- **Nothing `pub` in a `tests` module.** It is not an API.
- **`#[case]` for specific inputs, `#[values]` for combinations.** Do not stack
  three `#[values]` and call it thorough - 125 cases that all take the same
  code path is a slow suite pretending to be a good one.
- **No logic in tests.** No `if`, no loops deciding what to assert. A test with
  a branch in it has an untested branch in it. Loops that just repeat a call
  (like driving N control cycles in exercise 3) are fine.

## Where things live

Rust unit tests go **in the same file** as the code they test, in a
`#[cfg(test)] mod tests`. That is why a `rust_test` target takes
`crate = ":my_library"` instead of `srcs`, and why there are no separate test
files here.

## Dependencies

There is no `Cargo.toml` in this repo, and there is none in the main repo
either. Crates are declared once in `MODULE.bazel` with `crate.spec(...)` and
referenced from a `BUILD.bazel` as `@crates//:rstest`.

If you need a crate that is not declared yet: that is a conversation, not a
one-line change. A new dependency becomes the org-wide answer to whatever
problem it solves, and it applies to the whole repo. Ask before adding one.
