# Hints: Rust Exercise 5

This explains the Rust you'll use in this exercise. The examples are about other
things on purpose, so you still put the pieces together yourself. If something
doesn't click, please ask one of us.

- [Traits](#traits)
- [Default methods](#default-methods)
- [`Box<dyn Trait>`: different types in one `Vec`](#boxdyn-trait-different-types-in-one-vec)
- [Looping with `&mut`](#looping-with-mut)
- [Keeping errors without stopping: `if let Err(...)`](#keeping-errors-without-stopping-if-let-err)
- [Getting a reading: `ok_or`, `?`, and `*`](#getting-a-reading-ok_or--and-)
- [Destructuring in a `for` loop](#destructuring-in-a-for-loop)
- [`HashMap::insert` replaces](#hashmapinsert-replaces)
- [Ordering enums with `derive(Ord)`](#ordering-enums-with-deriveord)
- [A test-only type](#a-test-only-type)

---

## Traits

A trait is a set of methods a type promises to have, like an interface in other
languages:

```rust
trait Sensor {
    fn name(&self) -> &str;
    fn read(&mut self) -> f64;
}

struct Thermocouple { last: f64 }

impl Sensor for Thermocouple {
    fn name(&self) -> &str {
        "thermocouple"
    }

    fn read(&mut self) -> f64 {
        self.last
    }
}
```

Implementing the trait means writing every method it lists, with exactly the
same signature. `ScheduledOutputs` in `lib.rs` is a full example of
implementing `Operator`.

## Default methods

A trait can give a method a body. Types get that version for free unless they
write their own:

```rust
trait Sensor {
    fn units(&self) -> &str {
        "volts"            // the default
    }
}

impl Sensor for Thermocouple {
    fn units(&self) -> &str {
        "degF"             // this type overrides it
    }
}
```

That's how `priority` works: `ScheduledOutputs` doesn't mention it and gets
`Normal`, while `ThresholdAbort` overrides it to `Safety`.

## `Box<dyn Trait>`: different types in one `Vec`

A `Vec` holds one type. `Thermocouple` and `PressureTransducer` are different
types, so `vec![thermocouple, transducer]` won't compile.

`Box<dyn Sensor>` means "some type that implements `Sensor`, stored on the
heap". Every box is the same size, so they fit in one `Vec`:

```rust
let mut sensors: Vec<Box<dyn Sensor>> = Vec::new();
sensors.push(Box::new(Thermocouple { last: 70.0 }));
sensors.push(Box::new(PressureTransducer::new()));
```

You call trait methods on a box like normal: `sensor.name()`.

## Looping with `&mut`

```rust
for sensor in &sensors { }       // read-only: can call &self methods
for sensor in &mut sensors { }   // can call &mut self methods, like read()
```

`run` takes `&mut self`, so the loop over operators needs `&mut`. Passing the
context into each call is fine: `context` is already a `&mut LoopContext`, and
you can pass it to one call, then the next, then the next.

## Keeping errors without stopping: `if let Err(...)`

`?` returns at the first error, which is exactly what `run_cycle` must *not* do.
Instead, check each result and keep going:

```rust
let mut problems = Vec::new();
for sensor in &mut sensors {
    if let Err(problem) = sensor.calibrate() {
        problems.push(problem);
    }
}
```

## Getting a reading: `ok_or`, `?`, and `*`

`HashMap::get` gives an `Option<&f64>`. `ok_or` turns `None` into the error of
your choice, so `?` can return it:

```rust
let temp = *temps
    .get(&probe_id)
    .ok_or(SensorError::NoProbe(probe_id))?;
```

The `*` at the front copies the `f64` out from behind the `&`, so you have a
plain number to compare. (Exercise 2's `Condition::evaluate` does exactly this.)

## Destructuring in a `for` loop

When a `Vec` holds tuples, you can unpack each one in the loop header:

```rust
let limits = vec![(1, 450.0), (2, 600.0)];
for &(channel, limit) in &limits {
    // `channel` and `limit` are plain copies
}
```

The `&` in the pattern matches the `&` that iterating over `&limits` gives you.

## `HashMap::insert` replaces

If the key is already there, `insert` overwrites it:

```rust
let mut valves = HashMap::new();
valves.insert(4, "open");
valves.insert(4, "closed");
// valves[&4] is "closed"
```

That "last write wins" behavior is why the order operators run in matters.

## Ordering enums with `derive(Ord)`

`#[derive(PartialOrd, Ord)]` on an enum orders the variants by the order you
wrote them, first to last:

```rust
#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Severity { Info, Warning, Critical }

Severity::Info < Severity::Critical   // true
```

So `items.sort_by_key(|item| item.severity())` puts `Info` first and `Critical`
last. `|item| item.severity()` is a *closure*, a small unnamed function. Rust's
`sort_by_key` is *stable*: items with equal keys keep their original order.

## A test-only type

Test 5 needs an operator that always fails. Define it right in `mod tests`:

```rust
struct Broken;   // a "unit struct": no fields at all

impl Sensor for Broken {
    fn name(&self) -> &str {
        "broken"
    }

    fn read(&mut self) -> f64 {
        f64::NAN
    }
}
```

Then `Box::new(Broken)` is a `Box<dyn Sensor>` like any other. If a trait
method's parameter goes unused, start its name with `_` (like `_context`) and
the compiler won't complain.
