// Tests for Problem 2335: Minimum Amount of Time to Fill Cups
// Java reference: src/test/java/g2301_2400/s2335_minimum_amount_of_time_to_fill_cups/SolutionTest.java

use leetcode_in_rust::s2335::minimum_amount_of_time_to_fill_cups::Solution;

#[test]
fn test_fill_cups() {
    assert_eq!(Solution::fill_cups(vec![1, 4, 2]), 4);
}

#[test]
fn test_fill_cups2() {
    assert_eq!(Solution::fill_cups(vec![5, 4, 4]), 7);
}

#[test]
fn test_fill_cups3() {
    assert_eq!(Solution::fill_cups(vec![5, 0, 0]), 5);
}
