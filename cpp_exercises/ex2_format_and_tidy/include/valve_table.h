// Lookup of valve metadata by id. Compiles, passes its tests, and violates
// both the formatter and the linter. See EXERCISE.md.
#pragma once

#include <string>
#include <vector>

namespace trel::valves {

typedef int ValveId;

struct ValveInfo
{
    ValveId id;
    std::string name;
    bool normally_open;
};

class ValveTable {
 public:
  ValveTable();
  const ValveInfo* FindByName(std::string name) const;
  int CountNormallyOpen() const;
 private:
  std::vector<ValveInfo> entries;
};

}  // namespace trel::valves
