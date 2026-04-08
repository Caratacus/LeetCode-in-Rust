// Tests for Problem 3097: Shortest Subarray With OR at Least K II
// Java reference: src/test/java/g3001_3100/s3097_shortest_subarray_with_or_at_least_k_ii/SolutionTest.java

use leetcode_in_rust::s3097::shortest_subarray_with_or_at_least_k_ii::Solution;

#[test]
fn test_minimum_subarray_length() {
    assert_eq!(Solution::minimum_subarray_length(vec![1, 2, 3], 2), 1);
}

#[test]
fn test_minimum_subarray_length2() {
    assert_eq!(Solution::minimum_subarray_length(vec![2, 1, 8], 10), 3);
}

#[test]
fn test_minimum_subarray_length3() {
    assert_eq!(Solution::minimum_subarray_length(vec![1, 2], 0), 1);
}
