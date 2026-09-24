#include "valve_table.h"

#include <gtest/gtest.h>

namespace trel::valves {
namespace {

TEST(ValveTable, FindByNameReturnsTheMatchingEntry) {
    const ValveTable table;

    const ValveInfo* found = table.FindByName("ox_vent");

    ASSERT_NE(found, nullptr);
    EXPECT_EQ(found->id, 4);
    EXPECT_TRUE(found->normally_open);
}

TEST(ValveTable, FindByNameReturnsNullForAnUnknownName) {
    const ValveTable table;

    const ValveInfo* found = table.FindByName("not_a_valve");

    EXPECT_EQ(found, nullptr);
}

TEST(ValveTable, CountNormallyOpenCountsOnlyNormallyOpenValves) {
    const ValveTable table;

    const int count = table.CountNormallyOpen();

    EXPECT_EQ(count, 2);
}

}  // namespace
}  // namespace trel::valves
