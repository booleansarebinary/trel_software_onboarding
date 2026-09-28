#include "pressure_units.h"

namespace trel::units {

double psi_to_kpa(double psi) {
    return psi * KPA_PER_PSI;
}

double kpa_to_psi(double kpa) {
    return kpa / KPA_PER_PSI;
}

bool is_within_tolerance(double actual, double expected, double tolerance_fraction) {
    if (tolerance_fraction < 0) {
        return false;
    }

    if (expected == 0.0) {
        if (actual == 0.0) {
            return true;
        }
        return false;
    }
    double pct_error = (actual - expected) / expected;
    if (pct_error < 0) {
        pct_error *= -1;
    }

    if (pct_error <= tolerance_fraction) {
        return true;
    }
    return false;
}

}  // namespace trel::units
