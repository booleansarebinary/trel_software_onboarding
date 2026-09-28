// Lookup of valve metadata by id. Compiles, passes its tests, and violates
// both the formatter and the linter. See EXERCISE.md.
#pragma once

#include <string>
#include <vector>

namespace trel::valves {

using ValveId = int;

struct ValveInfo {
    ValveId id;
    std::string name;
    bool normally_open;
};

class ValveTable {
  public:
    ValveTable();
    const ValveInfo* find_by_name(const std::string& name) const;
    int count_normally_open() const;

  private:
    std::vector<ValveInfo> entries_;
};

}  // namespace trel::valves
