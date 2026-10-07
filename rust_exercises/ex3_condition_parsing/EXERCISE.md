# Rust Exercise 3: Parse a condition

**About 20 minutes. Optional extra practice** - do exercises 1 and 2 first.

Aborts and sequences are written as configuration, not code. Someone types
`ai_12 > 450.0` into a config file, and the ground software turns that text into
a `Condition` it can check every cycle. If there's a typo, we want a clear error
when the config loads, not a surprise during a test.

Most of the parser is already written. You'll write the piece that reads one
operand, like `ai_12` or `450.0`. You've used regexes before, so this is mostly
about what Rust gives you instead: `strip_prefix`, `.parse()`, and error enums.

[HINTS.md](HINTS.md) explains each piece, including a "regex → Rust" table.

## 1. Read the code.

```bash
bazel test //rust_exercises/ex3_condition_parsing:tests
```

One test fails inside `todo!()`. Read `src/lib.rs` top to bottom first. The
`FromStr` code for `Comparison` and for `Condition` is finished. Read both:
`Condition`'s is the code that calls yours.

## 2. Implement `parse_operand`.

Its doc comment says what to accept and reject. Watch for the trap it mentions:
`"NaN".parse::<f64>()` succeeds.

## 3. Write the tests.

The `TODO(you)` block at the bottom of the file lists them. Then do the sabotage
check it describes.

## 4. Open the PR.

```bash
bazel test //rust_exercises/ex3_condition_parsing:tests
```

## Done when

- All tests pass, including your three new ones.
- `"NaN"` and `"inf"` are rejected, and a test proves it.
- Every test uses `#[rstest]`, follows the naming rule, and has setup / call /
  assertions separated by blank lines.
- No `.unwrap()` in `parse_operand`. Errors go back to the caller.
- The `TODO(you)` comments and the `let _ = ...` line are gone.

## Why this one matters

Every abort and sequence in the real system starts life as config text. The
parser is the first line of defense: anything it lets through runs during a
hotfire.
