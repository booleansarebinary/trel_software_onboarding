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

TEST(PressureUnits, KpaToPsiConverts) {
    const double kpa = 101.325;

    const double psi = kpa_to_psi(kpa);

    EXPECT_NEAR(psi, 14.6959, 1e-3);
}

TEST(PressureUnits, RoundTripConversion) {
    double atmospheric_psi = 14.6959;

    const double kpa = psi_to_kpa(atmospheric_psi);

    atmospheric_psi = kpa_to_psi(kpa);

    EXPECT_NEAR(atmospheric_psi, 14.6959, 1e-3);
}

TEST(PressureUnits, WithinTolerance) {
    double actual_within = 99.0;

    double expected = 100.0;

    double tolerance = 0.05;

    bool result = is_within_tolerance(actual_within, expected, tolerance);

    EXPECT_TRUE(result);
}

TEST(PressureUnits, OutsideBand) {
    double actual_within = 94.5;

    double expected = 100.0;

    double tolerance = 0.05;

    bool result = is_within_tolerance(actual_within, expected, tolerance);

    EXPECT_FALSE(result);
}

TEST(PressureUnits, OnTolerance) {
    double actual_within = 105;

    double expected = 100.0;

    double tolerance = 0.05;

    bool result = is_within_tolerance(actual_within, expected, tolerance);

    EXPECT_TRUE(result);
}

TEST(PressureUnits, NegativeToleranceRejected) {
    double actual_within = 99.0;

    double expected = 100.0;

    double tolerance = -0.05;

    bool result = is_within_tolerance(actual_within, expected, tolerance);

    EXPECT_FALSE(result);
}

TEST(PressureUnits, ZeroExpectedReject) {
    double actual_within = 0.001;

    double expected = 0.0;

    double tolerance = 0.05;

    bool result = is_within_tolerance(actual_within, expected, tolerance);

    EXPECT_FALSE(result);
}

TEST(PressureUnits, ZeroExpectedAccept) {
    double actual_within = 0.0;

    double expected = 0.0;

    double tolerance = 0.05;

    bool result = is_within_tolerance(actual_within, expected, tolerance);

    EXPECT_TRUE(result);
}

}  // namespace
}  // namespace trel::units
