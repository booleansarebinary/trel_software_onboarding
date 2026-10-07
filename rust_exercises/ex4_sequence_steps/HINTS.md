# Hints: Rust Exercise 4

This explains the Rust you'll use in this exercise. The examples are about other
things on purpose, so you still put the pieces together yourself. If something
doesn't click, please ask one of us.

- [Enum variants that carry data](#enum-variants-that-carry-data)
- [Getting the data out: `let ... else` and `matches!`](#getting-the-data-out-let--else-and-matches)
- [`Copy`, and why it helps here](#copy-and-why-it-helps-here)
- [`if` gives back a value](#if-gives-back-a-value)
- [`.get()` vs. `[i]`](#get-vs-i)
- [`.clone()` on a `Vec`](#clone-on-a-vec)
- [The borrow checker and `&mut self.state`](#the-borrow-checker-and-mut-selfstate)
- [Working through it by hand](#working-through-it-by-hand)

---

## Enum variants that carry data

You know enums. In Rust, each variant can hold its own data, with named fields
like a little struct:

```rust
enum Pump {
    Off,
    Priming { seconds_left: u32 },
    Running { rpm: u32, minutes: u32 },
}
```

`seconds_left` only exists while priming, so it lives *inside* `Priming`. There
is no stale `seconds_left` lying around while the pump is off.

Making one:

```rust
let pump = Pump::Running { rpm: 3000, minutes: 0 };
```

## Getting the data out: `let ... else` and `matches!`

`match` handles every variant:

```rust
match pump {
    Pump::Off => println!("off"),
    Pump::Priming { seconds_left } => println!("{seconds_left}s"),
    Pump::Running { rpm, .. } => println!("{rpm} rpm"),   // .. ignores the rest
}
```

When you only care about one variant, `let ... else` reads better: get the
fields, or leave.

```rust
let Pump::Running { rpm, minutes } = pump else {
    return;   // not running: nothing to do
};
// `rpm` and `minutes` are usable from here on
```

`matches!` just asks "is it this variant?" and gives back a `bool`:

```rust
if matches!(pump, Pump::Priming { .. } | Pump::Running { .. }) {
    // either one; `|` means "or"
}
```

`abort` in `lib.rs` uses this.

## `Copy`, and why it helps here

`SequenceState` derives `Copy`, so assigning it copies it instead of moving it:

```rust
let state = self.state;   // a copy; self.state is untouched
```

That means you can take the fields out with `let ... else`, work out the new
state from those copies, and assign `self.state = ...` at the end, with no
borrowing to think about. (`Vec` isn't `Copy`, which is why `Step` and
`Sequence` aren't either.)

## `if` gives back a value

Like `match`, an `if`/`else` is an expression, so it can produce a value:

```rust
let label = if rpm > 0 { "spinning" } else { "stopped" };

let next = if minutes < limit {
    Pump::Running { rpm, minutes: minutes + 1 }
} else {
    Pump::Off
};
```

Every branch has to give back the same type. A chain of `else if`s works too.

## `.get()` vs. `[i]`

```rust
let steps = vec![10, 20, 30];
steps[5];         // crashes: index out of bounds
steps.get(5);     // None
steps.get(1);     // Some(&20)
```

When an index *should* always be valid but you'd rather not crash if you're
wrong, use `.get()` with `let Some(x) = ... else`.

`steps.len()` is how many there are. Indexes go from `0` to `len() - 1`, so
"is there another step after `i`?" is `i + 1 < steps.len()`.

## `.clone()` on a `Vec`

`tick` returns a `Vec<Command>`, but the step's commands belong to the step. You
can't hand out the original, because the step still needs it. `.clone()` makes
a copy to return:

```rust
let todo: Vec<&str> = checklist.items.clone();
```

To return "nothing", use `Vec::new()`, which is an empty `Vec`.

## The borrow checker and `&mut self.state`

You can also write `tick` by matching on a mutable reference and editing the
fields in place:

```rust
match &mut self.pump {
    Pump::Running { minutes, .. } => *minutes += 1,   // `*` writes through the reference
    _ => {}
}
```

That compiles, and so does assigning `self.pump = Pump::Off` inside the arm.
What *doesn't* compile is calling a method that takes `&mut self` inside that
arm and then using one of the fields afterwards:

```rust
match &mut self.pump {
    Pump::Running { minutes, .. } => {
        self.log_something();     // needs ALL of self, mutably...
        *minutes += 1;            // ...but `minutes` still borrows part of it
    }
    _ => {}
}
// error[E0499]: cannot borrow `*self` as mutable more than once at a time
```

Rust only allows one mutable borrow at a time, and `minutes` already is one. The
"copy it out, compute, write back once" style avoids the whole situation.

## Working through it by hand

Before you write code, take the two-step sequence in the test helper
(`two_steps(2)`) and fill in this table on paper:

| `tick` call | state before | commands returned | state after |
| --- | --- | --- | --- |
| 1 | `Running { step: 0, cycles_in_step: 0 }` | ? | ? |
| 2 | ? | ? | ? |
| 3 | ? | ? | ? |
| 4 | ? | ? | ? |

If your table matches the timeline at the top of `lib.rs`, the code is mostly
just writing the table down.
