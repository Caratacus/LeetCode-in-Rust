// Tests for Problem 0539: Minimum Time Difference
// Java reference: src/test/java/g0501_0600/s0539_minimum_time_difference/SolutionTest.java

use leetcode_in_rust::s0539::minimum_time_difference::Solution;

#[test]
fn test_find_min_difference() {
    assert_eq!(Solution::find_min_difference(vec!["23:59".to_string(), "00:00".to_string()]), 1);
}

#[test]
fn test_find_min_difference2() {
    assert_eq!(
        Solution::find_min_difference(vec!["00:00".to_string(), "23:59".to_string(), "00:00".to_string()]),
        0
    );
}
