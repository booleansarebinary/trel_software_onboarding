# Hints: C++ Exercise 2

This file explains each clang-tidy finding: what the fix looks like and why it
matters. The examples use different names from the exercise so you still do the
edits yourself. If something still doesn't click, please ask one of us.

- [Reading a clang-tidy message](#reading-a-clang-tidy-message)
- [`typedef` → `using`](#typedef--using)
- [Naming rules](#naming-rules)
- [`NULL` → `nullptr`](#null--nullptr)
- [Braces on every `if`](#braces-on-every-if)
- [Passing by `const std::string&`](#passing-by-const-stdstring)
- [Range-based `for`](#range-based-for)
- [When a rename breaks the tests](#when-a-rename-breaks-the-tests)

---

## Reading a clang-tidy message

```text
valve_table.cc:17:12: error: use nullptr [modernize-use-nullptr,-warnings-as-errors]
```

That's file, line, column, then what's wrong, then the **check name** in square
brackets. Search the check name to find its documentation, which always has a
before and after example. `-warnings-as-errors` just means our config turns
every warning into an error.

The linter checks the header too, so some findings will point at
`include/valve_table.h`.

## `typedef` → `using`

Both give a type a second name. `using` reads left to right, like assigning a
variable:

```cpp
typedef double Kelvin;    // old C style
using Kelvin = double;    // modern, same meaning
```

## Naming rules

The rules live in `.clang-tidy` at the repo root:

| Kind of name | Style | Example |
| --- | --- | --- |
| Functions and methods | `lower_case` | `find_by_id`, `total_mass` |
| Variables | `lower_case` | `open_count` |
| Classes and structs | `CamelCase` | `ValveTable` |
| Private members | `lower_case` with a trailing `_` | `readings_` |
| Constants | `UPPER_CASE` | `KPA_PER_PSI` |

The trailing `_` on private members lets you tell a member from a local variable
at a glance, which helps in long methods.

Heads up: **there are three functions/methods to rename, not two.**
`CountNormallyOpen` breaks the function rule too.

## `NULL` → `nullptr`

`NULL` is inherited from C and is secretly just the number `0`, so the compiler
can mix it up with an integer. `nullptr` can only ever mean "a pointer to
nothing." Swap one for the other.

## Braces on every `if`

```cpp
if (pressure > limit)
    vent();              // looks fine...

if (pressure > limit)
    vent();
    log_event();         // ...but this line ALWAYS runs. The indent lies.
```

Adding a second line to a braceless `if` is an easy mistake, and it has shipped
in real security code. Always use braces, even for one line:

```cpp
if (pressure > limit) {
    vent();
}
```

## Passing by `const std::string&`

```cpp
bool matches(std::string name);          // copies the whole string on every call
bool matches(const std::string& name);   // borrows it: no copy, and can't change it
```

The `&` means "reference" (a borrow, like `&` in Rust) and `const` means
"read-only." Update the declaration in the `.h` **and** the definition in the
`.cc`: they have to match.

## Range-based `for`

The exercise mentions this even though clang-tidy won't flag it. Indexing with a
counter:

```cpp
for (unsigned long i = 0; i < sensors.size(); i++) {
    check(sensors[i]);
}
```

reads more clearly as "for each item":

```cpp
for (const auto& sensor : sensors) {
    check(sensor);
}
```

`auto` lets the compiler work out the type, and `const auto&` borrows each
item read-only instead of copying it.

**Watch out if you return a pointer to the item.** `&sensor` is only the address
of the real element when the loop variable is a reference (`const auto&`). With
plain `auto`, `sensor` is a temporary copy, and its address points at nothing the
moment the loop moves on. The tests would probably still pass, which is exactly
what makes this bug nasty.

## When a rename breaks the tests

Renaming a method means every caller has to change too, including
`tests/valve_table_test.cc`. When the test build fails with "no member named
`FindByName`," that's expected. Go update the tests.

Checking you're done:

```bash
./dev_scripts/format.sh --check cpp_exercises/ex2_format_and_tidy
./dev_scripts/lint_cpp.sh cpp_exercises/ex2_format_and_tidy/src/valve_table.cc
bazel test //cpp_exercises/ex2_format_and_tidy:tests
```
