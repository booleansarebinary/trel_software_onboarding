# Hints: Rust Exercise 1

This file explains the Rust you will run into in this exercise, one idea at a
time. It explains the *tools*, not the answer - the examples use different
numbers and different problems on purpose, so you still get to put the pieces
together yourself.

Read it top to bottom once, or jump to whatever just confused you. And if
something here still doesn't click, please ask one of us. That's what we're here
for.

- [`mut` vs. no `mut`](#mut-vs-no-mut)
- [Things that start with `_`](#things-that-start-with-_)
- [`Option`, `Some`, and `None`](#option-some-and-none)
- [Rounding: `.round()` and friends](#rounding-round-and-friends)
- [`.clamp()`, `.min()`, and `.max()`](#clamp-min-and-max)
- [Converting numbers: `as u16` and `f64::from`](#converting-numbers-as-u16-and-f64from)
- [`return` vs. the last line](#return-vs-the-last-line)
- [Strings: `push`, `push_str`, `&str` vs. `&String`](#strings-push-push_str-str-vs-string)
- [`self`, `&self`, `mut self`, and `Self`](#self-self-mut-self-and-self)
- [`::` vs. `.`](#-vs-)
- [Tests: `#[rstest]`, `#[case]`, and asserts](#tests-rstest-case-and-asserts)

---

## `mut` vs. no `mut`

In Rust, **variables can't be changed unless you say so.** That's the opposite of
most languages.

```rust
let speed = 10;
speed = 20;          // compile error: cannot assign twice to immutable variable

let mut speed = 10;
speed = 20;          // fine
```

`mut` also applies to changing the *inside* of something, not just reassigning
it. `channel_label` needs `let mut label` because it calls `label.push(...)`,
which changes the string in place.

Why default to immutable? Because when you read `let x = ...`, you know `x` is
the same value for the rest of the function without reading any further. `mut`
is a little flag that says "watch this one, it changes."

**Shadowing is not `mut`.** You'll see this in the tests:

```rust
let channel = EngineeringUnit::new(0.1, -10.0);
let channel = channel.with_value(250.0);
```

That's not changing `channel`. The second `let` makes a brand new variable that
happens to reuse the name, and the old one is gone. It's handy when a value goes
through a few steps and you don't want to invent `channel2`.

## Things that start with `_`

The underscore shows up in a few places, and they all mean roughly "I know, and
I don't care about this":

| You see | It means |
| --- | --- |
| `let _ = something;` | Evaluate `something` and throw the result away. The stub in `to_counts` uses this so the compiler doesn't complain that `value` is unused. |
| `let _unused = 5;` | A variable you're keeping on purpose but not reading. The leading `_` switches off the "unused variable" warning. |
| `for _ in 0..3 { ... }` | Loop three times. You don't need the loop counter, so you don't name it. |
| `_ => ...` in a `match` | "Anything else." A catch-all arm. |
| `1_000_000.0` | Nothing at all! Underscores inside numbers are ignored; they're only there so you can read them. `1_000_000` is `1000000`. |
| `2.0_f64` | Same rule: the underscore is ignored, and the `f64` on the end tells Rust which number type you mean. `2.0f64` is identical. |

## `Option`, `Some`, and `None`

Rust has no `null`. When a function might not have an answer, it returns an
`Option`:

```rust
enum Option<T> {
    Some(T),   // "here's the answer", wrapped up
    None,      // "there's no answer"
}
```

So `Option<u16>` means "maybe a `u16`." You make one like this:

```rust
fn first_letter(word: &str) -> Option<char> {
    if word.is_empty() {
        return None;
    }
    Some(word.chars().next().unwrap())
}
```

The key idea: a `u16` wrapped in `Some(...)` is **not** a `u16`. You can't add 1
to an `Option<u16>`. Whoever calls your function has to unwrap it first, which
forces them to decide what to do in the `None` case. That's the whole point:
nobody can forget that "no answer" is possible.

In tests, you compare directly against the wrapped value:

```rust
assert_eq!(first_letter("valve"), Some('v'));
assert_eq!(first_letter(""), None);
```

## Rounding: `.round()` and friends

`f64` has four ways to get rid of the fractional part. Each gives back another
`f64`, not an integer:

| Method | `2.4` | `2.5` | `2.6` | `-2.6` |
| --- | --- | --- | --- | --- |
| `.round()` - nearest, halves go away from zero | `2.0` | `3.0` | `3.0` | `-3.0` |
| `.floor()` - always down | `2.0` | `2.0` | `2.0` | `-3.0` |
| `.ceil()` - always up | `3.0` | `3.0` | `3.0` | `-2.0` |
| `.trunc()` - just chop it off | `2.0` | `2.0` | `2.0` | `-2.0` |

Call them with a dot: `x.round()`.

## `.clamp()`, `.min()`, and `.max()`

`clamp` squeezes a value into a range. Anything below the bottom becomes the
bottom, anything above the top becomes the top, and anything already inside is
left alone:

```rust
let percent = 130.0_f64.clamp(0.0, 100.0);   // 100.0
let percent = (-5.0_f64).clamp(0.0, 100.0);  // 0.0
let percent = 42.0_f64.clamp(0.0, 100.0);    // 42.0
```

Engineers call this **saturating**: pushing past the limit just holds you at the
limit, like a gauge needle pinned at the top.

`min` and `max` are the one-sided versions. They pick the smaller or larger of
two values:

```rust
let at_least_one = requested.max(1);   // never below 1
let at_most_ten = requested.min(10);   // never above 10
```

(Exercise 2 uses exactly that `max(1)` trick.) The names feel backwards at first
- `max` sets a *floor* - so if you get the wrong one, that's normal. Read it as
"the max of `requested` and 1."

Links: [`f64::clamp`](https://doc.rust-lang.org/std/primitive.f64.html#method.clamp),
[`f64::round`](https://doc.rust-lang.org/std/primitive.f64.html#method.round).

## Converting numbers: `as u16` and `f64::from`

Rust never converts number types for you. An `f64` and a `u16` can't be added,
compared, or returned in place of each other until you convert one of them.
There are two ways:

**`f64::from(x)`** is for conversions that can never lose information. Every
`u16` fits perfectly in an `f64`, so `f64::from(counts)` is allowed. The
compiler won't let you write `u16::from(some_f64)`, because that could lose
information.

**`x as u16`** is the "I know what I'm doing" conversion. It always compiles.
When you go from a float to an integer, `as`:

- **chops off the fraction** - it does not round. `2.9 as u16` is `2`.
- **pins out-of-range values** to the nearest limit. `-5.0 as u16` is `0`, and
  `99999.0 as u16` is `65535`.
- turns `NaN` into `0`.

That second point surprises people. But relying on it makes a reader stop and go
"wait, is that safe?" So when the limits matter, it's clearer to put the value in
range yourself first and *then* use `as`, so the code says what it means.

`u16::MAX` is the largest `u16`, which is `65535`. There's also `u16::MIN`
(`0`).

## `return` vs. the last line

The last expression in a function is its return value, as long as it has **no
semicolon**:

```rust
fn double(x: i32) -> i32 {
    x * 2          // returned
}
```

Add a semicolon and it becomes a statement that returns nothing, and you'll get
a type error. That catches everyone at least once.

Use `return` for **early exits**, like bailing out at the top of a function:

```rust
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b == 0.0 {
        return None;     // early exit, so `return` is useful here
    }
    Some(a / b)          // last line, no `return` needed
}
```

That's exactly what clippy's `needless_return` complains about: `return` on the
very last line.

## Strings: `push`, `push_str`, `&str` vs. `&String`

Rust has two main string types:

- **`String`** - a string you own and can change. It lives on the heap and
  can grow.
- **`&str`** - a borrowed view of some text you can read but not change. String
  literals like `"hello"` are `&str`.

To add to a `String` (declared `mut`):

```rust
let mut s = String::from("ai");
s.push('_');          // push adds ONE character: single quotes
s.push_str("12");     // push_str adds a string: double quotes
// s is now "ai_12"
```

Single quotes (`'_'`) make a `char`, and double quotes (`"_"`) make a `&str`.
Mixing them up gives a type error that names both types, so it's easy to spot
once you know to look for it.

`format!` is often simpler when building a string from pieces:

```rust
let s = format!("{}_{}", prefix, id);
```

**Why `&str` beats `&String` for parameters:** a function that takes `&str`
accepts both a `&String` and a plain literal like `"AI"`. A function that takes
`&String` only accepts the first one. If you're only reading the text, ask for
`&str`. That's what clippy's `ptr_arg` is telling you.

`to_lowercase()` doesn't change the original. It builds and returns a new
`String`.

## `self`, `&self`, `mut self`, and `Self`

Inside an `impl` block:

| You see | It means |
| --- | --- |
| `&self` | The method *reads* the value it was called on. Most methods look like this. |
| `&mut self` | The method *changes* the value it was called on. |
| `self` / `mut self` | The method *takes* the value, consumes it, and usually gives back a new one. `with_value` does this so calls can chain. |
| `Self` (capital S) | Shorthand for the type we're inside, here `EngineeringUnit`. |

So `channel.from_counts(5)` calls `from_counts` with `self` set to `channel`,
and inside you read its fields as `self.scale` and `self.offset`.

## `::` vs. `.`

- `.` calls something **on a value**: `channel.value()`, `x.round()`.
- `::` reaches **into a type or module**: `EngineeringUnit::new(...)`,
  `u16::MAX`, `String::from("ai")`. There's no value yet, so there's nothing to
  put in front of a dot.

## Tests: `#[rstest]`, `#[case]`, and asserts

The things in `#[...]` are **attributes**, notes to the compiler or a library
about the item underneath.

- `#[cfg(test)]` on `mod tests` means "only compile this when running tests."
- `use super::*;` pulls everything from the file above into the test module.
- `#[rstest]` marks a test.
- `#[case(a, b)]` runs the same test once per line, filling the parameters
  marked `#[case]` in order. Three `#[case]` lines means three separate tests in
  the output.

```rust
#[rstest]
#[case(2.4, 2.0)]
#[case(2.6, 3.0)]
fn test_round_goes_to_the_nearest_whole_number(#[case] input: f64, #[case] expected: f64) {
    let rounded = input.round();

    assert_eq!(rounded, expected);
}
```

The asserts:

- `assert_eq!(actual, expected)` - fails if they differ, and prints both. This
  is the one you'll reach for most, and it works fine on `Option`s:
  `assert_eq!(result, Some(7))`.
- `assert!(condition)` - fails if the condition is false. Used for float
  comparisons with a tolerance, like `assert!((a - b).abs() < EPSILON)`, since
  two floats that "should" be equal often differ in the last decimal place.

Running just this exercise's tests:

```bash
bazel test //rust_exercises/ex1_engineering_units:tests --test_output=errors
```

`--test_output=errors` prints the failure messages right in your terminal
instead of making you open a log file.
