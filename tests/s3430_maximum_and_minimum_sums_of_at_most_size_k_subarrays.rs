// Tests for Problem 3430: Maximum and Minimum Sums of at Most Size K Subarrays
// Java reference: src/test/java/g3401_3500/s3430_maximum_and_minimum_sums_of_at_most_size_k_subarrays/SolutionTest.java

use leetcode_in_rust::s3430::maximum_and_minimum_sums_of_at_most_size_k_subarrays::Solution;

#[test]
fn test_min_max_subarray_sum() {
    assert_eq!(Solution::min_max_subarray_sum(vec![1, 2, 3], 2), 20i64);
}

#[test]
fn test_min_max_subarray_sum2() {
    assert_eq!(Solution::min_max_subarray_sum(vec![1, -3, 1], 2), -6i64);
}
