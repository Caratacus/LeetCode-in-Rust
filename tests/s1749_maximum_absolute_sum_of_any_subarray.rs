// Tests for Problem 1749: Maximum Absolute Sum of Any Subarray
// Java reference: src/test/java/g1701_1800/s1749_maximum_absolute_sum_of_any_subarray/SolutionTest.java

use leetcode_in_rust::s1749::maximum_absolute_sum_of_any_subarray::Solution;

#[test]
fn test_max_absolute_sum() {
    assert_eq!(Solution::max_absolute_sum(vec![1, -3, 2, 3, -4]), 5);
}

#[test]
fn test_max_absolute_sum2() {
    assert_eq!(Solution::max_absolute_sum(vec![2, -5, 1, -4, 3, -2]), 8);
}
