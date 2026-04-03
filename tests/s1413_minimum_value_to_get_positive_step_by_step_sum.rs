// Tests for Problem 1413: Minimum Value to Get Positive Step by Step Sum
// Java reference: src/test/java/g1401_1500/s1413_minimum_value_to_get_positive_step_by_step_sum/SolutionTest.java

use leetcode_in_rust::s1413::minimum_value_to_get_positive_step_by_step_sum::Solution;

#[test]
fn test_min_start_value() {
    assert_eq!(Solution::min_start_value(vec![-3, 2, -3, 4, 2]), 5);
}

#[test]
fn test_min_start_value2() {
    assert_eq!(Solution::min_start_value(vec![1, 2]), 1);
}

#[test]
fn test_min_start_value3() {
    assert_eq!(Solution::min_start_value(vec![1, -2, -3]), 5);
}
