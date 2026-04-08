// Tests for Problem 3107: Minimum Operations to Make Median of Array Equal to K
// Java reference: src/test/java/g3101_3200/s3107_minimum_operations_to_make_median_of_array_equal_to_k/SolutionTest.java

use leetcode_in_rust::s3107::minimum_operations_to_make_median_of_array_equal_to_k::Solution;

#[test]
fn test_min_operations_to_make_median_k() {
    assert_eq!(Solution::min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 4), 2);
}

#[test]
fn test_min_operations_to_make_median_k2() {
    assert_eq!(Solution::min_operations_to_make_median_k(vec![2, 5, 6, 8, 5], 7), 3);
}

#[test]
fn test_min_operations_to_make_median_k3() {
    assert_eq!(Solution::min_operations_to_make_median_k(vec![1, 2, 3, 4, 5, 6], 4), 0);
}
