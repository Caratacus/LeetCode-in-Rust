// Tests for Problem 1124: Longest Well-Performing Interval
// Java reference: src/test/java/g1101_1200/s1124_longest_well_performing_interval/SolutionTest.java

use leetcode_in_rust::s1124::longest_well_performing_interval::Solution;

#[test]
fn test_longest_wpi() {
    assert_eq!(Solution::longest_wpi(vec![9, 9, 6, 0, 6, 6, 9]), 3);
}

#[test]
fn test_longest_wpi2() {
    assert_eq!(Solution::longest_wpi(vec![6, 6, 6]), 0);
}
