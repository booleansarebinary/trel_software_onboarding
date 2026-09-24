# Exercise 2: Write Rust

**Goal:** implement three functions and test them. This is your introduction to
the parts of Rust that feel different from Python, Java, or C++.

Start by running the tests. They fail immediately, inside `todo!()`. That is
correct.

```bash
bazel test //rust_exercises/ex2_sensor_limits:tests
```

## What you are learning

| Concept | Where it shows up |
| --- | --- |
| Enums as closed sets, exhaustive `match` | `Severity`, `classify` |
| `Result<T, E>` instead of exceptions | `new` |
| Error types that are types, not strings | `LimitError` |
| Borrowed slices (`&[f64]`) vs owned `Vec<f64>` | `worst_of` |
| Deriving `Ord` to get comparison for free | `Severity`, and `.max()` in `worst_of` |

If you have not written Rust before, read
[chapters 4, 6, and 9 of the Rust Book](https://doc.rust-lang.org/book/)
first: ownership, enums and pattern matching, and error handling. That is about
an hour and it will save you three.

## Steps

1. Implement `new`. Test it. Do not move on yet.
2. Implement `classify`. Test it, boundaries included.
3. Implement `worst_of`. Test it.
4. `./dev_scripts/format.sh`, then read the clippy output from
   `bazel build //rust_exercises/ex2_sensor_limits:sensor_limits` and fix
   what it tells you.
5. Open a PR.

One function at a time, each with its tests, is not a suggestion. Writing all
three and then debugging all three at once is how you spend an evening on a
twenty-minute exercise.

## Done when

- All three functions implemented, no `todo!()` left.
- Tests cover every branch, including both `LimitError` variants and the exact
  threshold boundaries.
- `bazel test //rust_exercises/ex2_sensor_limits:tests` is green.

## Worth raising in your PR

`worst_of` on an empty slice returns `Nominal`. The doc comment says so. Do you
agree that is the right answer for a safety interlock? There is a defensible
argument either way. Say what you think in the PR description - noticing a
questionable spec while implementing it is a large part of the job.
