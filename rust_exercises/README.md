# Rust Exercises

Two exercises, about 45 minutes each. Each one ends in a pull request.

| # | Directory | What you do | What it teaches |
| --- | --- | --- | --- |
| 1 | `ex1_engineering_units/` | Fix what the tooling rejects, implement a function, test it | Bazel, rustfmt, clippy, `Option`, our test style, your first PR |
| 2 | `ex2_abort_debounce/` | Write a test that exposes a real bug, then fix it | The habit that matters most; enums, `Result`, rstest fixtures |

**Do them in order** - the second one assumes the first.

Try these without AI if you can. It's not forbidden, but you only get fluent with
the tooling by hitting the errors yourself. If you get stuck, ask one of us. We'd
much rather answer a question than have you stuck for an hour.

Each exercise has a `HINTS.md` that explains the Rust syntax it uses.

## Why Rust

Rust is where safety-critical and high-performance TREL code goes: flight
software, ground software, and the flight board firmware.

If you've written C or C++: Rust is memory-safe without a garbage collector. If
you've written Python or Java: the surprise is the **borrow checker**. The
compiler tracks who owns each value and won't build code that could have certain
memory bugs.

Fighting the borrow checker is completely normal for the first week or so. Every
error it gives you is a bug caught at compile time instead of during a hotfire.
For background, [the Rust Book](https://doc.rust-lang.org/book/) chapters 4, 6,
and 9 (ownership, enums, and error handling) take about an hour and will save you
three.

You don't need to know Rust going in. Both exercises are small, and the code is
commented for someone seeing it for the first time.

## Test conventions

These are the same conventions we use in the main repo, and we'll look for them
in review.

- **`#[rstest]` on every test, never `#[test]`.** We use
  [rstest](https://crates.io/crates/rstest) everywhere.
- **Naming:** `test_<function>`, or `test_<function>_<behavior>`, or
  `test_<function>_<behavior>_when_<condition>`. A good name tells a reviewer
  what broke without opening the file.
- **Three blocks, one blank line between each:** setup, the call under test,
  then assertions.
- **Nothing `pub` in a `tests` module.** It's not an API.
- **`#[case]` for specific inputs, `#[values]` for combinations.** Don't stack
  three `#[values]` and call it thorough: 125 cases that all test the same thing
  just make the suite slow.
- **No logic in tests.** No `if` deciding what to assert, and don't compute the
  expected value with the same formula the code uses. Loops that just repeat a
  call (like running N control cycles in exercise 2) are fine.

## Where things live

Rust unit tests go **in the same file** as the code they test, inside a
`#[cfg(test)] mod tests`. That surprises people coming from other languages. It's
also why a `rust_test` target takes `crate = ":my_library"` instead of `srcs`,
and why there are no separate test files here.

## A note on formatting and lint

You don't need to remember to run a formatter. `bazel build` runs `rustfmt` and
`clippy` for you and fails if either is unhappy. When it does,
`./dev_scripts/format.sh` fixes the formatting. Clippy findings you fix by hand,
since they're about the code itself, not its layout. Exercise 1 walks you through
both.

## Dependencies

There's no `Cargo.toml` here, or in the main repo. Crates (Rust libraries) are
declared once in `MODULE.bazel` with `crate.spec(...)`, then used from a
`BUILD.bazel` as `@crates//:rstest`.

If you need a crate that isn't declared yet, talk to us first rather than adding
it yourself. Every new dependency is shared by the whole repo, so it's worth a
two-minute conversation.
