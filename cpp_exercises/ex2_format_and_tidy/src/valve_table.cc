#include "valve_table.h"

namespace trel::valves {

ValveTable::ValveTable() {
    entries.push_back(ValveInfo{1, "fuel_main", false});
    entries.push_back(ValveInfo{2, "ox_main", false});
    entries.push_back(ValveInfo{3, "fuel_vent", true});
    entries.push_back(ValveInfo{4, "ox_vent", true});
}

const ValveInfo* ValveTable::FindByName(std::string name) const {
    for (unsigned long i = 0; i < entries.size(); i++) {
        if (entries[i].name == name)
            return &entries[i];
    }
    return NULL;
}

int ValveTable::CountNormallyOpen() const {
    int Count = 0;
    for (unsigned long i = 0; i < entries.size(); i++)
    {
        if (entries[i].normally_open) {
            Count = Count + 1;
        }
        else {
            continue;
        }
    }
    return Count;
}

}  // namespace trel::valves
