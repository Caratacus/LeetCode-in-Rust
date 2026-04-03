// Tests for Problem 0869: Reordered Power of 2
// Java reference: src/test/java/g0801_0900/s0869_reordered_power_of_2/SolutionTest.java

use leetcode_in_rust::s0869::reordered_power_of_2::Solution;

#[test]
fn test_reordered_power_of2() {
    assert_eq!(Solution::reordered_power_of2(1), true);
}

#[test]
fn test_reordered_power_of22() {
    assert_eq!(Solution::reordered_power_of2(10), false);
}
