//! Redline checking: deciding whether a sensor reading is fine, worrying, or
//! grounds for shutting down.
//!
//! You implement this one. The signatures and the docs are the spec; the
//! bodies are yours.
//!
//! # The Rust you need for this exercise
//!
//! **Enums are not integers.** A Rust enum is a closed set of alternatives,
//! and `match` on one must handle every alternative or the code will not
//! compile. That compile error is a feature: add a new severity later and the
//! compiler hands you the list of places that need updating.
//!
//! **`Result<T, E>` is how fallible operations report failure.** There are no
//! exceptions in Rust. A function that can fail says so in its return type,
//! and callers cannot ignore it without the compiler complaining.
//!
//! **`&[f64]` is a borrowed slice.** It is a view into someone else's array or
//! `Vec`, not a copy. Take slices in function parameters instead of `Vec<f64>`
//! so callers are not forced to hand over ownership of their data.
//!
//! **Iterators over loops.** `readings.iter().map(...).max()` is the idiomatic
//! form and clippy will nudge you toward it. Reach for it when it is clearer,
//! not as a matter of religion.

/// How bad a reading is.
///
/// `Ord` is derived, and the variants are declared least-to-most severe, so
/// `Severity::Critical > Severity::Nominal` and `.max()` does what you want.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Severity {
    Nominal,
    Warning,
    Critical,
}

/// Why a set of limits was rejected.
///
/// A real error type, not a `String`. Callers can `match` on the variant and
/// react differently per failure; a string forces them to parse prose.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LimitError {
    /// `warning_high` was not below `critical_high`, so the warning band is
    /// empty or inverted and a reading could never be merely "warning".
    WarningNotBelowCritical,
    /// One of the thresholds was NaN. NaN compares false against everything,
    /// so a NaN threshold silently disables the redline. That must not be
    /// possible to configure by accident.
    ThresholdNotANumber,
}

/// An upper-redline pair for one channel.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SensorLimits {
    warning_high: f64,
    critical_high: f64,
}

impl SensorLimits {
    /// Validates and builds a limit pair.
    ///
    /// # Errors
    ///
    /// - [`LimitError::ThresholdNotANumber`] if either threshold is NaN.
    /// - [`LimitError::WarningNotBelowCritical`] if `warning_high` is not
    ///   strictly less than `critical_high`.
    ///
    /// Check for NaN first. A NaN input would also fail the ordering check,
    /// and the more specific error is the more useful one.
    pub fn new(warning_high: f64, critical_high: f64) -> Result<Self, LimitError> {
        // TODO(you): implement.
        let _ = (warning_high, critical_high);
        todo!("ex2: implement SensorLimits::new")
    }

    /// Classifies a single reading.
    ///
    /// - at or above `critical_high` -> [`Severity::Critical`]
    /// - at or above `warning_high` (but below critical) -> [`Severity::Warning`]
    /// - otherwise -> [`Severity::Nominal`]
    ///
    /// Note that the thresholds are inclusive. Exactly hitting the redline is
    /// a redline; "close enough" is not a thing in fluid systems.
    pub fn classify(&self, reading: f64) -> Severity {
        // TODO(you): implement.
        let _ = reading;
        todo!("ex2: implement SensorLimits::classify")
    }

    /// Classifies a whole batch of readings and returns the worst severity
    /// found.
    ///
    /// An empty batch is [`Severity::Nominal`]: no readings means nothing has
    /// gone wrong yet. Think about whether you agree with that choice before
    /// you implement it, and say so in your PR if you do not. Spec bugs found
    /// during implementation are the cheapest bugs there are.
    pub fn worst_of(&self, readings: &[f64]) -> Severity {
        // TODO(you): implement.
        let _ = readings;
        todo!("ex2: implement SensorLimits::worst_of")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    /// One test to get you started, and to make the exercise fail loudly until
    /// you have done it. Run `bazel test //rust_exercises/ex2_sensor_limits:tests`
    /// right now: it panics inside `todo!()`. That is the starting line.
    #[rstest]
    fn test_new_accepts_a_warning_threshold_below_the_critical_threshold() {
        let limits = SensorLimits::new(400.0, 500.0);

        assert!(limits.is_ok());
    }

    // TODO(you): write the rest of the tests.
    //
    // Everything in this module must be private. No `pub fn` in a tests
    // module - it is not an API, and CI checks for it.
    //
    // Start by listing the code paths, then write one test per path. You
    // should end up with something close to:
    //
    //   new:
    //     - accepts a valid pair
    //     - rejects warning >= critical  (test both == and >)
    //     - rejects NaN in either position
    //   classify:
    //     - nominal / warning / critical bands
    //     - the exact boundary values, since inclusivity is the easiest thing
    //       in this file to get wrong
    //   worst_of:
    //     - returns the worst of a mixed batch
    //     - order of the readings does not change the answer
    //     - empty slice
    //
    // Two rstest features to use here:
    //
    //   #[case(...)] - one explicit case per attribute, when you care about
    //   specific input/expected pairs. Use this for the boundary tests.
    //
    //   #[values(...)] - runs the test once per value, and once per
    //   combination when several parameters use it. Handy for "any of these
    //   NaN positions is rejected". Do not stack three of them and call it
    //   thorough: 5 x 5 x 5 cases that all take the same code path is a slow
    //   test suite pretending to be a good one. trel3's CI flags this.
    //
    // Reminder on the required shape of every test:
    //
    //     <setup>
    //
    //     <the one call under test>
    //
    //     <assertions>
    //
    // `new` returns a Result, so unwrap it in setup with a message that says
    // what you expected:
    //
    //     let limits = SensorLimits::new(400.0, 500.0).expect("valid limits");
}
