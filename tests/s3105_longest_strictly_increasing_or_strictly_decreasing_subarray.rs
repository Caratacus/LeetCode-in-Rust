// Tests for Problem 3105: Longest Strictly Increasing or Strictly Decreasing Subarray
// Java reference: src/test/java/g3101_3200/s3105_longest_strictly_increasing_or_strictly_decreasing_subarray/SolutionTest.java

use leetcode_in_rust::s3105::longest_strictly_increasing_or_strictly_decreasing_subarray::Solution;

#[test]
fn test_longest_monotonic_subarray() {
    assert_eq!(Solution::longest_monotonic_subarray(vec![1, 4, 3, 3, 2]), 2);
}

#[test]
fn test_longest_monotonic_subarray2() {
    assert_eq!(Solution::longest_monotonic_subarray(vec![3, 3, 3, 3]), 1);
}
