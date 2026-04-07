// Tests for Problem 2543: Check If Point Is Reachable
// Java reference: src/test/java/g2501_2600/s2543_check_if_point_is_reachable/SolutionTest.java
use leetcode_in_rust::s2543::check_if_point_is_reachable::Solution;

#[test]
fn test_is_reachable() {
    assert_eq!(Solution::is_reachable(6, 9), false);
}
#[test]
fn test_is_reachable2() {
    assert_eq!(Solution::is_reachable(4, 7), true);
}
