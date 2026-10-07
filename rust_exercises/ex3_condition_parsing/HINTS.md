# Hints: Rust Exercise 3

This explains the Rust in this exercise: what you need for `parse_operand`, and
what you'll see in the finished code around it. The examples are about other
things on purpose, so you still put the pieces together yourself. If something
doesn't click, please ask one of us.

- [Regex → Rust](#regex--rust)
- [`FromStr` and `.parse()`](#fromstr-and-parse)
- [`strip_prefix`](#strip_prefix)
- [Slice patterns](#slice-patterns)
- [`?`, `map_err`, and `map`](#-map_err-and-map)
- [Match guards: `if` inside a `match` arm](#match-guards-if-inside-a-match-arm)
- [`&str` vs. `String` in errors](#str-vs-string-in-errors)
- [`&&str`: why there are two `&`s](#str-why-there-are-two-s)

---

## Regex → Rust

You'd probably reach for a regex like `^ai_(\d+)\s+(>=|<=|>|<)\s+(\S+)$`. It
would work, but when it fails, all it tells you is "no match". Rust's string
methods let you check one piece at a time and say exactly what was wrong.

| With a regex | In Rust |
| --- | --- |
| `\s+` to split | `text.split_whitespace()` |
| `^ai_` prefix check | `token.strip_prefix("ai_")` |
| `(\d+)` then convert | `digits.parse::<u16>()`, which also checks it fits |
| `(>=\|<=\|>\|<)` | `match token { ">=" => ..., "<=" => ..., ... }` |
| capture groups 1, 2, 3 | a slice pattern: `[a, b, c]` (below) |
| "didn't match" | your own error enum, one variant per reason |

(This repo doesn't depend on the `regex` crate, and for a grammar this small we
wouldn't add it.)

## `FromStr` and `.parse()`

Implement `FromStr` for a type, and `.parse()` works for it:

```rust
use std::str::FromStr;

enum Fuel { Ethanol, Kerosene }

impl FromStr for Fuel {
    type Err = String;   // what parse returns on failure

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        match text {
            "ethanol" => Ok(Fuel::Ethanol),
            "kerosene" => Ok(Fuel::Kerosene),
            other => Err(format!("unknown fuel: {other}")),
        }
    }
}

let fuel: Result<Fuel, String> = "ethanol".parse();
let fuel = "ethanol".parse::<Fuel>();   // same thing; ::<Fuel> names the type
```

`Comparison` in `lib.rs` is a complete example. `Self` means "the type we're
implementing for", and `Self::Err` means "the `Err` type we picked above".

## `strip_prefix`

Returns the rest of the string if it starts with the prefix, or `None` if it
doesn't:

```rust
"tank_7".strip_prefix("tank_")   // Some("7")
"pump_7".strip_prefix("tank_")   // None
"tank_".strip_prefix("tank_")    // Some("") - an empty rest still counts
```

Pairs nicely with `if let`:

```rust
if let Some(number) = label.strip_prefix("tank_") {
    // `number` is everything after "tank_"
}
```

## Slice patterns

You won't write one of these, but `Condition`'s `from_str` uses one. You can
`match` (or `let ... else`) on the *shape* of a slice, which is a lot like
capture groups:

```rust
let words: Vec<&str> = "open valve 4".split_whitespace().collect();

match words.as_slice() {
    [verb, noun, id] => { /* exactly three: each name binds one */ }
    [] => { /* empty */ }
    _ => { /* any other length */ }
}

// Or: get exactly two, or bail out.
let [first, second] = words.as_slice() else {
    return Err(format!("expected 2 words, got {}", words.len()));
};
```

`.as_slice()` turns the `Vec` into a slice, which is what the pattern matches
against.

## `?`, `map_err`, and `map`

`?` returns early on `Err`. It only works if the error type matches your
function's error type, so you often convert first:

```rust
// u16's parse error is std::num::ParseIntError, not our error.
// map_err swaps the error for one of ours:
let id = text
    .parse::<u16>()
    .map_err(|_| MyError::BadNumber(text.to_string()))?;
```

`map` does the same thing to the *success* value instead:

```rust
let wrapped = "12".parse::<u16>().map(Tank::Id);   // Ok(Tank::Id(12))
```

A tuple-style enum variant like `Tank::Id` can be used as a function, which is
why `.map(Tank::Id)` works.

## Match guards: `if` inside a `match` arm

A `match` arm can have an extra condition:

```rust
match reading.parse::<f64>() {
    Ok(value) if value >= 0.0 => Ok(value),
    _ => Err("not a non-negative number"),
}
```

The arm only matches if the pattern fits **and** the `if` is true. Otherwise it
falls through to the next arm. `f64` has `.is_finite()`, which is false for
NaN and both infinities.

## `&str` vs. `String` in errors

`ParseError::BadOperand(String)` owns its text. A `&str` borrowed from the input
would tie the error's lifetime to the input string, which gets awkward fast.
Turn a `&str` into a `String` with `.to_string()` (or `String::from(...)`).

## `&&str`: why there are two `&`s

Matching `[lhs, op, rhs]` against a `&[&str]` gives you `&&str`s: references to
the `&str`s inside the slice. You usually don't have to care. Rust adds or
removes `&`s automatically when you call methods or pass them to a function
that takes `&str`. If the compiler ever complains about `&&str`, a `*` in front
of the variable removes one layer.
