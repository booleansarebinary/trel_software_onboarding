#include "pressure_units.h"

#include <gtest/gtest.h>

namespace trel::units {
namespace {

// GoogleTest, not rstest. The structure of a good test does not change with
// the language: setup, the call under test, then assertions, separated by
// blank lines.
//
// TEST(<SuiteName>, <WhatShouldHappen>) is the C++ equivalent of our Rust
// naming rule. The suite is the thing under test; the second argument is the
// behavior, phrased so the failure output reads like a sentence.
//
// EXPECT_* continues after a failure and reports every problem it finds.
// ASSERT_* stops the test. Use EXPECT_* unless continuing would crash.
// For floating point use EXPECT_DOUBLE_EQ or EXPECT_NEAR, never EXPECT_EQ.

TEST(PressureUnits, PsiToKpaConvertsAtmosphericPressure) {
    const double atmospheric_psi = 14.6959;

    const double kpa = psi_to_kpa(atmospheric_psi);

    EXPECT_NEAR(kpa, 101.325, 1e-3);
}

TEST(PressureUnits, PsiToKpaMapsZeroToZero) {
    const double zero_psi = 0.0;

    const double kpa = psi_to_kpa(zero_psi);

    EXPECT_DOUBLE_EQ(kpa, 0.0);
}

// TODO(you): the two unimplemented functions have no tests. Add them.
//
//   kpa_to_psi:
//     - converts a known value correctly
//     - round-trips: psi_to_kpa then kpa_to_psi returns the original value
//       (use EXPECT_NEAR; floating point round trips are not exact)
//
//   is_within_tolerance:
//     - accepts a value inside the band
//     - rejects a value outside the band
//     - accepts a value exactly on the boundary, and say in your PR why you
//       chose inclusive or exclusive
//     - rejects a negative tolerance_fraction
//     - expected == 0.0 accepts only exactly 0.0
//
// Implement and test one function at a time. Do not write five tests against
// code that does not compile yet.

}  // namespace
}  // namespace trel::units
