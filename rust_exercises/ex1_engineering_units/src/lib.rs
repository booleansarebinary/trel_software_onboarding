//! Linear calibration for one analog input channel.
//!
//! Every analog sensor on the stand reports raw ADC counts. Nobody wants to
//! read "37122 counts" on a dashboard during a hotfire, so each channel
//! carries a calibration that turns counts into a real unit (psi, degF, lbf):
//!
//! ```text
//! value = counts * scale + offset
//! ```
//!
//! trel3 has a real, generic version of this type at `//lib_rust/engineering_units`.
//! This is a simplified copy so you have something small to read.
//!
//! # Rust notes for this file
//!
//! - `pub` is per-item. Anything without `pub` is private to this module, and
//!   the compiler will tell you about it rather than letting you guess.
//! - `#[derive(...)]` asks the compiler to write boilerplate trait impls.
//!   `Copy` here means this struct is cheap enough to duplicate on assignment
//!   instead of being moved, which is why the methods below can take `&self`
//!   and callers never have to think about ownership.
//! - Returning `Option<T>` instead of panicking is how Rust says "this can
//!   legitimately have no answer." The caller is forced to handle it.

/// Calibration and most recent reading for a single analog channel.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct EngineeringUnit {
    scale: f64,
    offset: f64,
    value: f64,
}

/// The raw range of the 16-bit ADCs and DACs on our hardware.
const MIN_COUNTS: f64 = 0.0;
const MAX_COUNTS: f64 = u16::MAX as f64;

impl EngineeringUnit {
    /// Creates a channel calibration. `scale` is engineering units per count.
    pub fn new(scale: f64, offset: f64) -> Self {
        Self {
            scale,
            offset,
            value: 0.0,
        }
    }

    /// Builder-style setter, so call sites read as one expression:
    /// `EngineeringUnit::new(0.1, -10.0).with_value(250.0)`.
    ///
    /// Takes `self` by value and returns it. This is a common Rust idiom and
    /// you will see it all over trel3.
    pub fn with_value(mut self, value: f64) -> Self {
        self.value = value;
        self
    }

    /// The most recent value in engineering units.
    pub fn value(&self) -> f64 {
        self.value
    }

    /// Converts a raw ADC reading into engineering units.
    pub fn from_counts(&self, counts: u16) -> f64 {
        f64::from(counts) * self.scale + self.offset
    }

    /// Converts an engineering value back into raw counts for a DAC.
    ///
    /// Returns `None` when this channel has no usable calibration, because
    /// inverting a zero scale is a division by zero, and silently handing the
    /// hardware a garbage setpoint is how valves end up in the wrong state.
    ///
    /// Values outside the DAC's range saturate rather than wrap. A wrapped
    /// setpoint would turn "slightly too high" into "fully closed".
    pub fn to_counts(&self, value: f64) -> Option<u16> {
        if self.scale == 0.0 {
            return None;
        }

        let raw = (value - self.offset) / self.scale;

        if raw < MIN_COUNTS {
            return Some(0);
        }
        if raw > MAX_COUNTS {
            return Some(u16::MAX);
        }

        Some(raw.round() as u16)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    /// Floating point equality is a trap. Compare within a tolerance.
    const EPSILON: f64 = 1e-9;

    // ------------------------------------------------------------------
    // READ THESE TWO TESTS CAREFULLY. They are the house style.
    //
    // Note four things:
    //
    // 1. `#[rstest]`, never `#[test]`. Every Rust test in TREL uses rstest.
    // 2. The name reads as a sentence:
    //        test_<function>_<expected behavior>[_when_<condition>]
    //    A reviewer should know what broke from the failure name alone,
    //    without opening the file.
    // 3. Three blocks separated by ONE blank line: setup, the single call
    //    under test, then assertions. This is enforced in review.
    // 4. `#[case(...)]` adds test cases without duplicating the test body.
    //    Each case is reported as its own test, so a failure tells you which
    //    input broke.
    // ------------------------------------------------------------------

    #[rstest]
    #[case(0, -10.0)] // bottom of the ADC range
    #[case(100, 0.0)] // the calibration's zero crossing
    #[case(65535, 6543.5)] // top of the ADC range
    fn test_from_counts_applies_scale_and_offset(#[case] counts: u16, #[case] expected: f64) {
        let channel = EngineeringUnit::new(0.1, -10.0);

        let value = channel.from_counts(counts);

        assert!(
            (value - expected).abs() < EPSILON,
            "expected {expected}, got {value}"
        );
    }

    #[rstest]
    fn test_with_value_sets_the_reported_value() {
        let channel = EngineeringUnit::new(0.1, -10.0);

        let channel = channel.with_value(250.0);

        assert!((channel.value() - 250.0).abs() < EPSILON);
    }

    // ------------------------------------------------------------------
    // TODO(you): `to_counts` has four distinct code paths and zero tests.
    //
    // Write one test per path. Follow the naming and layout above. Do not
    // write one giant test with four assertion groups in it - when that test
    // fails, the failure tells your reviewer nothing.
    //
    //   1. A value inside the DAC range converts back to the right counts.
    //      Use `#[case]` for at least three values, including a value that
    //      has to round.
    //   2. A value below the range saturates to 0 (not to a wrapped number).
    //   3. A value above the range saturates to u16::MAX.
    //   4. An uncalibrated channel (scale == 0.0) returns None.
    //
    // Sanity check on your own work before you open the PR: comment out the
    // `if self.scale == 0.0` guard in `to_counts`. If every test still
    // passes, your tests are decoration, not verification. Put the guard
    // back afterward.
    // ------------------------------------------------------------------
}
