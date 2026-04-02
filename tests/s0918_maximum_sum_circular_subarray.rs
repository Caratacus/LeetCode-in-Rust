// Tests for Problem 0918: Maximum Sum Circular Subarray
// Java reference: src/test/java/g0901_1000/s0918_maximum_sum_circular_subarray/SolutionTest.java

use leetcode_in_rust::s0918::maximum_sum_circular_subarray::Solution;

#[test]
fn test_max_subarray_sum_circular() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![1, -2, 3, -2]), 3);
}

#[test]
fn test_max_subarray_sum_circular2() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![5, -3, 5]), 10);
}

#[test]
fn test_max_subarray_sum_circular3() {
    assert_eq!(Solution::max_subarray_sum_circular(vec![-3, -2, -3]), -2);
}
