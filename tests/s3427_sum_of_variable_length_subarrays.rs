// Tests for Problem 3427: Sum of Variable Length Subarrays
// Java reference: src/test/java/g3401_3500/s3427_sum_of_variable_length_subarrays/SolutionTest.java

use leetcode_in_rust::s3427::sum_of_variable_length_subarrays::Solution;

#[test]
fn test_subarray_sum() {
    assert_eq!(Solution::subarray_sum(vec![2, 3, 1]), 11);
}

#[test]
fn test_subarray_sum2() {
    assert_eq!(Solution::subarray_sum(vec![3, 1, 1, 2]), 13);
}
