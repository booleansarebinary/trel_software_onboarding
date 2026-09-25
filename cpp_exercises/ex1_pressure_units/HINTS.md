# Hints: C++ Exercise 1

This file explains the C++ you'll run into in this exercise. It explains the
tools, not the answer, so the examples use different problems on purpose. If
something still doesn't click, please ask one of us.

- [Header (`.h`) vs. source (`.cc`)](#header-h-vs-source-cc)
- [`namespace` and `::`](#namespace-and-)
- [`constexpr` and `const`](#constexpr-and-const)
- [`static_cast<void>(x)`](#static_castvoidx)
- [Absolute value: `std::abs`](#absolute-value-stdabs)
- [Returning a `bool` directly](#returning-a-bool-directly)
- [`<` vs. `<=`](#-vs-)
- [Comparing a `double` to `0.0`](#comparing-a-double-to-00)
- [GoogleTest: `TEST`, `EXPECT_*`, `ASSERT_*`](#googletest-test-expect_-assert_)

---

## Header (`.h`) vs. source (`.cc`)

C++ splits each piece of code in two:

- The **header** (`include/pressure_units.h`) says *what exists*: function names,
  parameter types, and return types. Other files `#include` it to call your code,
  and it's the only part your tests can see.
- The **source file** (`src/pressure_units.cc`) holds *how it works*: the
  function bodies.

The signature in the `.cc` has to match the header exactly. If you change one,
change the other. You shouldn't need to touch the header at all in this
exercise.

## `namespace` and `::`

A namespace is a named box that stops names from colliding. Everything here
lives in `trel::units`, so the full name of the function is
`trel::units::kpa_to_psi`. Inside the same namespace you can just say
`kpa_to_psi`.

`::` means "look inside": `std::abs` is `abs` from the standard library's `std`
namespace.

## `constexpr` and `const`

- `const double x = 5.0;` - `x` can't be changed after this line.
- `constexpr double X = 5.0;` - same, and also fixed at compile time. Used for
  true constants like `KPA_PER_PSI`.

C++ is the opposite of Rust here: everything is changeable **unless** you write
`const`. We add `const` wherever we can, which is why the tests say
`const double kpa = ...`.

## `static_cast<void>(x)`

This is the C++ version of Rust's `let _ = x;`. It means "I know this parameter
is unused, stop warning me." The stubs have it only so they compile while
unfinished. **Delete those lines** once your function actually uses its
parameters.

## Absolute value: `std::abs`

The distance between two numbers, ignoring which is bigger, is the absolute
value of their difference:

```cpp
#include <cmath>

const double gap = std::abs(measured - target);   // always >= 0
```

Two things to watch:

- Add `#include <cmath>` at the top of the `.cc` file. Without it, you may pick
  up an integer-only `abs` that silently chops off the decimals.
- A percentage of a **negative** number is negative. If you ever multiply a
  tolerance by a value that might be negative, think about whether you need
  `std::abs` on that too.

## Returning a `bool` directly

A comparison is already a `bool`, so you can return it without an `if`:

```cpp
// Longer than it needs to be:
if (temp > limit) {
    return true;
}
return false;

// Same thing:
return temp > limit;
```

Early `return false;` for the edge cases at the top, then one `return` with the
real comparison at the bottom, is a clean shape for this kind of function.

## `<` vs. `<=`

`<` excludes the boundary and `<=` includes it. For `is_within_tolerance`, a
value *exactly* on the edge of the band is either in or out depending on which
you pick. Either is fine. The exercise asks you to choose on purpose and say why
in your PR.

## Comparing a `double` to `0.0`

Usually you should never compare floats with `==`, because `0.1 + 0.2 == 0.3` is
false. Checking whether a value is **exactly** `0.0` is the reasonable exception,
since the header asks for exactly that and you're not doing arithmetic first.

## GoogleTest: `TEST`, `EXPECT_*`, `ASSERT_*`

```cpp
TEST(SuiteName, WhatShouldHappen) {
    const double input = 2.0;

    const double result = square(input);

    EXPECT_DOUBLE_EQ(result, 4.0);
}
```

The assertions you'll want:

| Macro | Passes when |
| --- | --- |
| `EXPECT_TRUE(x)` / `EXPECT_FALSE(x)` | `x` is true / false. Good for `is_within_tolerance`. |
| `EXPECT_DOUBLE_EQ(a, b)` | `a` and `b` are equal to within a few tiny rounding steps. |
| `EXPECT_NEAR(a, b, tol)` | `a` and `b` are within `tol` of each other. Use this when you pick the tolerance yourself, like after a round trip. |
| `EXPECT_EQ(a, b)` | exactly equal. Fine for integers and `bool`s. **Never for `double`s.** |

`EXPECT_*` records a failure and keeps going. `ASSERT_*` stops the test on the
spot. Use `ASSERT_*` only when carrying on would crash, like using a pointer you
just checked for null.

A **round trip** test converts one way and then back, and checks you end up
where you started:

```cpp
const double back = celsius_from_f(f_from_celsius(original));
EXPECT_NEAR(back, original, 1e-9);
```

Running just this exercise, with failures printed:

```bash
bazel test //cpp_exercises/ex1_pressure_units:tests --test_output=errors
```
