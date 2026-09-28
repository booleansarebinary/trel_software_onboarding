#include "valve_table.h"

namespace trel::valves {

ValveTable::ValveTable() {
    entries_.push_back(ValveInfo{1, "fuel_main", false});
    entries_.push_back(ValveInfo{2, "ox_main", false});
    entries_.push_back(ValveInfo{3, "fuel_vent", true});
    entries_.push_back(ValveInfo{4, "ox_vent", true});
}

const ValveInfo* ValveTable::find_by_name(const std::string& name) const {
    for (unsigned long i = 0; i < entries_.size(); i++) {
        if (entries_[i].name == name) {
            return &entries_[i];
        }
    }
    return nullptr;
}

int ValveTable::count_normally_open() const {
    int count = 0;
    for (const ValveInfo& info : entries_) {
        if (info.normally_open) {
            count = count + 1;
        } else {
            continue;
        }
    }
    return count;
}

}  // namespace trel::valves
