// Tests for Problem 0829: Consecutive Numbers Sum
// Java reference: src/test/java/g0801_0900/s0829_consecutive_numbers_sum/SolutionTest.java

use leetcode_in_rust::s0829::consecutive_numbers_sum::Solution;

#[test]
fn test_consecutive_numbers_sum() {
    assert_eq!(Solution::consecutive_numbers_sum(9), 27);
}

#[test]
fn test_consecutive_numbers_sum2() {
    assert_eq!(Solution::consecutive_numbers_sum(15), 80);
}
