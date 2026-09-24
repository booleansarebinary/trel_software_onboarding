// Pressure unit conversions.
//
// C++ in TREL is a smaller story than Rust. New code goes in Rust; C++ exists
// where it has to - GNC algorithms, vendor SDKs, and code shared with tools
// that only speak C. It lives in its own repository now. You are here so that
// when you land in that repo you already know how the build and the hygiene
// tooling behave.
//
// Three administrative things worth internalizing:
//
// 1. `#pragma once` at the top of every header. We do not write include
//    guards by hand; they get copy-pasted and collide.
// 2. The compiler is Bazel's, not yours. `clang --version` on your laptop is
//    irrelevant; the toolchain in MODULE.bazel is what builds this.
// 3. Formatting and linting are SEPARATE TOOLS here, and unlike Rust they are
//    not wired into `bazel build`. Nothing stops you committing ugly C++
//    except you, your reviewer, and the CI job. See //cpp_exercises/README.md.

#pragma once

namespace trel::units {

// Exact by definition of the pound-force and the inch.
constexpr double KPA_PER_PSI = 6.894757293168361;

// Converts pounds per square inch to kilopascals.
double psi_to_kpa(double psi);

// Converts kilopascals to pounds per square inch.
//
// TODO(you): implement in pressure_units.cc.
double kpa_to_psi(double kpa);

// Returns true if `actual` is within `tolerance_fraction` of `expected`, where
// the tolerance is a fraction of `expected` (0.05 means 5%).
//
// Two decisions the tests must pin down, because the obvious implementation
// gets both wrong:
//
//   - A negative `tolerance_fraction` is nonsense and must return false rather
//     than quietly accepting everything or rejecting everything.
//   - When `expected` is 0.0, a percentage tolerance has no meaning. Require
//     `actual` to be exactly 0.0.
//
// TODO(you): implement in pressure_units.cc.
bool is_within_tolerance(double actual, double expected, double tolerance_fraction);

}  // namespace trel::units
