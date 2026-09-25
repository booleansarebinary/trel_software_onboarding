# Hints: Rust Exercise 2

This file explains the Rust and git you'll run into in this exercise. It builds
on [exercise 1's hints](../ex1_engineering_units/HINTS.md) (`mut`, `_`,
`Option`, `::` vs. `.`), so skim that first if those aren't familiar yet.

The examples here are deliberately about other things. At the bottom there's a
short section of nudges toward the bug, arranged from gentle to less gentle.
Try the exercise before you read those, and please ask one of us if you're
stuck. That's often faster than any hint.

- [References: `&`, `&mut`, and `*`](#references--mut-and-)
- [`HashMap`](#hashmap)
- [The `entry` API](#the-entry-api)
- [`Result`, `Ok`, `Err`, and `?`](#result-ok-err-and-)
- [`if let` and `let ... else`](#if-let-and-let--else)
- [Loops and ranges](#loops-and-ranges)
- [`Vec`: `vec![]`, `push`, `extend_from_slice`](#vec-vec-push-extend_from_slice)
- [`.expect()`](#expect)
- [rstest fixtures](#rstest-fixtures)
- [Two commits, and checking your test catches the bug](#two-commits-and-checking-your-test-catches-the-bug)
- [Nudges toward the bug](#nudges-toward-the-bug)

---

## References: `&`, `&mut`, and `*`

A **reference** lets you use a value without taking ownership of it:

- `&x` - a read-only reference. Any number of these can exist at once.
- `&mut x` - a reference you can change `x` through. Only **one** can exist at a
  time, and not while any `&x` exists.

That one rule is the borrow checker. It's why the file's top comment explains
that `run` collects ids into a `Vec` first: it can't read `abort_configs` and
change `write_queue` through the same `&mut context` at the same moment.

`*` goes the other way. It **follows** a reference to the value behind it:

```rust
let mut total = 0;
let r = &mut total;
*r += 5;          // change the value `r` points at
// total is now 5
```

If you see `*something = ...` or `*something += ...`, it's changing whatever
`something` refers to.

## `HashMap`

A map from keys to values, like a Python `dict`:

```rust
use std::collections::HashMap;

let mut pressures: HashMap<u16, f64> = HashMap::new();
pressures.insert(3, 450.0);

let p = pressures.get(&3);          // Option<&f64>: Some(&450.0)
let missing = pressures.get(&99);   // None

if let Some(p) = pressures.get_mut(&3) {
    *p = 500.0;                     // change the stored value in place
}
pressures.remove(&3);
```

`get` returns an `Option`, because the key might not be there. The order you get
when iterating is **random**, and can change between runs, so never write a test
that depends on it.

`for (key, value) in &map` loops over every pair, borrowing each one.

## The `entry` API

`entry(key).or_insert(default)` means "give me a mutable reference to this
key's value, and if the key doesn't exist yet, insert `default` first":

```rust
let mut visits: HashMap<&str, u32> = HashMap::new();

let count = visits.entry("pad_a").or_insert(0);   // count: &mut u32
*count += 1;
```

It's the standard way to keep a running counter per key, which is exactly what
`consecutive_true_cycles` is.

## `Result`, `Ok`, `Err`, and `?`

`Result` is `Option`'s sibling. Where `Option` says "maybe there's an answer,"
`Result` says "here's the answer, or here's what went wrong":

```rust
enum Result<T, E> {
    Ok(T),    // it worked, here's the value
    Err(E),   // it failed, here's why
}
```

`?` is a shortcut for "if this is an `Err`, return it from my function right now;
otherwise unwrap the `Ok` and keep going":

```rust
fn read_both(a: &str, b: &str) -> Result<(i32, i32), std::num::ParseIntError> {
    let x = a.parse::<i32>()?;   // bails out early if `a` isn't a number
    let y = b.parse::<i32>()?;
    Ok((x, y))
}
```

`run` returns `Result<(), AbortError>`. The `()` is Rust's "nothing": success,
but there's no value to hand back.

## `if let` and `let ... else`

Two ways to pull the value out of a `Some` (or an `Ok`) without a full `match`:

```rust
// Do something only if it's there.
if let Some(p) = pressures.get(&3) {
    println!("{p}");
}

// Get it or leave. `else` must exit: return, continue, or break.
let Some(p) = pressures.get(&3) else {
    return;
};
// `p` is usable from here on
```

## Loops and ranges

```rust
for i in 0..3 { }    // i = 0, 1, 2 (stops BEFORE 3)
for i in 0..=3 { }   // i = 0, 1, 2, 3 (the = includes the end)
for _ in 0..n { }    // repeat n times, counter not needed
```

Careful with `n - 1` on unsigned types like `u16`: if `n` is `0`, that's a crash
in tests ("attempt to subtract with overflow"), not `-1`. The exercise's
`min_cycles` values start at 2, so you're fine here, but it's worth knowing.

## `Vec`: `vec![]`, `push`, `extend_from_slice`

```rust
let mut ids = Vec::new();         // empty, growable list
ids.push(4);                      // add one item
let more = vec![5, 6];            // vec! builds one with items already in it
ids.extend_from_slice(&more);     // add a whole list's worth
// ids is [4, 5, 6]

ids.is_empty();                   // false
```

## `.expect()`

`.expect("message")` on a `Result` or `Option` means "unwrap it, and if it's an
`Err`/`None`, crash with this message." In tests that's what you want. It turns
an unexpected error into a clear failure. In real code, avoid it for anything
that can actually happen, and use `?` or handle the error instead.

## rstest fixtures

A `#[fixture]` is a function that builds test setup. Any test with a parameter
of the **same name** gets a fresh copy:

```rust
#[fixture]
fn context() -> LoopContext { /* ... */ }

#[rstest]
fn test_something(mut context: LoopContext) {   // `context` matches the fixture's name
    // ...
}
```

The `mut` goes in the parameter because the test is going to change it. Fixtures
and `#[case]` mix fine in the same test: the existing tests do both.

## Two commits, and checking your test catches the bug

```bash
# Commit 1: the test only, failing.
git add rust_exercises/ex2_abort_debounce/src/lib.rs
git commit -m "Add test for discontinuous abort cycles"

# Fix the code, then commit 2.
git add rust_exercises/ex2_abort_debounce/src/lib.rs
git commit -m "<what you fixed, in a few words>"
```

To check "reverting just your fix makes only your new test fail":

```bash
git checkout HEAD~1 -- rust_exercises/ex2_abort_debounce/src/lib.rs   # file as of commit 1
bazel test //rust_exercises/ex2_abort_debounce:tests --test_output=errors
git checkout HEAD -- rust_exercises/ex2_abort_debounce/src/lib.rs     # back to your fix
```

`HEAD` is your latest commit and `HEAD~1` is the one before it.

## Nudges toward the bug

Try the exercise first. Each nudge gives away a bit more than the one before.

<details>
<summary>Nudge 1</summary>

Write the test first and read its failure. Which assertion failed, and what was
actually in the write queue?

</details>

<details>
<summary>Nudge 2</summary>

Walk through `run` by hand for `min_cycles = 2` with the condition going true,
false, true. Write down the value of `*cycles` after each of the three cycles.
What *should* it be after the false one?

</details>

<details>
<summary>Nudge 3</summary>

Look at the `if condition_met { ... }` block. What happens to the counter when
the condition is **not** met?

</details>
