# Rust Exercises

Two exercises, about 45 minutes each. Each one ends in a pull request.

| # | Directory | What you do | What it teaches |
| --- | --- | --- | --- |
| 1 | `ex1_engineering_units/` | Fix what the tooling rejects, implement a function, test it | Bazel, rustfmt, clippy, `Option`, our test style, your first PR |
| 2 | `ex2_abort_debounce/` | Write a test that exposes a real bug, then fix it | The habit that matters most; enums, `Result`, rstest fixtures |

**Do them in order** - the second one assumes the first.

Try these without AI if you can. Not because AI is forbidden, but because the
point is for *you* to get fluent with the tooling, and you only get that from
hitting the errors yourself. If you get stuck, ask one of us. We would much
rather answer a question than have you grind on something for an hour.

## Why Rust

Rust is where safety-critical and high-performance TREL code goes: flight
software, ground software, the flight board firmware.

If you have written C or C++, the pitch is memory safety without a garbage
collector. If you have written Python or Java, the surprise is the borrow
checker - the compiler tracks who owns each value and refuses to build code that
could have a use-after-free or a data race.

Fighting the borrow checker is completely normal for the first week or so. It is
not a hazing ritual, it is a whole class of bug getting caught at compile time
instead of during a hotfire. If you want some background first,
[the Rust Book](https://doc.rust-lang.org/book/) chapters 4, 6, and 9 cover
ownership, enums and pattern matching, and error handling. That is about an hour
and it will save you three.

You do not need to know Rust going in. Both exercises are small, and the code
you are reading is commented for someone seeing it for the first time.

## Test conventions

These are the same conventions we use in the main repo, so getting used to them
here is the point. We will look for them in review.

- **`#[rstest]` on every test, never `#[test]`.** We use
  [rstest](https://crates.io/crates/rstest) everywhere.
- **Naming:** `test_<function>`, or `test_<function>_<behavior>`, or
  `test_<function>_<behavior>_when_<condition>`. A good name means a reviewer
  reading a CI failure knows what broke without opening the file.
- **Three blocks, one blank line between each:** setup, the call under test,
  then assertions. It makes tests skimmable, and we do ask for it in review.
- **Nothing `pub` in a `tests` module.** It is not an API.
- **`#[case]` for specific inputs, `#[values]` for combinations.** Try not to
  stack three `#[values]` and call it thorough - 125 cases that all take the
  same code path is a slow test suite pretending to be a good one.
- **No logic in tests.** No `if` deciding what to assert, and don't compute the
  expected value with the same formula the code uses. A test with a branch in it
  has an untested branch in it. Loops that just repeat a call (like driving N
  control cycles in exercise 2) are completely fine.

## Where things live

Rust unit tests go **in the same file** as the code they test, inside a
`#[cfg(test)] mod tests`. This surprises people coming from other languages.
It is also why a `rust_test` target takes `crate = ":my_library"` instead of
`srcs`, and why you will not find separate test files here.

## A note on formatting and lint

You do not need to remember to run a formatter. `bazel build` runs `rustfmt` and
`clippy` for you and fails if either is unhappy, so the build tells you. When it
does, `./dev_scripts/format.sh` fixes the formatting part; clippy findings you
fix by hand, since they are suggestions about the code itself rather than its
layout. Exercise 1 walks you through both.

## Dependencies

There is no `Cargo.toml` in this repo, and there is none in the main repo
either. Crates are declared once in `MODULE.bazel` with `crate.spec(...)` and
then referenced from a `BUILD.bazel` as `@crates//:rstest`.

If you need a crate that is not declared yet, come talk to us rather than adding
it yourself. Because it is declared repo-wide, a new dependency effectively
becomes the org-wide answer to whatever problem it solves - worth a two-minute
conversation.
