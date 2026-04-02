// Tests for Problem 0560: Subarray Sum Equals K
// Java reference: src/test/java/g0501_0600/s0560_subarray_sum_equals_k/SolutionTest.java

use leetcode_in_rust::s0560::subarray_sum_equals_k::Solution;

#[test]
fn test_subarray_sum() {
    assert_eq!(Solution::subarray_sum(vec![1, 1, 1], 2), 2);
}

#[test]
fn test_subarray_sum2() {
    assert_eq!(Solution::subarray_sum(vec![1, 2, 3], 3), 2);
}
