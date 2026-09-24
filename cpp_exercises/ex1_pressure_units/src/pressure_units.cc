#include "pressure_units.h"

namespace trel::units {

double psi_to_kpa(double psi) {
    return psi * KPA_PER_PSI;
}

double kpa_to_psi(double kpa) {
    // TODO(you): implement. Do not hardcode a second conversion constant -
    // two constants that must agree are one constant and one future bug.
    static_cast<void>(kpa);
    return 0.0;
}

bool is_within_tolerance(double actual, double expected, double tolerance_fraction) {
    // TODO(you): implement, including the two edge cases in the header.
    static_cast<void>(actual);
    static_cast<void>(expected);
    static_cast<void>(tolerance_fraction);
    return false;
}

}  // namespace trel::units
