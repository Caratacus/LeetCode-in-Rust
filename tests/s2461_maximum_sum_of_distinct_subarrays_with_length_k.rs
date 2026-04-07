// Tests for Problem 2461: Maximum Sum of Distinct Subarrays with Length K
// Java reference: src/test/java/g2401_2500/s2461_maximum_sum_of_distinct_subarrays_with_length_k/SolutionTest.java

use leetcode_in_rust::s2461::maximum_sum_of_distinct_subarrays_with_length_k::Solution;

#[test]
fn test_maximum_subarray_sum() {
    assert_eq!(Solution::maximum_subarray_sum(vec![1, 5, 4, 2, 9, 9, 9], 3), 15);
}

#[test]
fn test_maximum_subarray_sum2() {
    assert_eq!(Solution::maximum_subarray_sum(vec![4, 4, 4], 3), 0);
}
