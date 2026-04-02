// Tests for Problem 0327: Count of Range Sum
// Java reference: src/test/java/g0301_0400/s0327_count_of_range_sum/SolutionTest.java

use leetcode_in_rust::s0327::count_of_range_sum::Solution;

#[test]
fn test_count_range_sum() {
    assert_eq!(Solution::count_range_sum(vec![-2, 5, -1], -2, 2), 3);
}

#[test]
fn test_count_range_sum2() {
    assert_eq!(Solution::count_range_sum(vec![0], 0, 0), 1);
}
